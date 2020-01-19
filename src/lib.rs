//! Crate-level documentation for pretty-env-log-test
//!
//!

extern crate pretty_env_logger;
extern crate proc_macro;
extern crate proc_macro2;

use pretty_env_logger::try_init;
use proc_macro::{TokenStream, TokenTree};
use proc_macro2::{Group, Span};
use quote::{format_ident, quote, quote_spanned};
use std::str::FromStr;
use syn::parse::{Parse, ParseBuffer, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Token;
use syn::{braced, parse_macro_input, token, Block, Error, Field, Fields, Ident, Token, TypeInfer};
use syn::{Attribute, DeriveInput, Expr, ExprBlock, Type, Visibility};

/// The pretty-env-logger parse struct. It makes up all the parts of the input that will be parsed
/// from the attribute macro input.
///
#[derive(Debug)]
struct PrettyEnvTestLogger {
    pub vis: Visibility,
    pub async_marker: Option<Token![async]>,
    pub name: Ident,
    pub paren: Group,
    pub ret_type: Option<Type>,
    pub body: ExprBlock,
}

impl<'a> Parse for PrettyEnvTestLogger {
    fn parse(input: &ParseBuffer) -> Result<Self, Error> {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis = input.parse::<Visibility>()?;
        let async_marker = input.parse::<Token![async]>().ok();
        let fn_mark = input.parse::<Token![fn]>()?;
        let name = input.parse::<Ident>()?;
        let paren = input.parse::<Group>()?;
        let arrow = input.parse::<Token![->]>();
        let ret_type = input.parse::<Type>();
        let body = input.parse::<ExprBlock>()?;
        Ok(PrettyEnvTestLogger {
            vis,
            async_marker,
            name,
            paren,
            ret_type: ret_type.ok(),
            body,
        })
    }
}

/// A proc-macro attribute that will insert the necessary stuff to initialize pretty-env-logger
/// so that you can use info!, debug!, etc.. logging macros in your test.
///
/// These aren't run as part of the test suite because we can't actually reference this proc-macros
/// from within itself.
///
/// **Examples:**
/// ```no-run
/// # use std::error::Error;
///
/// #[logtest]
/// pub fn some_test_fn() -> Result<(), dyn Error> {
///     // body
///     Ok(())
/// }
///
/// #[logtest]
/// pub async fn some_test_fn_async() -> Result<(), dyn Error> {
///     // body
///     Ok(())
/// }
/// ```
///
#[proc_macro_attribute]
pub fn logtest(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse + log the important data we need to process over to the macro-template
    let out = parse_macro_input!(item as PrettyEnvTestLogger);

    // Pull out template variables for easier use with 'quote!'
    let vis = out.vis;
    let async_marker = out.async_marker;
    let name = out.name;
    let output = out.ret_type;
    let body = out.body;
    let body_span = body.span();

    let attribute = match async_marker {
        Some(v) => {
            let async_span = v.span();
            quote_spanned! {async_span=> #[tokio::test] }
        }
        None => quote! { #[test] },
    };

    let out = match output {
        Some(v) => {
            let out_span = v.span();
            quote_spanned! {out_span=> -> #v }
        }
        None => quote! {},
    };

    let main = quote_spanned! {body_span=>
        #attribute
        #vis #async_marker fn #name() #out {
            let _ = pretty_env_logger::try_init();
            #body
        }
    };

    // print fully expanded string for auditing purposes
    // let expanded = main.to_string();

    // convert it back into the proper TokenStream that the compiler understands
    main.into()
}
