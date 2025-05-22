#![doc(hidden)]
#![cfg(feature = "ffi")]

pub mod c_ffi;
pub mod wasm_ffi;

pub type RawMutPtr = *mut ();

pub type NullPtr = *const ();

#[repr(C)]
pub enum FfiErrorCode {
    None,
    ThisNullPointer,
    PathNullPointer,
    PrefixNullPointer,
    SourceNullPointer,
    BufferNullPointer,
    ExprPtrNullPointer,
    StrNullPointer,
    InlineStyleTextNullPointer,
    InlineRuleNullPointer,
    StyleTextNullPointer,
    SelectorTextNullPointer,
    InvalidPath,
    JsonNullPointer,
    ArrayNullPointer,
    SelectorNullPointer,
    StyleSheetNullPointer,
    MapNullPointer,
    Unknown,
}

#[repr(C)]
pub struct FfiResult<T> {
    pub value: T,
    pub err: FfiErrorCode,
}

impl<T> FfiResult<T> {
    pub fn ok(value: T) -> Self {
        Self {
            value,
            err: FfiErrorCode::None,
        }
    }
    pub fn error(err: FfiErrorCode, default: T) -> Self {
        Self {
            value: default,
            err,
        }
    }
}
