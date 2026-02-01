use std::ops::{Add, Index, IndexMut};
use crate::SELECTED_PARAMETER_SET;

#[derive(Clone, Debug)]
pub struct Polynomial<T: num_traits::PrimInt, const N: usize>([T; N]);

pub type Poly16 = Polynomial<u16, {SELECTED_PARAMETER_SET.n}>;
pub type Poly32 = Polynomial<u32, {SELECTED_PARAMETER_SET.n}>;

impl<T: num_traits::PrimInt, const N: usize, Idx: std::slice::SliceIndex<[T]>> Index<Idx> for Polynomial<T, N> {
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index]
    }
}

impl<T: num_traits::PrimInt, const N: usize, Idx: std::slice::SliceIndex<[T]>> IndexMut<Idx> for Polynomial<T, N> {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.0[index] 
    }
}

impl<T: num_traits::PrimInt, const N: usize> Add for Polynomial<T, N> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut coeffs: [T; N] = [T::zero(); N];

        for ((coefref, selfvalue), othervalue) in coeffs.iter_mut().zip(&self.0).zip(&other.0) {
            *coefref = *selfvalue + *othervalue;
        }

        return Polynomial::new(&coeffs);
    }
}

impl<T: num_traits::PrimInt, const N: usize> Polynomial<T, N> {
    pub fn new(coefficients: &[T;N]) -> Polynomial<T, N> {
        return Polynomial { 0: coefficients.clone() }
    }
}
