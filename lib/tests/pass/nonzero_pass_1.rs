#[allow(unused_imports)]
use enum_extra::NonZeroRepr;
use strum::EnumMetadata;

#[derive(NonZeroRepr, EnumMetadata)]
#[repr(u8)]
enum A {
    A = 1 << 1,
}

fn main() {
}


