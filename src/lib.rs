use proc_macro::TokenStream;
use quote::quote;
use syn::{parse2, Attribute, Field, Item, ItemStruct, Result, Token, Type};
use syn::fold::Fold;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

#[derive(Debug)]
enum MapStruct {
    TypeArg(Type),
    AttrArgs(Vec<Attribute>)
}

#[derive(Debug)]
struct MacroArguments(Vec<MapStruct>);

impl Fold for MapStruct {
    fn fold_field(&mut self, node: Field) -> Field {
        let mut new_node = node.clone();
        match self {
            MapStruct::AttrArgs(attr) => {
                new_node.attrs.extend(attr.clone());
            }
            MapStruct::TypeArg(ty) => {
                let old_ty = node.ty;
                new_node.ty = parse2::<Type>(quote! { #ty<#old_ty> }).unwrap();
            }
        }
        new_node
    }
}

impl Fold for MacroArguments {
    fn fold_item(&mut self, node: Item) -> Item {
        self.0
            .iter_mut()
            .fold(node, |acc, x| x.fold_item(acc))
            .into()
    }
}

impl Parse for MapStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![#]) {
            Ok(MapStruct::AttrArgs(input.call(Attribute::parse_outer)?))
        }
        else {
            Ok(MapStruct::TypeArg(input.parse::<Type>()?))
        }
    }
}

impl Parse for MacroArguments {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(MacroArguments(<Punctuated<MapStruct, Token![,]>>::parse_terminated(input)?
            .into_iter()
            .collect()))
    }
}

#[proc_macro_attribute]
pub fn map(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast: ItemStruct = parse2(item.into())
        .unwrap();
    let out = parse2::<MacroArguments>(attr.into())
        .unwrap()
        .fold_item(ast.into());

    (quote! { #out }).into()
}
