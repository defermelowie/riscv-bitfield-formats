use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};

#[proc_macro_derive(Csr)]
pub fn csr(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_csr_macro(&ast)
}

fn impl_csr_macro(ast: &DeriveInput) -> TokenStream {
    // Rust source identifiers
    let struct_name = &ast.ident;
    let fields = match &ast.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected struct with named fields"),
    };
    let field_name = fields.iter().map(|field| {
        field
            .ident
            .clone()
            .expect("expected struct with named fields")
    });

    // Identifiers for printing
    let name_str = format!("{}", struct_name).to_lowercase();
    let hdiv_str = "-".repeat(name_str.len());
    let field_fname = field_name.clone();
    let field_fstr = field_name.clone().map(|field| format!("{}: {{}}", field));

    // Generate code
    let gen = quote! {
        impl Display for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                writeln!(f, #name_str)?;
                writeln!(f, #hdiv_str)?;
                #(
                    writeln!(f, #field_fstr, &self.#field_fname)?;
                )*
                Ok(())
            }
        }

        impl Csr for #struct_name {
            fn new(value: u64) -> Self {
                #struct_name {
                    #(
                        #field_name: value.into(),
                    )*
                }
            }

            fn name() -> String {
                "#name_p".into()
            }
        }
    };
    gen.into()
}
