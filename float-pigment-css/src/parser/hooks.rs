//! Parser hooks can be used to attach some compilation information.

use alloc::vec::Vec;

#[cfg(feature = "ffi")]
use core::ffi::{c_char, CStr};

use cssparser::SourceLocation;

use super::{Warning, WarningKind};

#[cfg(feature = "ffi")]
use crate::ffi::{FfiResult, NullPtr};

use crate::property::Property;

/// A `context` for current sompilation step.
pub struct ParserHooksContext<'a> {
    pub(super) warnings: &'a mut Vec<Warning>,
    pub(super) start_loc: SourceLocation,
    pub(super) end_loc: SourceLocation,
}

impl<'a> ParserHooksContext<'a> {
    /// Generate a new warning in the current location.
    pub fn generate_warning(&mut self, message: &str) {
        let start = self.start_loc;
        let end = self.end_loc;
        self.warnings.push(Warning {
            kind: WarningKind::HooksGenerated,
            message: message.into(),
            start_line: start.line,
            start_col: start.column,
            end_line: end.line,
            end_col: end.column,
        })
    }
}

/// A list of hooks that can be implemented.
pub trait Hooks {
    /// Trigger once whenever a property has been parsed.
    fn parsed_property(&mut self, _ctx: &mut ParserHooksContext, _p: &mut Property) {}
}

/// The C FFI for `ParserHooksContext`.
#[cfg(feature = "ffi")]
#[repr(C)]
pub struct CParserHooksContext {
    inner: *mut (),
}

#[cfg(feature = "ffi")]
impl CParserHooksContext {
    /// The C FFI for `ParserHooksContext::generate_warning`.
    ///
    /// # Safety
    ///
    /// The message should be a valid C string.
    #[no_mangle]
    pub unsafe extern "C" fn generate_warning(
        &mut self,
        message: *const c_char,
    ) -> FfiResult<NullPtr> {
        use crate::check_null;
        use crate::ffi::FfiErrorCode;
        use core::ptr::null;
        check_null!(message, FfiErrorCode::StrNullPointer, null());
        let message = CStr::from_ptr(message).to_string_lossy();
        let ctx = &mut *(self.inner as *mut ParserHooksContext);
        ctx.generate_warning(&message);
        FfiResult::ok(null())
    }
}

/// The C FFI for `ParserHooks`.
#[cfg(feature = "ffi")]
#[repr(C)]
pub struct CParserHooks {
    parsed_property: extern "C" fn(CParserHooksContext, *mut Property),
}

#[cfg(feature = "ffi")]
impl Hooks for CParserHooks {
    fn parsed_property(&mut self, ctx: &mut ParserHooksContext, p: &mut Property) {
        let ctx = CParserHooksContext {
            inner: ctx as *mut _ as *mut (),
        };
        let f = &mut self.parsed_property;
        f(ctx, p);
    }
}
