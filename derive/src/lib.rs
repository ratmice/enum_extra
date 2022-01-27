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
    let variants = if let syn::Data::Enum(e) = &ast.data {
        e.variants.iter()
    } else {
        panic!("NonZeroRepr only supports enums");
    };

    let discriminants = variants.filter_map(|v| match v {
        syn::Variant {
            discriminant: Some((_, d)),
            ..
        } => Some(d),
        _ => None,
    });

    let discriminants = discriminants.filter_map(|d| match d {
        syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Int(value),
            ..
        }) => {
            let discriminant = value.base10_parse::<u64>().unwrap();
            assert!(discriminant != 0);
            None
        }

        _ => Some(d),
    });

    let impl_body = quote!(
        #( const _: () = if ::core::num::NonZeroU8::new(#discriminants).is_some() { () } else { panic!("Expected non-zero discriminant") }; )*
        impl NonZeroRepr for #name {
        }

    );
    impl_body.into()
}
