use proc_macro::TokenStream;
use quote::quote;
use syn::token::In;

impl Into<Vec<u8>> for u32 {
    fn into(self) -> Vec<u8> {
        bytemuck::cast_slice(&[self]).to_owned()
    }
}

#[proc_macro_derive(ToWgslShaderSlice)]
pub fn derive_to_wgsl_shader_slice(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);

    let struct_idendifier = input.ident;

    Vec

    match input.data {
        #[automaticly_derived]
        syn::Data::Struct(syn::DataStruct{ fields, .. }) => {
            quote! {
                impl From<#struct_idendifier> for &[u8] {
                    fn from()
                }
            }
        },

        _ => unimplemented!()
    }

    todo!()
}