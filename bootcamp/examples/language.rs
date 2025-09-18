// 《The Rust Programming Language》第一部分

fn main() -> () {
    println!("language!")
}

#[cfg(test)]
mod chapter10 {

    // 提取为i32的函数
    fn largest_i32(list: &[i32]) -> &i32 {
        let mut largest: &i32 = &list[0];
        for item in list {
            if item > largest {
                largest = item
            }
        }
        largest
    }

    #[test]
    fn test01() {
        let number_list = vec![34, 50, 25, 100, 65];
        let mut largest = &number_list[0];

        // 遍历比较获取最大值
        for number in &number_list {
            if number > largest {
                largest = number;
            }
        }

        println!("The largest number is {largest}");
        assert_eq!(*largest, 100);
        assert_eq!(largest, &100);
        assert_eq!(largest, &mut 100);
        assert_eq!(*largest_i32(&number_list), 100);

        let number_list2 = vec![102, 34, 6000, 89, 54, 2, 43, 8];
        assert_eq!(largest_i32(&number_list2), &6000);
        // ✅
    }
}
