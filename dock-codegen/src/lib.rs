mod args;

use args::CommandArgs;
#[allow(unused)]
use dock::command::Command;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn};

#[proc_macro_attribute]
pub fn command(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let attr_args = parse_macro_input!(attr as AttributeArgs);

    let parsed_args = CommandArgs::new(attr_args).unwrap();

    let fn_name = input_fn.clone().sig.ident;

    let mut name = fn_name.to_string();
    let mut description = String::new();
    let mut disabled = false;

    if let Some(val) = parsed_args.name {
        name = val.value()
    }

    if let Some(val) = parsed_args.description {
        description = val.value()
    }

    if let Some(val) = parsed_args.disabled {
        disabled = val.value()
    }

    let fn_name = input_fn.clone().sig.ident;

    TokenStream::from(quote! {

        #[derive(Debug)]
        #[allow(non_camel_case_types)]
        pub struct #fn_name;


        impl Command for #fn_name{

            fn name(&self) -> String{

                #name.to_string()
            }

            fn description(&self) -> String{

                #description.to_string()

            }

            fn disabled(&self) -> bool{
                #disabled
            }

            fn call(&self){
                #input_fn

                #fn_name()
            }


        }
    })
}
