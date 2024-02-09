use crate::field::Field;
use crate::operation::select::condition::Condition;
use crate::operation::select::condition::Op;

pub trait Utils {
    fn eq<V>(self, value: &V) -> Condition<V>;

    fn not_eq<V>(self, value: &V) -> Condition<V>;

    fn greater<V>(self, value: &V) -> Condition<V>;

    fn greater_eq<V>(self, value: &V) -> Condition<V>;

    fn less<V>(self, value: &V) -> Condition<V>;

    fn less_eq<V>(self, value: &V) -> Condition<V>;
}

impl Utils for Field {
    #[inline]
    fn eq<V>(self, value: &V) -> Condition<V> {
        Condition { field: self, op: Op::Eq, value }
    }

    #[inline]
    fn not_eq<V>(self, value: &V) -> Condition<V> {
        Condition { field: self, op: Op::NotEq, value }
    }

    #[inline]
    fn greater<V>(self, value: &V) -> Condition<V> {
        Condition { field: self, op: Op::Greater, value }
    }

    #[inline]
    fn greater_eq<V>(self, value: &V) -> Condition<V> {
        Condition { field: self, op: Op::GreaterEq, value }
    }

    #[inline]
    fn less<V>(self, value: &V) -> Condition<V> {
        Condition { field: self, op: Op::Less, value }
    }

    #[inline]
    fn less_eq<V>(self, value: &V) -> Condition<V> {
        Condition { field: self, op: Op::LessEq, value }
    }
}