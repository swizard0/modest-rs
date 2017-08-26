extern crate modest_sys;

pub mod modest;
pub mod myhtml;
pub mod mycss;

pub mod encoding;
pub use self::encoding::Encoding;

trait ForeignRaw<T> {
    fn get_raw(&mut self) -> *mut T;
}

trait FromRaw<T> {
    fn from_raw(ptr: *mut T) -> Self;
}
