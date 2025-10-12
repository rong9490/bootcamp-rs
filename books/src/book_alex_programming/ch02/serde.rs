// 过程宏

/**

介绍：[serde.rs](https://serde.rs/)

参阅 :  [https://github.com/ZhangHanDong/proc_codegen](https://github.com/ZhangHanDong/proc_codegen)

过程宏三件套：

- [syn](https://github.com/dtolnay/syn)
- [quote](https://github.com/dtolnay/quote)
- [proc-macro2](https://github.com/alexcrichton/proc-macro2)

示例：封装 Diesel 方便 crud

```rust

    // find_by_or!{ Person -> people::[name:String || company_name:String]   }

    use super::*;

    pub struct DbOpByOrBy {
        pub model: Type,
        pub table: Ident,
        pub bracket_token: token::Bracket,
        pub content: FieldContentOr,
    }

    pub struct FieldContentOr {
        pub name1: Ident,
        pub ty1: Type,
        pub name2: Ident,
        pub ty2: Type,
    }

    impl Parse for DbOpByOrBy {
        fn parse(input: ParseStream) -> Result<Self> {
            let content;
            let model: Type = input.parse()?;
            input.parse::<Token![->]>()?;
            let table: Ident = input.parse()?;
            input.parse::<Token![::]>()?;
            let bracket_token = bracketed!(content in input);
            let content = content.parse()?;
            Ok(DbOpByOrBy {
                model,
                table,
                bracket_token,
                content,
            })
        }
    }

    impl Parse for FieldContentOr {
        fn parse(input: ParseStream) -> Result<Self> {
            let name1: Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let ty1: Type = input.parse()?;
            input.parse::<Token![||]>()?;
            let name2: Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let ty2: Type = input.parse()?;
            Ok(FieldContentOr {
                name1,
                ty1,
                name2,
                ty2,
            })
        }
    }

    // in lib.rs

    // find_by_or!{ Person -> people::[name:String || company_name:String]   }

    #[proc_macro]
    pub fn find_by_or(input: TokenStream) -> TokenStream {
        let DbOpByOrBy {
            model,
            table,
            bracket_token,
            content,
        } = parse_macro_input!(input as DbOpByOrBy);
        let (name1, name2) = (content.name1, content.name2);
        let (ty1, ty2) = (content.ty1, content.ty2);
        let fn_name = format!("find_by_{}_or_{}", name1, name2);
        let fn_name = Ident::new(&fn_name, proc_macro2::Span::call_site());

        let expanded = quote! {
            impl #model {
                pub fn #fn_name(conn: &PgConnection, #name1: #ty1, #name2: #ty2) -> QueryResult<#model> {
                    #table::table
                    .filter(#table::dsl::#name1.eq(#name1))
                    .or_filter(#table::dsl::#name2.eq(#name2))
                    .get_result(conn)
                }
            }
        };
        TokenStream::from(expanded)
    }


```

有用的第三方库：

- [derive_more](https://github.com/JelteF/derive_more)

*/
pub fn derive_proc_macros(){}

