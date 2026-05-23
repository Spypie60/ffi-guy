use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, parse_quote};

#[proc_macro_attribute]
pub fn ffi_guy(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);
    input.sig.unsafety = Some(syn::token::Unsafe { span: input.sig.fn_token.span });
    input.sig.abi = Some(parse_quote!(extern "C"));

    let vis = &input.vis;
    let sig = &input.sig;
    let body = &input.block;
    let expanded = quote! {
        #[unsafe(no_mangle)]
        #vis #sig #body
    };
    TokenStream::from(expanded)
}
