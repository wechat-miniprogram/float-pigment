pub mod extra {
    #[repr(C)]
    pub struct Box<T> {
        ptr: *mut T,
    }

    #[repr(C)]
    pub struct Array<T> {
        ptr: *mut T,
        len: usize,
    }

    #[repr(C)]
    pub struct Nullable<T> {
        non_null: bool,
        data: T,
    }

    #[repr(C)]
    pub struct NullablePtr<T> {
        ptr: *mut T,
    }

    #[repr(C)]
    pub struct StyleSheetRcBuffer {
        ptr: *mut (),
    }

    #[repr(C)]
    pub struct StrBuffer {
        ptr: *mut (),
    }

    #[repr(C)]
    pub struct StrRef {
        offset: usize,
        len: usize,
        ptr: *mut (),
    }
}
