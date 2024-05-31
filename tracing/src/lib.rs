extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn auto_log(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as syn::ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;
    let fn_async = &input_fn.sig.asyncness;
    let fn_const = &input_fn.sig.constness;
    let fn_unsafe = &input_fn.sig.unsafety;
    let fn_abi = &input_fn.sig.abi;
    let fn_generics = &input_fn.sig.generics;
    let fn_inputs = &input_fn.sig.inputs;
    let fn_output = &input_fn.sig.output;
    let fn_block = &input_fn.block;

    let expanded =
        quote! {
            #fn_vis #fn_async #fn_const #fn_unsafe #fn_abi #fn_generics fn #fn_name(#fn_inputs) #fn_output {
                let indent = std::env::var("auto_log_indent").unwrap_or("0".to_string()).as_str().parse::<usize>().unwrap();
                let start = std::time::Instant::now();
                println!("{}BEGIN: {}", String::from(" ").repeat(indent), stringify!(#fn_name));
                let new_indent =(indent + 4).to_string();
                std::env::set_var("auto_log_indent", new_indent);
                let result = #fn_block;
                println!("{}END: {} (took {} ms)", String::from(" ").repeat(indent), stringify!(#fn_name), start.elapsed().as_millis());
                let new_indent = indent.to_string();
                std::env::set_var("auto_log_indent", new_indent);
                result
            }
    };

    TokenStream::from(expanded)
}
