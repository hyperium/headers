extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::{Data, Fields, Ident};

#[proc_macro_derive(Header, attributes(header))]
pub fn derive_header(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_header(&ast).into()
}

fn impl_header(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let fns = match impl_fns(ast) {
        Ok(fns) => fns,
        Err(msg) => {
            return quote! {
                compile_error!(#msg);
            }.into();
        }
    };

    let decode = fns.decode;
    let encode = fns.encode;

    let ty = &ast.ident;
    let hname = Ident::new(&ty.to_string().to_uppercase(), Span::call_site());
    let dummy_const = Ident::new(&format!("_IMPL_HEADER_FOR_{}", ty), Span::call_site());
    let impl_block = quote! {
        impl __hc::Header for #ty {
            const NAME: &'static __hc::HeaderName = &__hc::header::#hname;
            fn decode(values: &mut __hc::Values) -> __hc::Result<Self> {
                #decode
            }

            fn encode(&self, values: &mut __hc::ToValues) {
                #encode
            }
        }
    };

    quote! {
        const #dummy_const: () = {
            extern crate headers_core as __hc;
            #impl_block
        };
    }
}

struct Fns {
    encode: proc_macro2::TokenStream,
    decode: proc_macro2::TokenStream,
}

fn impl_fns(ast: &syn::DeriveInput) -> Result<Fns, String> {
    let ty = &ast.ident;
    let st = match ast.data {
        Data::Struct(ref st) => st,
        _ => {
            return Err("derive(Header) only works on structs".into())
        }
    };

    match st.fields {
        Fields::Named(ref fields) => {
            if fields.named.len() != 1 {
                return Err("derive(Header) doesn't support multiple fields".into());
            }

            let _field = fields
                .named
                .iter()
                .next()
                .expect("just checked for len() == 1");

            let decode = quote! {
                unimplemented!("derive(Header) decode for named fields")
            };
            let encode = quote! {
                unimplemented!("derive(Header) encode for named fields")
            };

            Ok(Fns {
                decode,
                encode,
            })
        },
        Fields::Unnamed(ref fields) => {
            if fields.unnamed.len() != 1 {
                return Err("derive(Header) doesn't support multiple fields".into());
            }

            let decode = quote! {
                __hc::decode::TryFromValues::try_from_values(values)
                    .map(#ty)
            };
            let encode = quote! {
                values.append((&self.0).into());
            };

            Ok(Fns {
                decode,
                encode,
            })
        },
        Fields::Unit => {
            Err("ah".into())
        }
    }
}
