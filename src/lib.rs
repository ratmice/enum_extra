use core::cmp;
use core::marker::PhantomData;
use core::ops;
use num_traits::int::PrimInt;
use ops::{
    Add, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, Mul, Not, Rem, Shl,
    ShlAssign, Shr, ShrAssign, Sub,
};
use strum::EnumMetadata;

pub mod mask;

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

// Currently this implements *all* arithmetic traits,
// I kind of think it would be better to just add the traits you need in various
// OpaqueRepr newtypes,
//
// For instantce e.g. MaskIterator may just need the `Shl`, `PrimInt`, `BitXor` `BitOr`, ...
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

impl<E: EnumMetadata + OpaqueMetadata> OpaqueRepr<E> {
    pub fn zero() -> OpaqueRepr<E> {
        OpaqueRepr::<E> {
            repr: num_traits::identities::zero(),
            _phantom_: PhantomData,
        }
    }

    // Not unsafe but logically unchecked that from_repr will return Some(repr).
    fn from_repr_unchecked(repr: <E as OpaqueMetadata>::Repr) -> OpaqueRepr<E> {
        OpaqueRepr::<E> {
            repr,
            _phantom_: PhantomData,
        }
    }
}

macro_rules! binary_op {
    ($trait:ident, $op:ident) => {
        impl<E: EnumMetadata + OpaqueMetadata> $trait<E> for OpaqueRepr<E>
        where
            Self: EnumMetadata<Repr = <E as EnumMetadata>::Repr>,
            <E as EnumMetadata>::Repr: $trait<Output = <E as EnumMetadata>::Repr>,
        {
            type Output = Self;
            fn $op(self, other: E) -> OpaqueRepr<E> {
                Self::from_repr_unchecked(<Self as EnumMetadata>::Repr::$op(
                    self.to_repr(),
                    other.to_repr(),
                ))
            }
        }

        impl<E: EnumMetadata + OpaqueMetadata> $trait<Self> for OpaqueRepr<E>
        where
            Self: EnumMetadata<Repr = <E as EnumMetadata>::Repr>,
            <E as EnumMetadata>::Repr: $trait<Output = <E as EnumMetadata>::Repr>,
        {
            type Output = Self;
            fn $op(self, other: OpaqueRepr<E>) -> OpaqueRepr<E> {
                Self::from_repr_unchecked(<Self as EnumMetadata>::Repr::$op(
                    self.to_repr(),
                    other.to_repr(),
                ))
            }
        }
    };
}

macro_rules! unary_op {
    ($trait:ident, $f:ident) => {
        impl<E: EnumMetadata + OpaqueMetadata> $trait for OpaqueRepr<E>
        where
            Self: OpaqueMetadata<Repr = <E as OpaqueMetadata>::Repr>,
            <E as OpaqueMetadata>::Repr: $trait<Output = <E as EnumMetadata>::Repr>,
        {
            type Output = Self;
            fn $f(self) -> OpaqueRepr<E> {
                Self::from_repr_unchecked(<Self as EnumMetadata>::Repr::$f(self.to_repr()))
            }
        }
    };
}

macro_rules! binary_op_mut {
    ($trait:ident, $op:ident) => {
        impl<E: EnumMetadata + OpaqueMetadata> $trait<E> for OpaqueRepr<E>
        where
            Self: OpaqueMetadata<Repr = <E as OpaqueMetadata>::Repr>,
            <E as OpaqueMetadata>::Repr: $trait,
        {
            fn $op(&mut self, other: E) {
                <Self as EnumMetadata>::Repr::$op(&mut self.to_repr(), other.to_repr());
            }
        }

        impl<E: EnumMetadata + OpaqueMetadata> $trait<Self> for OpaqueRepr<E>
        where
            Self: OpaqueMetadata<Repr = <E as OpaqueMetadata>::Repr>,
            <E as OpaqueMetadata>::Repr: $trait,
        {
            fn $op(&mut self, other: OpaqueRepr<E>) {
                <Self as OpaqueMetadata>::Repr::$op(&mut self.to_repr(), other.to_repr());
            }
        }
    };
}

binary_op!(BitOr, bitor);
binary_op!(BitAnd, bitand);
binary_op!(BitXor, bitxor);
binary_op!(Shr, shr);
binary_op!(Shl, shl);
unary_op!(Not, not);

binary_op!(Add, add);
binary_op!(Sub, sub);
binary_op!(Mul, mul);
binary_op!(Div, div);
binary_op!(Rem, rem);

binary_op_mut!(BitOrAssign, bitor_assign);
binary_op_mut!(BitAndAssign, bitand_assign);
binary_op_mut!(BitXorAssign, bitxor_assign);
binary_op_mut!(ShrAssign, shr_assign);
binary_op_mut!(ShlAssign, shl_assign);

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

        // For better or worse.
        assert_eq!(
            ABC::B.opaque_repr().to_repr(),
            (ABC::A.opaque_repr() + ABC::B.opaque_repr()).to_repr()
        );

        assert_eq!(
            OpaqueRepr::<ABC>::zero().to_repr(),
            ABC::A.opaque_repr().to_repr(),
        )
    }
}
