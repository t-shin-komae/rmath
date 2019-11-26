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
#[cfg(test)]
mod tests{
    #[test]
    fn csrvec_operation() {
        use super::*;
        let x = CsrVec::from_vec(&vec![0.,0.,0.,3.,4.,0.,0.,0.]);
        let y = CsrVec::from_vec(&vec![0.,0.,3.,0.,4.,0.,0.,0.]);
        assert_eq!(CsrVec::from_vec(&vec![0.,0.,3.,3.,8.,0.,0.,0.]),x.add(&y));
        assert_eq!(CsrVec::from_vec(&vec![0.,0.,-3.,3.,0.,0.,0.,0.]),x.sub(&y));
        // assert_eq!(4.*4.,x.dot(&y));
        // assert_eq!(0.,x.get(0));
        // assert_eq!(3.,x.get(3));
    }
}

pub struct Csr<T> {
    size:(usize,usize),
    data:Vec<T>,
    i_start_index:Vec<usize>,
    j_index:Vec<usize>,
}

// impl<T:Zero+Copy> Csr<T> {
//     pub fn from_vec(src:&[&[T]])->Self{
//         let data:Vec<(usize,T)>  = src.iter().enumerate().filter(|(_,val)| !val.is_zero()).map(|(i,&val)| (i,val)).collect();
//         Self{
//             size:src.len(),
//             data:data
//         }
//     }
// }
// impl<T:Zero+Add<Output=T>> Matrix for Csr<T> {
//     type Scalar = T;

//     fn add(&self,other:&Self) -> Self{

//     }
//     fn sub(&self,other:&Self) -> Self{

//     }
// }

