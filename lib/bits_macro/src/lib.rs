use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Attribute, DataStruct, DeriveInput, Fields::Unnamed, FieldsUnnamed, Meta,
    MetaList,
};

#[proc_macro_derive(Bits, attributes(size))]
pub fn bits_macro_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_bits_macro(&ast)
}

fn impl_bits_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let field = match &ast.data {
        syn::Data::Struct(DataStruct {
            fields: Unnamed(FieldsUnnamed { unnamed: f, .. }),
            ..
        }) => f.first().unwrap(),
        _ => unimplemented!("Only works for structs with a unnamed field"),
    };
    let field_size = match field.attrs.first().unwrap() {
        Attribute {
            meta: Meta::List(MetaList { tokens: t, .. }),
            ..
        } => t.to_string().parse::<u8>().unwrap(),
        _ => unimplemented!(),
    };
    let format_str = format!("0b{{:0{}b}}", field_size);

    let gen = quote! {
        impl Bits for #name {
            fn to_u64(&self) -> u64{
                self.0.into()
            }
            fn size(&self) -> u8{
                #field_size
            }
        }

        impl Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, #format_str, &self.0)
            }
        }
    };
    gen.into()
}
