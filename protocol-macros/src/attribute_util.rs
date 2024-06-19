use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::Parse, Error, Ident};

pub mod decoder;
pub mod encoder;

#[derive(Debug)]
pub struct SerializeType {
    pub ty: TokenStream,
    pub contained_option: bool,
}

#[derive(Debug)]
pub struct CodecWith {
    pub path: TokenStream,
}

#[derive(Debug)]
pub struct Dependency {
    pub expr: TokenStream,
}

#[derive(Debug)]
pub enum Order {
    LE,
    BE,
}

impl Parse for Order {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        let result = match &ident.to_string()[..] {
            "BE" | "BigEndian" | "be" => Order::BE,
            "LE" | "LittleEndian" | "le" => Order::LE,
            _ => return Err(Error::new_spanned(ident, "Unrecognized OrderingType")),
        };
        Ok(result)
    }
}

impl ToTokens for Order {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Order::BE => quote! {::protodef::BE},
            Order::LE => quote! {::protodef::LE},
        });
    }
}
