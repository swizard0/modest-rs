use modest_sys::modest as ffi;

pub mod finder;

pub struct Modest {
    raw: *mut ffi::modest_t,
}

#[derive(Debug)]
pub enum Error {
    NoMemory,
    Init,
}

impl Modest {
    pub fn new() -> Result<Modest, Error> {
        let raw = unsafe { ffi::modest_create() };
        if raw.is_null() {
            Err(Error::NoMemory)
        } else {
            let obj = Modest {
                raw: raw,
            };
            if unsafe { ffi::modest_init(obj.raw) } != 0 {
                Err(Error::Init)
            } else {
                Ok(obj)
            }
        }
    }
}

impl Drop for Modest {
    fn drop(&mut self) {
        assert!(!self.raw.is_null());
        let free_result = unsafe { ffi::modest_destroy(self.raw, true) };
        assert!(free_result.is_null());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modest_make_destroy() {
        let _modest = Modest::new().unwrap();
    }
}
