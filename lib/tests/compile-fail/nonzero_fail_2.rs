#[allow(unused_imports)]
use enum_extra::marker::NonZeroRepr;
use enum_extra_derive::NonZeroRepr;

#[derive(NonZeroRepr)]
enum A {
    A = 0 << 1,
}

fn main() {
}


