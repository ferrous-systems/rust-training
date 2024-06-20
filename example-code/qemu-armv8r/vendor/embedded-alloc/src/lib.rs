#![doc = include_str!("../README.md")]
#![no_std]
#![cfg_attr(feature = "allocator_api", feature(allocator_api, alloc_layout_extra))]

use core::alloc::{GlobalAlloc, Layout};
use core::cell::RefCell;
use core::ptr::{self, NonNull};

use critical_section::Mutex;
use linked_list_allocator::Heap as LLHeap;

pub struct Heap {
    heap: Mutex<RefCell<LLHeap>>,
}

impl Heap {
    /// Crate a new UNINITIALIZED heap allocator
    ///
    /// You must initialize this heap using the
    /// [`init`](Self::init) method before using the allocator.
    pub const fn empty() -> Heap {
        Heap {
            heap: Mutex::new(RefCell::new(LLHeap::empty())),
        }
    }

    /// Initializes the heap
    ///
    /// This function must be called BEFORE you run any code that makes use of the
    /// allocator.
    ///
    /// `start_addr` is the address where the heap will be located.
    ///
    /// `size` is the size of the heap in bytes.
    ///
    /// Note that:
    ///
    /// - The heap grows "upwards", towards larger addresses. Thus `start_addr` will
    ///   be the smallest address used.
    ///
    /// - The largest address used is `start_addr + size - 1`, so if `start_addr` is
    ///   `0x1000` and `size` is `0x30000` then the allocator won't use memory at
    ///   addresses `0x31000` and larger.
    ///
    /// # Safety
    ///
    /// Obey these or Bad Stuff will happen.
    ///
    /// - This function must be called exactly ONCE.
    /// - `size > 0`
    pub unsafe fn init(&self, start_addr: usize, size: usize) {
        critical_section::with(|cs| {
            self.heap
                .borrow(cs)
                .borrow_mut()
                .init(start_addr as *mut u8, size);
        });
    }

    /// Returns an estimate of the amount of bytes in use.
    pub fn used(&self) -> usize {
        critical_section::with(|cs| self.heap.borrow(cs).borrow_mut().used())
    }

    /// Returns an estimate of the amount of bytes available.
    pub fn free(&self) -> usize {
        critical_section::with(|cs| self.heap.borrow(cs).borrow_mut().free())
    }

    fn alloc_first_fit(&self, layout: Layout) -> Result<NonNull<u8>, ()> {
        critical_section::with(|cs| self.heap.borrow(cs).borrow_mut().allocate_first_fit(layout))
    }
}

unsafe impl GlobalAlloc for Heap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.alloc_first_fit(layout)
            .ok()
            .map_or(ptr::null_mut(), |allocation| allocation.as_ptr())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        critical_section::with(|cs| {
            self.heap
                .borrow(cs)
                .borrow_mut()
                .deallocate(NonNull::new_unchecked(ptr), layout)
        });
    }
}

#[cfg(feature = "allocator_api")]
mod allocator_api {
    use core::alloc::{AllocError, Allocator, GlobalAlloc, Layout};
    use core::ptr::NonNull;

    unsafe impl Allocator for crate::Heap {
        fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
            match layout.size() {
                0 => Ok(NonNull::slice_from_raw_parts(layout.dangling(), 0)),
                size => self
                    .alloc_first_fit(layout)
                    .map(|allocation| NonNull::slice_from_raw_parts(allocation, size))
                    .map_err(|_| AllocError),
            }
        }

        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
            if layout.size() != 0 {
                self.dealloc(ptr.as_ptr(), layout);
            }
        }
    }
}
