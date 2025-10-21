#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

//! 40 | 借用检查: 深入理解生命周期和生命周期参数
//! https://time.geekbang.org/course/detail/100060601-319373
//!
//! 生命周期参数的意义 -- 避免悬垂指针
//!
//!

// book_alex_programming::ch02::_40_lifetime::return_str::return_str
// cargo watch -x "test book_alex_programming::ch02::_40_lifetime::return_str::return_str -- --nocapture"
mod return_str {

	fn return_str<'a>() -> &'a str {
		// 整体一个词法作用域 --> 栈帧'开辟/释放'
		let mut s = "Rust".to_string(); // 局部变量, 函数内的堆内存
		// println!("{:?}", s);
		for i in 0..3 {
			s.push_str("Good ");
		}

		// Error: returns a value referencing data owned by the current function
		&s[..]; // 返回其切片指针 -> 帧释放后, 形成 悬垂指针
		todo!()
	}

	#[test]
	fn return_str_test() {
		// let x = return_str();
		// println!("x = {:?}", x)
	}

	fn foo<'a>(x: &'a str, y: &'a str) -> &'a str {
		let result = String::from("really long string...");
		// error 返回引用 --> 悬垂指针
		result.as_str();
		todo!()
	}

	// 缺乏生命周期, 会在返回后丢失哪个字符串还有效, 哪个已失效(释放)的信息, 无法判断! 造成悬垂指针
	// 添加生命周期后 'a --> 形成约定, 输入的 >= 输出, 保证返回的引用必定有效
	fn the_longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
		if s1.len() > s2.len() { s1 } else { s2 }
	}

	// late bound vs early bound
	struct A<T>(T);

	#[test]
	fn aa() {
		// 结构体被定义时, 泛型T是没有确定的, 只有调用时才被确定
		let _a = A::<i32>(3); // 这里是late bound 为什么? 如何判断
		let _b = A::<String>("Hello".to_string());
		let _c = A::<char>(10 as char);

		// early bound 早绑定
		// 'a 'b 'static
		// &'static str 和 &'a str
		// 长的: 短的
		// long: short
		// 'static: 'a
	}
}
