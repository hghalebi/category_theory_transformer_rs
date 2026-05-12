use std::marker::PhantomData;

use crate::error::CtResult;

/// A typed category-theory arrow: `Input -> Output`.
pub trait Morphism<Input, Output> {
    fn name(&self) -> &'static str;
    fn apply(&self, input: Input) -> CtResult<Output>;
}

/// Identity morphism: `id_A : A -> A`.
#[derive(Debug, Clone, Copy)]
pub struct Identity<T> {
    _marker: PhantomData<T>,
}

impl<T> Identity<T> {
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<T> Default for Identity<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Morphism<T, T> for Identity<T> {
    fn name(&self) -> &'static str {
        "identity"
    }

    fn apply(&self, input: T) -> CtResult<T> {
        Ok(input)
    }
}

/// Composition of two morphisms: if `f : A -> B` and `g : B -> C`, this is
/// `g after f : A -> C`.
#[derive(Debug, Clone)]
pub struct Compose<F, G, Middle> {
    first: F,
    second: G,
    _middle: PhantomData<Middle>,
}

impl<F, G, Middle> Compose<F, G, Middle> {
    pub fn new(first: F, second: G) -> Self {
        Self {
            first,
            second,
            _middle: PhantomData,
        }
    }
}

impl<Input, Middle, Output, F, G> Morphism<Input, Output> for Compose<F, G, Middle>
where
    F: Morphism<Input, Middle>,
    G: Morphism<Middle, Output>,
{
    fn name(&self) -> &'static str {
        "composition"
    }

    fn apply(&self, input: Input) -> CtResult<Output> {
        let middle = self.first.apply(input)?;
        self.second.apply(middle)
    }
}

/// Endomorphism: a morphism from a type back to itself.
pub trait Endomorphism<T>: Morphism<T, T> {}

impl<T, M> Endomorphism<T> for M where M: Morphism<T, T> {}

/// How many times to repeat an endomorphism.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StepCount(usize);

impl StepCount {
    pub fn new(value: usize) -> Self {
        Self(value)
    }

    pub fn value(&self) -> usize {
        self.0
    }
}

/// Repeatedly apply an endomorphism: `A0 -> A1 -> ... -> An`.
pub fn apply_endomorphism_n_times<T, E>(endo: &E, mut value: T, count: StepCount) -> CtResult<T>
where
    E: Endomorphism<T>,
{
    for _ in 0..count.value() {
        value = endo.apply(value)?;
    }

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::CtError;

    #[derive(Debug, Clone, Copy)]
    struct AddOne;

    impl Morphism<i32, i32> for AddOne {
        fn name(&self) -> &'static str {
            "add_one"
        }

        fn apply(&self, input: i32) -> CtResult<i32> {
            Ok(input + 1)
        }
    }

    #[derive(Debug, Clone, Copy)]
    struct Double;

    impl Morphism<i32, i32> for Double {
        fn name(&self) -> &'static str {
            "double"
        }

        fn apply(&self, input: i32) -> CtResult<i32> {
            Ok(input * 2)
        }
    }

    #[derive(Debug, Clone, Copy)]
    struct Fail;

    impl Morphism<i32, i32> for Fail {
        fn name(&self) -> &'static str {
            "fail"
        }

        fn apply(&self, _input: i32) -> CtResult<i32> {
            Err(CtError::InvalidQuantity {
                kind: "test morphism",
                value: -1,
            })
        }
    }

    #[test]
    fn identity_returns_the_same_value() -> CtResult<()> {
        let value = String::from("same");

        assert_eq!(Identity::<String>::new().apply(value.clone())?, value);
        Ok(())
    }

    #[test]
    fn identity_composes_without_changing_behavior() -> CtResult<()> {
        let left_identity = Compose::<_, _, i32>::new(Identity::<i32>::new(), AddOne);
        let right_identity = Compose::<_, _, i32>::new(AddOne, Identity::<i32>::new());

        assert_eq!(left_identity.apply(41)?, AddOne.apply(41)?);
        assert_eq!(right_identity.apply(41)?, AddOne.apply(41)?);
        Ok(())
    }

    #[test]
    fn composition_applies_first_then_second() -> CtResult<()> {
        let add_then_double = Compose::<_, _, i32>::new(AddOne, Double);

        assert_eq!(add_then_double.apply(4)?, 10);
        Ok(())
    }

    #[test]
    fn composition_returns_the_first_error() {
        let composed = Compose::<_, _, i32>::new(Fail, AddOne);

        assert!(matches!(
            composed.apply(4),
            Err(CtError::InvalidQuantity {
                kind: "test morphism",
                value: -1,
            })
        ));
    }
}
