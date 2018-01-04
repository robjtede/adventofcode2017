use super::*;

#[test]
fn memory_redistribution() {
    assert_eq!(redistribute(&vec![0, 2, 7, 0]), vec![2, 4, 1, 2]);
    assert_eq!(redistribute(&vec![2, 4, 1, 2]), vec![3, 1, 2, 3]);
    assert_eq!(redistribute(&vec![3, 1, 2, 3]), vec![0, 2, 3, 4]);
    assert_eq!(redistribute(&vec![0, 2, 3, 4]), vec![1, 3, 4, 1]);
    assert_eq!(redistribute(&vec![1, 3, 4, 1]), vec![2, 4, 1, 2]);
}
