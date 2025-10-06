// 括号匹配

use crate::book_algorithm::_422_stack::Stack;


fn par_checker1(par: &str) -> bool {
    let mut char_list: Vec<char> = Vec::new();

    for c in par.chars() { char_list.push(c); }

    let mut idx = 0;
    let mut balance: bool = true;
    let mut stack: Stack<char> = Stack::new();

    // while idx < char_list.len() && balance {
    // }

    balance && stack.is_empty()

    // for c in par.chars() {
    //     match c {
    //         '(' | '[' | '{' => stack.push(c),
    //         ')' => if stack.pop() != Some('(') { return false; },
    //         ']' => if stack.pop() != Some('[') { return false; },
    //         '}' => if stack.pop() != Some('{') { return false; },
    //         _ => (),
    //     }
    // }
    // stack.is_empty()
}

fn par_match() {}
fn par_checker2() {}
fn par_checker3() {}