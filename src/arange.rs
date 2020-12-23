use num_traits::{Float, One, ToPrimitive, Zero};
use std::ops::{AddAssign, Div, Range, Sub};

/// Iterator over a range, stepping by a fixed amount each time
#[derive(Clone, Copy)]
pub struct Arange<F> {
    start: F,
    end: F,
    step_size: F,
    step: F,
}

impl<F> Arange<F>
where
    F: Zero,
{
    /// Create a new iterator over the range, stepping by `step` each time
    /// This allows you to create simple float iterators
    ///
    /// ```
    /// use iter_num_tools::Arange;
    /// use itertools::Itertools;
    ///
    /// let it = Arange::new(0.0..2.0, 0.5);
    /// itertools::assert_equal(it, vec![0.0, 0.5, 1.0, 1.5])
    /// ```
    ///
    /// Arange isn't perfect, you might want [lin_space](crate::lin_space) if
    /// `step` isn't 'whole' float
    ///
    /// ```
    /// use iter_num_tools::{Arange, lin_space};
    /// use itertools::Itertools;
    ///
    /// // With Arange, you get some accuracy loss
    /// let it = Arange::new(0.0..0.5, 0.1);
    /// itertools::assert_equal(it, vec![0.0, 0.1, 0.2, 0.30000000000000004, 0.4]);
    ///
    /// let it = lin_space(0.0..0.5, 5);
    /// itertools::assert_equal(it, vec![0.0, 0.1, 0.2, 0.3, 0.4]);
    /// ```
    pub fn new(range: Range<F>, step: F) -> Self {
        let Range { start, end } = range;
        Arange {
            end,
            step_size: step,
            start,
            step: F::zero(),
        }
    }
}

