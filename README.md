extra enum things, currently this contains 2 things:

This is currently unreleased and depends upon git branches of upstream crates,
getting it into a releasable state is currently a work in progress.

* A `OpaqueRepr` type `OpaqueRepr<YourEnum>` generic over all types which implement `EnumMetadata` from [strum](https://github.com)
```
  enum Foo { A };
  let foo: OpaqueRepr<Foo> = Foo::A;
```
* A `MaskIterator` which iterates over unique single bit non-zero enum variants.
```
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
```
* A `NonZeroRepr` trait/derive macro which checks that your descriminants aren't zero.
```
#[derive(NonZeroRepr, EnumMetadata)]
#[repr(u8)]
enum Foo {
	A = 1,
	A = 1 << 2,
}
```

minimum supported rust versions by feature
1.32: (MaskIterator, OpaqueRepr).
1.57: (NonZeroRepr)
unknown: trybuild_tests
no_std: enabled by default (all features, all tests).
