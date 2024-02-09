use core::marker::PhantomData;
use crate::dialect::Dialect;
use crate::field::selection::Selection;
use crate::operation::bind::Bind;
use crate::operation::select::condition::{Condition, SelectIf};
use crate::response::From;

pub trait Select<D: Dialect, T: From<D>> {
    fn selection() -> Selection<T>;
}

impl<D: Dialect, T: From<D>, O: Select<D, T>> Bind<D, T> for O {
}

pub struct All<D, S>(PhantomData<(D, S)>);

impl<D, T> All<D, T>
where D: Dialect,
      T: From<D> {
    #[inline]
    #[must_use]
    pub const fn only_if<C>(self, condition: Condition<C>) -> SelectIf<D, T, C> {
        SelectIf { condition, _marker: PhantomData }
    }
}

pub mod condition {
    use core::marker::PhantomData;
    use crate::field::Field;

    #[non_exhaustive]
    pub struct Condition<'value, T> {
        pub field: Field,
        pub op: Op,
        pub value: &'value T
    }

    #[non_exhaustive]
    #[derive(Copy, Clone)]
    pub enum Op {
        Eq,
        NotEq,
        Greater,
        GreaterEq,
        Less,
        LessEq
    }

    impl AsRef<str> for Op {
        #[inline]
        fn as_ref(&self) -> &str {
            match *self {
                Self::Eq => "=",
                Self::NotEq => "!=",
                Self::Greater => ">",
                Self::GreaterEq => ">=",
                Self::Less => "<",
                Self::LessEq => "<=",
            }
        }
    }

    #[non_exhaustive]
    pub struct SelectIf<'value, D, T, C> {
        pub condition: Condition<'value, C>,
        pub _marker: PhantomData<(D, T)>,
    }
}