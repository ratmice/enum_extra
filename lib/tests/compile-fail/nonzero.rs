#[allow(unused_imports)]
use enum_extra::marker::NonZeroRepr;
use enum_extra_derive::NonZeroRepr;
use strum::EnumMetadata;
use strum_macros::EnumMetadata;

#[derive(EnumMetadata, NonZeroRepr)]
enum ABC {
    C = 2,
    B = 1,
    A = 0,
}

fn main() {
}


