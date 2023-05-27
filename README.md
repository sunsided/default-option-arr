# Default Array-of-Option\<T\> Macros

Macros to make your life easier when dealing with default-initialized
arrays of `Option<T>` for non-`Copy` types of `T` to `[None, ..]`.

```rust
use std::cell::Cell;
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
    let slice: &[Option<Complicated>] = &arr;
    assert_eq!(slice.len(), 10);
}
```