impl<F> Iterator for Arange<F>
where
    F: Float + AddAssign + Sub<Output = F> + Div<Output = F> + ToPrimitive + One,
{
    type Item = F;
    fn next(&mut self) -> Option<Self::Item> {
        let x = self.start + self.step * self.step_size;
        if x < self.end {
            self.step += F::one();
            Some(x)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let length = self.end - (self.start + self.step * self.step_size);
        match (length / self.step_size).ceil().to_usize() {
            Some(steps_left) => (steps_left, Some(steps_left)),
            None => (usize::MAX, None),
        }
    }

    fn count(self) -> usize
    where
        Self: Sized,
    {
        match self.size_hint() {
            (_, Some(x)) => x,
            (_, None) => panic!("iterator is infinite"),
        }
    }
}

use itertools::{Itertools, Product};

use crate::combine::Combine;
/// Creates a grid space over the range made up of fixed step intervals
///
/// ```
/// use iter_num_tools::arange_grid;
/// use itertools::Itertools;
///
/// let it = arange_grid((0.0, 0.0)..(1.0, 2.0), 0.5);
/// itertools::assert_equal(it, vec![
///     (0.0, 0.0), (0.0, 0.5), (0.0, 1.0), (0.0, 1.5),
///     (0.5, 0.0), (0.5, 0.5), (0.5, 1.0), (0.5, 1.5),
/// ]);
///
/// // different step count in each direction
/// let it = arange_grid((0.0, 0.0)..(1.0, 2.0), (0.5, 1.0));
/// itertools::assert_equal(it, vec![
///     (0.0, 0.0), (0.0, 1.0),
///     (0.5, 0.0), (0.5, 1.0),
/// ]);
///
/// // even 3d spaces
/// let it = arange_grid((0.0, 0.0, 0.0)..(2.0, 2.0, 2.0), 1.0);
/// itertools::assert_equal(it, vec![
///     (0.0, 0.0, 0.0), (0.0, 0.0, 1.0),
///     (0.0, 1.0, 0.0), (0.0, 1.0, 1.0),
///
///     (1.0, 0.0, 0.0), (1.0, 0.0, 1.0),
///     (1.0, 1.0, 0.0), (1.0, 1.0, 1.0),
/// ]);
/// ```
pub fn arange_grid<R, S>(range: R, size: S) -> <R as IntoArangeGrid<S>>::ArangeGrid
where
    R: IntoArangeGrid<S>,
{
    range.into_arange_grid(size)
}

pub trait IntoArangeGrid<S> {
    type ArangeGrid;
    fn into_arange_grid(self, size: S) -> Self::ArangeGrid;
}

type Grid2<F1, F2> = Product<Arange<F1>, Arange<F2>>;
type Grid3<F1, F2, F3> = Combine<Product<Grid2<F1, F2>, Arange<F3>>>;

// Implements IntoArangeGrid for (w0, h1)..(w1, h1) with control over both width and height step counts
impl<F1, F2> IntoArangeGrid<(F1, F2)> for Range<(F1, F2)>
where
    F1: Float + AddAssign + Sub<Output = F1> + Div<Output = F1> + ToPrimitive,
    F2: Float + AddAssign + Sub<Output = F2> + Div<Output = F2> + ToPrimitive,
    Arange<F1>: Clone,
    Arange<F2>: Clone,
{
    type ArangeGrid = Grid2<F1, F2>;
    fn into_arange_grid(self, (w, h): (F1, F2)) -> Self::ArangeGrid {
        let Range {
            start: (w0, h0),
            end: (w1, h1),
        } = self;

        let first = Arange::new(w0..w1, w);
        let second = Arange::new(h0..h1, h);
        first.cartesian_product(second)
    }
}

// Implements IntoArangeGrid for (w0, h0, d0)..(w1, h1, d1) with control over both width, height and depth step counts
impl<F1, F2, F3> IntoArangeGrid<(F1, F2, F3)> for Range<(F1, F2, F3)>
where
    F1: Float + AddAssign + Sub<Output = F1> + Div<Output = F1> + ToPrimitive,
    F2: Float + AddAssign + Sub<Output = F2> + Div<Output = F2> + ToPrimitive,
    F3: Float + AddAssign + Sub<Output = F3> + Div<Output = F3> + ToPrimitive,
    Arange<F1>: Clone,
    Arange<F2>: Clone,
    Arange<F3>: Clone,
{
    type ArangeGrid = Grid3<F1, F2, F3>;
    fn into_arange_grid(self, (w, h, d): (F1, F2, F3)) -> Self::ArangeGrid {
        let Range {
            start: (w0, h0, d0),
            end: (w1, h1, d1),
        } = self;

        let first = ((w0, h0)..(w1, h1)).into_arange_grid((w, h));
        let second = Arange::new(d0..d1, d);
        Combine::new(first.cartesian_product(second))
    }
}

// Implements IntoArangeGrid for (w0, h0)..(w1, h1) with the same width and height step count
impl<F> IntoArangeGrid<F> for Range<(F, F)>
where
    F: Float + AddAssign + Sub<Output = F> + Div<Output = F> + ToPrimitive,
    Arange<F>: Clone,
{
    type ArangeGrid = Grid2<F, F>;
    fn into_arange_grid(self, steps: F) -> Self::ArangeGrid {
        self.into_arange_grid((steps, steps))
    }
}

// Implements IntoArangeGrid for (w0, h0, d0)..(w1, h1, d1) with the same width, height and depth step count
impl<F> IntoArangeGrid<F> for Range<(F, F, F)>
where
    F: Float + AddAssign + Sub<Output = F> + Div<Output = F> + ToPrimitive,
    Arange<F>: Clone,
{
    type ArangeGrid = Grid3<F, F, F>;
    fn into_arange_grid(self, steps: F) -> Self::ArangeGrid {
        self.into_arange_grid((steps, steps, steps))
    }
}

#[test]
fn test_size_hint() {
    let it = Arange::new(0.0..0.55, 0.1);
    assert_eq!(it.size_hint(), (6, Some(6)));

    let it = Arange::new(0.0..0.5, 0.1);
    assert_eq!(it.size_hint(), (5, Some(5)));

    let it = Arange::new(0.0..0.5, 0.0);
    assert_eq!(it.size_hint(), (usize::MAX, None));
}
