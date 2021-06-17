use crate::linspace::{LinSpace, Linear, LinearInterpolation};
use core::ops::Range;
use num_traits::real::Real;

/// Iterator returned by [`arange`]
pub type Arange<T> = LinSpace<T>;

/// Create a new iterator over the range, stepping by `step` each time
/// This allows you to create simple float iterators
///
/// ```
/// use iter_num_tools::arange;
///
/// let it = arange(0.0..2.0, 0.5);
/// assert!(it.eq(vec![0.0, 0.5, 1.0, 1.5]));
/// ```
pub fn arange<R, F>(range: R, step: F) -> Arange<F>
where
    R: IntoArange<F>,
{
    range.into_arange(step)
}

/// Used by [`arange`]
pub trait IntoArange<F> {
    /// Convert self into an [`Arange`]
    fn into_arange(self, step: F) -> Arange<F>;
}

impl<F> IntoArange<F> for Range<F>
where
    (Range<F>, F): Into<ArangeImpl<F>>,
{
    fn into_arange(self, step: F) -> Arange<F> {
        let ArangeImpl { interpolate, steps } = (self, step).into();
        LinSpace::new(steps, interpolate)
    }
}

pub struct ArangeImpl<T> {
    pub interpolate: LinearInterpolation<T>,
    pub steps: usize,
}

impl<F: Real + Linear> From<(Range<F>, F)> for ArangeImpl<F> {
    fn from((range, step): (Range<F>, F)) -> Self {
        let Range { start, end } = range;

        ArangeImpl {
            interpolate: LinearInterpolation { start, step },
            steps: ((end - start) / step).ceil().to_usize().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arange() {
        let it = arange(0.0..2.0, 0.5);
        assert!(it.eq(vec![0.0, 0.5, 1.0, 1.5]));
    }
}
