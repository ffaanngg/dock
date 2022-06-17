#[allow(unused)]
use dock::command::Command;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn command(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);

    let name = input_fn.clone().sig.ident;

    TokenStream::from(quote! {

        #[derive(Debug)]
        #[allow(non_camel_case_types)]
        pub struct #name;


        impl Command for #name{
            fn name(&self) -> String{
                format!("{:#?}", #name)
            }

            fn call(&self){
                #input_fn

                #name()
            }


        }
    })
}
