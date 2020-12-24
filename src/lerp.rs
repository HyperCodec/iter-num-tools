use num_traits::FromPrimitive;
use std::ops::{Add, Div, Mul, RangeInclusive, Sub};
use crate::map::Function;

#[derive(Copy, Clone)]
pub struct Lerp<T> {
    x0: T,
    x1: T,
    y0: T,
    y1: T,
}

impl<T> Lerp<T> {
    pub fn new(from: RangeInclusive<T>, to: RangeInclusive<T>) -> Self {
        let (x0, x1) = from.into_inner();
        let (y0, y1) = to.into_inner();
        Lerp { x0, x1, y0, y1 }
    }
}

impl<T> Lerp<T>
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Div<Output = T> + Copy,
{
    #[inline]
    fn lerp(&self, x: T) -> T {
        let Lerp { x0, x1, y0, y1 } = *self;

        ((y0 * (x1 - x)) + (y1 * (x - x0))) / (x1 - x0)
    }
}

impl<T> Function<T> for Lerp<T>
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Div<Output = T> + Copy,
{
    type Output = T;

    #[inline]
    fn call(&self, x: T) -> Self::Output {
        self.lerp(x)
    }
}

#[derive(Copy, Clone)]
pub struct LerpPrim<T>(Lerp<T>);

macro_rules! LerpPrimitive {
    ($($new_lerp:ident; $t:ty; $from:ident),*) => {

$(

impl<T> Lerp<T> where T: FromPrimitive {
    pub fn $new_lerp(from: RangeInclusive<$t>, to: RangeInclusive<T>) -> Self {
        let (x0, x1) = from.into_inner();
        let (y0, y1) = to.into_inner();
        Lerp {
            x0: T::$from(x0).unwrap(),
            x1: T::$from(x1).unwrap(),
            y0,
            y1,
        }
    }
}

impl<T> LerpPrim<T> where T: FromPrimitive {
    #[inline]
    pub fn $new_lerp(from: RangeInclusive<$t>, to: RangeInclusive<T>) -> Self {
        LerpPrim(Lerp::$new_lerp(from, to))
    }
}

impl<T> Function<$t> for LerpPrim<T>
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Div<Output = T> + Copy + FromPrimitive,
{
    type Output = T;

    #[inline]
    fn call(&self, x: $t) -> Self::Output {
        self.0.lerp(T::$from(x).unwrap())
    }
}

)*

    }
}

LerpPrimitive![
    new_usize; usize; from_usize,
    new_u128; u128; from_u128,
    new_u64; u64; from_u64,
    new_u32; u32; from_u32,
    new_u8; u8; from_u8,

    new_isize; isize; from_isize,
    new_i128; i128; from_i128,
    new_i64; i64; from_i64,
    new_i32; i32; from_i32,
    new_i8; i8; from_i8,

    new_f32; f32; from_f32,
    new_f64; f64; from_f64
];
