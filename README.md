extra enum things, currently this contains 2 things:

This is currently unreleased and depends upon git branches of upstream crates,
getting it into a releasable state is currently a work in progress.

* A `OpaqueRepr` type `OpaqueRepr<YourEnum>` generic over all types which implement `EnumMetadata` from [strum](https://github.com)
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

minimum supported rust version: 1.32 (currently all features).
