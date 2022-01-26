extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(NonZeroRepr)]
pub fn non_zero_repr(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_non_zero_repr(&ast)
}

fn impl_non_zero_repr(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    match &ast.data {
        syn::Data::Enum(e) => {
            for v in &e.variants {
                match v {
                    syn::Variant {
                        discriminant:
                            Some((
                                _,
                                syn::Expr::Lit(syn::ExprLit {
                                    lit: syn::Lit::Int(value),
                                    ..
                                }),
                            )),
                        ..
                    } => {
                        let discriminant = value.base10_parse::<u64>().unwrap();
                        assert!(discriminant != 0)
                    }
                    _ => {
                        panic!("Discriminant not a literal int.");
                    }
                }
            }
        }
        _ => {
            panic!("Not an enum");
        }
    }
    let impl_body = quote!(
        impl NonZeroRepr for #name {
        }
    );
    impl_body.into()
}
