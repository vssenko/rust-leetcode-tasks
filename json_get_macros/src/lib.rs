use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, parse_macro_input};

#[proc_macro]
pub fn get(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as Expr);

    fn expand(expr: &Expr) -> proc_macro2::TokenStream {
        match expr {
            // доступ к полю: obj.field
            Expr::Field(field) => {
                let base = expand(&field.base);
                let member = &field.member;
                let key = quote!(stringify!(#member));
                quote! {
                    #base.and_then(|v| v.get(#key))
                }
            }
            // доступ по индексу: obj[123]
            Expr::Index(index) => {
                let base = expand(&index.expr);
                let idx = &index.index;
                quote! {
                    #base.and_then(|v| v.get(#idx))
                }
            }
            // базовый случай: просто expr (например, my_obj)
            other => {
                quote!(Some(&#other))
            }
        }
    }

    let expanded = expand(&expr);
    let tokens = quote! {
        { use serde_json::Value; (#expanded) }
    };
    tokens.into()
}
