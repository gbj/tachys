use proc_macro2::Span;
use proc_macro_error::{abort, abort_call_site, proc_macro_error};
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream, Parser},
    punctuated::Punctuated,
    token::Comma,
    Data, Field, Fields, Generics, Ident, Meta, MetaList, Result, Visibility,
    WhereClause, Type,
};

#[proc_macro_error]
#[proc_macro_derive(Store, attributes(store))]
pub fn derive_store(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    syn::parse_macro_input!(input as Model)
        .into_token_stream()
        .into()
}

struct Model {
    pub vis: Visibility,
    pub struct_name: Ident,
    pub generics: Generics,
    pub is_tuple_struct: bool,
    pub fields: Vec<Field>,
}

impl Parse for Model {
    fn parse(input: ParseStream) -> Result<Self> {
        let input = syn::DeriveInput::parse(input)?;

        let syn::Data::Struct(s) = input.data else {
            abort_call_site!("only structs can be used with `Store`");
        };

        let (is_tuple_struct, fields) = match s.fields {
            syn::Fields::Unit => {
                abort!(s.semi_token, "unit structs are not supported");
            }
            syn::Fields::Named(fields) => {
                (false, fields.named.into_iter().collect::<Vec<_>>())
            }
            syn::Fields::Unnamed(fields) => (
                true,
                fields
                    .unnamed
                    .into_iter()
                    .map(Into::into)
                    .collect::<Vec<_>>(),
            ),
        };

        Ok(Self {
            vis: input.vis,
            struct_name: input.ident,
            generics: input.generics,
            is_tuple_struct,
            fields,
        })
    }
}

#[derive(Clone)]
enum SubfieldMode {
    Subfield,
    Keyed(Ident),
}

impl Parse for SubfieldMode {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        if ident == "keyed" {
            Ok(SubfieldMode::Keyed(ident))
        } else {
            Err(input.error("expected `keyed`"))
        }
    }
}

fn modes_to_tokens(include_body: bool, modes: Option<&[SubfieldMode]>, library_path: &proc_macro2::TokenStream, ident: Option<&Ident>, generics: &Generics, any_store_field: &Ident, struct_name: &Ident, ty: &Type) -> proc_macro2::TokenStream {
    if let Some(modes) = modes {
        if modes.len() == 1 {
            let mode = &modes[0];
            match mode {
                SubfieldMode::Keyed(ident) => return quote! {
                    #[inline(always)]
                    fn #ident(self) ->  #library_path::Keyed<#any_store_field, #struct_name #generics, #ty>;
                },
                _ => {}
            }
        }
    }

    quote! {
        #[inline(always)]
        fn #ident(self) ->  #library_path::Subfield<#any_store_field, #struct_name #generics, #ty>;
    }
}

impl ToTokens for Model {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let library_path = quote! { ::tachys::tachy_reaccy::store };
        let Model {
            vis,
            struct_name,
            generics,
            is_tuple_struct,
            fields,
        } = &self;
        let any_store_field = Ident::new("AnyStoreField", Span::call_site());
        let field_names_struct_name =
            Ident::new(&format!("{struct_name}Fields"), struct_name.span());
        let trait_name = Ident::new(
            &format!("{struct_name}StoreFields"),
            struct_name.span(),
        );
        let generics_with_orig = {
            let params = &generics.params;
            quote! { <#any_store_field, #params> }
        };
        let where_with_orig = {
            generics
                .where_clause
                .as_ref()
                .map(|w| {
                    let WhereClause {
                        where_token,
                        predicates,
                    } = &w;
                    quote! {
                        #where_token
                            #any_store_field: #library_path::StoreField<#struct_name #generics>,
                            #predicates
                    }
                })
                .unwrap_or_else(|| quote! { where #any_store_field: #library_path::StoreField<#struct_name #generics> })
        };

        let field_names = fields.iter().map(|field| {
            let Field { vis, ident, ty, .. } = &field;

            quote! {
                fn #ident() {}
            }
        });

        // define an extension trait that matches this struct
        let trait_fields = fields.iter().map(|field| {
            let Field { vis, ident, ty, attrs, .. } = &field;
            let modes = attrs.iter().find_map(|attr| {
                attr.meta.path().is_ident("store").then(|| {
                    match &attr.meta {
                        Meta::List(list) => {
                            match Punctuated::<SubfieldMode, Comma>::parse_terminated.parse2(list.tokens.clone()) {
                                Ok(modes) => Some(modes.iter().cloned().collect::<Vec<_>>()),
                                Err(e) => abort!(list, e)
                            }
                        },
                        _ => None
                    }
                })
            }).flatten();

            modes_to_tokens(false, modes.as_deref(), &library_path, ident.as_ref(), generics, &any_store_field, struct_name, ty)            
        });

        // implement that trait for all StoreFields
        let read_fields = fields.iter().map(|field| {
            let Field { vis, ident, ty, .. } = &field;

            quote! {
                #[inline(always)]
                fn #ident(self) ->  #library_path::Subfield<#any_store_field, #struct_name #generics, #ty> {
                    #library_path::Subfield::new(
                        self,
                        (#field_names_struct_name::#ident as usize).into(),
                        |prev| &prev.#ident,
                        |prev| &mut prev.#ident,
                    )
                }
            }
        });

        // read access
        tokens.extend(quote! {
            struct #field_names_struct_name #generics {}

            impl #generics #field_names_struct_name #generics {
                #(#field_names)*
            }

            #vis trait #trait_name <AnyStoreField>
            #where_with_orig
            {
                #(#trait_fields)*
            }

            impl #generics_with_orig #trait_name <AnyStoreField> for AnyStoreField
            #where_with_orig
            {
               #(#read_fields)*
            }
        });
    }
}
