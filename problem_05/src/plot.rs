use crate::geometry::*;

use num_traits::{FromPrimitive, PrimInt, Signed};
use std::iter::Step;

pub trait PlottingNumber: 'static + FromPrimitive + PrimInt + Signed + Step {}

// Default impl
impl<T> PlottingNumber for T where T: 'static + FromPrimitive + PrimInt + Signed + Step {}

/*
    Bresenham's line algorithm as described at https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
*/
pub fn plot_line<T>((a, b): &Line<T>) -> Box<dyn std::iter::Iterator<Item = Vec2<T>>>
where
    T: PlottingNumber,
{
    let x0 = a.x;
    let y0 = a.y;
    let x1 = b.x;
    let y1 = b.y;

    #[allow(clippy::collapsible_else_if)]
    if (y1 - y0).abs() < (x1 - x0).abs() {
        if x0 > x1 {
            plot_line_low(b, a)
        } else {
            plot_line_low(a, b)
        }
    } else {
        if y0 > y1 {
            plot_line_high(b, a)
        } else {
            plot_line_high(a, b)
        }
    }
}

fn plot_line_low<T>(a: &Vec2<T>, b: &Vec2<T>) -> Box<dyn std::iter::Iterator<Item = Vec2<T>>>
where
    T: PlottingNumber,
{
    let dx = b.x - a.x;
    let dy_pre = b.y - a.y;
    let dy = dy_pre.abs();
    let yi = dy_pre.signum();

    let two = T::from_u8(2).unwrap();

    let mut d = (two * dy) - dx;
    let mut y = a.y;

    Box::new((a.x..(b.x + T::one())).map(move |x| {
        let result = Vec2::<T> { x, y };

        if d > T::zero() {
            y = y + yi;
            d = d + (two * (dy - dx));
        } else {
            d = d + two * dy;
        }

        result
    }))
}

fn plot_line_high<T>(a: &Vec2<T>, b: &Vec2<T>) -> Box<dyn std::iter::Iterator<Item = Vec2<T>>>
where
    T: PlottingNumber,
{
    let dx_pre = b.x - a.x;
    let dy = b.y - a.y;
    let dx = dx_pre.abs();
    let xi = dx_pre.signum();

    let two = T::from_u8(2).unwrap();

    let mut d: T = (two * dx) - dy;
    let mut x = a.x;

    Box::new((a.y..(b.y + T::one())).map(move |y| {
        let result = Vec2::<T> { x, y };

        if d > T::zero() {
            x = x + xi;
            d = d + two * (dx - dy);
        } else {
            d = d + two * dx;
        }

        result
    }))
}
