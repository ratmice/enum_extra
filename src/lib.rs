use core::cmp;
use core::marker::PhantomData;
use core::ops;
use num_traits::int::PrimInt;
use strum::EnumMetadata;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct OpaqueRepr<O>
where
    O: OpaqueMetadata,
{
    repr: <O as OpaqueMetadata>::Repr,
    _phantom_: PhantomData<O>,
}

impl<O> Clone for OpaqueRepr<O>
where
    O: OpaqueMetadata,
{
    fn clone(&self) -> Self {
        OpaqueRepr {
            repr: self.repr,
            _phantom_: PhantomData,
        }
    }
}

impl<O> Copy for OpaqueRepr<O> where O: OpaqueMetadata {}

pub trait OpaqueMetadata: EnumMetadata<Repr = <Self as OpaqueMetadata>::Repr> + Sized {
    type Repr: Copy
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
        + core::fmt::Debug
        + cmp::Eq
        + cmp::PartialEq
        + cmp::Ord
        + cmp::PartialOrd
        + PrimInt;

    type EnumT;

    fn opaque_repr(self) -> OpaqueRepr<Self> {
        OpaqueRepr {
            repr: Self::to_repr(self),
            _phantom_: PhantomData,
        }
    }
}

impl<R, E> OpaqueMetadata for E
where
    R: Copy
        + ops::BitOr<Output = R>
        + ops::BitAnd<Output = R>
        + ops::BitXor<Output = R>
        + ops::Shr<Output = R>
        + ops::Shl<Output = R>
        + ops::Not<Output = R>
        + ops::BitOrAssign
        + ops::BitAndAssign
        + ops::BitXorAssign
        + ops::ShrAssign
        + ops::ShlAssign
        + core::fmt::Debug
        + PrimInt,
    E: EnumMetadata<Repr = R>,
{
    type Repr = R;
    type EnumT = E::EnumT;
}

impl<E: OpaqueMetadata<EnumT = E> + EnumMetadata<EnumT = E>> EnumMetadata for OpaqueRepr<E> {
    type Repr = <E as EnumMetadata>::Repr;
    type EnumT = E;

    const VARIANTS: &'static [&'static str] = Self::EnumT::VARIANTS;
    const COUNT: usize = Self::EnumT::COUNT;
    const REPR_SIZE: usize = Self::EnumT::REPR_SIZE;

    fn to_repr(self) -> Self::Repr {
        self.repr
    }

    fn from_repr(repr: Self::Repr) -> Option<Self::EnumT> {
        Self::EnumT::from_repr(repr)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use strum_macros::EnumMetadata;
    #[derive(Eq, PartialEq, Ord, PartialOrd, Debug, EnumMetadata)]
    #[repr(u8)]
    enum ABC {
        A = 0,
        B = 1,
        C = 2,
    }

    #[test]
    fn test_opaque_repr_eq() {
        let foo: OpaqueRepr<ABC> = ABC::A.opaque_repr();
        assert_eq!(ABC::A.opaque_repr(), foo);
        assert_ne!(ABC::B.opaque_repr(), foo);
    }

    #[test]
    fn test_from_repr() {
        assert_eq!(Some(ABC::A), ABC::from_repr(ABC::A.to_repr()));
        assert_ne!(Some(ABC::C), ABC::from_repr(ABC::A.to_repr()));
    }

    #[test]
    fn test_from_repr_opq() {
        assert_eq!(
            OpaqueRepr::from_repr(ABC::A.to_repr()),
            ABC::from_repr(ABC::A.to_repr())
        );
        assert_ne!(
            OpaqueRepr::from_repr(ABC::B.to_repr()),
            ABC::from_repr(ABC::A.to_repr())
        );
    }

    #[test]
    fn test_to_repr_opq() {
        assert_eq!(ABC::A.to_repr(), ABC::A.opaque_repr().to_repr());
        assert_eq!(
            ABC::A.opaque_repr().to_repr(),
            ABC::A.opaque_repr().to_repr()
        );
        assert_ne!(
            ABC::B.opaque_repr().to_repr(),
            ABC::A.opaque_repr().to_repr()
        );
        assert_eq!(
            ABC::B.opaque_repr().to_repr(),
            ABC::A.opaque_repr().to_repr() + 1
        );
    }
}
