use matrix_trait::*;
use std::ops::{Add,Sub,Mul,Neg};

#[derive(Clone,Copy,Hash,PartialEq,Eq)]
pub struct Coordinate {
    pub i:usize,pub j:usize
}

pub struct Coo<T> {
    pub size:(usize,usize),
    pub(crate)data:Vec<(Coordinate,T)>
}

/// data is always sorted
pub struct CooVec<T> {
    size:usize,
    data:Vec<(usize,T)>
}

impl<T:Zero+Copy> CooVec<T> {
    pub fn from_vec(src:&[T])->Self{
        Self{
            size:src.len(),
            data: src.iter().enumerate().filter(|(_,val)| !val.is_zero()).map(|(i,&v)| (i,v)).collect()
        }
    }
}

use crate::dok::Dok;
use crate::csr::Csr;
impl<T:Zero+Copy> Coo<T> {
    pub fn from_dok(src:Dok<T>) -> Self {
        let mut data_with_ij :Vec<(Coordinate,T)> = src.data.into_iter().collect();
        data_with_ij.sort_unstable_by(|(coo1,_),(coo2,_)|{
            coo1.i.cmp(&coo2.i).then_with(|| coo1.j.cmp(&coo2.j))
        });
        Self{
            size:src.size,
            data:data_with_ij
        }
    }

    pub fn from_array(src:&[(usize,usize,T)],size:(usize,usize)) -> Self {
        Self{
            size:size,
            data:src.iter().map(|&(i,j,val)| (Coordinate{i:i,j:j},val)).collect()
        }
    }
}
