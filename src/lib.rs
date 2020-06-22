use proc_macro::TokenStream;
use quote::quote;
use syn::{token, Data, DeriveInput, Fields};

macro_rules! quote_flat {
    ($x: expr) => {
        {
            $x
                .iter()
                .fold(quote! { }, |xs, x| {
                    quote!{ #xs #x }
                })
        }
    };
}

#[proc_macro_attribute]
pub fn anyof(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(item).unwrap();

    let attrs = quote_flat![&ast.attrs];
    let vis = ast.vis;
    let ident = ast.ident;
    let generics = ast.generics;

    match ast.data {
        Data::Struct(s) => {
            let token = s.struct_token;
            let fields = s.fields
                .iter()
                .map(|x| {
                    let attrs = quote_flat![&x.attrs];
                    let vis = &x.vis;
                    let ident = &x.ident;
                    let colon_token = &x.colon_token;
                    let ty = &x.ty;

                    quote! {
                        #attrs #vis #ident #colon_token Option<#ty>,
                    }
                })
                .fold(quote! { }, |xs, x| {
                    quote! { #xs #x }
                });
            let semi = s.semi_token;
            quote! {
                #attrs #vis #token #ident #generics (#fields) #semi
            }
        }
        _ => panic!("#[anyof] only valid for structs")
    }.into()
}
