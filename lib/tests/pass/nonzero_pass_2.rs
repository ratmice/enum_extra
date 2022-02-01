#[allow(unused_imports)]
use enum_extra::NonZeroRepr;
use strum::EnumMetadata;

// Not sure what kind of macro-rules token #[repr(..)] wants
#[derive(NonZeroRepr, EnumMetadata)]
#[repr(u8)]
enum TestU8 {
    A = 1 << 1,
}
#[derive(NonZeroRepr, EnumMetadata)]
#[repr(u16)]
enum TestU16 {
    A = 1 << 1,
}

#[derive(NonZeroRepr, EnumMetadata)]
#[repr(u32)]
enum TestU32 {
    A = 1 << 1,
}

#[derive(NonZeroRepr, EnumMetadata)]
#[repr(u64)]
enum TestU64 {
    A = 1 << 1,
}

#[derive(NonZeroRepr, EnumMetadata)]
#[repr(usize)]
enum TestUsize {
    A = 1 << 1,
}


#[derive(NonZeroRepr, EnumMetadata)]
#[repr(i8)]
enum TestI8 {
    A = 1 << 1,
}
#[derive(NonZeroRepr, EnumMetadata)]
#[repr(i16)]
enum TestI16 {
    A = 1 << 1,
}

#[derive(NonZeroRepr, EnumMetadata)]
#[repr(i32)]
enum TestI32 {
    A = 1 << 1,
}

#[derive(NonZeroRepr, EnumMetadata)]
#[repr(i64)]
enum TestI64 {
    A = 1 << 1,
}

#[derive(NonZeroRepr, EnumMetadata)]
#[repr(isize)]
enum TestIsize {
    A = 1 << 1,
}

fn main() {
}


