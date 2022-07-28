use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use std::collections::HashMap;
use syn::{Arm, Attribute, Expr, ExprMatch, Ident, LitStr, Token};

#[proc_macro_attribute]
pub fn bitmatch(attr: TokenStream, stream: TokenStream) -> TokenStream {
    assert!(
        attr.is_empty(),
        "bitmatch attribute may only be written as bare attribute name"
    );
    let expr = syn::parse::<Expr>(stream).expect("Invalid expression following bitmatch attribute");
    match expr {
        Expr::Match(expr) => parse(expr),
        _ => panic!("bitmatch attribute only accepts match expressions"),
    }
}

fn parse(expr: ExprMatch) -> TokenStream {
    let parse = Match { expr };
    quote! { #parse }.into()
}

struct Match {
    expr: ExprMatch,
}

struct BitPattern {
    vars: Vec<MatchVar>,
    mask: u128,
    val: u128,
}

enum MatchPattern {
    Bit {
        attrs: Vec<Attribute>,
        bp: BitPattern,
    },
    Or {
        attrs: Vec<Attribute>,
        cases: Vec<(Vec<Attribute>, BitPattern)>,
    },
    Ident {
        attrs: Vec<Attribute>,
        mutability: Option<Token![mut]>,
        ident: Ident,
    },
    Wild {
        attrs: Vec<Attribute>,
    },
}

struct MatchArm {
    attrs: Vec<Attribute>,
    pattern: MatchPattern,
    body: Box<Expr>,
}

#[derive(Clone)]
struct MatchVar {
    ident: char,
    mask: u128,
    len: usize,
}

impl ToTokens for Match {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let expr = self.expr.clone();
        let arms: Vec<MatchArm> = expr
            .arms
            .into_iter()
            .map(|arm| MatchArm::from_arm(arm))
            .collect();
        let match_attributes = &expr.attrs;
        let match_expr = &expr.expr;
        tokens.extend(quote! {
            #(#match_attributes)*
            match #match_expr {
                #(#arms)*
            }
        });
    }
}

