macro_rules! forward {
    ($(fn $method:ident(&self $(, $arg:ident: $ty:ty)*) -> $rty:ty;)*) => {
        $(
            #[inline]
            fn $method(&self $(, $arg: $ty )*) -> $rty {
                Self::$method(*self $(, $arg)*)
            }
        )*
    };
    ($(fn $method:ident(self $(, $arg:ident: $ty:ty)*) -> $rty:ty;)*) => {
        $(
            #[inline]
            fn $method(self $(, $arg: $ty )*) -> $rty {
                Self::$method(self $(, $arg)*)
            }
        )*
    };
}

macro_rules! impl_set {
    ($($operator:ident | $operation:ident => $($set:ty),*);* $(;)?) => {
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
    ($($marker:ty => $($set:ident),*);* $(;)?) => {
        $(
            $(
                impl $marker for $set {}
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
