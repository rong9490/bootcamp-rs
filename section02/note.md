```
  https://u.geekbang.org/lesson/610?article=769259

  第二章: Rust并发处理

  cargo generate tyr-rust-bootcamp/template

  构建线程(thread spawn) std::thread::spawn 两个泛型参数
  https://rustwiki.org/zh-CN//std/thread/fn.spawn.html
  阅读其签名, 泛型约束, 

  where
  F: FnOnce() -> T + Send + 'static,
  T: Send + 'static,

  lambda -> closure

  Send Trait 特殊/标记/线程间移动 "跨线程边界传输的类型"
  rc::Rc 典型是没有实现Send Trait, 临界区共享访问问题,
  Arc 是实现Send
  *mut T 不能Send
  *const T 不能Send
  !Send
  RawWeaker Lock相关都不能Send(锁起来)
  作为划分 --> 组件数据结构"具有传染性"

  T 可能需要传送回去, 'static 所有权

  后续: Mutex, Arc+Mutex, thread::scope
```

```
  JoinHandle<T> 如果多个线程做的事情需要汇聚 --> Map / Reduce / PromiseAll / forkeJoin
  需要等待所有线程都完成了
  等待同步

  线程间共享内存, 临界区保护

  GO语言 channel概念, 搭桥, 并发任务的同步

  Actor另一种方式, 传递消息, '反应传播' 轮询
```

```
  闭包写法 move || { thread code }

  move 发生移动, 所有权变更, scope

  线程执行完自动退出 --> 除非 写个死循环

  改为等待中...

  
```

```
  FnOnce / Send & Move / 'static
```