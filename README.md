# Default Array-of-Option\<T\> Macros

Macros to make your life easier when dealing with default-initialized
arrays of `Option<T>` or `Cell<Option<T>>` for non-`Copy` types of `T` to `[None, ..]`.

### You may need it if ...

- You need an array of `[Option<T>; N]` initialized to `[None; N]`, or
- You need an array of `[Cell<Option<T>>; N]` initialized to `[Cell::new(None); N]`, or
- You need an array of `[RefCell<Option<T>>; N]` initialized to `[RefCell::new(None); N]`.

### You will not need it if ...

- Your types already implement `Copy` or `Clone` and you don't need cells.
- You require `#![forbid(unsafe_code)]`.

## Examples

```rust
use core::cell::Cell;
use arraysetcell::ArraySetCell;

// This type does not implement Copy.
struct Complicated;

fn it_works() {
    // This doesn't compile:
    let arr: [Option<Complicated>; 10] = [None; 10];

    // This does:
    let arr = none_arr![Complicated; 10];

    // [None, None, None, ...]
    assert_eq!(arr.len(), 10);
    for item in arr.into_iter() {
        assert!(item.is_none());
    }

    // The created type is an array.
    let arr: [Option<Complicated>; 10] = arr;
    assert_eq!(arr.len(), 10);
}
```

Likewise, arrays of `Cell<Option<T>>` can be created.

```rust
fn cell_works() {
    let arr: [Cell<Option<Complicated>>; 10] = none_cell_arr![Complicated; 10];
    let arr: [RefCell<Option<Complicated>>; 10] = none_refcell_arr![Complicated; 10];
}
```

## I cannot have unsafe code

If you cannot have `unsafe` code in your project, something like the following can be used:

```rust
fn not_fun() {
    let arr: [Option<Complicated>; 10] = (0..10)
        .into_iter()
        .map(|_| None)
        .collect::<Vec<_>>()
        .try_into()
        .map_err(|_| "try_into failed") // Debug required otherwise
        .expect("initialization failed");
}
```
