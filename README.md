Mag is a library for dealing with units of measure.  Magnitude!

## Highlights

* Easy to understand and use
* No external dependencies
* Fast compile time
* Units are not discarded when creating quantities.  In keeping with Rust
  philosohpy, conversions must be done manually (using the `to` method).

## Example

```rust
use mag::{Length, length::{ft, m}};

let a = 3.5 * ft;
let b = a.to::<m>();
assert_eq!(b.to_string(), "1.0668 m");
```

## Room For Improvement

* Small set of quantities and units implemented
* Quantities are f64 only

## Alternatives

There are similar crates out there.  Here is a partial list:

* [uom]
* [dimensioned]
* [yaiouom]
* [measurements]
* [simple_units]
* [metric]
* [unit]
* Plus many more!

[uom]: https://docs.rs/uom/0.26.0/uom/
[dimensioned]: https://docs.rs/dimensioned/0.7.0/dimensioned/
[yaiouom]: https://docs.rs/yaiouom/0.1.3/yaiouom/
[measurements]: https://docs.rs/measurements/0.10.3/measurements/
[simple_units]: https://docs.rs/simple_units/0.1.0/simple_units/
[metric]: https://docs.rs/metric/0.1.2/metric/
[unit]: https://docs.rs/unit/0.1.0/unit/
