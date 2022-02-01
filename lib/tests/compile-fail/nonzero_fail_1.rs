#[allow(unused_imports)]
use enum_extra::NonZeroRepr;
use strum::EnumMetadata;

#[derive(EnumMetadata, NonZeroRepr)]
#[repr(u8)]
enum ABC {
    C = 2,
    B = 1,
    A = 0,
}

fn main() {
}


