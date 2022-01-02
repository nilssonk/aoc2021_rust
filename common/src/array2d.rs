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
            data,
            width,
            height,
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
            data,
            width,
            height,
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

    /// # Safety
    /// No bounds checking will occur.
    pub unsafe fn get_unchecked(&self, (x, y): (U, U)) -> &T
    where
        U: PrimInt,
    {
        let index: usize =
            y.to_usize().unwrap() * self.width.to_usize().unwrap() + x.to_usize().unwrap();
        self.data.get_unchecked(index)
    }
    /// # Safety
    /// No bounds checking will occur.
    pub unsafe fn get_unchecked_mut(&mut self, (x, y): (U, U)) -> &mut T
    where
        U: PrimInt,
    {
        let index: usize =
            y.to_usize().unwrap() * self.width.to_usize().unwrap() + x.to_usize().unwrap();
        self.data.get_unchecked_mut(index)
    }
}

impl<'a, T, U> std::fmt::Debug for Array2D<'a, T, U>
where
    T: std::fmt::Debug,
    U: std::fmt::Debug + PrimInt + std::iter::Step,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in U::zero()..self.height {
            let start = r.to_usize().unwrap() * self.width.to_usize().unwrap();
            let end = start + self.width.to_usize().unwrap();
            writeln!(f, "{:?}", &self.data[start..end])?;
        }
        Ok(())
    }
}
