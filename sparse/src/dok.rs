use std::collections::HashMap;
use matrix_trait::*;
use std::ops::{Add,Sub,Mul,Neg};

#[derive(Clone)]
pub struct Dok<T> {
    size:(usize,usize),
    data:HashMap<(usize,usize),T>,
}
#[derive(Clone,Debug)]
pub struct DokVec<T> {
    size:usize,
    data:HashMap<usize,T>,
}

impl<T:Zero+PartialEq+Copy> DokVec<T> {
    pub fn from_vec(src:&[T])->Self{
        let data : HashMap<usize,T>= src.iter().enumerate().filter(|(_,&val)| val !=Zero::zero()).map(|(i,&v)|(i,v)).collect();
        Self{
            size:src.len(),
            data:data
        }
    }
}

impl<T> Dok<T>{
    pub fn zeros(m:usize,n:usize)->Self{
        Self{
            size:(m,n),
            data:HashMap::new()
        }
    }
}

impl<T:Copy+Add<Output=T>+Sub<Output=T>+Mul<Output=T>+Neg<Output=T>+Zero> Vector for DokVec<T> {
    type Scalar = T;
    fn add(&self,other:&Self) -> Self{
        let mut result = self.clone();
        for (&k,&v) in other.data.iter(){
            match result.data.get_mut(&k){
                Some(val) => *val= *val+v,
                None => {
                    result.data.insert(k,v);
                }
            }
        }
        result
    }
    fn sub(&self,other:&Self) -> Self{
        let mut result = self.clone();
        for (&k,&v) in other.data.iter(){
            match result.data.get_mut(&k){
                Some(val) => *val= *val-v,
                None => {
                    result.data.insert(k,-v);
                }
            }
        }
        result
    }
    fn dot(&self,other:&Self) -> Self::Scalar{
        let mut result = Zero::zero();
        for (&k,&v) in other.data.iter(){
            match self.data.get(&k){
                Some(&val) => result=result+val*v,
                None =>{}
            }
        }
        result
    }
    fn mul(&self,scalar:Self::Scalar) -> Self{
        let mut result = self.clone();
        result.data.iter_mut().for_each(|(_,v)|{
            *v=*v*scalar
        });
        result
    }
    fn get(&self,index:usize) -> Self::Scalar{
        self.data.get(&index).map_or(Zero::zero(),|v|*v)
    }
}

impl<T:PartialEq+Zero+std::fmt::Debug> PartialEq for DokVec<T> {
    // TODO This is wasteful because the computing time is twice as the ordinary comparation
    fn eq(&self,other:&Self)->bool{
        self.data.iter().all(|(key,lval)|{
            other.data.get(key).map_or(lval.is_zero(),|rval| *lval == *rval)
        }) && 
        other.data.iter().all(|(key,rval)|{
            self.data.get(key).map_or(rval.is_zero(),|lval| *rval == *lval)
        })
    }
}

#[cfg(test)]
mod tests{
    #[test]
    fn dokvec_operation() {
        use super::*;
        let x = DokVec::from_vec(&vec![0.,0.,0.,3.,4.,0.,0.,0.]);
        let y = DokVec::from_vec(&vec![0.,0.,3.,0.,4.,0.,0.,0.]);
        assert_eq!(DokVec::from_vec(&vec![0.,0.,3.,3.,8.,0.,0.,0.]),x.add(&y));
        assert_eq!(DokVec::from_vec(&vec![0.,0.,-3.,3.,0.,0.,0.,0.]),x.sub(&y));
        assert_eq!(DokVec::from_vec(&vec![0.,0.,3.*3.,0.,4.*3.,0.,0.,0.]),y.mul(3.0));
        assert_eq!(4.*4.,x.dot(&y));
        assert_eq!(0.,x.get(0));
        assert_eq!(3.,x.get(3));
    }
}

