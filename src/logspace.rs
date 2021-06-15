use core::{
    iter::FusedIterator,
    ops::{Add, Div, Mul, Range, RangeInclusive, Sub},
};
use num_traits::{real::Real, FromPrimitive};

/// Creates a logarithmic space over range with a fixed number of steps
///
/// ```
/// use iter_num_tools::log_space;
/// use itertools::zip_eq;
///
/// // Inclusive
/// let it = log_space(1.0..=1000.0, 4);
/// let expected: [f64; 4] = [1.0, 10.0, 100.0, 1000.0];
///
/// // all approx equal
/// assert!(zip_eq(it, expected).all(|(x, y)| (x-y).abs() < 1e-10));
///
/// // Exclusive
/// let it = log_space(1.0..1000.0, 3);
/// let expected: [f64; 3] = [1.0, 10.0, 100.0];
///
/// // all approx equal
/// assert!(zip_eq(it, expected).all(|(x, y)| (x-y).abs() < 1e-10));
/// ```
pub fn log_space<R, T>(range: R, steps: usize) -> LogSpace<T>
where
    R: IntoLogSpace<T>,
{
    range.into_log_space(steps)
}

/// Used by [`log_space`]
pub trait IntoLogSpace<T> {
    /// Convert self into a [`LogSpace`]
    fn into_log_space(self, steps: usize) -> LogSpace<T>;
}

impl<T> IntoLogSpace<T> for RangeInclusive<T>
where
    T: Logarithmic,
{
    fn into_log_space(self, steps: usize) -> LogSpace<T> {
        LogSpace {
            x: 0,
            steps,
            lerp: (self, steps).into(),
        }
    }
}

impl<T> IntoLogSpace<T> for Range<T>
where
    T: Logarithmic,
{
    fn into_log_space(self, steps: usize) -> LogSpace<T> {
        LogSpace {
            x: 0,
            steps,
            lerp: (self, steps).into(),
        }
    }
}

/// Trait required for [`log_space`] implementations.
pub trait Logarithmic:
    FromPrimitive
    + Real
    + Mul<Output = Self>
    + Sub<Output = Self>
    + Add<Output = Self>
    + Div<Output = Self>
    + Copy
{
}
impl<T> Logarithmic for T where
    T: FromPrimitive + Real + Mul<Output = Self> + Div<Output = Self> + Copy
{
}

#[derive(Clone, Copy, Debug)]
pub struct LogInterpolation<T> {
    pub start: T,
    pub step: T,
}

impl<T: Logarithmic> LogInterpolation<T> {
    #[inline]
    pub fn lerp(self, x: usize) -> T {
        let Self { start, step } = self;
        start * step.powi(x as i32)
    }
}

impl<T: Logarithmic> From<(Range<T>, usize)> for LogInterpolation<T> {
    fn from((range, steps): (Range<T>, usize)) -> Self {
        let Range { start, end } = range;
        let step = (end / start).powf(T::from_usize(steps).unwrap().recip());
        Self { start, step }
    }
}

impl<T: Logarithmic> From<(RangeInclusive<T>, usize)> for LogInterpolation<T> {
    fn from((range, steps): (RangeInclusive<T>, usize)) -> Self {
        let (start, end) = range.into_inner();
        let step = (end / start).powf(T::from_usize(steps - 1).unwrap().recip());
        Self { start, step }
    }
}

/// Iterator returned by [`log_space`]
#[derive(Clone, Debug)]
pub struct LogSpace<T> {
    pub(crate) x: usize,
    pub(crate) steps: usize,
    pub(crate) lerp: LogInterpolation<T>,
}

impl<T: Logarithmic> Iterator for LogSpace<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x < self.steps {
            let n = self.x + 1;
            Some(self.lerp.lerp(core::mem::replace(&mut self.x, n)))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<T: Logarithmic> DoubleEndedIterator for LogSpace<T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.x < self.steps {
            self.steps -= 1;
            Some(self.lerp.lerp(self.steps))
        } else {
            None
        }
    }
}

impl<T: Logarithmic> ExactSizeIterator for LogSpace<T> {
    #[inline]
    fn len(&self) -> usize {
        self.steps - self.x
    }
}

impl<T: Logarithmic> FusedIterator for LogSpace<T> {}

#[cfg(feature = "trusted_len")]
use core::iter::TrustedLen;
#[cfg(feature = "trusted_len")]
unsafe impl<T: Logarithmic> TrustedLen for LogSpace<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    use itertools::zip_eq;

    #[test]
    fn test_log_space_inclusive() {
        let it = log_space(1.0..=1000.0, 4);
        assert!(zip_eq(it, [1.0, 10.0, 100.0, 1000.0]).all(|(a, b)| (a - b).abs() < 1e-10))
    }

    #[test]
    fn test_log_space_exclusive() {
        let it = log_space(1.0..1000.0, 3);
        assert!(zip_eq(it, [1.0, 10.0, 100.0]).all(|(a, b)| (a - b).abs() < 1e-10))
    }

    #[test]
    fn test_log_space_inclusive_rev() {
        let it = log_space(1.0..=1000.0, 4);
        assert!(zip_eq(it.rev(), [1000.0, 100.0, 10.0, 1.0]).all(|(a, b)| (a - b).abs() < 1e-10))
    }

    #[test]
    fn test_log_space_exclusive_rev() {
        let it = log_space(1.0..1000.0, 3);
        assert!(zip_eq(it.rev(), [100.0, 10.0, 1.0]).all(|(a, b)| (a - b).abs() < 1e-10))
    }

    #[test]
    fn test_log_space_exclusive_len() {
        let mut it = log_space(1.0..=1000.0, 4);
        let mut expected_len = 4;

        assert_eq!(it.size_hint(), (expected_len, Some(expected_len)));

        while expected_len > 0 {
            assert_eq!(it.len(), expected_len);
            it.next();
            expected_len -= 1;

            assert_eq!(it.len(), expected_len);
            it.next_back();
            expected_len -= 1;
        }

        assert_eq!(it.len(), expected_len);
    }
}
