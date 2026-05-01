/// A minimal functor interface for this tutorial.
pub trait Functor<A, B> {
    type WrappedA;
    type WrappedB;

    fn fmap<F>(wrapped: Self::WrappedA, f: F) -> Self::WrappedB
    where
        F: Fn(A) -> B;
}

pub struct VecFunctor;

impl<A, B> Functor<A, B> for VecFunctor {
    type WrappedA = Vec<A>;
    type WrappedB = Vec<B>;

    fn fmap<F>(wrapped: Vec<A>, f: F) -> Vec<B>
    where
        F: Fn(A) -> B,
    {
        wrapped.into_iter().map(f).collect()
    }
}

pub struct OptionFunctor;

impl<A, B> Functor<A, B> for OptionFunctor {
    type WrappedA = Option<A>;
    type WrappedB = Option<B>;

    fn fmap<F>(wrapped: Option<A>, f: F) -> Option<B>
    where
        F: Fn(A) -> B,
    {
        wrapped.map(f)
    }
}

/// A structure-preserving conversion between wrappers.
pub trait NaturalTransformation<A> {
    type From;
    type To;

    fn transform(from: Self::From) -> Self::To;
}

/// Natural transformation from `Vec<A>` to `Option<A>` by taking the first item.
pub struct VecToFirstOption;

impl<A> NaturalTransformation<A> for VecToFirstOption {
    type From = Vec<A>;
    type To = Option<A>;

    fn transform(from: Vec<A>) -> Option<A> {
        from.into_iter().next()
    }
}

pub fn naturality_square_holds_for_first_option() -> bool {
    let xs = vec![1, 2, 3];
    let f = |x| x * 10;

    let path_top_then_right = VecToFirstOption::transform(VecFunctor::fmap(xs.clone(), f));
    let path_left_then_bottom = OptionFunctor::fmap(VecToFirstOption::transform(xs), f);

    path_top_then_right == path_left_then_bottom
}

pub trait Monoid: Sized {
    fn empty() -> Self;
    fn combine(&self, other: &Self) -> Self;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TraceStep(&'static str);

impl TraceStep {
    pub fn new(name: &'static str) -> Self {
        Self(name)
    }

    pub fn name(&self) -> &'static str {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineTrace(Vec<TraceStep>);

impl PipelineTrace {
    pub fn from_steps(steps: impl IntoIterator<Item = TraceStep>) -> Self {
        Self(steps.into_iter().collect())
    }

    pub fn names(&self) -> Vec<&'static str> {
        self.0.iter().map(TraceStep::name).collect()
    }
}

impl Monoid for PipelineTrace {
    fn empty() -> Self {
        PipelineTrace(vec![])
    }

    fn combine(&self, other: &Self) -> Self {
        let mut combined = self.0.clone();
        combined.extend_from_slice(&other.0);
        PipelineTrace(combined)
    }
}

pub fn monoid_laws_hold_for_pipeline_trace() -> bool {
    let a = PipelineTrace::from_steps(vec![TraceStep::new("embedding")]);
    let b = PipelineTrace::from_steps(vec![TraceStep::new("linear")]);
    let c = PipelineTrace::from_steps(vec![TraceStep::new("softmax")]);
    let identity = PipelineTrace::empty();

    let left_identity = identity.combine(&a) == a;
    let right_identity = a.combine(&identity) == a;
    let associativity = a.combine(&b).combine(&c) == a.combine(&b.combine(&c));

    left_identity && right_identity && associativity
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn naturality_square_commutes() {
        assert!(naturality_square_holds_for_first_option());
    }

    #[test]
    fn pipeline_trace_obeys_monoid_laws() {
        assert!(monoid_laws_hold_for_pipeline_trace());
    }
}
