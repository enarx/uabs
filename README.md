[![Workflow Status](https://github.com/enarx/uabs/workflows/test/badge.svg)](https://github.com/enarx/uabs/actions?query=workflow%3A%22test%22)
[![Average time to resolve an issue](https://isitmaintained.com/badge/resolution/enarx/uabs.svg)](https://isitmaintained.com/project/enarx/uabs "Average time to resolve an issue")
[![Percentage of issues still open](https://isitmaintained.com/badge/open/enarx/uabs.svg)](https://isitmaintained.com/project/enarx/uabs "Percentage of issues still open")
![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)

# uabs

This crate provides the `UnsignedAbs` trait containing a function `uabs()`
which calculates the absolute value of the input and returns it as an
unsigned integer of the same size as the input. For example:

```rust
use uabs::Uabs;

let x: i8 = -128;
let y: u8 = x.uabs();
assert_eq!(y, 128);
```

License: Apache-2.0
