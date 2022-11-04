use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{braced, punctuated::Punctuated, Attribute, DeriveInput, Path, Token};

#[proc_macro_derive(FullPrint)]
pub fn full_print(ts: TokenStream) -> TokenStream {
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
                    quote!(format!("{}: {}", #ident_str, ::arm_decode::FullPrint::full_print(&self.#ident)))
                })
                .collect();
            let inputs: String = (0..fields.len()).map(|_| "{}, ").collect();
            let ident_str = ident.to_string();
            quote!(format!("{} {{ {} }}", #ident_str, format!(#inputs, #(#fields),*)))
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
                    let head = format!("{}::{}", ident, var_ident);
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

        impl ::core::fmt::Display for #ident #generics {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str(&self.full_print())
            }
        }
    }
    .into()
}

#[proc_macro]
#[allow(dead_code)]
pub fn struct_enum(ts: TokenStream) -> TokenStream {
    struct Field {
        atrs: Vec<Attribute>,
        ident: Ident,
        colon_tk: Token![:],
        ty: Path,
    }

    impl syn::parse::Parse for Field {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            let atrs = Attribute::parse_outer(input)?;
            let ident = input.parse()?;
            let colon_tk = input.parse()?;
            let ty = input.parse()?;
            Ok(Self {
                atrs,
                ident,
                colon_tk,
                ty,
            })
        }
    }

    impl quote::ToTokens for Field {
        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            let Self {
                atrs,
                ident,
                colon_tk,
                ty,
            } = self;
            tokens.extend(quote! {
                #(#atrs)*
                pub #ident #colon_tk #ty
            });
        }
    }

    struct Fields {
        brace: syn::token::Brace,
        fields: Punctuated<Field, Token![,]>,
    }

    impl syn::parse::Parse for Fields {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            let contents;
            let brace = braced!(contents in input);
            let fields = Punctuated::parse_terminated(&contents)?;
            Ok(Self { brace, fields })
        }
    }

    impl quote::ToTokens for Fields {
        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            let fields = &self.fields;
            tokens.extend(quote! {
                { #fields }
            });
        }
    }

    struct Variant {
        atrs: Vec<Attribute>,
        ident: Ident,
        fields: Option<Fields>,
    }

    impl syn::parse::Parse for Variant {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            let atrs = Attribute::parse_outer(input)?;
            let ident = input.parse()?;
            let fields = if input.peek(syn::token::Brace) {
                Some(input.parse()?)
            } else {
                None
            };
            Ok(Self {
                atrs,
                ident,
                fields,
            })
        }
    }

    struct Parse {
        atrs: Vec<Attribute>,
        pub_tk: Option<Token![pub]>,
        enum_tk: Token![enum],
        ident: Ident,
        brace: syn::token::Brace,
        variants: Punctuated<Variant, Token![,]>,
    }

    impl syn::parse::Parse for Parse {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            let atrs = Attribute::parse_outer(input)?;
            let pub_tk = input.parse()?;
            let enum_tk = input.parse()?;
            let ident = input.parse()?;
            let contents;
            let brace = braced!(contents in input);
            let variants = Punctuated::parse_terminated(&contents)?;
            Ok(Self {
                atrs,
                pub_tk,
                enum_tk,
                ident,
                brace,
                variants,
            })
        }
    }

    impl quote::ToTokens for Parse {
        fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
            let Self {
                atrs,
                pub_tk,
                enum_tk,
                ident,
                variants,
                ..
            } = self;
            let mut inner = vec![];
            let mut outer = vec![];
            for variant in variants {
                let Variant {
                    atrs,
                    ident,
                    fields,
                    ..
                } = variant;
                if let Some(fields) = fields {
                    inner.push(quote!(#ident (#ident)));
                    outer.push(quote! {
                        #(#atrs)*
                        #pub_tk struct #ident #fields
                    });
                } else {
                    inner.push(quote!(#ident));
                }
            }
            tokens.extend(quote! {
                #(#atrs)*
                #pub_tk #enum_tk #ident {
                    #(#inner),*
                }
                #(#outer)*
            });
        }
    }

    let parsed: Parse = syn::parse(ts).expect("failed to parse");
    quote!(#parsed).into()
}
