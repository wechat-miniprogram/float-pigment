//! String utilities for the binary format.

use alloc::{boxed::Box, rc::Rc, string::String, vec::Vec};
use core::fmt::Debug;

use serde::{Deserialize, Serialize};

use super::*;
use inner::StrBufferInner;

#[cfg(all(not(feature = "std"), not(feature = "no-std-lock")))]
compile_error!("One of the `std` or `no-std-lock` features should be enabled");

// HACK
// Currently there is no global state for a ser/de call, but we need it to handle `StrRef` .
// To resolve this in a simple way, we use a thread-global state instead.
// Because there cannot be two ser/de in progressing in a single thread.
// However, this does not work on `no_std` env.
// So we just use a spin lock to prevent two ser/de in progressing in multiple `no_std` thread.
// This should be finally resolved by customizing ser/de process.
pub(super) enum SerdeThreadGlobalState {
    None,
    Ser(SerdeThreadGlobalStateSer),
    De(SerdeThreadGlobalStateDe),
    DePrepare {
        zero_copy: Box<dyn 'static + FnOnce()>,
    },
}

pub(super) struct SerdeThreadGlobalStateSer {
    str_buffer: Rc<StrBufferInner>,
    offset_gen: Vec<usize>,
    offsets: Option<alloc::vec::IntoIter<usize>>,
}

pub(super) struct SerdeThreadGlobalStateDe {
    str_buffer: Rc<StrBufferInner>,
    pub(super) zero_copy: Option<Box<dyn 'static + FnOnce()>>,
}

// It is safe because it will not be used across threads!
unsafe impl Send for SerdeThreadGlobalState {}
unsafe impl Sync for SerdeThreadGlobalState {}

impl SerdeThreadGlobalState {
    #[cfg(feature = "std")]
    #[allow(dead_code)]
    fn get<R>(f: impl FnOnce(&mut SerdeThreadGlobalState) -> R) -> R {
        thread_local! {
            static SERDE_THREAD_GLOBAL_STATE: RefCell<SerdeThreadGlobalState> = const { core::cell::RefCell::new(SerdeThreadGlobalState::None) };
        }
        SERDE_THREAD_GLOBAL_STATE.with(|x| {
            let mut x = x.borrow_mut();
            f(&mut x)
        })
    }

    #[cfg(all(not(feature = "std"), feature = "no-std-lock"))]
    #[allow(dead_code)]
    fn get<R>(f: impl FnOnce(&mut SerdeThreadGlobalState) -> R) -> R {
        static SERDE_THREAD_GLOBAL_STATE: spin::Lazy<spin::Mutex<SerdeThreadGlobalState>> =
            spin::Lazy::new(|| spin::Mutex::new(SerdeThreadGlobalState::None));
        f(&mut SERDE_THREAD_GLOBAL_STATE.lock())
    }

    #[allow(dead_code)]
    pub(super) fn ser<R>(ser: SerdeThreadGlobalStateSer, f: impl FnOnce() -> R) -> R {
        Self::get(|state| {
            let SerdeThreadGlobalState::None = state else {
                panic!("Invalid SerdeThreadGlobalState state");
            };
            *state = SerdeThreadGlobalState::Ser(ser);
        });
        let ret = f();
        Self::get(|state| {
            *state = SerdeThreadGlobalState::None;
        });
        ret
    }

    #[allow(dead_code)]
    pub(super) fn get_ser<R>(f: impl FnOnce(&mut SerdeThreadGlobalStateSer) -> R) -> R {
        Self::get(|state| {
            let SerdeThreadGlobalState::Ser(ser) = state else {
                panic!("Invalid SerdeThreadGlobalState state");
            };
            f(ser)
        })
    }

    #[allow(dead_code)]
    pub(super) fn de_prepare<R>(
        zero_copy: Box<dyn 'static + FnOnce()>,
        f: impl FnOnce() -> R,
    ) -> R {
        Self::get(|state| {
            let SerdeThreadGlobalState::None = state else {
                panic!("Invalid SerdeThreadGlobalState state");
            };
            *state = SerdeThreadGlobalState::DePrepare { zero_copy };
        });
        let ret = f();
        Self::get(|state| {
            *state = SerdeThreadGlobalState::None;
        });
        ret
    }

