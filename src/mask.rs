use crate::OpaqueMetadata;
use core::ops;
use num_traits::int::PrimInt;
use num_traits::Num;
use strum::EnumMetadata;
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MaskIterator<
    R: Copy
        + ops::BitOr
        + ops::BitAnd
        + ops::BitXor
        + ops::Shr
        + ops::Shl
        + ops::Not
        + ops::BitOrAssign
        + ops::BitAndAssign
        + ops::BitXorAssign
        + ops::ShrAssign
        + ops::ShlAssign
        + Num
        + PrimInt
        + core::fmt::Debug,
    E: OpaqueMetadata<Repr = R, EnumT = E>,
    O: OpaqueMetadata<Repr = R, EnumT = E>,
> {
    mask: R,
    step: Option<R>,
    phantom: core::marker::PhantomData<(O, R)>,
}

pub trait MaskIter: Sized
where
    Self: OpaqueMetadata,
    <Self as OpaqueMetadata>::Repr: core::ops::BitOr + core::ops::BitOrAssign,
    <Self as EnumMetadata>::EnumT: EnumMetadata,
{
    type I: Iterator<Item = <Self as OpaqueMetadata>::EnumT>;

    fn mask_iter(&self) -> Self::I;
}

impl<
        R: Copy
            + ops::BitOr
            + ops::BitAnd
            + ops::BitXor
            + ops::Shr
            + ops::Shl
            + ops::Not
            + ops::BitOrAssign
            + ops::BitAndAssign
            + ops::BitXorAssign
            + ops::ShrAssign
            + ops::ShlAssign
            + Num
            + PrimInt
            + core::fmt::Debug,
        E: OpaqueMetadata<EnumT = E, Repr = R> + EnumMetadata<EnumT = E>,
        O: Copy + OpaqueMetadata<EnumT = E, Repr = R>,
    > MaskIter for O
{
    type I = MaskIterator<R, E, O>;

    fn mask_iter(&self) -> MaskIterator<R, E, O> {
        let nextpos = |x: R| {
            let pos: usize = x.trailing_zeros() as usize;
            if pos >= <E as OpaqueMetadata>::EnumT::REPR_SIZE * 8_usize {
                None
            } else {
                let one_r: R = num_traits::identities::one();
                Some(one_r << pos)
            }
        };
        let mask = self.to_repr();

        MaskIterator {
            mask,
            step: nextpos(mask),
            phantom: core::marker::PhantomData,
        }
    }
}

