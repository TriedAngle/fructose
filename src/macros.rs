macro_rules! impl_set {
    ($($operator:ident | $operation:ident => $($set:ident),*);* $(;)?) => {
        $(
            $(
                impl Set<$operator> for $set {
                    #[inline]
                    fn operate(&self, rhs: Self) -> Self {
                        self.$operation(rhs)
                    }
                }
            )*
        )*
    }
}

macro_rules! impl_identity {
    ($($identity:expr => $operator:ident => $($t:ty),*);* $(;)?) => {
        $(
            $(
                impl Identity<$operator> for $t {
                    #[inline]
                    fn identity() -> $t {
                        $identity
                    }

                    #[inline]
                    fn is_identity(&self) -> bool {
                        *self == $identity
                    }
                }
            )*
        )*
    }
}

macro_rules! impl_properties {
    ($($marker:ident<$operator:ident> => $($set:ident),*);* $(;)?) => {
        $(
            $(
                impl $marker<$operator> for $set {}
            )*
        )*
    }
}

macro_rules! impl_invertible_add {
    ($($set:ident)*) => {
        $(
            impl Invertible<Additive> for $set {
                #[inline]
                fn inverse(&self) -> Self {
                    -*self
                }

                #[inline]
                fn inverted(&mut self) {
                    *self = -*self
                }
            }
        )*
    };
}

macro_rules! impl_invertible_mul {
    ($($set:ident)*) => {
        $(
            impl Invertible<Multiplicative> for $set {
                #[inline]
                fn inverse(&self) -> Self {
                    1.0 / self
                }

                #[inline]
                fn inverted(&mut self) {
                    *self = 1.0 / *self
                }
            }
        )*
    }
}

macro_rules! impl_module {
    ($($set:ident)*) => {
        $(
            impl Module for $set {
                type Ring = $set;
            }
        )*
    }
}

macro_rules! impl_commutative_module {
    ($($set:ident)*) => {
        $(
            impl CommutativeModule for $set {
                type Ring = $set;
            }
        )*
    }
}

macro_rules! impl_vector_space {
    ($($set:ident)*) => {
        $(
            impl VectorSpace for $set {
                type Field = $set;
            }
        )*
    }
}