    #[allow(dead_code)]
    pub(super) fn de<R>(mut de: SerdeThreadGlobalStateDe, f: impl FnOnce() -> R) -> R {
        Self::get(|state| {
            let old_state = core::mem::replace(state, SerdeThreadGlobalState::None);
            de.zero_copy = match old_state {
                SerdeThreadGlobalState::None => None,
                SerdeThreadGlobalState::DePrepare { zero_copy } => Some(zero_copy),
                _ => panic!("Invalid SerdeThreadGlobalState state"),
            };
            *state = SerdeThreadGlobalState::De(de);
        });
        let ret = f();
        Self::get(|state| {
            *state = SerdeThreadGlobalState::None;
        });
        ret
    }

    #[allow(dead_code)]
    pub(super) fn get_de<R>(f: impl FnOnce(&mut SerdeThreadGlobalStateDe) -> R) -> R {
        Self::get(|state| {
            let SerdeThreadGlobalState::De(de) = state else {
                panic!("Invalid SerdeThreadGlobalState state");
            };
            f(de)
        })
    }

    #[allow(dead_code)]
    pub(super) fn get_de_optional<R>(
        f: impl FnOnce(Option<&mut SerdeThreadGlobalStateDe>) -> R,
    ) -> R {
        Self::get(|state| {
            if let SerdeThreadGlobalState::De(de) = state {
                f(Some(de))
            } else {
                f(None)
            }
        })
    }
}

pub(crate) fn str_buffer_de_env<R>(str_buffer: &StrBuffer, f: impl FnOnce() -> R) -> R {
    SerdeThreadGlobalState::de(
        SerdeThreadGlobalStateDe {
            str_buffer: str_buffer.inner.clone(),
            zero_copy: None,
        },
        f,
    )
}

pub(crate) fn str_buffer_ser_env<R, T>(
    first_gen_f: impl FnOnce() -> T,
    final_gen_f: impl FnOnce(T, StrBuffer) -> R,
) -> R {
    SerdeThreadGlobalState::ser(
        SerdeThreadGlobalStateSer {
            str_buffer: Rc::new(StrBufferInner::new()),
            offset_gen: vec![],
            offsets: None,
        },
        || {
            let r = first_gen_f();
            let buf = SerdeThreadGlobalState::get_ser(|state| {
                let buf = state.str_buffer.clone();
                let offset_gen = core::mem::take(&mut state.offset_gen);
                buf.freeze();
                state.offsets = Some(offset_gen.into_iter());
                buf
            });
            final_gen_f(r, StrBuffer { inner: buf })
        },
    )
}

pub(crate) mod inner {
    use alloc::{boxed::Box, vec::Vec};
    use core::cell::{Cell, UnsafeCell};

    pub(crate) struct StrBufferInner {
        writable: Cell<bool>,
        static_borrowed: Option<Box<dyn 'static + FnOnce()>>,
        buf: UnsafeCell<Vec<u8>>,
    }

    impl Drop for StrBufferInner {
        fn drop(&mut self) {
            if let Some(f) = self.static_borrowed.take() {
                let buf = unsafe { &mut *self.buf.get() };
                let mut empty = vec![];
                core::mem::swap(&mut empty, buf);
                let _ = Box::into_raw(empty.into_boxed_slice());
                f();
            }
        }
    }

    impl core::fmt::Debug for StrBufferInner {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(
                f,
                "StrBufferInner {{ writable: {}, buf: [...] }}",
                self.writable.get()
            )
        }
    }

    impl StrBufferInner {
        pub(super) fn new() -> Self {
            Self {
                writable: Cell::new(true),
                static_borrowed: None,
                buf: UnsafeCell::new(vec![]),
            }
        }

        pub(super) fn new_with_buf(buf: Vec<u8>) -> Self {
            Self {
                writable: Cell::new(false),
                static_borrowed: None,
                buf: UnsafeCell::new(buf),
            }
        }

        pub(super) unsafe fn new_static_borrowed(
            buf: *mut [u8],
            drop_callback: Box<dyn 'static + FnOnce()>,
        ) -> Self {
            Self {
                writable: Cell::new(false),
                static_borrowed: Some(Box::new(drop_callback)),
                buf: UnsafeCell::new(Box::from_raw(buf).into_vec()),
            }
        }

        pub(super) fn freeze(&self) {
            self.writable.set(false);
        }

        pub(super) fn append(&self, s: &str) -> usize {
            if !self.writable.get() {
                panic!("StrBuffer is not in writable stage");
            }
            let buf = unsafe { &mut *self.buf.get() };
            let offset = buf.len();
            buf.append(&mut Vec::from(s.as_bytes()));
            offset
        }

        pub(super) fn read(&self) -> &[u8] {
            if self.writable.get() {
                panic!("StrBuffer is not in writable stage");
            }
            let buf = unsafe { &mut *self.buf.get() };
            buf.as_slice()
        }

        pub(super) fn len(&self) -> usize {
            let buf = unsafe { &mut *self.buf.get() };
            buf.len()
        }
    }
}

