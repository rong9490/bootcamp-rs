/**
    # 声明宏

    宏展开命令: cargo rustc -- -Z unstable-options --pretty=expanded

    示例1:

    ```rust
    macro_rules! unless {
        ($arg: expr, $branch: expr) => ( if !$arg { $branch }; );
    }
    fn cmp(a: i32, b: i32) {
        unless!(a > b, {
            println!("{} < {}", a, b);
        });
    }
    fn main () {
        let (a, b): (i32, i32) = (42, 11);
        cmp(a, b);
    }
    ```

    支持 token 类型：

    ```text
          item — an item, like a function, struct, module, etc.
          block — a block (i.e. a block of statements and/or an expression, surrounded by braces)
          stmt — a statement
          pat — a pattern
          expr — an expression
          ty — a type
          ident — an identifier
          path — a path (e.g., foo, ::std::mem::replace, transmute::<_, int>, …)
          meta — a meta item; the things that go inside #[...] and #![...] attributes
          tt — a single token tree
          vis — a possibly empty Visibility qualifier
    ```

    示例2: 个数不确定, 正则的技巧, 星号
           学习如何处理尾逗号

    ```rust

      macro_rules! hashmap {
          ($($key:expr => $value:expr),* ) => {
              {
                  let mut _map = ::std::collections::HashMap::new();
                  $(
                      _map.insert($key, $value);
                  )*
                  _map
              }
          };
      }
      fn main(){
          let map = hashmap!{
              "a" => 1,
              "b" => 2
          //  "c" => 3, // V1.0不支持结尾有逗号
          };
          assert_eq!(map["a"], 1);
      }
    ```

    示例3: 定义多套流程(匹配模式)

    ```rust
      macro_rules! hashmap {
          ($($key:expr => $value:expr,)*) =>
              {  hashmap!($($key => $value),*) };
          ($($key:expr => $value:expr),* ) => {
              {
                  let mut _map = ::std::collections::HashMap::new();
                  $(
                      _map.insert($key, $value);
                  )*
              _map
          }
      };
      }
      fn main(){
          let map = hashmap!{
              "a" => 1,
              "b" => 2,
              "c" => 3,
          };
          assert_eq!(map["a"], 1);
      }
    ```

      示例5:

```rust
  macro_rules! unit {
      ($($x:tt)*) => (());
  }
  macro_rules! count {
      ($($key:expr),*) => (<[()]>::len(&[$(unit!($key)),*]));
  }
  macro_rules! hashmap {
      ($($key:expr => $value:expr),* $(,)*) => {
          {
          let _cap = count!($($key),*);
          let mut _map
              = ::std::collections::HashMap::with_capacity(_cap);
          $(
              _map.insert($key, $value);
          )*
          _map
      }
  };
  }
  fn main(){
      let map = hashmap!{
          "a" => 1,
          "b" => 2,
          "c" => 3,
      };
      assert_eq!(map["a"], 1);
  }

```

示例6:

```rust
  macro_rules! hashmap {
      (@unit $($x:tt)*) => (());
      (@count $($rest:expr),*) =>
          (<[()]>::len(&[$(hashmap!(@unit $rest)),*]));
      ($($key:expr => $value:expr),* $(,)*) => {
          {
              let _cap = hashmap!(@count $($key),*);
              let mut _map =
                  ::std::collections::HashMap::with_capacity(_cap);
          $(
              _map.insert($key, $value);
          )*
          _map
      }
  };
  }
  fn main(){
  let map = hashmap!{
      "a" => 1,
      "b" => 2,
      "c" => 3,
  };
  assert_eq!(map["a"], 1);
  }
```

示例7: 理解卫生性, 消除重复, 作用域独立, 

```rust
  macro_rules! sum {
      ($e:expr) => ({
          let a = 2;
          $e + a
      })
  }
  fn main(){
      // error[E0425]: cannot find value `a` in this scope
      let four = sum!(a);
  }
```
*/
pub fn declarative_macros() {}