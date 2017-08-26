use modest_sys::myhtml as ffi;
use super::super::FromRaw;

pub struct Collection {
    raw: *mut ffi::myhtml_collection_t,
}

impl FromRaw<ffi::myhtml_collection_t> for Collection {
    fn from_raw(ptr: *mut ffi::myhtml_collection_t) -> Self {
        Collection {
            raw: ptr,
        }
    }
}

impl Drop for Collection {
    fn drop(&mut self) {
        assert!(!self.raw.is_null());
        let free_result = unsafe { ffi::myhtml_collection_destroy(self.raw) };
        assert!(free_result.is_null());
    }
}
