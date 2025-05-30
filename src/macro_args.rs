use proc_macro2::TokenStream;
use syn::spanned::Spanned;
use syn::{
    Ident, Result as SynResult, Token, Visibility,
    parse::{Parse, ParseStream, discouraged::Speculative},
};

const DEFAULT_BASE_FACTORY_NAME: &str = "build";
const DEFAULT_GENERIC_NAME: &str = "G";

pub struct MacroArgs {
    pub base_factory_name: Ident,
    pub visibility: Visibility,
    pub no_std: bool,
    pub generic_name: Ident,
}

impl Parse for MacroArgs {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut result = Self {
            base_factory_name: Ident::new(DEFAULT_BASE_FACTORY_NAME, input.span()),
            visibility: Visibility::Inherited,
            no_std: false,
            generic_name: Ident::new(DEFAULT_GENERIC_NAME, input.span()),
        };

        // Check for factory name
        if input.peek(syn::Ident) {
            let ahead = input.fork();
            let ident = ahead.parse::<Ident>()?;
            if ident != "no_std" && ident != "pub" && ident != "generic" {
                result.base_factory_name = ident;

                input.advance_to(&ahead);
                if !input.is_empty() {
                    input
                        .parse::<Token![,]>()
                        .map_err(|_| input.error("Expected comma after factory name"))?;
                }
            }
        }

        // Check for visibility
        if input.peek(Token![pub]) {
            result.visibility = input
                .parse()
                .map_err(|_| input.error("Failed to parse visibility"))?;

            if !input.is_empty() {
                input
                    .parse::<Token![,]>()
                    .map_err(|_| input.error("Expected comma after visibility"))?;
            }
        }

        // Check for no_std
        if input.peek(syn::Ident) {
            let ahead = input.fork();
            let ident = ahead.parse::<Ident>()?;
            if ident == "no_std" {
                result.no_std = true;
                input.advance_to(&ahead);
            }
        }

        // Check for generic = <ident>
        if input.peek(syn::Ident) {
            let ahead = input.fork();
            let ident = ahead.parse::<Ident>()?;
            if ident == "generic" {
                input.advance_to(&ahead);
                input.parse::<Token![=]>()?;
                result.generic_name = input
                    .parse::<Ident>()
                    .map_err(|_| input.error("Expected identifier after `generic =`"))?;
            }
        }

        if input.is_empty() {
            Ok(result)
        } else {
            Err(input.error("Unexpected input"))
        }
    }
}

impl MacroArgs {
    pub fn parse(attr_args_ts: TokenStream) -> SynResult<Self> {
        if attr_args_ts.is_empty() {
            Ok(Self {
                base_factory_name: Ident::new(DEFAULT_BASE_FACTORY_NAME, attr_args_ts.span()),
                visibility: Visibility::Inherited,
                no_std: false,
                generic_name: Ident::new(DEFAULT_GENERIC_NAME, attr_args_ts.span()),
            })
        } else {
            syn::parse2(attr_args_ts)
        }
    }
}
