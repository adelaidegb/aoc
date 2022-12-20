use once_cell::sync::Lazy;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use regex::Regex;
use syn::*;

fn get_day(input: &str) -> Option<u32> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^day\s*(?:=|=>)?\s*(\d{1,2})").unwrap());
    RE.captures(input).and_then(|c| {
        Some(c.get(1)?.as_str().parse().ok()?)
    })
}

fn get_day_and_part(input: &str) -> Option<(u32, u32)> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^day\s*(?:=|=>)?\s*(\d{1,2}),\s*part\s*(?:=|=>)?\s*([12])$").unwrap());
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

    let ident = item.sig.ident.to_owned();

    let wrapper_ident = format_ident!("__aoclib_wrapper_day{}_part{}", day, part);

    assert_valid_input("aoc", &item);

    let call_token = quote! { #ident(input) };

    let output: TokenStream = quote! {
        fn #wrapper_ident(input: ::std::string::String) -> ::std::string::String {
            #call_token
        }
        #item
        ::aoclib::add_entry!(#day, #part, #wrapper_ident);
    };

    proc_macro::TokenStream::from(output)
}

#[proc_macro_attribute]
pub fn aoc_test(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let attr = TokenStream::from(attr);
    let item = parse_macro_input!(item as ItemFn);

    let day = get_day(&attr.to_string()).or_else(|| {
        panic!("Invalid #[aoc_test] attribute: {}", attr.to_string());
    }).unwrap();

    let ident = item.sig.ident.to_owned();

    let wrapper_ident = format_ident!("__aoclib_wrapper_day{}_test_{}", day, ident.to_string());

    assert_valid_input("aoc_test", &item);

    let output: TokenStream = quote! {
        #[test]
        fn #wrapper_ident() {
            #ident(::aoclib::__load_test_data(env!("CARGO_MANIFEST_DIR"), #day))
        }
        #item
    };

    proc_macro::TokenStream::from(output)
}

fn assert_valid_input(attr_name: &str, item: &ItemFn) {
    if item.sig.inputs.is_empty() {
        panic!("Invalid #[{}] function: no input arg: {}", attr_name, item.sig.ident);
    }

    if item.sig.inputs.len() > 1 {
        panic!("Invalid #[{}] function: max of 1 input arg supported: {}", attr_name, item.sig.ident);
    }

    let fn_input = item.sig.inputs.first().unwrap();

    if let FnArg::Typed(t) = fn_input {
            if let Type::Path(p) = &*t.ty {
                if let Some(ident) = p.path.get_ident() {
                    if ident == "String" {
                        return;
                    }
                }
            }
    }

    panic!("Invalid #[{}] function: only String is allowed as an input: {}", attr_name, item.sig.ident);
}

#[proc_macro]
pub fn aoc_entry(_: proc_macro::TokenStream) -> proc_macro::TokenStream {
    proc_macro::TokenStream::from(quote! {
        #[macro_use] extern crate inventory;
        #[macro_use] extern crate aoclib;

        fn main() {
            ::aoclib::__main(env!("CARGO_MANIFEST_DIR"));
        }
    })
}
