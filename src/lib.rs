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

#![cfg_attr(not(feature = "std" ), no_std)]

// Unsafe code is required for the initialization.
#![allow(unsafe_code)]

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
        use core::mem::MaybeUninit;

        let mut uninit_data: MaybeUninit<[Option<$t>; $n]> = MaybeUninit::uninit();
        let array = uninit_data.as_mut_ptr();
        unsafe { &mut *array }.fill_with(|| None);
        unsafe { uninit_data.assume_init() }
    }};
}

/// Creates an array of type `[Cell<Option<T>>; N]` and default-initializes it to `Cell:new(None)`
/// for every element. Similar to [`none_arr`], plus a [`Cell`](core::cell::Cell).
///
/// ## Macro Arguments
///
/// * `$t` - The type to wrap into an `Cell<Option<T>>`.
/// * `$n` - The number of elements for the array.
///
/// ## Example
///
/// When dealing with non-`Copy`/`Clone` types, arrays of type `[Option<T>; N]` cannot be
/// created even though the default type `None` could be applied:
///
/// ```compile_fail
/// # use core::cell::Cell;
/// // This type does not implement Copy/Clone.
/// struct Complicated;
///
/// // Fails with "the trait `Copy` is not implemented for `Complicated`":
/// let arr: [Cell<Option<Complicated>>; 10] = [Cell::new(None); 10];
/// ```
///
/// This crate simplifies array creation of these cases through the `none_cell_arr` macro:
///
/// ```
/// # use core::cell::Cell;
/// # use default_option_arr::none_cell_arr;
/// # struct Complicated;
/// let arr = none_cell_arr![Complicated; 10];
///
/// // The result is an array of Cell<Option<Complicated>>.
/// let arr: [Cell<Option<Complicated>>; 10] = arr;
///
/// assert_eq!(arr.len(), 10);
/// for item in arr.into_iter() {
///     assert!(item.take().is_none());
/// }
/// ```
#[macro_export]
macro_rules! none_cell_arr {
    ($t:ty; $n:expr) => {{
        use core::cell::Cell;
        use core::mem::MaybeUninit;

        let mut uninit_data: MaybeUninit<[Cell<Option<$t>>; $n]> = MaybeUninit::uninit();
        let array = uninit_data.as_mut_ptr();
        unsafe { &mut *array }.fill_with(|| Cell::new(None));
        unsafe { uninit_data.assume_init() }
    }};
}

/// Creates an array of type `[RefCell<Option<T>>; N]` and default-initializes it to `Cell:new(None)`
/// for every element. Similar to [`none_cell_arr`], but with a [`RefCell`](core::cell::RefCell).
///
/// ## Macro Arguments
///
/// * `$t` - The type to wrap into an `RefCell<Option<T>>`.
/// * `$n` - The number of elements for the array.
///
/// ## Example
///
/// When dealing with non-`Copy`/`Clone` types, arrays of type `[RefCell<Option<T>>; N]` cannot be
/// created even though the default type `None` could be applied:
///
/// ```compile_fail
/// # use core::cell::RefCell;
/// // This type does not implement Copy/Clone.
/// struct Complicated;
///
/// // Fails with "the trait `Copy` is not implemented for `Complicated`":
/// let arr: [RefCell<Option<Complicated>>; 10] = [RefCell::new(None); 10];
/// ```
///
/// This crate simplifies array creation of these cases through the `none_refcell_arr` macro:
///
/// ```
/// # use core::cell::RefCell;
/// # use default_option_arr::none_refcell_arr;
/// # struct Complicated;
/// let arr = none_refcell_arr![Complicated; 10];
///
/// // The result is an array of Cell<Option<Complicated>>.
/// let arr: [RefCell<Option<Complicated>>; 10] = arr;
///
/// assert_eq!(arr.len(), 10);
/// for item in arr.into_iter() {
///     assert!(item.take().is_none());
/// }
/// ```
#[macro_export]
macro_rules! none_refcell_arr {
    ($t:ty; $n:expr) => {{
        use core::cell::RefCell;
        use core::mem::MaybeUninit;

        let mut uninit_data: MaybeUninit<[RefCell<Option<$t>>; $n]> = MaybeUninit::uninit();
        let array = uninit_data.as_mut_ptr();
        unsafe { &mut *array }.fill_with(|| RefCell::new(None));
        unsafe { uninit_data.assume_init() }
    }};
}

#[cfg(test)]
mod tests {
    use core::mem::MaybeUninit;

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
    fn cell_works() {
        let arr = none_cell_arr![Complicated; 10];

        assert_eq!(arr.len(), 10);
        for item in arr.into_iter() {
            assert!(item.take().is_none());
        }
    }

    #[test]
    fn refcell_works() {
        let arr = none_refcell_arr![Complicated; 10];

        assert_eq!(arr.len(), 10);
        for item in arr.into_iter() {
            assert!(item.borrow().is_none());
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

    #[test]
    #[cfg(feature = "std")]
    fn reference_vec() {
        let arr: [Option<Complicated>; 10] = (0..10)
            .into_iter()
            .map(|_| None)
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| "try_into failed") // Debug required otherwise
            .expect("initialization failed");

        assert_eq!(arr.len(), 10);
        for item in arr.into_iter() {
            assert!(item.is_none());
        }
    }
}
