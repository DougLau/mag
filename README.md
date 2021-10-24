Mag is a library for dealing with common units of measure.  Magnitude!

## Highlights

* Easy to understand and use
* No external dependencies
* Fast compile time
* Units are not discarded when creating quantities

## Example

```rust
use mag::{length::{ft, m, mi}, time::{h, s}};

let a = 1.0 * ft; // Length<ft>
let b = a.to::<m>(); // convert to Length<m>
let c = 30.0 * s; // Period<s>
let d = 60.0 / s; // Frequency<s>
let e = 55.0 * mi / h; // Speed<mi, h>

assert_eq!(a.to_string(), "1 ft");
assert_eq!(b.to_string(), "0.3048 m");
assert_eq!(c.to_string(), "30 s");
assert_eq!(d.to_string(), "60 ㎐");
assert_eq!(e.to_string(), "55 mi/h");
```

## Possible Improvements

* Add Current, Mass, Density, etc.
* Improve Area to allow acre, hectare units

## Alternative

If mag doesn't fit your needs, you could try the [uom] crate, which has many
more features.

[uom]: https://docs.rs/uom/latest/uom/
