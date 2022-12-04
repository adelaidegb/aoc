use once_cell::sync::Lazy;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use regex::Regex;
use syn::*;

fn get_day_and_part(input: &str) -> Option<(u32, u32)> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^day(\d{1,2}), ?part([12])$").unwrap());
    RE.captures(input).and_then(|c| {
        let day = c.get(1)?.as_str().parse().ok()?;
        let part = c.get(2)?.as_str().parse().ok()?;
        Some((day, part))
    })
}

#[proc_macro_attribute]
pub fn aoc(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let attr = TokenStream::from(attr);
    let item = parse_macro_input!(item as ItemFn);

    let (day, part) = get_day_and_part(&attr.to_string()).or_else(|| {
        panic!("Invalid #[aoc] attribute: {}", attr.to_string());
    }).unwrap();

    let mut needs_input_string = false;
    if !item.sig.inputs.is_empty() {
        if item.sig.inputs.len() > 1 {
            panic!("Invalid #[aoc] function: max of 1 input arg supported: {}", item.sig.ident);
        }

        let fn_input = item.sig.inputs.first().unwrap();

        if let FnArg::Typed(t) = fn_input {
            if let Type::Reference(r) = &*t.ty {
                if let Type::Path(p) = &*r.elem {
                    if let Some(ident) = p.path.get_ident() {
                        if ident == "str" {
                            needs_input_string = true;
                        } else {
                            panic!("Invalid #[aoc] function: only &str is allowed as an input: {}", item.sig.ident);
                        }
                    }
                }
            }
        }
    }

    let ident = item.sig.ident.to_owned();

    let wrapper_ident = format_ident!("__aoclib_wrapper_day{}_part{}", day, part);
    let input_ident = format_ident!("input");
    let call_token = if needs_input_string {
        quote! {
            #ident(#input_ident)
        }
    } else {
        quote! {
            #ident()
        }
    };

    let output: TokenStream = quote! {
        fn #wrapper_ident(input: &str) -> String {
            let result = #call_token;
            result.to_owned()
        }
        #item
        ::aoclib::add_entry!(#day, #part, #wrapper_ident);
    };

    println!("output: \"{}\"", output.to_string());

    proc_macro::TokenStream::from(output)
}

#[proc_macro]
pub fn aoc_entry(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(quote! {
        #[macro_use] extern crate inventory;
        #[macro_use] extern crate aoclib;

        fn main() {
            ::aoclib::aoclib_main();
        }
    })
}
