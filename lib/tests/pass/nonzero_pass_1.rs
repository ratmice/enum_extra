#[allow(unused_imports)]
use enum_extra::NonZeroRepr;
use enum_extra_derive::NonZeroRepr;
use strum::EnumMetadata;
use strum_macros::EnumMetadata;

#[derive(NonZeroRepr, EnumMetadata)]
#[repr(u8)]
enum A {
    A = 1 << 1,
}

fn main() {
}

