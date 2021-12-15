use num_traits::PrimInt;
use std::ops::Index;
use std::ops::IndexMut;

use super::Array2D;

impl<'a, T, U> Index<(U, U)> for Array2D<'a, T, U>
where
    U: PrimInt,
{
    type Output = T;

    fn index(&self, index: (U, U)) -> &Self::Output {
        if self.out_of_bounds(index) {
            panic!("Index out of bounds");
        }

        unsafe { self.get_unchecked(index) }
    }
}

impl<'a, T, U> IndexMut<(U, U)> for Array2D<'a, T, U>
where
    U: PrimInt,
{
    fn index_mut(&mut self, index: (U, U)) -> &mut Self::Output {
        if self.out_of_bounds(index) {
            panic!("Index out of bounds");
        }

        unsafe { self.get_unchecked_mut(index) }
    }
}
