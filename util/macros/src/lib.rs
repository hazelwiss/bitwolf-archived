use quote::quote;
use syn::ItemStruct;

mod custom_tokens {
    syn::custom_keyword!(ignore);
    syn::custom_keyword!(unsafe_cast_str);
}

enum BinaryFormat {
    B8,
    B16,
    B32,
    B64,
    B128,
    None,
}
struct Dump {
    bformat: BinaryFormat,
    name: syn::Ident,
    str_cast: bool,
}

fn dump_to_tokens(dump: Dump, str_padding: &str) -> proc_macro2::TokenStream {
    let (capture, capture_bin) = match &dump.bformat {
        BinaryFormat::B8 => ("0x{:02X}", Some("0b{:08b}")),
        BinaryFormat::B16 => ("0x{:04X}", Some("0b{:016b}")),
        BinaryFormat::B32 => ("0x{:08X}", Some("0b{:032b}")),
        BinaryFormat::B64 => ("0x{:016X}", None),
        BinaryFormat::B128 => ("0x{:32X}", None),
        BinaryFormat::None => ("{}", None),
    };
    let capture = if let Some(capture_bin) = capture_bin {
        format!("{capture} ({capture_bin})")
    } else {
        format!("{capture}")
    };
    let name = &dump.name;
    let str = format!("{}: {}{}", name, str_padding, capture);
    let expr = if dump.str_cast {
        quote!(std::ffi::CStr::from_ptr(&self.#name as *const _ as *const i8).to_str().unwrap())
    } else {
        quote!(self.#name)
    };
    let output = if capture_bin.is_some() {
        quote!(format!(#str, {#expr}, {#expr}))
    } else {
        quote!(format!(#str, {#expr}))
    };
    output
}

/// # Custom attributes:
///
/// ## dump(ignore)
/// this till ignore the field and not dump it.
#[proc_macro_derive(DumpString, attributes(dump))]
pub fn derive_dump_string(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    dump_derive::<false>(ts)
}

/// # Custom attributes:
///
/// ## dump(ignore)
/// this till ignore the field and not dump it.
///
/// ## dump(unsafe_cast_str)
/// this will treat a field as if it was a utf-8 string. This is unsafe!
#[proc_macro_derive(UnsafeDumpString, attributes(dump))]
pub fn derive_unsafe_dump_string(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    dump_derive::<true>(ts)
}

fn dump_derive<const UNSAFE: bool>(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    if let Ok(parsed) = syn::parse::<ItemStruct>(ts) {
        let mut dumps = vec![];
        let mut longest_ident = 0;
        for field in &parsed.fields {
            let mut str_cast = false;
            if let Some(attr) = field
                .attrs
                .iter()
                .find(|attr| attr.path.segments.last().unwrap().ident.to_string() == "dump")
            {
                if attr.parse_args::<custom_tokens::ignore>().is_ok() {
                    continue;
                } else if attr.parse_args::<custom_tokens::unsafe_cast_str>().is_ok() && UNSAFE {
                    str_cast = true;
                } else {
                    panic!("invalid dump attribute {:?}", attr.tokens.to_string());
                }
            }
            let bformat = match &field.ty {
                syn::Type::Path(p) if !str_cast => {
                    match p.path.segments.last().unwrap().ident.to_string().as_str() {
                        "u8" => BinaryFormat::B8,
                        "u16" => BinaryFormat::B16,
                        "u32" => BinaryFormat::B32,
                        "u64" => BinaryFormat::B64,
                        "u128" => BinaryFormat::B128,
                        _ => BinaryFormat::None,
                    }
                }
                _ => BinaryFormat::None,
            };
            let ident = field.ident.clone().unwrap();
            longest_ident = longest_ident.max(ident.to_string().len());
            dumps.push(Dump {
                bformat,
                name: field.ident.clone().unwrap(),
                str_cast,
            });
        }
        let mut brackets = String::new();
        let mut tokens = vec![];
        for dump in dumps {
            brackets.push_str("{}\n");
            let mut padding = String::with_capacity(longest_ident);
            for _ in 0..longest_ident - dump.name.to_string().len() {
                padding.push(' ');
            }
            tokens.push(dump_to_tokens(dump, &padding));
        }
        let struct_ident = &parsed.ident;
        let impl_quote = if UNSAFE {
            quote!(
                unsafe impl UnsafeDumpString for #struct_ident {
                    unsafe fn dump(&self) -> String{
                        format!(
                            #brackets,
                            #(#tokens),*
                        )
                    }

                    unsafe fn dump_as_lines(&self) -> Vec<String>{
                        let mut vec = vec![];
                        #(vec.push(#tokens));*;
                        vec
                    }
                }
            )
        } else {
            quote!(
                impl DumpString for #struct_ident {
                    fn dump(&self) -> String{
                        format!(
                            #brackets,
                            #(#tokens),*
                        )
                    }

                    fn dump_as_lines(&self) -> Vec<String>{
                        let mut vec = vec![];
                        #(vec.push(#tokens));*;
                        vec
                    }
                }
            )
        };
        impl_quote.into()
    } else {
        panic!("Can only derive DumpString for structs.")
    }
}
