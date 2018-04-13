#[allow(unused_imports)]
use super::{twist};

#[test]
fn twisting() {
    let mut one = vec![0, 1, 2, 3, 4];
    let mut two = vec![2, 1, 0, 3, 4];
    
    twist(0, 3, &mut one);
    twist(3, 4, &mut two);
    
    assert_eq!(&mut one, &mut vec![2, 1, 0, 3, 4]);
    assert_eq!(&mut two, &mut vec![4, 3, 0, 1, 2]);
}
