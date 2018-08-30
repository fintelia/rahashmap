#![feature(allocator_api)]
#![feature(dropck_eyepatch)]
#![feature(hashmap_internals)]
#![feature(ptr_internals)]
#![feature(test)]
#![feature(try_reserve)]

extern crate rand;

mod bench;
mod table;
mod map;

pub use map::*;
pub use table::{make_hash, SafeHash};

trait Recover<Q: ?Sized> {
    type Key;

    fn get(&self, key: &Q) -> Option<&Self::Key>;
    fn take(&mut self, key: &Q) -> Option<Self::Key>;
    fn replace(&mut self, key: Self::Key) -> Option<Self::Key>;
}
