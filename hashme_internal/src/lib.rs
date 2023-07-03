extern crate proc_macro;

use std::{fs::OpenOptions};
use std::io::Write;

use std::sync::Mutex;
use syn::{ItemFn, parse_quote};
use proc_macro::TokenStream;
use syn::parse_macro_input;

use quote::*;

static COUNTER: Mutex<usize> = Mutex::new(0);
const MAX_COUNT: usize = 50;

#[proc_macro_attribute]
pub fn assign_verification_id(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut fn_item = parse_macro_input!(input as ItemFn);
    
    // read next available id and increment
    let mut id = COUNTER.lock().unwrap();
    let value = *id;
    *id += 1;
    
    // write function name and id to file
    let fn_ident = fn_item.sig.ident.to_string();
    let mut file = OpenOptions::new().append(true).create(true).open("hash-data").unwrap();
    writeln!(file, "{}:{}", fn_ident, value).unwrap();
    
    // verify that enough space is available
    if value >= MAX_COUNT {
        return quote! {
            compile_error!("Hash-Capacity exceeded");
        }.into();
    }

    // create assignment code
    let assigment = quote! {
        let internal_hash_id = #value;
    };
    let block = fn_item.block;

    // prepend assignment to function
    let new_block = parse_quote!{{
        #assigment
        #block
    }};

    fn_item.block = Box::new(new_block);
    proc_macro::TokenStream::from(fn_item.to_token_stream())
}

#[proc_macro]
pub fn create_link_section(_item: TokenStream) -> TokenStream {
    proc_macro::TokenStream::from(quote! {
        #[link_section = ".hdata"]
        pub static HASHES: [HashData; #MAX_COUNT] = [HashData{hash: [0x00; 32], len: 0}; #MAX_COUNT];
    })

    //let field = "#[link_section = \".hdata\"]
    //static HASHE: [HashData; 100] = [HashData{hash: [0x00; 16], len: 0}; 100];";

    //field.parse().unwrap()
}
