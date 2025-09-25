/// 书本 《Rust权威指南》 《The Rust Programming Language》
/// https://weread.qq.com/web/reader/89632740813ab9d0dg014a8f

// TODO 迁移整理
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

#[cfg(test)]
mod chapter16 {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn thread01() {
        // 这里会交替输出...这是因为sleep会暂停线程, 交给另外的执行
        let handle = thread::spawn(|| {
            for i in 1..=5 {
                println!("hi num {i} from the spawned thread!");
                thread::sleep(Duration::from_millis(10));
            }
        });
        // 如果在main循环前
        handle.join().unwrap();

        for i in 1..=5 {
            println!("yo num {i} from the main thread!");
            thread::sleep(Duration::from_millis(10));
        }
        // 如果在最后
        // handle.join().unwrap();
    }

    #[test]
    fn thread02_move() {
        let v: Vec<i32> = vec![1, 2, 3, 4];

        // 编译报错: may outlive borrowed value `v`
        // 提示需要 move 转移所有权
        let handle = thread::spawn(move || {
            println!("Here is {:?}", v);
            drop(v); // ✅
        });

        // 编译报错: value used here after move
        // drop(v);

        handle.join().unwrap();
    }

    #[test]
    fn sync_mpsc01() {
        let (tx, rx) = mpsc::channel::<String>();

        thread::spawn(move || {
            let val: String = String::from("hi");
            tx.send(val).unwrap(); // 这里send后已经转移了所有权
            // 编译错误: value used here after move
            // drop(val);
        });

        let received: String = rx.recv().unwrap();
        println!("Got: {received}");
        drop(received); // ✅ 所有权到这边了, 负责释放内存
    }
}
