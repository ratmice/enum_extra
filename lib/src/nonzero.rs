use core::{cmp, fmt};
use strum::EnumMetadata;
pub trait NonZeroRepr: EnumMetadata {
    type NonZeroRepr: Copy
        + cmp::Eq
        + cmp::Ord
        + cmp::PartialEq
        + cmp::PartialOrd
        + fmt::Display
        + fmt::Debug;

    fn nonzero_repr(self) -> Self::NonZeroRepr;
}

#[cfg(test)]
#[allow(unused)]
mod test {
    use super::*;
    use core::num::NonZeroU8;
    use enum_extra::NonZeroRepr;
    use strum::EnumMetadata;

    #[derive(NonZeroRepr, EnumMetadata, Clone, Copy)]
    #[repr(u8)]
    enum XYZ {
        X = 24,
        Y = 25,
        Z = 26,
    }

    #[test]
    fn test_literal_discriminant() {
        assert_eq!(XYZ::X.to_repr(), 24);
        assert!(generic_derived(XYZ::X))
    }

    // So convenient
    fn generic_derived<X: NonZeroRepr<NonZeroRepr = core::num::NonZeroU8, Repr = u8>>(x: X) -> bool
    where
        X: Copy,
    {
        Some(x.nonzero_repr()) == NonZeroU8::new(x.to_repr())
    }

    #[derive(NonZeroRepr, EnumMetadata, Clone, Copy)]
    #[repr(u8)]
    enum XYZZ {
        X = 1 << 0,
        Y = 1 << 1,
        Z = 1 << 2,
    }

    #[test]
    fn test_expr_discriminant() {
        assert_eq!(XYZZ::X.to_repr(), 1);
        assert!(generic_derived(XYZZ::X))
    }
}
