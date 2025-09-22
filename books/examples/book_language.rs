/// 书本 《Rust权威指南》 《The Rust Programming Language》
/// https://weread.qq.com/web/reader/89632740813ab9d0dg014a8f

fn main() -> () {
    println!("book_language!")
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

#[cfg(test)]
mod chapter15 {
    #[test]
    fn box_t() {
        let b: Box<i32> = Box::new(5);
        println!("b = {b}");
    }

    /// 递归类型
    #[test]
    fn box_recursive() {
        #[allow(dead_code)]
        #[derive(Debug)]
        enum List {
            Cons(i32, Box<List>),
            Nil,
        }
    }

    /// Box递归类型
    #[test]
    fn box_recursive2() {
        #[allow(dead_code)]
        #[derive(Debug)]
        enum List {
            Cons(i32, Box<List>),
            Nil,
        }
        let list: List = List::Cons(
            1,
            Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
        );
        println!("{:?}", list); // ✅
        println!("{:?}", std::mem::size_of::<List>()); // 理解:16byte
        println!("{:?}", std::mem::size_of_val(&list)); // 理解:16byte
    }
}
