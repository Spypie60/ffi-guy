use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, parse_quote};

#[proc_macro_attribute]
pub fn ffi_safe(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the target function
    let mut input = parse_macro_input!(item as ItemFn);

    // Sets abi to C
    input.sig.abi = Some(parse_quote!(extern "C"));

    // Clears the unsafety of the function
    input.sig.unsafety = None;

    let vis = &input.vis;
    let sig = &input.sig;
    let body = &input.block;

    let expanded = quote! {
        #[no_mangle]
        #vis unsafe #sig #body
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn ffi_guy(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);
    input.sig.abi = Some(parse_quote!(extern "C"));
    input.sig.unsafety = None;

    let vis = &input.vis;
    let sig = &input.sig;
    let body = &input.block;
    let expanded = quote! {
        #[no_mangle]
        #vis #sig #body
    };
    TokenStream::from(expanded)
}
