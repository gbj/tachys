mod component;
mod view;
use crate::component::unmodified_fn_name_from_fn_name;
use component::DummyModel;
use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use proc_macro_error::abort;
use quote::{quote, ToTokens};
use syn::{spanned::Spanned, token::Pub, Visibility};

#[proc_macro_error::proc_macro_error]
#[proc_macro]
pub fn view(tokens: TokenStream) -> TokenStream {
    let tokens: proc_macro2::TokenStream = tokens.into();
    let mut tokens = tokens.into_iter();

    let first = tokens.next();
    let second = tokens.next();
    let third = tokens.next();
    let fourth = tokens.next();
    let global_class = match (&first, &second) {
        (Some(TokenTree::Ident(first)), Some(TokenTree::Punct(eq)))
            if *first == "class" && eq.as_char() == '=' =>
        {
            match &fourth {
                Some(TokenTree::Punct(comma)) if comma.as_char() == ',' => {
                    third.clone()
                }
                _ => {
                    abort!(
                        second, "To create a scope class with the view! macro you must put a comma `,` after the value";
                        help = r#"e.g., view!{ class="my-class", <div>...</div>}"#
                    )
                }
            }
        }
        _ => None,
    };
    let tokens = if global_class.is_some() {
        tokens.collect::<proc_macro2::TokenStream>()
    } else {
        [first, second, third, fourth]
            .into_iter()
            .flatten()
            .chain(tokens)
            .collect()
    };
    let config = rstml::ParserConfig::default().recover_block(true);
    let parser = rstml::Parser::new(config);
    let (nodes, errors) = parser.parse_recoverable(tokens).split_vec();
    let errors = errors.into_iter().map(|e| e.emit_as_expr_tokens());
    let nodes_output = view::render_view(&nodes, global_class.as_ref(), None);
    quote! {
        {
            #(#errors;)*
            #nodes_output
        }
    }
    .into()
}

#[proc_macro_error::proc_macro_error]
#[proc_macro_attribute]
pub fn component(
    _args: proc_macro::TokenStream,
    s: TokenStream,
) -> TokenStream {
    let mut dummy = syn::parse::<DummyModel>(s.clone());
    let parse_result = syn::parse::<component::Model>(s);

    if let (Ok(ref mut unexpanded), Ok(model)) = (&mut dummy, parse_result) {
        let expanded = model.into_token_stream();
        if !matches!(unexpanded.vis, Visibility::Public(_)) {
            unexpanded.vis = Visibility::Public(Pub {
                span: unexpanded.vis.span(),
            })
        }
        unexpanded.sig.ident =
            unmodified_fn_name_from_fn_name(&unexpanded.sig.ident);
        quote! {
            #expanded

            #[doc(hidden)]
            #[allow(non_snake_case, dead_code, clippy::too_many_arguments)]
            #unexpanded
        }
    } else if let Ok(mut dummy) = dummy {
        dummy.sig.ident = unmodified_fn_name_from_fn_name(&dummy.sig.ident);
        quote! {
            #[doc(hidden)]
            #[allow(non_snake_case, dead_code, clippy::too_many_arguments)]
            #dummy
        }
    } else {
        quote! {}
    }
    .into()
}
