use std::ops::{Add,Sub,Mul};

pub enum CalcError{
    NonRegular,
    NotSquare((usize,usize)),
}

pub trait Vector {
    type Scalar:Copy+Add+Sub+Mul;
    fn add(&self,other:&Self) -> Self;
    fn sub(&self,other:&Self) -> Self;
    fn dot(&self,other:&Self) -> Self::Scalar;
    fn mul(&self,scalar:Self::Scalar) -> Self;
    fn get(&self,index:usize) -> Self::Scalar;
}

pub trait Matrix: Sized {
    type Scalar:Copy+Add+Sub+Mul;
    type Row:Vector;
    type Col:Vector;
    fn add(&self,other:&Self) -> Self;
    fn sub(&self,other:&Self) -> Self;
    fn mul(&self,other:&Self) -> Self;
    fn product(&self,other:&Self) -> Self;
    fn scalar_mul(&self,scalar:Self::Scalar) -> Self;
    fn get(&self,i:usize,j:usize) -> Self::Scalar;
    fn get_row(&self,i:usize) -> Self::Row;
    fn get_col(&self,i:usize) -> Self::Col;
    fn inv(&self) -> Result<Self,CalcError>;
    fn rank(&self) -> usize;
    fn t(&self) -> Self;
    fn size(&self) -> (usize,usize);
    fn det(&self) -> Result<Self::Scalar,CalcError>;
}

pub trait Zero{
    fn zero() -> Self;
}
impl Zero for f32 {
    fn zero() -> Self {
        0.
    }
}
impl Zero for f64 {
    fn zero() -> Self {
        0.
    }
}
