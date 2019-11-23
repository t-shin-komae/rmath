use std::collections::HashMap;
pub struct Dok<T> {
    size:(usize,usize),
    data:HashMap<(usize,usize),T>,
}

impl<T> Dok<T>{
    pub fn zeros(m:usize,n:usize)->Self{
        Self{
            size:(m,n),
            data:HashMap::new()
        }
    }
}

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