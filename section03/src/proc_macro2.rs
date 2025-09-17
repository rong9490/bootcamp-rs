// 官方demo
// 教程文档 https://kirigaya.cn/blog/article?seq=289
extern crate proc_macro;

// the `#[proc_macro_derive]` attribute is only usable with crates of the `proc-macro` crate type
// #[proc_macro_derive(MyDerive)]
// pub fn my_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     let input = proc_macro2::TokenStream::from(input);

//     let output: proc_macro2::TokenStream = {
//         /* transform input */
//     };

//     proc_macro::TokenStream::from(output)
// }