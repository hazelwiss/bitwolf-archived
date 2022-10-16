use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(FullPrint)]
pub fn arm_decode(ts: TokenStream) -> TokenStream {
    let parse = syn::parse::<DeriveInput>(ts).expect("failed to parse derive input");
    let DeriveInput {
        ident,
        generics,
        data,
        ..
    } = parse;
    let mut generic_args = generics.clone();
    generic_args.where_clause = None;
    let print: proc_macro2::TokenStream = match data {
        syn::Data::Struct(data) => {
            let fields: Vec<proc_macro2::TokenStream> = data
                .fields
                .into_iter()
                .map(|e| {
                    let ident = e.ident.expect("expected named field");
                    let ident_str = ident.to_string();
                    quote!(format!("{}: {}", #ident_str, ::arm_decode::FullPrint::full_print(self.#ident)))
                })
                .collect();
            let inputs: String = (0..fields.len()).map(|_| "{}, ").collect();
            quote!(format!("{} {{ {} }}", #ident, format!(#inputs, #(#fields),*)))
        }
        syn::Data::Enum(data) => {
            let variants: Vec<proc_macro2::TokenStream> = data
                .variants
                .into_iter()
                .map(|e| {
                    let var_ident = e.ident;
                    let (declarations, tail) = match e.fields {
                        syn::Fields::Named(named) => {
                            let mut field_declarations = vec![];
                            let mut field_logic = vec![];
                            named.named.into_iter().for_each(|e| {
                                let name = e.ident.expect("missing identifier");
                                let name_str = name.to_string();
                                field_declarations.push(quote!(#name));
                                field_logic.push(quote!(format!("{}: {}", #name_str, ::arm_decode::FullPrint::full_print(#name))))
                            });
                            let inputs: String = (0..field_logic.len()).map(|_| "{}, ").collect();
                            let inputs = format!("{{{{ {inputs} }}}}");
                            (
                                quote!({ #(#field_declarations),* }),
                                quote!(format!(#inputs, #(#field_logic),*)),
                            )
                        }
                        syn::Fields::Unnamed(unnamed) => {
                            let mut field_declarations = vec![];
                            let mut field_logic = vec![];
                            unnamed.unnamed.into_iter().for_each(|e| {
                                let name = e.ident;
                                field_declarations.push(quote!(#name));
                                field_logic.push(quote!(::arm_decode::FullPrint::full_print(#name)))
                            });
                            let inputs: String = (0..field_logic.len()).map(|_| "{}, ").collect();
                            let inputs = format!("({inputs})");
                            (
                                quote!(( #(#field_declarations),* )),
                                quote!(format!(#inputs, #(#field_logic),*)),
                            )
                        }
                        syn::Fields::Unit => (quote!(), quote!("")),
                    };
                    let head = format!("{}::{}", ident.to_string(), var_ident.to_string());
                    quote!(#ident :: #var_ident #declarations => format!("{} {}", #head, #tail))
                })
                .collect();
            quote!(
                match &self {
                    #(#variants),*
                }
            )
        }
        syn::Data::Union(_) => panic!("unions are unsupported"),
    };
    quote! {
        #[automatically_derived]
        impl #generic_args FullPrint for #ident #generics{
            fn full_print(&self) -> String{
                #print
            }
        }
    }
    .into()
}
