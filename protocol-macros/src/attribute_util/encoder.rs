use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{Error, GenericArgument, Ident, PathArguments, Result, TypePath};

use crate::Field;

use super::{CodecWith, Dependency, Order, SerializeType};

pub fn create_token(name: Ident, members: &[Ident], fields: &[Field]) -> Result<TokenStream> {
    let encode_tokens = fields
        .iter()
        .map(
            |Field {
                 member,
                 ty,
                 order,
                 dependency,
                 encode_with,
                 ..
             }| {
                if dependency.is_some() {
                    create_option_stmt(member, dependency, order, ty, encode_with)
                } else {
                    let serialize_type = ty.to_token_stream();
                    create_stmt(
                        member,
                        dependency,
                        order,
                        &SerializeType {
                            ty: serialize_type,
                            contained_option: false,
                        },
                        encode_with,
                    )
                }
            },
        )
        .collect::<Result<Vec<TokenStream>>>()?;
    Ok(quote! {
        impl ::protocol_core::Encoder for #name {
            fn encode<W: ::protodef::WriteBytesExt>(self, w: &mut W) -> ::anyhow::Result<()> {
                let Self { #(#members),* } = self;
                #( #encode_tokens )*
                Ok(())
            }
        }
    })
}

fn create_option_stmt(
    member: &Ident,
    dependency: &Option<Dependency>,
    order: &Option<Order>,
    ty: &TypePath,
    encode_with: &Option<CodecWith>,
) -> Result<TokenStream> {
    if ty.path.segments[0].ident != "Option" {
        return Err(Error::new_spanned(ty, "must be wrapped Option."));
    }
    let PathArguments::AngleBracketed(ty) = &ty.path.segments[0].arguments else {
        return Err(Error::new_spanned(ty, "Unexpected Option's token."));
    };
    let GenericArgument::Type(ty) = &ty.args[0] else {
        return Err(Error::new_spanned(
            ty,
            "Unexpected Option's Generics token.",
        ));
    };
    let serialize_type = ty.to_token_stream();
    let v = create_stmt(
        member,
        dependency,
        order,
        &SerializeType {
            ty: serialize_type,
            contained_option: true,
        },
        encode_with,
    )?;
    Ok(v)
}

fn create_stmt(
    member: &Ident,
    dependency: &Option<Dependency>,
    order: &Option<Order>,
    ser_type: &SerializeType,
    encode_with: &Option<CodecWith>,
) -> Result<TokenStream> {
    let body = create_body(member, order, ser_type, encode_with)?;
    let result = if let Some(dep) = dependency {
        if ser_type.contained_option {
            let expr = dep.expr.clone();
            quote! {
                if #expr {
                    if let Some(#member) = #member {
                        #body
                    } else {
                        return Err(::anyhow::anyhow!("None"))
                    }
                }
            }
        } else {
            return Err(Error::new_spanned(
                member,
                "this type must be wrapped Option.",
            ));
        }
    } else {
        quote! { #body }
    };
    Ok(result)
}

fn create_body(
    member: &Ident,
    order: &Option<Order>,
    ser_type: &SerializeType,
    encode_with: &Option<CodecWith>,
) -> Result<TokenStream> {
    if let Some(CodecWith { path }) = encode_with {
        Ok(quote! { #path(#member, w)?; })
    } else {
        let ty = ser_type.ty.clone();
        match &ty.to_string()[..] {
            "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" | "f32" | "f64" => {
                if let Some(ord) = order {
                    let ty = format_ident!("write_{}", ty.to_string());
                    Ok(quote! { w.#ty::<#ord>(#member)?; })
                } else {
                    Err(Error::new_spanned(ty, "premitive integer needed Order"))
                }
            }
            _ => Ok(quote! { <#ty as ::protocol_core::Encoder>::encode(#member, w)?; }),
        }
    }
}
