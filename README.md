[Mag] is a library for dealing with physical quantities and units.

Quantities are created by multiplying scalar values by a unit type.  These units
are named after common abbreviations:

```rust
use mag::{length::{ft, m, mi}, time::{h, s}};

let a = 1.0 * ft;
assert_eq!(a.to_string(), "1 ft");

let b = a.to::<m>();
assert_eq!(b.to_string(), "0.3048 m");

let c = 30 * s;
assert_eq!(c.to_string(), "30 s");

let d = 60.0 / s;
assert_eq!(d.to_string(), "60 ㎐");

let e = 55.0 * mi / h;
assert_eq!(e.to_string(), "55 mi/h");
```

## Highlights

* Easy to understand and use
* Performs conversions between units (SI, imperial)
* Units are not discarded when creating quantities
* Fast compile time
* No external dependencies

## Alternative

If mag doesn't fit your needs, you could try the [uom] crate, which has many
more features.

[mag]: https://docs.rs/mag/latest/mag/
[uom]: https://docs.rs/uom/latest/uom/
