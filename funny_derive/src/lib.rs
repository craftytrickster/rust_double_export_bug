extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(Funny)]
pub fn funny_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_funny_macro(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}


fn impl_funny_macro(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl Funny for #name {
            fn joke(&self) {
                println!("Too lazy to think of a joke");
            }
        }
    }
}