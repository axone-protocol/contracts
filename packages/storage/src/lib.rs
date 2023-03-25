#![forbid(unsafe_code)]
#![deny(
    warnings,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unused_lifetimes,
    unused_import_braces,
    unused_qualifications
)]

mod object;

pub use object::ObjectRef;
