// This should be no_std, but we want to be able to use dbg! in development and std conveniences in testing
// Having this be no_std isn't strictly necessary, but it reduces the risk of accidental heap allocations.
#![cfg_attr(not(any(debug_assertions, test)), no_std)]

use arena::{Arena, ArenaRefMut};
use core::{
    alloc::Layout,
    mem::{self, align_of, size_of},
};

pub trait Int: Default + Copy + Sized + PartialEq + Ord + Into<u32> + From<u32> {}

pub type Vec32<'a, T> = Vec<'a, T, u32>;
pub type Vec16<'a, T> = Vec<'a, T, u16>;
pub type Vec8<'a, T> = Vec<'a, T, u8>;

pub struct Vec<'a, T, Len: Int> {
    start: ArenaRefMut<'a, T>,
    len: Len,
    capacity: Len,
}

impl<'a, T, Len: Int> Vec<'a, T, Len> {
    pub fn with_capacity_in(capacity: Len, arena: &'a mut Arena) -> Self {
        let start = unsafe {
            arena
                .alloc_layout(Layout::from_size_align_unchecked(
                    Into::<u32>::into(capacity) as usize * mem::size_of::<T>(),
                    align_of::<T>(),
                ))
                .cast()
        };

        Self {
            start,
            len: Default::default(),
            capacity,
        }
    }

    pub fn capacity(&self) -> Len {
        self.capacity
    }

    pub fn len(&self) -> Len {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == Default::default()
    }

    pub fn get(&self, arena: &'a Arena<'a>, index: Len) -> Option<&'a T> {
        if index < self.len {
            #[cfg(debug_assertions)]
            {
                self.start.debug_verify_arena(arena, "Vec::get");
            }

            Some(unsafe {
                arena.get_unchecked(
                    self.start
                        .add_bytes(size_of::<T>() as u32 * Into::<u32>::into(index)),
                )
            })
        } else {
            None
        }
    }
}
