#[allow(unused_imports)]
use enum_extra::marker::NonZeroRepr;
use enum_extra_derive::NonZeroRepr;
use strum::EnumMetadata;
use strum_macros::EnumMetadata;

#[derive(EnumMetadata, NonZeroRepr)]
enum A {
    A = 0 << 1,
}

fn main() {
}


