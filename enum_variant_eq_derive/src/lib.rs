extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    DeriveInput,
    Data,
    Fields,
};


#[proc_macro_derive(EnumVariantEq)]
pub fn enum_variant_eq_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = match syn::parse(input) {
        Ok(x) => x,
        Err(e) => {
            panic!("Build Ast Error: {}", e);
        }
    };
    impl_enum_variant_eq(&ast)
}
fn impl_enum_variant_eq(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let data = if let Data::Enum(data) = &ast.data {
        data
    } else { panic!("Type Is Not Enum") };
    let mut body = quote!();
    let mut empty_enum: bool = true;
    for i in &data.variants {
        empty_enum &= false;
        let ident = &i.ident;
        let pat = match i.fields {
            Fields::Unit => quote!( Self::#ident ),
            Fields::Named(..) => quote!( Self::#ident { .. } ),
            Fields::Unnamed(..) => quote!( Self::#ident(..) ),
        };
        let item = quote!{
            #pat => if let #pat = other { true } else { false },
        };
        body = quote!( #body #item );
    }
    if empty_enum {
        return TokenStream::new()
    }
    let (impl_generics,
         ty_generics,
         where_clause)
        = ast.generics.split_for_impl();
    let gen = quote!{
        impl #impl_generics EnumVariantEq for #name #ty_generics #where_clause {
            fn enum_variant_eq(&self, other: &Self) -> bool {
                match self {
                    #body
                }
            }
        }
    };
    gen.into()
}
