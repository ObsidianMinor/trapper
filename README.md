# trapper

A simple Rust library for creating transparent newtypes that don't require ownership to be used.

## Example

```rust
use trapper::{Wrapper, newtype};
newtype!(#[derive(PartialEq, Debug)] type NumberWrapper(i32));

fn foo(r: &i32, m: &mut i32) {
    let ref_wrapper: &NumberWrapper = NumberWrapper::wrap_ref(r);
    let mut_wrapper: &mut NumberWrapper = NumberWrapper::wrap_mut(m);
}

let mut wrapper = NumberWrapper::wrap(12);
*wrapper.unwrap_mut() = 13;

assert_eq!(wrapper, NumberWrapper::wrap(13));
```