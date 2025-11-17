// use darling::{FromDeriveInput, FromField, ast::Data};
// use proc_macro2::TokenStream; // proc-macro2
// use quote::quote;
// use syn::DeriveInput;

// #[derive(Debug, FromDeriveInput)]
// struct AutoDebugInfo {
//     ident: syn::Ident, // 标识
//     generics: syn::Generics,
//     data: Data<(), AutoDebugFieldsInfo>, // 嵌套结构
// }

// #[derive(Debug, FromField)]
// #[darling(attributes(debug))]
// struct AutoDebugFieldsInfo {
//     ident: Option<syn::Ident>,
//     #[darling(default)]
//     skip: bool,
// }

// // TODO maaccro宏, 能否进行单测 ??

// // TODO 一级消费使用
// pub(crate) fn process_auto_debug(input: DeriveInput) -> TokenStream {
//     let AutoDebugInfo {
//         ident,
//         generics,
//         data: Data::Struct(fields),
//     } = AutoDebugInfo::from_derive_input(&input).unwrap()
//     else {
//         panic!("AutoDebug only works on structs");
//     };

//     let fields = fields.iter().map(|field| {
//         let ident = field.ident.as_ref().unwrap();
//         let skip = field.skip;
//         if skip {
//             quote! {}
//         } else {
//             quote! {
//                 .field(stringify!(#ident), &self.#ident)
//             }
//         }
//     });

//     quote! {
//         impl ::core::fmt::Debug for #ident #generics {
//             #[inline]
//             fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
//                 f.debug_struct(stringify!(#ident))
//                     #(#fields)*
//                     .finish()
//             }
//         }
//     }
// }

// // TODO 二级消费使用
// // #[proc_macro_derive(AutoDebug, attributes(debug))]
// // pub fn derive_auto_debug(input: TokenStream) -> TokenStream {
// //     let input = syn::parse_macro_input!(input as DeriveInput);
// //     // println!("{:#?}", input);

// //     process_auto_debug(input).into()
// // }
