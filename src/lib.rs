use proc_macro::TokenStream;
use quote::quote;
use syn::{parse2, parse_macro_input, parse_str, Attribute, Field, Ident, ItemStruct, Result, Type};
use syn::fold::Fold;
use syn::parse::{Parse, ParseStream};

struct Flag(bool);
struct OptionalFields;
#[derive(Default)]
struct SerdeSkip {
    attrs: Vec<Attribute>
}

impl SerdeSkip {
    fn new() -> Self {
        Default::default()
    }
}

impl Fold for OptionalFields {
    fn fold_type(&mut self, node: Type) -> Type {
        parse2(quote! { Option<#node> })
            .unwrap()
    }
}

impl Fold for SerdeSkip {
    fn fold_field(&mut self, node: Field) -> Field {
        let mut new_node = node.clone();
        let serde_hint: Vec<Attribute> = parse_str::<SerdeSkip>(r#"#[serde(skip_serializing_if = "Option::is_none")]"#)
            .unwrap()
            .attrs;
        new_node.attrs
            .extend(serde_hint);
        new_node
    }
}

impl Parse for Flag {
    fn parse(input: ParseStream) -> Result<Self> {
        let flag = match input.parse::<Ident>() {
            Ok(s) if s.to_string() == "compact" => Flag(true),
            _ => Flag(false)
        };
        Ok(flag)
    }
}

impl Parse for SerdeSkip {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(SerdeSkip {
            attrs: input.call(Attribute::parse_outer)?
        })
    }
}

#[proc_macro_attribute]
pub fn anyof(attr: TokenStream, item: TokenStream) -> TokenStream {
    let ast: ItemStruct = parse2(item.into())
        .unwrap();
    let flag = parse_macro_input!(attr as Flag);

    let mut out = OptionalFields.fold_item(ast.into());

    if flag.0 {
        out = SerdeSkip::new().fold_item(out);
    }

    (quote! { #out }).into()
}
