use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;

pub fn init(mut function: syn::ItemFn) -> TokenStream {
    let ident = function.sig.ident;
    let fn_ident = Ident::new(&format!("{}_body", &ident), Span::call_site());

    function.sig.ident = fn_ident.clone();

    let output = quote! {
        #[allow(non_upper_case_globals)]
        const #ident: crate::core::init::Init = Init {
            name: stringify!(#ident),
            __exec: #fn_ident,
        };

        #function
    };

    output.into()
}