impl<
        R: Copy
            + ops::BitOr
            + ops::BitAnd
            + ops::BitXor
            + ops::Shr
            + ops::Shl
            + ops::Not
            + ops::BitOrAssign
            + ops::BitAndAssign
            + ops::BitXorAssign
            + ops::ShrAssign
            + ops::ShlAssign
            + Num
            + PrimInt
            + core::fmt::Debug,
        E: OpaqueMetadata<Repr = R, EnumT = E> + EnumMetadata<EnumT = E>,
        O: OpaqueMetadata<Repr = R, EnumT = E>,
    > Iterator for MaskIterator<R, E, O>
{
    type Item = <E as OpaqueMetadata>::EnumT;

    fn next(&mut self) -> Option<Self::Item> {
        let nextpos = |x: R| {
            let pos: usize = x.trailing_zeros() as usize;
            if pos >= (<E as OpaqueMetadata>::EnumT::REPR_SIZE * 8_usize) as usize {
                None
            } else {
                let one_r: R = num_traits::identities::one();
                Some(one_r << pos)
            }
        };

        let mut ret = None;
        while let None = ret {
            if let Some(step) = &mut self.step {
                let proposed_repr = *step & self.mask;
                // Assumption: the single 1 bit in Some(step) is also 1 in self.mask.
                assert_eq!(proposed_repr, *step);
                ret = <E as OpaqueMetadata>::EnumT::from_repr(proposed_repr);
                // Strip that bit out of mask.
                self.mask ^= *step;
                self.step = nextpos(self.mask);
            } else {
                break;
            }
        }
        ret
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        // TODO if we know which bits will never be in a valid repr,
        // and that the other single bits all represent valid representations
        // We could return an exact lower == upper bounds with `count_ones()`,
        // then derive ExactSizeIterator.
        (0, Some(self.mask.count_ones() as usize))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::OpaqueRepr;
    use strum_macros::EnumMetadata;
    use strum_macros::FromRepr;

    #[derive(Debug, Eq, PartialEq, EnumMetadata, FromRepr)]
    #[repr(u8)]
    enum ABC {
        A = 1 << 0,
        B = 1 << 1,
        C = 1 << 2,
    }

    #[test]
    fn test_mask_iter() {
        let mask = ABC::A.opaque_repr();
        assert_eq!(mask.mask_iter().collect::<Vec<ABC>>(), [ABC::A]);
        let mask = ABC::A.opaque_repr() | ABC::B;
        assert_eq!(mask.mask_iter().collect::<Vec<ABC>>(), [ABC::A, ABC::B]);
        let mask = ABC::C.opaque_repr() | ABC::B.opaque_repr();
        assert_eq!(mask.mask_iter().collect::<Vec<ABC>>(), [ABC::B, ABC::C]);
    }

    #[test]
    fn test_opaque_zero_repr() {
        let mask: OpaqueRepr<ABC> = OpaqueRepr::zero();
        assert_eq!(mask.mask_iter().collect::<Vec<ABC>>(), []);
    }

    #[derive(Debug, Eq, PartialEq, EnumMetadata)]
    #[repr(u8)]
    enum ReprBoundary {
        End = 1 << 7,
    }

    #[test]
    fn test_repr_boundary() {
        let mask = ReprBoundary::End.opaque_repr();
        assert_eq!(
            mask.mask_iter().collect::<Vec<ReprBoundary>>(),
            [ReprBoundary::End]
        );
    }

    #[derive(Debug, Eq, PartialEq, EnumMetadata)]
    #[repr(u8)]
    enum ReprSaturated {
        A = 1 << 0,
        B = 1 << 1,
        C = 1 << 2,
        D = 1 << 3,
        E = 1 << 4,
        F = 1 << 5,
        G = 1 << 6,
        H = 1 << 7,
    }

    #[test]
    fn test_repr_saturated() {
        use ReprSaturated::*;
        let mask = A.opaque_repr();
        assert_eq!(mask.mask_iter().collect::<Vec<ReprSaturated>>(), [A]);
        let mask = A.opaque_repr() | H;
        assert_eq!(mask.mask_iter().collect::<Vec<ReprSaturated>>(), [A, H]);
        let mask = B.opaque_repr() | H;
        assert_eq!(mask.mask_iter().collect::<Vec<ReprSaturated>>(), [B, H]);
        let mask = B.opaque_repr() | G;
        assert_eq!(mask.mask_iter().collect::<Vec<ReprSaturated>>(), [B, G]);
        let mask = G.opaque_repr() | B;
        assert_eq!(mask.mask_iter().collect::<Vec<ReprSaturated>>(), [B, G]);
    }

    // Neither of these variants fulfill the constraints on variants
    // required for EnumMaskIter to work
    //
    // In particular all reprs should have a single unique 1 bit.
    // Ox0 has zero unique 1 bits, and Ox11 has 2.
    //
    // Just check that it handles being given these gracefully.
    #[derive(Debug, Eq, PartialEq, EnumMetadata)]
    enum UnsupportedByMaskIter {
        Ox0 = 0,
        Ox11 = 0x3,
    }

    #[test]
    fn test_mask_iter_enum_constraints() {
        use UnsupportedByMaskIter::*;

        let mask = Ox0.opaque_repr();
        assert_eq!(mask.mask_iter().collect::<Vec<UnsupportedByMaskIter>>(), []);

        let mask = Ox11.opaque_repr();
        assert_eq!(mask.mask_iter().collect::<Vec<UnsupportedByMaskIter>>(), []);
    }
}
