// 使用 use 引入本模块
use test_struct;

#[test]
fn it_adds_two() {
    assert_eq!(4, test_struct::add_two(2));
}