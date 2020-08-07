#[cfg(test)]
mod tests{
    use rand::prelude::*;
    use crate::IntervalVec;

    #[test]
    fn random_numbers_test(){
        let mut v = IntervalVec::new();
        let mut v_check = Vec::new();
        for i in 0..1000000{
            assert_eq!(v.len(), i);
            let element = rand::random::<i32>() % 100;
            let _ = v.push(element);
            v_check.push(element);
            assert_eq!(v.get(v.len() - 1).unwrap(), v_check.get(v_check.len() - 1).unwrap());
        }
    } 
}