use num_traits::PrimInt;
use std::marker::PhantomData;

pub mod index;

pub struct Array2D<'a, T, U> {
    data: Vec<T>,
    pub width: U,
    pub height: U,
    phantom: PhantomData<&'a T>,
}

impl<'a, T, U> Array2D<'a, T, U> {
    pub fn from_vec(data: Vec<T>, (width, height): (U, U)) -> Self {
        Self {
            data: data,
            width: width,
            height: height,
            phantom: PhantomData,
        }
    }

    pub fn from_iter<I>(input: I, width: U) -> Self
    where
        I: Iterator<Item = T>,
        U: PrimInt,
    {
        let data: Vec<T> = input.collect();
        let height = U::from(data.len() / width.to_usize().unwrap()).unwrap();
        Self {
            data: data,
            width: width,
            height: height,
            phantom: PhantomData,
        }
    }

    pub fn get(&self, index: (U, U)) -> Option<&T>
    where
        U: PrimInt,
    {
        if self.out_of_bounds(index) {
            return None;
        }

        Some(unsafe { self.get_unchecked(index) })
    }

    fn out_of_bounds(&self, (x, y): (U, U)) -> bool
    where
        U: PrimInt,
    {
        x < U::zero() || x >= self.width || y < U::zero() || y >= self.height
    }

    pub unsafe fn get_unchecked(&self, (x, y): (U, U)) -> &T
    where
        U: PrimInt,
    {
        let index: usize =
            y.to_usize().unwrap() * self.width.to_usize().unwrap() + x.to_usize().unwrap();
        self.data.get_unchecked(index)
    }

    pub unsafe fn get_unchecked_mut(&mut self, (x, y): (U, U)) -> &mut T
    where
        U: PrimInt,
    {
        let index: usize =
            y.to_usize().unwrap() * self.width.to_usize().unwrap() + x.to_usize().unwrap();
        self.data.get_unchecked_mut(index)
    }
}
