#![recursion_limit = "128"]

extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::ToTokens;
use syn::{self, parse_macro_input, parse_quote, AttributeArgs, FnArg, ItemFn, Stmt};

/// Custom attribute for recursive parser
#[proc_macro_attribute]
pub fn recursive_parser(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as AttributeArgs);
    let item = parse_macro_input!(item as ItemFn);
    impl_recursive_parser(&attr, &item)
}

fn impl_recursive_parser(_attr: &AttributeArgs, item: &ItemFn) -> TokenStream {
    let before = impl_recursive_parser_bofore(&item);
    let body = impl_recursive_parser_body(&item);

    let mut item = item.clone();

    item.block.stmts.clear();
    item.block.stmts.push(before);
    item.block.stmts.push(body);

    item.into_token_stream().into()
}

fn impl_recursive_parser_bofore(item: &ItemFn) -> Stmt {
    let ident = &item.sig.ident;

    let input = if let Some(x) = &item.sig.inputs.first() {
        match x {
            FnArg::Typed(arg) => &arg.pat,
            _ => panic!("function with #[recursive_parser] must have an argument"),
        }
    } else {
        panic!("function with #[recursive_parser] must have an argument");
    };

    parse_quote! {
        let #input = {
            let id = nom_recursive::RECURSIVE_STORAGE.with(|storage| {
                storage.borrow_mut().get(stringify!(#ident))
            });
            use nom_recursive::HasRecursiveInfo;
            let mut info = #input.get_recursive_info();

            use nom::AsBytes;
            let ptr = #input.as_bytes().as_ptr();

            if ptr != info.get_ptr() {
                #[cfg(feature = "trace")]
                {
                    use nom_tracable::Tracable;
                    nom_tracable::custom_trace(&#input, stringify!(#ident), "recursion flag clear", "\u{001b}[1;36m")
                };
                info.clear_flags();
                info.set_ptr(ptr);
            }

            if info.check_flag(id) {
                #[cfg(feature = "trace")]
                {
                    use nom_tracable::Tracable;
                    nom_tracable::custom_trace(&#input, stringify!(#ident), "recursion detected", "\u{001b}[1;36m")
                };
                return Err(nom::Err::Error(nom::error::make_error(s, nom::error::ErrorKind::Fix)));
            }
            #[cfg(feature = "trace")]
            {
                use nom_tracable::Tracable;
                nom_tracable::custom_trace(&#input, stringify!(#ident), "recursion flag set", "\u{001b}[1;36m")
            };
            info.set_flag(id);

            #input.set_recursive_info(info)
        };
    }
}

fn impl_recursive_parser_body(item: &ItemFn) -> Stmt {
    let body = item.block.as_ref();
    parse_quote! {
        #body
    }
}
