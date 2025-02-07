pub fn get_two_sites() {
  let thread_one = thread::spawn(|| download("https://www.foo.com"));
  let thread_two = thread::spawn(|| download("https://www.bar.com"));
}

pub fn get_two_sites_async() {
  let future_one = download_async("https://www.foo.com");
  let future_two = download_async("https://www.bar.com");

  // 并发请求
  join!(future_one, future_two);
}