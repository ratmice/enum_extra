fn main() {
    use enum_extra::mask::Mask;
    use enum_extra::{OpaqueMetadata, OpaqueRepr};
    use strum_macros::EnumMetadata;
    use strum::EnumMetadata;

    #[derive(EnumMetadata, Debug, Eq, PartialEq)]
    enum Foo {
        Bar = 1 << 0,
        Baz = 1 << 1,
    }

    let repr: OpaqueRepr<Foo> = Foo::Bar.opaque_repr() | Foo::Baz;
    let things = repr.mask_iter().collect::<Vec<Foo>>();
    assert_eq!(things, [Foo::Bar, Foo::Baz]);
}
