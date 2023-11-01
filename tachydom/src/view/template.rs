use super::{Mountable, PositionState, Render, RenderHtml, ToTemplate};
use crate::{
    dom::{document, log},
    hydration::Cursor,
    renderer::dom::Dom,
};
use once_cell::unsync::{Lazy, OnceCell};
use rustc_hash::FxHashMap;
use std::{any::TypeId, cell::RefCell};
use wasm_bindgen::JsCast;
use web_sys::HtmlTemplateElement;

thread_local! {
    static TEMPLATE_ELEMENT: Lazy<HtmlTemplateElement> =
        Lazy::new(|| document().create_element("template").unwrap().unchecked_into());
}

pub struct ViewTemplate<V: Render<Dom> + ToTemplate> {
    view: V,
}

thread_local! {
    static TEMPLATES: RefCell<FxHashMap<TypeId, HtmlTemplateElement>> = Default::default();
}

impl<V: Render<Dom> + ToTemplate + 'static> ViewTemplate<V> {
    pub fn new(view: V) -> Self {
        Self { view }
    }

    fn to_template() -> HtmlTemplateElement {
        let tpl = TEMPLATES.with(|t| {
            t.borrow_mut()
                .entry(TypeId::of::<V>())
                .or_insert_with(|| {
                    let tpl = TEMPLATE_ELEMENT.with(|t| {
                        t.clone_node()
                            .unwrap()
                            .unchecked_into::<HtmlTemplateElement>()
                    });
                    /* let mut buf = String::new();
                    let mut class = String::new();
                    let mut style = String::new();
                    V::to_template(
                        &mut buf,
                        &mut class,
                        &mut style,
                        &mut Default::default(),
                    );
                    tpl.set_inner_html(&buf); */
                    //log(&format!("setting template to {:?}", V::TEMPLATE));
                    tpl.set_inner_html(V::TEMPLATE);
                    tpl
                })
                .clone()
        });
        //web_sys::console::log_1(&tpl);
        tpl
    }
}

impl<V> Render<Dom> for ViewTemplate<V>
where
    V: Render<Dom> + RenderHtml<Dom> + ToTemplate + 'static,
    V::State: Mountable<Dom>,
{
    type State = V::State;

    fn build(self) -> Self::State {
        let tpl = Self::to_template();
        let contents = tpl.content().clone_node_with_deep(true).unwrap();
        self.view.hydrate::<false>(
            &Cursor::new(contents.unchecked_into()),
            &Default::default(),
        )
    }

    fn rebuild(self, state: &mut Self::State) {
        self.view.rebuild(state)
    }
}

impl<V> RenderHtml<Dom> for ViewTemplate<V>
where
    V: RenderHtml<Dom> + ToTemplate + 'static,
    V::State: Mountable<Dom>,
{
    fn to_html(self, buf: &mut String, position: &PositionState) {
        self.view.to_html(buf, position)
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor<Dom>,
        position: &PositionState,
    ) -> Self::State {
        self.view.hydrate::<FROM_SERVER>(cursor, position)
    }
}

impl<V> ToTemplate for ViewTemplate<V>
where
    V: RenderHtml<Dom> + ToTemplate + 'static,
    V::State: Mountable<Dom>,
{
    const TEMPLATE: &'static str = V::TEMPLATE;

    fn to_template(
        buf: &mut String,
        class: &mut String,
        style: &mut String,
        position: &mut super::Position,
    ) {
        V::to_template(buf, class, style, position);
    }
}

pub(crate) const MAX_TEMPLATE_SIZE: usize = 4096;

pub(crate) const fn str_from_buffer(buf: &[u8; MAX_TEMPLATE_SIZE]) -> &str {
    match std::ffi::CStr::from_bytes_until_nul(buf) {
        Ok(cstr) => match cstr.to_str() {
            Ok(str) => str,
            Err(_) => panic!("TEMPLATE FAILURE"),
        },
        Err(_) => panic!("TEMPLATE FAILURE"),
    }
}

// credit to Rainer Stropek, "Constant fun," Rust Linz, June 2022
pub(crate) const fn const_concat(
    strs: &'static [&'static str],
) -> [u8; MAX_TEMPLATE_SIZE] {
    let mut buffer = [0; MAX_TEMPLATE_SIZE];
    let mut position = 0;
    let mut remaining = strs;

    while let [current, tail @ ..] = remaining {
        let x = current.as_bytes();
        let mut i = 0;

        // have it iterate over bytes manually, because, again,
        // no mutable refernces in const fns
        while i < x.len() {
            buffer[position] = x[i];
            position += 1;
            i += 1;
        }

        remaining = tail;
    }

    buffer
}

pub(crate) const fn const_concat_with_prefix(
    strs: &'static [&'static str],
    prefix: &'static str,
    suffix: &'static str,
) -> [u8; MAX_TEMPLATE_SIZE] {
    let mut buffer = [0; MAX_TEMPLATE_SIZE];
    let mut position = 0;
    let mut remaining = strs;

    while let [current, tail @ ..] = remaining {
        let x = current.as_bytes();
        let mut i = 0;

        // have it iterate over bytes manually, because, again,
        // no mutable refernces in const fns
        while i < x.len() {
            buffer[position] = x[i];
            position += 1;
            i += 1;
        }

        remaining = tail;
    }

    if buffer[0] == 0 {
        buffer
    } else {
        let mut new_buf = [0; MAX_TEMPLATE_SIZE];
        let prefix = prefix.as_bytes();
        let suffix = suffix.as_bytes();
        let mut position = 0;
        let mut i = 0;
        while i < prefix.len() {
            new_buf[position] = prefix[i];
            position += 1;
            i += 1;
        }
        i = 0;
        while i < buffer.len() {
            if buffer[i] == 0 {
                break;
            }
            new_buf[position] = buffer[i];
            position += 1;
            i += 1;
        }
        i = 0;
        while i < suffix.len() {
            new_buf[position] = suffix[i];
            position += 1;
            i += 1;
        }

        new_buf
    }
}

pub(crate) const fn const_concat_with_separator(
    strs: &[&str],
    separator: &'static str,
) -> [u8; MAX_TEMPLATE_SIZE] {
    let mut buffer = [0; MAX_TEMPLATE_SIZE];
    let mut position = 0;
    let mut remaining = strs;

    while let [current, tail @ ..] = remaining {
        let x = current.as_bytes();
        let mut i = 0;

        // have it iterate over bytes manually, because, again,
        // no mutable refernces in const fns
        while i < x.len() {
            buffer[position] = x[i];
            position += 1;
            i += 1;
        }
        if !x.is_empty() {
            let mut position = 0;
            let separator = separator.as_bytes();
            while i < separator.len() {
                buffer[position] = separator[i];
                position += 1;
                i += 1;
            }
        }

        remaining = tail;
    }

    buffer
}
