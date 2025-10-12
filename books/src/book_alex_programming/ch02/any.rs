// 动态自省


/**

# 动态自省

示例1:

[https://doc.rust-lang.org/std/any/index.html](https://doc.rust-lang.org/std/any/index.html)

 示例2:

 ```rust

 use std::any::Any;

 trait Foo: Any {
     fn as_any(&self) -> &Any;
 }

 impl<T: Any> Foo for T {
     fn as_any(&self) -> &Any {
         self
     }
 }

 struct Bar {}

 struct Baz {}

 impl PartialEq for Foo {
     fn eq(&self, other: &Foo) -> bool {
         let me = self.as_any();
         let you = other.as_any();
         if me.is::<Bar>() && you.is::<Bar>() {
             true
         } else if me.is::<Baz>() && you.is::<Baz>() {
             true
         } else {
             false
         }
     }
 }

 fn main() {
     let bar: Bar = Bar {};
     let baz: Baz = Baz {};
     let foo1: &Foo = &bar;
     let foo2: &Foo = &baz;
     println!("{:?}", foo1 == foo2);
 }

 ```

 示例 3:

 ```rust
     use std::any::Any;
     struct UnStatic<'a> { x: &'a i32 }
     fn main() {
         let a = 42;
         let v = UnStatic { x: &a };
         let mut any: &Any;
         //any = &v;  // Compile Error!
     }
 ```
 
 修正：

 ```rust
 use std::any::Any;
 struct UnStatic<'a> { x: &'a i32 }
 static ANSWER: i32 = 42;
 fn main() {
     let v = UnStatic { x: &ANSWER };
     let mut a: &Any;
     a = &v;
     assert!(a.is::<UnStatic>());
 }
 ```

 示例4:

 oso 库应用

 [https://github.com/osohq/oso/blob/main/languages/rust/oso/src/host/class.rs](https://github.com/osohq/oso/blob/main/languages/rust/oso/src/host/class.rs)

 示例 5:

 bevy_reflect 库应用

 [https://github.com/bevyengine/bevy/blob/main/crates/bevy_reflect/src/lib.rs](https://github.com/bevyengine/bevy/blob/main/crates/bevy_reflect/src/lib.rs)

 */
pub fn any_refection(){}