/// cbindgen:ignore
#[repr(C)]
#[derive(Debug, Clone)]
pub struct StrBuffer {
    inner: Rc<StrBufferInner>,
}

impl StrBuffer {
    #[cfg(feature = "serialize")]
    pub(crate) fn new() -> Self {
        Self {
            inner: Rc::new(StrBufferInner::new()),
        }
    }

    pub(crate) fn new_with_buf(buf: Vec<u8>) -> Self {
        Self {
            inner: Rc::new(StrBufferInner::new_with_buf(buf)),
        }
    }

    pub(crate) unsafe fn new_static_borrowed(
        buf: *mut [u8],
        drop_callback: Box<dyn 'static + FnOnce()>,
    ) -> Self {
        Self {
            inner: Rc::new(StrBufferInner::new_static_borrowed(buf, drop_callback)),
        }
    }

    #[cfg(feature = "serialize")]
    pub(crate) fn freeze(&mut self) {
        self.inner.freeze()
    }

    pub(crate) fn whole_buffer(&self) -> &[u8] {
        self.inner.read()
    }
}

/// An string format which is compatible with the binary format.
///
/// cbindgen:ignore
#[repr(C)]
#[derive(Clone)]
pub struct StrRef {
    offset: usize,
    len: usize,
    buf: Rc<StrBufferInner>,
}

impl StrRef {
    /// Convert it to `[u8]`.
    pub fn as_slice<'a>(&'a self) -> &'a [u8] {
        let buf = self.buf.read();
        unsafe {
            let ptr = (buf as *const [u8] as *const u8).add(self.offset);
            core::slice::from_raw_parts::<'a, u8>(ptr, self.len)
        }
    }

    /// Convert it to `str`.
    pub fn as_str(&self) -> &str {
        core::str::from_utf8(self.as_slice()).unwrap_or_default()
    }

    #[doc(hidden)]
    /// # Safety
    ///
    pub unsafe fn as_str_unchecked(&self) -> &str {
        core::str::from_utf8_unchecked(self.as_slice())
    }

    /// Convert it to `String`.
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        String::from_utf8_lossy(self.as_slice()).into_owned()
    }

    /// Compare it with `str`.
    pub fn equal(&self, s: &str) -> bool {
        self.as_slice() == s.as_bytes()
    }

    #[doc(hidden)]
    #[cfg(feature = "ffi")]
    #[no_mangle]
    pub extern "C" fn str_ptr(&self) -> *const u8 {
        let buf = self.buf.read();
        unsafe { (buf as *const [u8] as *const u8).add(self.offset) }
    }

    #[doc(hidden)]
    #[cfg(feature = "ffi")]
    #[no_mangle]
    pub extern "C" fn str_len(&self) -> usize {
        self.len
    }
}

impl<T: alloc::string::ToString> From<T> for StrRef {
    fn from(s: T) -> Self {
        let s = s.to_string();
        let len = s.len();
        let buf = Rc::new(StrBufferInner::new_with_buf(s.into_bytes()));
        Self {
            offset: 0,
            len,
            buf,
        }
    }
}

impl Default for StrRef {
    fn default() -> Self {
        Self::from(String::new())
    }
}

impl PartialEq for StrRef {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Serialize for StrRef {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let offset = SerdeThreadGlobalState::get_ser(|state| {
            if let Some(offsets) = state.offsets.as_mut() {
                offsets.next().unwrap_or_default()
            } else {
                let x = state.str_buffer.append(self.as_str());
                state.offset_gen.push(x);
                0
            }
        });
        (offset, self.len).serialize(ser)
    }
}

impl<'de> Deserialize<'de> for StrRef {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let buf = SerdeThreadGlobalState::get_de(|state| state.str_buffer.clone());
        let (offset, len) = <(usize, usize)>::deserialize(de)?;
        let offset = offset.min(buf.len());
        Ok(Self { offset, len, buf })
    }
}
impl Debug for StrRef {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
#[cfg(debug_assertions)]
impl crate::CompatibilityCheck for StrRef {
    fn check() {}
}