impl MatchArm {
    fn from_arm(arm: Arm) -> Self {
        fn parse_bp(str: LitStr) -> BitPattern {
            let string = str.value();
            let mut mask = 0u128;
            let mut val = 0u128;
            let mut pat_vars: HashMap<char, MatchVar> = HashMap::new();
            let mut i = 0;
            for c in string.chars().rev() {
                let bit = 1 << i;
                match c {
                    '0' => {
                        mask |= bit;
                        i += 1;
                    }
                    '1' => {
                        mask |= bit;
                        val |= bit;
                        i += 1;
                    }
                    'z' | 'Z' => i += 1,
                    'a'..='y' | 'A'..='Y' => {
                        if let Some(var) = pat_vars.get_mut(&c) {
                            var.mask |= bit;
                            var.len += 1;
                        } else {
                            pat_vars.insert(
                                c,
                                MatchVar {
                                    ident: c,
                                    mask: bit,
                                    len: 1,
                                },
                            );
                        }
                        i += 1
                    }
                    '_' | '-' | ' ' => {}
                    _ => panic!("Invalid character '{c}' in string literal"),
                }
            }
            BitPattern {
                vars: pat_vars.into_values().into_iter().collect(),
                mask,
                val,
            }
        }
        if arm.guard.is_some() {
            panic!("match arm cannot have guard")
        }
        let pattern = match arm.pat {
            syn::Pat::Wild(wild) => MatchPattern::Wild { attrs: wild.attrs },
            syn::Pat::Ident(ident) => {
                assert!(!ident.by_ref.is_some(), "identifier cannot be a reference");
                assert!(
                    !ident.subpat.is_some(),
                    "illegal subpatern detected in identifier"
                );
                MatchPattern::Ident {
                    attrs: ident.attrs,
                    ident: ident.ident,
                    mutability: ident.mutability,
                }
            }
            syn::Pat::Or(or) => {
                let cases: Vec<(Vec<Attribute>, BitPattern)> = or
                    .cases
                    .into_iter()
                    .map(|case| match case {
                        syn::Pat::Lit(pat) => match *pat.expr {
                            Expr::Lit(mut lit) => match lit.lit {
                                syn::Lit::Str(str) => (
                                    {
                                        let mut vec = pat.attrs;
                                        vec.append(&mut lit.attrs);
                                        vec
                                    },
                                    parse_bp(str),
                                ),
                                _ => panic!("literal has to be a string"),
                            },
                            _ => panic!("pattern expression may only be a string literal"),
                        },
                        _ => panic!(
                            "bitmatch only accepts string literals or identifiers as patterns"
                        ),
                    })
                    .collect();
                assert!(
                    !cases.is_empty(),
                    "multiple arm pattern cases cannot be empty."
                );
                let base_case = &cases[0];
                for i in 1..cases.len() {
                    let vars = &base_case.1.vars;
                    let cur = &cases[i].1.vars;
                    vars.iter().for_each(|var| {
                        assert!(
                            cur.iter()
                                .find(|search| var.len == search.len && var.ident == search.ident)
                                .is_some(),
                            "mismatching variable '{}' within multiple patterns in match arm",
                            var.ident
                        )
                    });
                }
                MatchPattern::Or {
                    attrs: or.attrs,
                    cases,
                }
            }
            syn::Pat::Lit(pat) => match *pat.expr {
                Expr::Lit(mut lit) => match lit.lit {
                    syn::Lit::Str(str) => MatchPattern::Bit {
                        attrs: {
                            let mut vec = pat.attrs;
                            vec.append(&mut lit.attrs);
                            vec
                        },
                        bp: parse_bp(str),
                    },
                    _ => panic!("literal has to be a string"),
                },
                _ => panic!("pattern expression may only be a string literal"),
            },
            _ => panic!("bitmatch only accepts string literals or identifiers as patterns"),
        };
        Self {
            attrs: arm.attrs,
            pattern,
            body: arm.body,
        }
    }
}

impl ToTokens for MatchArm {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let bit_pattern_body = |body, attrs: &Vec<Attribute>, bp: &BitPattern| {
            let BitPattern { vars, mask, val } = &bp;
            let vars_init_vec: Vec<proc_macro2::TokenStream> = vars
                .iter()
                .map(|var| {
                    let var_name = Ident::new(var.ident.to_string().as_str(), Span::call_site());
                    let mask = var.mask;
                    quote! {
                        let #var_name = ::bitmatch::from_mask(_full_bitmatch, #mask);
                    }
                })
                .collect();
            quote! {
                #(#attrs)*
                _full_bitmatch if ({let val: u128 = _full_bitmatch.into(); val} & #mask) == #val => {
                    #(#vars_init_vec)*
                    {
                        // static assertions.
                    }
                    { #body }
                }
            }
        };
        let body = &self.body;
        let body = match &self.pattern {
            MatchPattern::Bit { attrs, bp } => bit_pattern_body(body, attrs, bp),
            MatchPattern::Or { attrs, cases } => {
                let cases: Vec<proc_macro2::TokenStream> = cases
                    .iter()
                    .map(|(attrs, case)| {
                        let body = bit_pattern_body(body, attrs, case);
                        quote! { #body }
                    })
                    .collect();
                quote!(
                    #(#attrs)*
                    #(#cases)*
                )
            }
            MatchPattern::Ident {
                ident,
                mutability,
                attrs,
            } => quote! {
                #(#attrs)*
                #mutability #ident => { #body }
            },
            MatchPattern::Wild { attrs } => {
                quote! {
                    #(#attrs)*
                    _ => { #body }
                }
            }
        };
        let attrs = &self.attrs;
        tokens.extend(quote! {
            #(#attrs)*
            #body
        });
    }
}
