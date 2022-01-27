pub trait NonZeroRepr {
    // TODO add an associated type
    // I guess that would technically not be a marker any longer
    // type NonZeroRepr;
}

#[cfg(test)]
#[allow(unused)]
mod test {
    use super::*;
    use enum_extra_derive::NonZeroRepr;
    use strum::EnumMetadata;
    use strum_macros::EnumMetadata;

    #[derive(NonZeroRepr, EnumMetadata)]
    enum XYZ {
        X = 24,
        Y = 25,
        Z = 26,
    }

    #[test]
    fn test_literal_discriminant() {
        assert!(generic_derived(XYZ::X))
    }

    fn generic_derived<X: NonZeroRepr>(_foo: X) -> bool {
        true
    }

    #[derive(NonZeroRepr)]
    enum XYZZ {
        X = 1 << 0,
        Y = 1 << 1,
        Z = 1 << 2,
    }

    #[test]
    fn test_expr_discriminant() {
        assert!(generic_derived(XYZZ::X))
    }
}
