extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(WrapBox)]
pub fn wrapped_box_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_wrapped_box(&ast)
}

fn impl_wrapped_box(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl WrappedBox for #name {
            fn get_box(&self) -> ErgoBox {
                self.ergo_box.clone()
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(SpecBox)]
pub fn specified_box_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_specified_box(&ast)
}

fn impl_specified_box(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl SpecifiedBox for #name {
            pub fn new(b: &ErgoBox) -> std::result::Result<#name, ergo_protocol_framework::ProtocolFrameworkError> {
                Self::box_spec().verify_box(&b)?;
                return Ok(#name {
                    ergo_box: b.clone(),
                });
            }
        }
    };
    gen.into()
}
