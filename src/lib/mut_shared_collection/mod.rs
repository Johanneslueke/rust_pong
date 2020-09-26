pub mod test;

use std::cmp::Eq;
use std::fmt;
use std::error;
use std::cell::RefCell;
use std::rc::Rc;
use std::iter::FromIterator;

#[allow(dead_code)]
type Result<T> = std::result::Result<T, MutSharedCollectionError>;

#[derive(Debug,Eq,PartialEq)]
pub enum MutSharedCollectionError{
    EmptyCollection,
}


impl fmt::Display for MutSharedCollectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MutSharedCollectionError::EmptyCollection =>  write!(f, "please use a vector with at least one element"),
            _ => write!(f, "Unkown Error")
        }
    }
}

impl error::Error for MutSharedCollectionError{
    fn source(&self) -> Option<&(dyn error::Error + 'static)>{
        match *self{
            MutSharedCollectionError::EmptyCollection => None
        }
    }
}

pub struct MutSharedCollection<T>{
    collection : Rc<RefCell<Vec<T>>>
}

impl<T  > MutSharedCollection<T>{
    pub fn new() -> Self{
        MutSharedCollection{
            collection: Rc::new(
                RefCell::default()
            )
        }
    }

    /**
     * Pushed a new item to the end of the collection.
     * Returns () on success
     */
    pub fn push(&mut self, item : T) -> Result<()>{
        let  clone = Rc::clone(&self.collection);
        let mut refcell = clone.borrow_mut();
        refcell.push(item);
        Ok(())
    }

    /**
     * Removes the last element of the collection and
     * returns it as Option on success. If no Value is
     * left None is returned.
     */
    pub fn pop(&mut self) -> Result<T>
    {
        let  clone = Rc::clone(&self.collection);
        let mut refcell = clone.borrow_mut();
        let res = refcell
                    .pop()
                    .ok_or(MutSharedCollectionError::EmptyCollection);
        res
    }

}


impl<T: Clone> MutSharedCollection<T>{
    /** 
     * Retrieves a clone of an element at index.
     * If no Value exist, None is returned
     **/
     pub fn get(&mut self,index:usize) -> Result<T>{
        let  clone = Rc::clone(&self.collection);
        let  refcell = clone.borrow_mut();
        Ok(refcell[index].clone())
    }
}



impl<T: Clone> Default for MutSharedCollection<T>{
    
    fn default() -> Self { 
        MutSharedCollection::new()
     }
}

impl<T> Clone for MutSharedCollection<T>{
    
    fn clone(&self) -> Self {
         MutSharedCollection{
             collection: self.collection.clone()
         }
    }
}


impl<T : Clone> IntoIterator for MutSharedCollection<T>{
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let clone = Rc::clone(&self.collection);
        let refcell = clone.borrow_mut();
        refcell.clone().into_iter()
    }
}

 impl<T :  Clone> FromIterator<T> for MutSharedCollection<T>{
     fn from_iter<I:IntoIterator<Item=T>>(iter: I) -> Self {
        let mut collection = MutSharedCollection::new();
        for i in iter{
            collection.push(i).unwrap();
        }
        collection
    }
}