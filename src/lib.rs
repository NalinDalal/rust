use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr};

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    let method = parse_macro_input!(attr as LitStr);
    let func = parse_macro_input!(item as ItemFn);
    let func_name = &func.sig.ident;

    let expanded = quote! {
        fn #func_name() {
            println!("Handling {} request", #method);
            #func
        }
    };

    TokenStream::from(expanded)
}
