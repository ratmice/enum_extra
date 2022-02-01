#[allow(unused_imports)]
use enum_extra::NonZeroRepr;
use strum::EnumMetadata;

#[derive(NonZeroRepr, EnumMetadata)]
#[repr(u8)]
enum XYZZZ {
    A,
}


fn main() {
}
