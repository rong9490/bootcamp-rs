#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

//! 39 | 借用检查: 完全理解Scope和NLL
//! https://time.geekbang.org/course/detail/100060601-319372
//!
//! 理解 词法作用域 & 非词法作用域
//!
//!

// 理解词法作用域 & 基本数据类型
// book_alex_programming::ch02::_39_scope_nll::understand_scope::a
// cargo watch -x 'test book_alex_programming::ch02::_39_scope_nll::understand_scope::a -- --nocapture'
pub mod understand_scope {
	#[test]
	fn a() -> () {
		let mut v = vec![];
		v.push(1);

		// 花括号是什么含义, 查看MIR: scope _ {} 产生一个词法作用域上下文 -> 产生一个生命周期
		// StorageDead
		{
			println!("{:?}", v[0]);
			v.push(2);
			println!("{:?}", v[1]);
			// println!("{:?}", v[2]); // 编译时无法判断越界!
		}
	} // 一个函数一个栈帧 stack frame

	// text -> tokens -> ast -> hir -> mir -> llvm -> 1100
	// NLL = Non Lexica Lifetime
	#[test]
	fn b() -> () {
		let mut v = vec![];
		v.push(1); // 这里
		{
			let vv = &v; // 这里是不可变借用
		}
		// 花括号分开, 就知道结束地方, 不是同时存在的! "技巧"
		{
			println!("{:?}", v[0]);
			v.push(2); // 这里是可变借用, 同时存在!
		}
	}
}

mod _leetcode_1576 {
	#[test]
	fn a() {
		let s = "abc?d";
		let mut chars = s.chars().collect::<Vec<char>>(); // 按字符处理

		// chars.iter_mut 是可变借用!!
		// 处理方法, 避免借用, 换成索引 0..s.len()
		// for (i, c) in chars.iter_mut().enumerate() {
		for i in 0..s.len() {
			// 定义a-z字母集, 拥有所有权的; words需要是可变的!
			let mut words = ('a'..='z').into_iter();
			// 此处 chars[i] 是不可变借用
			if chars[i] == '?' {
				let left = if i == 0 { None } else { Some(chars[i - 1]) };
				let right = if i == s.len() - 1 { None } else { Some(chars[i + 1]) };

				// 这里是可变借用! 冲突了
				chars[i] = words.find(|&w| Some(w) != left && Some(w) != right).unwrap()
			}
		}
		let _s = chars.into_iter().collect::<String>();
	}
}
