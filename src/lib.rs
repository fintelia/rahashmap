#![feature(allocator_api)]
#![feature(dropck_eyepatch)]
#![feature(generic_param_attrs)]
#![feature(placement_in_syntax)]
#![feature(placement_new_protocol)]
#![feature(ptr_internals)]
#![feature(sip_hash_13)]

extern crate rand;

mod table;
mod map;

pub use map::*;

trait Recover<Q: ?Sized> {
    type Key;

    fn get(&self, key: &Q) -> Option<&Self::Key>;
    fn take(&mut self, key: &Q) -> Option<Self::Key>;
    fn replace(&mut self, key: Self::Key) -> Option<Self::Key>;
}
