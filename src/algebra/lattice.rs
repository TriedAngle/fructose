use std::cmp::Ordering;

pub trait MeetSemilattice: Sized {
    fn infimum(&self, rhs: &Self) -> Self;
}

pub trait JoinSemilattice: Sized {
    fn supremum(&self, rhs: &Self) -> Self;
}

pub trait Lattice: MeetSemilattice + JoinSemilattice + PartialOrd {
    #[inline]
    fn infimum_supremum(&self, rhs: &Self) -> (Self, Self) {
        (self.infimum(rhs), self.supremum(rhs))
    }

    #[inline]
    fn partial_min<'a>(&'a self, other: &'a Self) -> Option<&'a Self> {
        if let Some(ord) = self.partial_cmp(other) {
            match ord {
                Ordering::Greater => Some(other),
                _ => Some(self),
            }
        } else {
            None
        }
    }

    #[inline]
    fn partial_max<'a>(&'a self, other: &'a Self) -> Option<&'a Self> {
        if let Some(ord) = self.partial_cmp(other) {
            match ord {
                Ordering::Less => Some(other),
                _ => Some(self),
            }
        } else {
            None
        }
    }
}

macro_rules! impl_lattice {
    ($($set:ident)*) => {
        $(
            impl MeetSemilattice for $set {
                #[inline]
                fn infimum(&self, other: &Self) -> Self {
                    if *self <= *other {
                        *self
                    }
                    else {
                        *other
                    }
                }
            }

            impl JoinSemilattice for $set {
                #[inline]
                fn supremum(&self, other: &Self) -> Self {
                    if *self >= *other {
                        *self
                    }
                    else {
                        *other
                    }
                }
            }

            impl Lattice for $set {
                #[inline]
                fn infimum_supremum(&self, other: &Self) -> (Self, Self) {
                    if *self >= *other {
                        (*other, *self)
                    }
                    else {
                        (*self, *other)
                    }
                }
            }
        )*
    }
}

impl_lattice!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize f32 f64);
