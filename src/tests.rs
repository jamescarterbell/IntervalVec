#[cfg(test)]
mod tests{
    use rand::prelude::*;
    use crate::IntervalVec;

    #[test]
    fn random_numbers_test(){
        let mut v = IntervalVec::new();
        let mut v_check = Vec::new();
        for i in 0..1000{
            assert_eq!(v.len(), i);
            let element = rand::random::<u32>() % 5;
            //println!("inserting: {}", element);
            let _ = v.push(element);
            v_check.push(element);
        }
        v.get(999);
    } 
}