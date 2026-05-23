A simple attribute macro to autmaticlly make functions FFI safe.
Mostly for lazy people like me, not to much to this.

Usage:
```rust
use ffi-guy::*;
#[ffi_guy]
pub fn add(a:i32, b:i32) -> i32 {
  return a+b;
}
```

- [x] Basic Function
- [ ] Structs
- [ ] More?
