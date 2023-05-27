//! When dealing with non-`Copy`/`Clone` types `T`, such as arrays of type `[Option<T>; N]` cannot
//! be created even though the default type `None` could be applied.
//!
//! For a given type
//!
//! ```
//! // This type does not implement Copy.
//! struct Complicated;
//! ```
//!
//! The following code fails to compile with the compiler error "the trait `Copy` is not implemented for `Complicated`":
//!
//! ```compile_fail
//! # struct Complicated;
//! let arr: [Option<Complicated>; 10] = [None; 10];
//! ```
//!
//! This crate simplifies array creation of these cases through the `none_arr!` macro:
//!
//! ```
//! # use default_option_arr::none_arr;
//! # struct Complicated;
//! let arr = none_arr![Complicated; 10];
//!
//! // The result is an array of Option<Complicated>.
//! let arr: [Option<Complicated>; 10] = arr;
//!
//! assert_eq!(arr.len(), 10);
//! for item in arr.into_iter() {
//!     assert!(item.is_none());
//! }
//! ```

/// Creates an array of type `[Option<T>; N]` and default-initializes it to `None`
/// for every element.
///
/// ## Macro Arguments
///
/// * `$t` - The type to wrap into an `Option<T>`.
/// * `$n` - The number of elements for the array.
///
/// ## Example
///
/// When dealing with non-`Copy`/`Clone` types, arrays of type `[Option<T>; N]` cannot be
/// created even though the default type `None` could be applied:
///
/// ```compile_fail
/// // This type does not implement Copy/Clone.
/// struct Complicated;
///
/// // Fails with "the trait `Copy` is not implemented for `Complicated`":
/// let arr: [Option<Complicated>; 10] = [None; 10];
/// ```
///
/// This crate simplifies array creation of these cases through the `none_arr` macro:
///
/// ```
/// # use default_option_arr::none_arr;
/// # struct Complicated;
/// let arr = none_arr![Complicated; 10];
///
/// // The result is an array of Option<Complicated>.
/// let arr: [Option<Complicated>; 10] = arr;
///
/// assert_eq!(arr.len(), 10);
/// for item in arr.into_iter() {
///     assert!(item.is_none());
/// }
/// ```
#[macro_export]
macro_rules! none_arr {
    ($t:ty; $n:expr) => {{
        use std::mem::MaybeUninit;

        let mut uninit_data: MaybeUninit<[Option<$t>; $n]> = MaybeUninit::uninit();
        let array = uninit_data.as_mut_ptr();
        unsafe { &mut *array }.fill_with(|| None);
        unsafe { uninit_data.assume_init() }
    }};
}

#[cfg(test)]
mod tests {
    use std::mem::MaybeUninit;

    // No copy, no clone.
    struct Complicated;

    #[test]
    fn it_works() {
        let arr = none_arr![Complicated; 10];

        assert_eq!(arr.len(), 10);
        for item in arr.into_iter() {
            assert!(item.is_none());
        }
    }

    #[test]
    fn reference_loop() {
        let mut uninit_data: MaybeUninit<[Option<Complicated>; 10]> = MaybeUninit::uninit();
        let mut ptr = uninit_data.as_mut_ptr() as *mut Option<Complicated>;
        for _ in 0..10 {
            unsafe {
                ptr.write(None);
                ptr = ptr.add(1);
            }
        }
        let arr = unsafe { uninit_data.assume_init() };

        assert_eq!(arr.len(), 10);
        for item in arr.into_iter() {
            assert!(item.is_none());
        }
    }

    #[test]
    fn reference_fill_with() {
        let mut uninit_data: MaybeUninit<[Option<Complicated>; 10]> = MaybeUninit::uninit();
        let array = uninit_data.as_mut_ptr();
        unsafe { &mut *array }.fill_with(|| None);
        let arr = unsafe { uninit_data.assume_init() };

        assert_eq!(arr.len(), 10);
        for item in arr.into_iter() {
            assert!(item.is_none());
        }
    }
}
