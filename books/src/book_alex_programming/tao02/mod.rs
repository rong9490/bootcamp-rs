
mod tao02 {
	/// 临时值: Basic usage:
	#[test]
	fn case01() {
		fn temp() -> i32 {
			return 1;
		}
		let _x = &temp(); // 绑定 binding
		// 为什么不可以? 理解 位置表达式, 值表达式
		// temp() = *x; error[E0070]: invalid left-hand side expression
	}

	#[test]
	fn case02() {
		// 不变与可变
		pub fn immutable_and_mutable() {
	    let a = 1;  // 默认不可变
	    // a = 2; // immutable and error: cannot assign twice to immutable variable
	    let mut b = 2;  // 使用mut关键字声明可变绑定
	    b = 3; // mutable
		}
		immutable_and_mutable();
	}

	#[test]
	fn case03() {
		// 所有权
		pub fn ownership(){
	    let place1 = "hello";
	    //  ^^ 位置表达式 ^^  值表达式
	    //   ^ 位置上下文  ^  值上下文
	    let place2 = "hello".to_string();
	    let other = place1;    // Copy
	                 // ^^ 位置表达式出现在了值上下文中
	    println!("{:?}", place1);  // place1还可以继续使用
	    let other = place2;    // Move
	                 // ^^ 位置表达式出现在了值上下文中
	    println!("{:?}", place2); // place2不能再被使用，编译出错
		}
	}
}
