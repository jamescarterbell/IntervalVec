#[cfg(test)]
mod tests{
    use rand::prelude::*;
    use crate::IntervalVec;

    #[test]
    fn random_numbers_test(){
        let mut v = IntervalVec::new();
        for _ in 0..10000{
            v.push(rand::random::<i32>());
        }
    } 
}