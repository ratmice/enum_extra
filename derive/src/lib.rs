extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(NonZeroRepr)]
pub fn non_zero_repr(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_non_zero_repr(&ast)
}

fn impl_non_zero_repr(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let mut variants = if let syn::Data::Enum(e) = &ast.data {
        e.variants.iter()
    } else {
        panic!("NonZeroRepr only supports enums")
    };

    let first = variants.next();

    let mut first = match first {
        Some(syn::Variant {
            discriminant: None, ..
        }) => {
            panic!("Enum element defaults to zero")
        }
        Some(syn::Variant {
            discriminant: Some((_, expr)),
            ..
        }) => {
            match expr {
                syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Int(value),
                    ..
                }) => {
                    let discriminant = value.base10_parse::<u64>().unwrap();
                    assert!(discriminant != 0);
                    vec![expr]
                }
                _ => {
                    vec![expr]
                }
            }
        }
        None => {
            vec![]
        }
    };

    first.extend(variants.filter_map(|v| match v {
        syn::Variant {
            discriminant: Some((_, discriminant)),
            ..
        } => Some(discriminant),
        _ => None,
    }));

    let discriminants = first.iter().filter(|d| match d {
        syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Int(value),
            ..
        }) => {
            let discriminant = value.base10_parse::<u64>().unwrap();
            assert!(discriminant != 0);
            false
        }

        _ => true,
    });

    let impl_body = quote!(


            #( #[allow(clippy::unused_unit)] const _: () = if ::core::num::NonZeroU8::new(#discriminants).is_some() { () } else { panic!("Expected non-zero discriminant") }; )*
            impl NonZeroRepr for #name {

            }


    );
    impl_body.into()
}
