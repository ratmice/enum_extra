use num_traits::int::PrimInt;
use core::ops;
use core::marker::PhantomData;
use strum::EnumMetadata;
use core::cmp;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct OpaqueRepr<O>
where O: OpaqueMetadata
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

impl<O> Copy for OpaqueRepr<O>
where
    O: OpaqueMetadata,
{
}


pub trait OpaqueMetadata: EnumMetadata<Repr=<Self as OpaqueMetadata>::Repr> + Sized
{
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

    fn to_repr(self) -> <Self as OpaqueMetadata>::Repr
    {
       <Self as EnumMetadata>::to_repr(self)
    }

    fn opaque_repr(self) -> OpaqueRepr<Self>
    {
        let repr: <Self as OpaqueMetadata>::Repr = <Self as OpaqueMetadata>::to_repr(self);
        OpaqueRepr {
            repr,
            _phantom_: PhantomData,
        }
    }
}

impl<R, E> OpaqueMetadata for E 
where R:  Copy
        + ops::BitOr<Output=R>
        + ops::BitAnd<Output=R>
        + ops::BitXor<Output=R>
        + ops::Shr<Output=R>
        + ops::Shl<Output=R>
        + ops::Not<Output=R>
        + ops::BitOrAssign
        + ops::BitAndAssign
        + ops::BitXorAssign
        + ops::ShrAssign
        + ops::ShlAssign
        + core::fmt::Debug
        + PrimInt,
	E: EnumMetadata<Repr=R>
{
    type Repr = R;
    type EnumT = E::EnumT;
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
    fn test() {
        assert_eq!(ABC::A.opaque_repr(), ABC::A.opaque_repr());
        assert_ne!(ABC::A.opaque_repr(), ABC::B.opaque_repr());
    }
}
