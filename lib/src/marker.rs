use strum::EnumMetadata;

pub trait NonZeroRepr: EnumMetadata {
    // TODO add an associated type
    // I guess that would technically not be a marker any longer
    // type NonZeroRepr;
}

#[cfg(test)]
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
    fn test_simple() {
        assert!(generic_derived(XYZ::X))
    }

    fn generic_derived<X: NonZeroRepr>(_foo: X) -> bool {
        true
    }
}
