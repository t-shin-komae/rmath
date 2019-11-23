pub mod dok;
pub use dok::{Dok,DokVec};

pub struct Coo<T> {
    size:(usize,usize),
    data:Vec<(usize,usize,T)>
}

pub struct Csr<T> {
    size:(usize,usize),
    data:Vec<T>,
    i_start_index:Vec<usize>,
    j_index:Vec<usize>,
}
