use proc_macro::TokenStream;

mod init;

// TODO: at least validate that args is empty? Or use it later.
#[proc_macro_attribute]
pub fn init(_args: TokenStream, function: TokenStream) -> TokenStream {
    let function = syn::parse_macro_input!(function as syn::ItemFn);
    init::init(function)
}
