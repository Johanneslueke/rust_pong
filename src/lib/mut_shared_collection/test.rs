#[cfg(test)]
pub mod tests{

    use crate::mut_shared_collection::MutSharedCollection;
    use crate::mut_shared_collection::MutSharedCollectionError;

    #[test]
    fn ensure_that_push_one_element_does_work() {
        
        let mut   sut : MutSharedCollection<u32> = MutSharedCollection::default();

        let res = sut.push(42).unwrap();

        assert_eq!(res,())
    }

    #[test]
    fn ensure_that_pop_returns_none() {
        let mut  sut : MutSharedCollection<u32> = MutSharedCollection::default();

        let _res = sut.pop().unwrap_or_else(|err|{
            assert_eq!(err, MutSharedCollectionError::EmptyCollection);
            0
        });
    }

    #[test]
    fn ensure_that_pop_one_element_does_work() {
        let mut  sut : MutSharedCollection<u32> = MutSharedCollection::default();

        sut.push(42).unwrap();
        let res = sut.pop().unwrap();

        assert_eq!(res,(42))
    }
    

    #[test]
    fn ensure_that_two_mutable_references_can_be_shared(){

        let sut : MutSharedCollection<u32> = MutSharedCollection::default();

        let clone1 = &mut sut.clone();
        let clone2 = &mut sut.clone();

        clone1.push(42).unwrap();
        clone2.push(42*2).unwrap();

        let pop1 = clone2.pop().unwrap();
        let pop2 = clone2.pop().unwrap();

        assert_eq!(pop1,(42*2));
        assert_eq!(pop2,(42));
    }

}