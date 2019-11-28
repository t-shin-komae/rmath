use matrix_trait::*;
use std::ops::{
    Add,Sub,Mul,Neg
};
#[derive(Debug)]
pub struct CsrVec<T> {
    size:usize,
    data:Vec<(usize,T)>,
}
impl<T:Zero+Copy> CsrVec<T> {
    pub fn from_vec(src:&[T])->Self{
        let data:Vec<(usize,T)>  = src.iter().enumerate().filter(|(_,val)| !val.is_zero()).map(|(i,&val)| (i,val)).collect();
        Self{
            size:src.len(),
            data:data
        }
    }
}

impl<T:Copy+Zero+Add<Output=T>+Sub<Output=T>+Neg<Output=T>> Vector for CsrVec<T> { 
    type Scalar = T;
    fn add(&self,other:&Self) -> Self{
        let mut self_iter = self.data.iter().peekable();
        let mut other_iter = other.data.iter().peekable();
        let mut new_data = Vec::new();
        loop{
            match self_iter.peek(){
                None => {
                    match other_iter.peek(){
                        None => break,
                        Some(&&(index,data)) => {
                            other_iter.next();
                            new_data.push((index,data));
                        }
                    }
                },
                Some(&&(xindex,xdata)) => {
                    match other_iter.peek(){
                        None => {
                            self_iter.next();
                            new_data.push((xindex,xdata));
                        },
                        Some(&&(yindex,ydata)) =>{
                            if xindex < yindex {
                                self_iter.next();
                                new_data.push((xindex,xdata));
                            } else if xindex == yindex {
                                self_iter.next();
                                other_iter.next();
                                new_data.push((xindex,xdata+ydata));
                            }else {
                                other_iter.next();
                                new_data.push((yindex,ydata));
                            }
                        }
                    }
                }
            }
        }
        Self{
            size:self.size,
            data:new_data,
        }
    }
    fn sub(&self,other:&Self) -> Self{
        let mut self_iter = self.data.iter().peekable();
        let mut other_iter = other.data.iter().peekable();
        let mut new_data = Vec::new();
        loop{
            match self_iter.peek(){
                None => {
                    match other_iter.peek(){
                        None => break,
                        Some(&&(index,data)) => {
                            other_iter.next();
                            new_data.push((index,-data));
                        }
                    }
                },
                Some(&&(xindex,xdata)) => {
                    match other_iter.peek(){
                        None => {
                            self_iter.next();
                            new_data.push((xindex,xdata));
                        },
                        Some(&&(yindex,ydata)) =>{
                            if xindex < yindex {
                                self_iter.next();
                                new_data.push((xindex,xdata));
                            } else if xindex == yindex {
                                self_iter.next();
                                other_iter.next();
                                new_data.push((xindex,xdata-ydata));
                            }else {
                                other_iter.next();
                                new_data.push((yindex,-ydata));
                            }
                        }
                    }
                }
            }
        }
        Self{
            size:self.size,
            data:new_data,
        }
    }
    fn dot(&self,other:&Self) -> Self::Scalar{
        T::zero()
    }
    fn mul(&self,scalar:Self::Scalar) -> Self{
        Self{
            size:self.size,
            data:self.data.clone(),
        }
    }
    fn get(&self,index:usize) -> Self::Scalar{
        self.data.binary_search_by(|(i,_)|i.cmp(&index)).map(|i|self.data[i].1).unwrap_or(Zero::zero())
    }
}
impl<T:Copy+Zero+Add<Output=T>+Sub<Output=T>+Neg<Output=T>> PartialEq for CsrVec<T> {
    // TODO This is wasteful because the computing time is twice as the ordinary comparation
    fn eq(&self,other:&Self)->bool{
        self.sub(other).data.iter().all(|(_,val)| val.is_zero())
    }
}

/// Example
/// [
///     [0,0,1,0,0],
///     [3,0,1,0,0],
///     [0,4,0,1,0],
///     [0,0,0,0,0],
///     [0,0,0,0,5],
/// ]
/// 
/// then
///
/// size:(5,5)
/// data: [1,3,1,4,1,5]
/// i_start: [0,1,1,2,2,4] --> [0,1,3,5,5]
/// j_index: [2,0,2,1,3,4]

#[derive(Debug)]
pub struct Csr<T> {
    size:(usize,usize),
    data:Vec<T>,
    i_start_index:Vec<usize>,
    j_index:Vec<usize>,
}

use crate::Dok;
use crate::coo::{Coo,Coordinate};
impl<T:Zero+Copy> Csr<T> {
    pub fn from_dok(src:Dok<T>)->Self{
        Self::from_coo(Coo::from_dok(src))
    }
    pub fn from_coo(src:Coo<T>)-> Self{
        let mut new_data = Vec::new();
        let mut new_istart_index = Vec::new();
        let mut new_j_index = Vec::new();
        let mut last_row_number=0;
        new_istart_index.push(0);
        for (i,(coordinate,data)) in src.data.into_iter().enumerate(){
            new_data.push(data);
            new_j_index.push(coordinate.j);
            if last_row_number != coordinate.i {
                for _ in 0..(coordinate.i - last_row_number) {
                    new_istart_index.push(i);
                }
                last_row_number = coordinate.i;
            }
        }

        Self{
            size:src.size,
            data:new_data,
            i_start_index:new_istart_index,
            j_index:new_j_index,
        }
    }
}


#[cfg(test)]
mod tests{
    #[test]
    fn csrvec_operation() {
        use super::*;
        let x = CsrVec::from_vec(&vec![0.,0.,0.,3.,4.,0.,0.,0.]);
        let y = CsrVec::from_vec(&vec![0.,0.,3.,0.,4.,0.,0.,0.]);
        assert_eq!(CsrVec::from_vec(&vec![0.,0.,3.,3.,8.,0.,0.,0.]),x.add(&y));
        assert_eq!(CsrVec::from_vec(&vec![0.,0.,-3.,3.,0.,0.,0.,0.]),x.sub(&y));
        
    }
    #[test]
    fn csr_operation() {
        /// Example
        /// x = 
        /// [
        ///     [0,0,1,0,0],
        ///     [3,0,1,0,0],
        ///     [0,4,0,1,0],
        ///     [0,0,0,0,0],
        ///     [0,0,0,0,5],
        /// ]
        /// 
        /// then
        ///
        /// size:(5,5)
        /// data: [1,3,1,4,1,5]
        /// i_start: [0,1,1,2,2,4] --> [0,1,3,5,5]
        /// j_index: [2,0,2,1,3,4]
        ///
        /// y = 
        /// [
        ///     [0,0,0,3,1]
        ///     [0,0,0,0,0]
        ///     [0,0,0,0,0]
        ///     [0,0,0,3,0]
        ///     [0,0,0,0,1]
        /// ]
        use super::*;
        use crate::coo::Coo;
        let x  = Csr::from_coo(Coo::from_array(&[(0,2,1.),(1,0,3.),(1,2,1.),(2,1,4.),(2,3,1.),(4,4,5.)],(5,5)));
        let y  = Csr::from_coo(Coo::from_array(&[(0,3,3.),(0,4,1.),(3,3,3.),(4,4,1.)],(5,5)));
        assert_eq!(vec![1.,3.,1.,4.,1.,5.],x.data);
        assert_eq!(vec![0,1,3,5,5],x.i_start_index);
        assert_eq!(vec![2,0,2,1,3,4],x.j_index);
        
    }
}
