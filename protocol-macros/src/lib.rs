use attribute_util::{decoder, encoder, CodecWith, Dependency, Order};
use proc_macro::TokenStream;
use syn::{
    parse_macro_input, punctuated::Punctuated, Data, DeriveInput, Ident, Meta, Token, Type,
    TypePath,
};

mod attribute_util;

#[proc_macro_derive(BinaryStream, attributes(proto))]
pub fn derive_packet(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident;
    match input.data {
        Data::Struct(v) => {
            let mut members: Vec<Ident> = Vec::new();
            let mut fields: Vec<Field> = Vec::new();
            for field in v.fields {
                let f = Field::from_ast(&field).unwrap();
                members.push(f.member.clone());
                fields.push(f);
            }
            let encode = encoder::create_token(name.clone(), &members, &fields).unwrap();
            let decode = decoder::create_token(name.clone(), &members, &fields).unwrap();
            quote::quote! {
                #encode
                #decode
            }
            .into()
        }
        _ => panic!(""),
    }
}

#[derive(Debug)]
struct Field {
    member: Ident,
    ty: TypePath,
    order: Option<Order>,
    dependency: Option<Dependency>,
    encode_with: Option<CodecWith>,
    decode_with: Option<CodecWith>,
}
impl Field {
    fn from_ast(field: &syn::Field) -> syn::Result<Self> {
        let mut order: Option<Order> = None;
        let mut dependency: Option<Dependency> = None;
        let mut encode_with: Option<CodecWith> = None;
        let mut decode_with: Option<CodecWith> = None;
        for attrs in field.attrs.iter() {
            for attr in attrs.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)? {
                if attr.path().is_ident("order") {
                    order = Some(attr.require_list()?.parse_args()?);
                } else if attr.path().is_ident("dependency") {
                    dependency = Some(Dependency {
                        expr: attr.require_list()?.tokens.clone(),
                    })
                } else if attr.path().is_ident("encode_with") {
                    encode_with = Some(CodecWith {
                        path: attr.require_list()?.tokens.clone(),
                    })
                } else if attr.path().is_ident("decode_with") {
                    decode_with = Some(CodecWith {
                        path: attr.require_list()?.tokens.clone(),
                    })
                } else {
                    return Err(syn::Error::new_spanned(attr, "Unrecognized Attribute."));
                }
            }
        }
        match &field.ty {
            Type::Path(ty) => Ok(Field {
                member: field.ident.clone().unwrap(),
                ty: ty.clone(),
                order,
                dependency,
                encode_with,
                decode_with,
            }),
            _ => Err(syn::Error::new_spanned(
                field.ty.clone(),
                "Unsupported Type.",
            )),
        }
    }
}
