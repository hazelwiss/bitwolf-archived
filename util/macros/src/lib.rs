use quote::quote;
use syn::{ItemStruct, Type};

mod custom_tokens {
    syn::custom_keyword!(ignore);
    syn::custom_keyword!(unsafe_cast_str);
}

struct Dump {
    name: syn::Ident,
    str_cast: bool,
    ty: Type,
}

fn dump_to_tokens(dump: Dump, unsafe_impl: bool) -> proc_macro2::TokenStream {
    let name = dump.name;
    let str = format!("{}{}:{}", "{}", name, "{}");
    let ty = dump.ty;
    let val = if unsafe_impl {
        quote!({
            ::core::ptr::addr_of!((*ptr).#name) as *const #ty
        })
    } else {
        quote!(&self.#name)
    };
    let expr = if dump.str_cast {
        quote!(::core::ffi::CStr::from_ptr(#val as *const _ as *const i8).to_str().unwrap())
    } else {
        quote!(#val)
    };
    let initial_expr = if unsafe_impl {
        quote!(::util::dumpable::UnsafeDumpString::dump_as_lines(#expr, depth + 1))
    } else {
        quote!(::util::dumpable::DumpString::dump_as_lines(#expr, depth + 1))
    };
    let printable = quote! {
        {
            let mut printable = ::alloc::string::String::new();
            let mut lines = #initial_expr;
            if lines.len() == 1 {
                printable = ::alloc::format!("{}{}", lines.pop().unwrap_unchecked().trim(), util::dumpable::__private::PAD_STR);
            } else{
                for line in lines{
                    printable = ::alloc::format!("{}\n{}", printable, line);
                }
            }
            printable
        }
    };
    let pad = quote!({
        let pad_str = ::util::dumpable::__private::PAD_STR;
        let mut padding = ::alloc::string::String::new();
        for _ in 0..depth {
            padding.push_str(pad_str);
        }
        padding
    });
    quote!(::alloc::format!(#str, #pad, #printable))
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
            let mut skip = false;
            let iter = field
                .attrs
                .iter()
                .filter(|attr| attr.path.segments.last().unwrap().ident == "dump");
            for attr in iter {
                if attr.parse_args::<custom_tokens::ignore>().is_ok() {
                    skip = true;
                    break;
                } else if attr.parse_args::<custom_tokens::unsafe_cast_str>().is_ok() && UNSAFE {
                    str_cast = true;
                } else {
                    panic!("invalid dump attribute {:?}", attr.tokens.to_string());
                }
            }
            if skip {
                continue;
            }
            let ident = field.ident.clone().unwrap();
            longest_ident = longest_ident.max(ident.to_string().len());
            dumps.push(Dump {
                name: field.ident.clone().unwrap(),
                str_cast,
                ty: field.ty.clone(),
            });
        }
        let mut brackets = String::new();
        let mut tokens = vec![];
        for dump in dumps {
            brackets.push_str("{}\n");
            tokens.push(dump_to_tokens(dump, UNSAFE));
        }
        let struct_ident = &parsed.ident;
        let dump_impl = quote! {
            ::alloc::format!(
                #brackets,
                #(#tokens),*
            )
        };
        let dump_lines_impl = quote! {
                let mut vec = ::alloc::vec![];
                #(vec.push(#tokens));*;
                vec
        };
        let impl_quote = if UNSAFE {
            quote!(
                unsafe impl UnsafeDumpString for #struct_ident {
                    unsafe fn dump(ptr: *const Self, depth: usize) -> ::alloc::string::String {
                        #dump_impl
                    }

                    unsafe fn dump_as_lines(ptr: *const Self, depth: usize) -> ::alloc::vec::Vec<::alloc::string::String> {
                        #dump_lines_impl
                    }
                }
            )
        } else {
            quote!(
                impl DumpString for #struct_ident {
                    fn dump(&self, depth: usize) -> ::alloc::string::String {
                        #dump_impl
                    }

                    fn dump_as_lines(&self, depth: usize) -> ::alloc::vec::Vec<::alloc::string::String> {
                        #dump_lines_impl
                    }
                }
            )
        };
        impl_quote.into()
    } else {
        panic!("Can only derive DumpString for structs.")
    }
}
