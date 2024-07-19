
/// Used for converting (Option<A>, Option<B>, ..., Option<K>) to Option<(A, B, ..., K)>
pub trait Meld {
    type Output;

    fn meld(self) -> Self::Output;
}

macro_rules! _impl_meld {
    ($($x:tt $n:tt),+) => {
        impl<$($x),+> Meld for ($(Option<$x>),+) {
            type Output = Option<($($x),+)>;

            /// Returns None if any of the members are None, otherwise returns Some of a tuple of the inner values
            fn meld(self) -> Self::Output {
                Some((
                    $(
                        self.$n?,
                    )+
                ))
            }
        }
    };
}

_impl_meld!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11);
_impl_meld!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10);
_impl_meld!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9);
_impl_meld!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8);
_impl_meld!(A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7);
_impl_meld!(A 0, B 1, C 2, D 3, E 4, F 5, G 6);
_impl_meld!(A 0, B 1, C 2, D 3, E 4, F 5);
_impl_meld!(A 0, B 1, C 2, D 3, E 4);
_impl_meld!(A 0, B 1, C 2, D 3);
_impl_meld!(A 0, B 1, C 2);
_impl_meld!(A 0, B 1);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_type() {
        #[derive(Debug, PartialEq, Eq)]
        struct Foo;
        
        assert_eq!(
            (Some(Foo), Some(Foo), Some(Foo)).meld(),
            Some((Foo, Foo, Foo))
        );

        assert_eq!(
            (Some(Foo), None::<Foo>, Some(Foo)).meld(),
            None
        );

        assert_eq!(
            (None::<Foo>, None::<Foo>, None::<Foo>).meld(),
            None
        );
    }

    #[test]
    fn test_mixed_type() {
        #[derive(Debug, PartialEq, Eq)]
        struct Foo;

        assert_eq!(
            (Some(154u32), Some(Foo), Some("bar")).meld(),
            Some((154u32, Foo, "bar"))
        );

        assert_eq!(
            (Some(Foo), None::<u32>, Some("bar")).meld(),
            None
        );

        assert_eq!(
            (None::<u32>, None::<&str>, None::<Foo>).meld(),
            None
        );
    }
}
