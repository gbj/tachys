use proc_macro2::Span;
use proc_macro_error::{abort, abort_call_site, proc_macro_error};
use quote::{quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    Data, Field, Fields, Generics, Ident, Result, Visibility, WhereClause,
};

#[proc_macro_error]
#[proc_macro_derive(Store, attributes(bundle))]
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
            abort_call_site!("only structs can be used with `SignalBundle`");
        };

        let (is_tuple_struct, fields) = match s.fields {
            syn::Fields::Unit => {
                abort!(s.semi_token, "unit structs are not supported");
            }
            syn::Fields::Named(fields) => (
                false,
                fields.named.into_iter().map(Into::into).collect::<Vec<_>>(),
            ),
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
        let orig = Ident::new("OriginTy", Span::call_site());
        let field_names_struct_name =
            Ident::new(&format!("{struct_name}Fields"), struct_name.span());
        let read_trait_name = Ident::new(
            &format!("{struct_name}ReadStoreFields"),
            struct_name.span(),
        );
        let write_trait_name = Ident::new(
            &format!("{struct_name}WriteStoreFields"),
            struct_name.span(),
        );
        let generics_with_orig = {
            let params = &generics.params;
            quote! { <#orig, Path, Reader, Writer, #params> }
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
                        #where_token #orig: 'static,
                        Path: Iterator<Item = #library_path::StorePathSegment>,
                        Reader: for<'a> Fn(&'a #orig) -> &'a #struct_name #generics + 'static,
                        Writer: for<'a> Fn(&'a mut #orig) -> &'a mut #struct_name #generics + 'static
                        #predicates
                    }
                })
                .unwrap_or_else(|| quote! { where #orig: 'static })
        };

        let field_names = fields.iter().map(|field| {
            let Field { vis, ident, ty, .. } = &field;

            quote! {
                fn #ident() -> () {}
            }
        });

        // define an extension trait that matches this struct
        let read_trait_fields = fields.iter().map(|field| {
            let Field { vis, ident, ty, .. } = &field;

            quote! {
                fn #ident(self) -> #library_path::StorePathBuilder<
                    #orig,
                    #ty,
                    impl Iterator<Item = #library_path::StorePathSegment>,
                    impl for<'a> Fn(&'a #orig) -> &'a #ty + 'static,
                    impl for<'a> Fn(&'a mut #orig) -> &'a mut #ty + 'static
                >
                where
                    #orig: 'static,
                    Path: Iterator<Item = #library_path::StorePathSegment>,
                    Reader: for<'a> Fn(&'a #orig) -> &'a #struct_name #generics + 'static,
                    Writer: for<'a> Fn(&'a mut #orig) -> &'a mut #struct_name #generics + 'static;
            }
        });

        /* let write_trait_fields = fields.iter().map(|field| {
            let Field { vis, ident, ty, .. } = &field;

            quote! {
                fn #ident(self) -> #library_path::StorePathBuilder<
                    #orig,
                    #ty,
                    impl Iterator<Item = #library_path::StorePathSegment>,
                    impl Fn(&#orig) -> &#ty,
                    impl Fn(&mut #orig) -> &mut #ty
                >;
            }
        }); */

        // implement that trait for ReadStoreField
        let read_fields = fields.iter().map(|field| {
            let Field { vis, ident, ty, .. } = &field;

            quote! {
                #[inline(always)]
                fn #ident(self) -> #library_path::StorePathBuilder<
                    #orig,
                    #ty,
                    impl Iterator<Item = #library_path::StorePathSegment>,
                    impl for<'a> Fn(&'a #orig) -> &'a #ty + 'static,
                    impl for<'a> Fn(&'a mut #orig) -> &'a mut #ty + 'static
                >
                where
                    #orig: 'static,
                    Path: Iterator<Item = #library_path::StorePathSegment>,
                    Reader: for<'a> Fn(&'a #orig) -> &'a #struct_name #generics + 'static,
                    Writer: for<'a> Fn(&'a mut #orig) -> &'a mut #struct_name #generics + 'static,
                {
                    let #library_path::StorePathBuilder {
                        data, path, reader, writer, ty
                    } = self;
                    #library_path::StorePathBuilder {
                        data,
                        path: path.chain(
                            ::std::iter::once(
                                 #library_path::StorePathSegment::from(
                                    #field_names_struct_name #generics :: #ident as usize
                                 )
                            )
                        ),
                        reader: move |orig| &(reader(orig)).#ident,
                        writer: move |orig| {
                            let prev = writer(orig);
                            &mut prev.#ident
                        },
                        ty: ::std::marker::PhantomData
                    }
                }
            }
        });

        // implement that trait for WriteStoreField
        /* let write_fields = fields.iter().map(|field| {
            let Field { vis, ident, ty, .. } = &field;

            quote! {
                fn #ident(self) -> #library_path::WriteStoreField<
                    #orig,
                    #ty,
                    impl Iterator<Item = #library_path::StorePathSegment>,
                    impl Fn(&#orig) -> &#ty,
                    impl Fn(&mut #orig) -> &mut #ty
                > {
                    let #library_path::StorePathBuilder {
                        data: Arc::downgrade(&self.value),
                        path: iter::empty(),
                        reader: |val| val,
                        writer: |val| val,
                    } = self;
                    todo!()
                }
            }
        }); */

        // read access
        tokens.extend(quote! {
            struct #field_names_struct_name #generics {}

            impl #generics #field_names_struct_name #generics {
                #(#field_names)*
            }

            #vis trait #read_trait_name #generics_with_orig
            {
                #(#read_trait_fields)*
            }

            impl #generics_with_orig #read_trait_name #generics_with_orig
            for #library_path::StorePathBuilder<#orig, #struct_name #generics, Path, Reader, Writer>
            #where_with_orig
            {
               #(#read_fields)*
            }

            /* #vis trait #write_trait_name #generics_with_orig {
                #(#write_trait_fields)*
            }

            impl #generics_with_orig #write_trait_name #generics_with_orig
            for #library_path::WriteStoreField<#orig, #struct_name #generics>
            #where_with_orig
            {
                #(#write_fields)*
            } */

            /* impl #generics #library_path::WriteStoreField {
                #(#write_fields)*
            } */
        });
    }
}
