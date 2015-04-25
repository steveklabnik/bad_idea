#![feature(core)]
#![feature(unique)]
#![feature(alloc)]

extern crate libc;
extern crate core;

extern crate alloc;

use std::mem;
use std::slice;
use std::io::Cursor;
use std::io::Write;

use core::ptr;
use core::ptr::Unique;

use alloc::heap::{allocate, reallocate};

use libc::uintptr_t;

#[repr(C)]
pub struct Array {
    ptr: Unique<uintptr_t>,
    len: usize,
    cap: usize,
}

#[no_mangle]
pub extern fn array_push(a: *mut Array, item: uintptr_t) {
    let a = unsafe { &mut *a };
    a.push(item);
}

#[no_mangle]
pub extern fn array_inspect(a: *const Array, s: *mut u8) {
    let a = unsafe { &*a };
    a.inspect(s);
}

impl Array {
    fn push(&mut self, value: uintptr_t) {
        if self.len == self.cap {
            unsafe {
                let old_size = self.cap * mem::size_of::<uintptr_t>();
                let size = old_size + mem::size_of::<uintptr_t>();
                let ptr = alloc_or_realloc(*self.ptr, old_size, size);
                // not checking for null because YOLO

                self.ptr = Unique::new(ptr);
            }
            self.cap += 1;
        }
        unsafe {
            let end = (*self.ptr).offset(self.len as isize);
            ptr::write(&mut *end, value);
            self.len += 1;
        }
    }

    fn inspect(&self, s: *mut u8) {
        let slice = unsafe { slice::from_raw_parts_mut(s, 100) };
        let mut cursor = Cursor::new(slice);
        if self.len == 0 {
            cursor.write_all(b"[]").ok().expect("couldn't write empty array");
        } else {
            let msg = format!("[{} things]", self.len);
            cursor.write_all(msg.as_bytes()).ok().expect("Couldn't write array");
        }
    }
}

#[inline(never)]
unsafe fn alloc_or_realloc<T>(ptr: *mut T, old_size: usize, size: usize) -> *mut T {
    if old_size == 0 {
        allocate(size, mem::min_align_of::<T>()) as *mut T
    } else {
        reallocate(ptr as *mut u8, old_size, size, mem::min_align_of::<T>()) as *mut T
    }
}


#[no_mangle]
pub extern fn bad_idea_init() { }

