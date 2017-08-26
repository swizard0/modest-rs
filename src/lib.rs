extern crate modest_sys;

pub mod modest;
pub mod myhtml;
pub mod mycss;

pub mod encoding;
pub use self::encoding::Encoding;

trait ForeignRaw<T> {
    fn get_raw_mut(&mut self) -> *mut T;
    fn get_raw(&self) -> *const T;
}

trait FromRaw<T> {
    fn from_raw(init: T) -> Self;
}
