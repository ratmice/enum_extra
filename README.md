extra enum things:

This is unreleased and depends upon git branches of upstream crates,
getting it into a releasable state is a work in progress.

* A `OpaqueRepr` type `OpaqueRepr<YourEnum>` generic over all types that implement `EnumMetadata` from [strum](https://github.com)
```
  #[derive(EnumMetadata)]
  enum Foo { A };
  let foo: OpaqueRepr<Foo> = Foo::A.opaque_repr();
  let _ = (foo | Foo::A).to_repr();
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
* A `NonZeroRepr` trait/derive macro that checks at compile time that discriminants aren't zero.
```
#[derive(NonZeroRepr, EnumMetadata)]
#[repr(u8)]
enum Foo {
	A = 1,
	B = 1 << 2,
}

// This generates an associated type NonZeroRepr equal to NonZeroU8.
// The usage of which is currently rather obtuse due to the lack of any
// traits on NonZero*
let nz = Foo::A.nonzero_repr();
```

The compile time checks done by `NonZeroRepr` work with complex constant expressions such as:
```
const X: i32 = -1;

#[derive(NonZeroRepr, EnumMetadata)]
#[repr(i32)
enum CompileFail {
  // Should fail to compile.
  A = X + 1,
}
```

minimum supported rust versions by feature
* 1.32: (MaskIterator, OpaqueRepr).
* 1.57: (NonZeroRepr)

no_std: enabled by default (all features, all tests).

features:
* (claims 1.34): trybuild_tests
