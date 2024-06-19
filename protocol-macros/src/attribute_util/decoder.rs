use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{Error, GenericArgument, Ident, PathArguments, Result, TypePath};

use crate::Field;

use super::{CodecWith, Dependency, Order, SerializeType};

pub fn create_token(name: Ident, members: &[Ident], fields: &[Field]) -> Result<TokenStream> {
    let decode_tokens = fields
        .iter()
        .map(
            |Field {
                 member,
                 ty,
                 order,
                 dependency,
                 decode_with,
                 ..
             }| {
                if dependency.is_some() {
                    create_option_stmt(member, dependency, order, ty, decode_with)
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
                        decode_with,
                    )
                }
            },
        )
        .collect::<Result<Vec<TokenStream>>>()?;
    Ok(quote! {
        impl ::protocol_core::Decoder for #name {
            fn decode<R>(r: &mut R) -> ::anyhow::Result<Self>
            where
                R: ::protodef::ReadBytesExt,
                Self: Sized,
            {
                #( #decode_tokens )*
                Ok(
                    Self {
                        #(#members),*
                    }
                )
            }
        }
    })
}

fn create_option_stmt(
    member: &Ident,
    dependency: &Option<Dependency>,
    order: &Option<Order>,
    ty: &TypePath,
    decode_with: &Option<CodecWith>,
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
        decode_with,
    )?;
    Ok(v)
}

fn create_stmt(
    member: &Ident,
    dependency: &Option<Dependency>,
    order: &Option<Order>,
    ser_type: &SerializeType,
    decode_with: &Option<CodecWith>,
) -> Result<TokenStream> {
    let body = create_body(order, ser_type, decode_with)?;
    let result = if let Some(dep) = dependency {
        if ser_type.contained_option {
            let expr = dep.expr.clone();
            quote! {
                let #member = if #expr {
                    Some(#body)
                } else {
                    None
                };
            }
        } else {
            return Err(Error::new_spanned(
                member,
                "this type must be wrapped Option.",
            ));
        }
    } else {
        quote! {
            let #member = #body;
        }
    };
    Ok(result)
}

fn create_body(
    order: &Option<Order>,
    ser_type: &SerializeType,
    decode_with: &Option<CodecWith>,
) -> Result<TokenStream> {
    if let Some(CodecWith { path }) = decode_with {
        Ok(quote! { #path(r)? })
    } else {
        let ty = ser_type.ty.clone();
        match &ty.to_string()[..] {
            "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" | "f32" | "f64" => {
                if let Some(ord) = order {
                    let ty = format_ident!("read_{}", ty.to_string());
                    Ok(quote! { r.#ty::<#ord>()? })
                } else {
                    Err(Error::new_spanned(ty, "premitive integer needed Order"))
                }
            }
            _ => Ok(quote! { <#ty as ::protocol_core::Decoder>::decode(r)? }),
        }
    }
}
