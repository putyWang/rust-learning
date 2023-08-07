fn main() {
    let v1 = vec![1, 2, 3];
    // 调用 v1 中的 iter 方法创建一个以 v1 为基础的迭代器
    let v3_iter = v1.iter();
    // 对 迭代器使用 for 循环遍历 v1 中的元素
    for val in v3_iter {
        println!("Got: {}", val);
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test] 
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        // 调用 v1 中的 iter 方法创建一个以 v1 为基础的迭代器
        let mut v1_iter = v1.iter();
        // 使用 next() 方法 移动迭代器指针
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);

        // 由于 next() 方法已经获取到 v1_iter 的所有权 因此 v1_iter 已经失效
        // let total: i32 = v1_iter.sum();
        // 要再次使用迭代器需要重新声明
        let v2_iter = v1.iter();
        let total: i32 = v2_iter.sum();
        assert_eq!(total, 6);
    }

    #[test] 
    fn iterator_adaptors() {
        let v1: Vec<i32> = vec![1, 2, 3];
        // 调用 map 方法创建一个新迭代器，接着调用 collect 方法消费新迭代器并创建一个 vector
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(v2, vec![2, 3, 4]);
    }

    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}


#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // filter 迭代适配器 使用一个返回 bool 类型的闭包作为 参数
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}