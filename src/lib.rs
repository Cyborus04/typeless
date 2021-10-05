#![deny(missing_docs)]

//! `unsafe` API for type erasure on the stack
//!
//! Storing any value `x` of type `T` in `TypeErased` completely destroys all type data associated
//! with it.
//!
//! # Restrictions
//! While this erases all type data, leaving only the pure bytes, the compiler still requires 2
//! things:
//!
//! - Size: The size of a `TypeErased` is not based on the data it contains, but
//! rather a const generic parameter `C`, effectively a "maximum size" on the types it can contain.
//!
//! - Alignment: Until there is a way to define alignment by a const parameter, the alignment of `TypeErased` is
//! 8 bytes, so anything with an alignment of 8 or less can be contained
//!
//! ## Access
//! Since there is no type data anymore, any access to the inner data is `unsafe` (except [getting the bytes directly](crate::TypeErased::raw))
//!
//! `TypeErased` is not `Send` nor `Sync` since it can't be known if that's safe

#[cfg(not(feature = "std"))]
use core::{
    marker::PhantomData,
    mem::{align_of, size_of, MaybeUninit},
    ptr,
};
#[cfg(feature = "std")]
use std::{
    marker::PhantomData,
    mem::{align_of, size_of, MaybeUninit},
    ptr,
};

/// Type-erased data on the stack
///
/// See the [crate-level docs](crate) for more info
#[repr(C, align(8))]
pub struct TypeErased<const C: usize> {
    buf: [MaybeUninit<u8>; C],
    __no_send_sync: PhantomData<*const ()>,
}

impl<const C: usize> TypeErased<C> {
    /// Creates a new `TypeErased` by erasing the type of `value`
    /// # Panics
    /// Panics if `size_of::<T>()` is greater than `C` or `align_of::<T>()` is greater than 8
    /// (eventually these will become compile-time restrictions)
    pub fn new<T: 'static>(value: T) -> Self {
        assert!(
            size_of::<T>() <= C,
            "typeless: size of T ({}) > capacity ({})",
            size_of::<T>(),
            C
        ); // ensure size of C or less
        assert!(
            align_of::<T>() <= 8,
            "typeless: alignment of T ({}) > max align (8)",
            align_of::<T>()
        ); // ensure alignment of 8 or less

        unsafe { Self::new_unchecked(value) }
    }

    /// Creates a new `TypeErased` containing no value
    ///
    /// This is effectively equivalent to `TypeErased::new::<()>(())`
    pub const fn empty() -> Self {
        const U8_UNINIT: MaybeUninit<u8> = MaybeUninit::uninit();

        Self {
            buf: [U8_UNINIT; C],
            __no_send_sync: PhantomData,
        }
    }

    /// Creates a new `TypeErased` by erasing the type of `value`
    /// # Safety
    /// `size_of::<T>()` must be less than or equal to `C` and `align_of::<T>()` must be less than
    /// or equal to 8.
    ///
    /// This function will be deprecated once it is possible to ensure these at compile-time.
    pub unsafe fn new_unchecked<T>(value: T) -> Self {
        debug_assert!(
            size_of::<T>() <= C,
            "typeless: safety requirement violated: size of T ({}) > capacity ({})",
            size_of::<T>(),
            C
        ); // ensure size of C or less
        debug_assert!(
            align_of::<T>() <= 8,
            "typeless: safety requirement violated: alignment of T ({}) > max align (8)",
            align_of::<T>()
        ); // ensure alignment of 8 or less

        let mut this = Self::empty();
        let ptr = this.as_mut_ptr::<T>();
        debug_assert_eq!(
            ptr as usize % 8,
            0,
            "typeless: internal error: ptr not 8-byte aligned"
        ); // ensure pointer alignment of 8
        ptr::write(ptr, value);
        this
    }

    /// Gets a pointer to some type `T` contained in this `TypeErased`
    /// # Dereferencability
    /// The returned pointer is valid to dereference if:
    /// - The size of `T` is less than or equal to `C`
    /// - The alignment of `T` is less than or equal to 8
    /// - The data in this `TypeErased` is a valid instance of `T` (expired references are not valid)
    pub const fn as_ptr<T>(&self) -> *const T {
        self.buf.as_ptr().cast()
    }

    /// Gets a mutable pointer to some type `T` contained in this `TypeErased`
    /// # Dereferencability
    /// The returned pointer is valid to dereference if:
    /// - The size of `T` is less than or equal to `C`
    /// - The alignment of `T` is less than or equal to 8
    /// - The data in this `TypeErased` is a valid instance of `T` (expired references are not valid)
    pub fn as_mut_ptr<T>(&mut self) -> *mut T {
        self.buf.as_mut_ptr().cast()
    }

    /// Assumes this `TypeErased` contains a valid `T` and returns a reference to it
    /// # Safety
    /// `size_of::<T>()` must be less than or equal to `C`, `align_of::<T>()` must be less than
    /// or equal to 8, and this must contain a valid instance of `T` (expired references are not valid).
    pub unsafe fn assume_type_ref<T>(&self) -> &T {
        &*self.as_ptr()
    }

    /// Assumes this `TypeErased` contains a valid `T` and returns a mutable reference to it
    /// # Safety
    /// `size_of::<T>()` must be less than or equal to `C`, `align_of::<T>()` must be less than
    /// or equal to 8, and this must contain a valid instance of `T` (expired references are not valid).
    pub unsafe fn assume_type_mut<T>(&mut self) -> &mut T {
        &mut *self.as_mut_ptr()
    }

    /// Assumes this `TypeErased` contains a valid `T` and takes ownership of it
    /// # Safety
    /// `size_of::<T>()` must be less than or equal to `C`, `align_of::<T>()` must be less than
    /// or equal to 8, and this must contain a valid instance of `T` (expired references are not valid).
    pub unsafe fn assume_type_take<T>(self) -> T {
        ptr::read(self.as_ptr())
    }

    /// Gets the buffer of raw bytes inside
    pub const fn raw(&self) -> &[MaybeUninit<u8>; C] {
        &self.buf
    }

    /// Gets a mutable reference to the buffer of raw bytes inside
    pub fn raw_mut(&mut self) -> &mut [MaybeUninit<u8>; C] {
        &mut self.buf
    }
}
