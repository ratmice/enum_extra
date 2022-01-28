extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenTree;
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

    let mut types = std::collections::HashMap::new();
    // FIXME more types..
    for (k, v) in [("u8", "NonZeroU8")] {
        types.insert(k.to_string(), v.to_string());
    }
    let nz_type_str = (|| {
        for syn::Attribute {
            tokens,
            path: syn::Path { segments, .. },
            ..
        } in &ast.attrs
        {
            for syn::PathSegment { ident, .. } in segments {
                if ident == "repr" {
                    let mut tokens = tokens.clone().into_iter();
                    // Not sure how to excercise these assertions.
                    let token = tokens.next().unwrap();
                    assert!(tokens.count() == 0);
                    match token {
                        TokenTree::Group(group) => {
                            for tree in group.stream() {
                                match tree {
                                    TokenTree::Ident(ident) => {
                                        let r#type = format!("{}", ident);
                                        return types.get(&r#type);
                                    }
                                    TokenTree::Group(_)
                                    | TokenTree::Punct(_)
                                    | TokenTree::Literal(_) => panic!(),
                                }
                            }
                        }
                        _ => continue,
                    }
                }
            }
        }
        None
    })();

    let nz_type: syn::Type = if let Some(nz_type_str) = nz_type_str {
        syn::parse2(nz_type_str.parse().unwrap()).unwrap()
    } else {
        panic!("Unknown repr type");
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
        }) => match expr {
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
        },
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


            #( #[allow(clippy::unused_unit)] const _: () = if ::core::num::NonZeroU8::new(#discriminants).is_some() { () } else { panic!("Expected non-zero discriminant expression") }; )*
            impl NonZeroRepr for #name {
                type NonZeroRepr = ::core::num::#nz_type;
                fn nonzero_repr(self) -> Self::NonZeroRepr {
                    let repr = self.to_repr();
                    unsafe { ::core::num::#nz_type::new_unchecked(repr) }
                }
            }


    );
    impl_body.into()
}
