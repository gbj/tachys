use self::state::State;
use crate::matching::Params;
use core::fmt::Debug;
use js_sys::{try_iter, Array, JsString, Reflect};
use std::{cell::RefCell, rc::Rc};
use tachydom::{
    dom::{document, window},
    log,
};
use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use web_sys::{Event, HtmlAnchorElement, MouseEvent, UrlSearchParams};

pub mod state;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Url {
    pub origin: String,
    pub pathname: String,
    pub search: String,
    pub search_params: Params<String>,
    pub hash: String,
}

/// A description of a navigation.
#[derive(Debug, Clone, PartialEq)]
pub struct LocationChange {
    /// The new URL.
    pub value: String,
    /// If true, the new location will replace the current one in the history stack, i.e.,
    /// clicking the "back" button will not return to the current location.
    pub replace: bool,
    /// If true, the router will scroll to the top of the page at the end of the navigation.
    pub scroll: bool,
    /// The [`state`](https://developer.mozilla.org/en-US/docs/Web/API/History/state) that will be added during navigation.
    pub state: State,
}

impl Default for LocationChange {
    fn default() -> Self {
        Self {
            value: Default::default(),
            replace: true,
            scroll: true,
            state: Default::default(),
        }
    }
}

pub trait Location {
    type Error: Debug;

    /// Sets up any global event listeners or other initialization needed.
    fn init(&self);

    /// Returns the current URL.
    fn try_to_url(&self) -> Result<Url, Self::Error>;

    fn set_navigation_hook(&mut self, cb: impl Fn(Url) + 'static);

    /// Navigate to a new location.
    fn navigate(&self, loc: &LocationChange);
}

const BASE: &str = "http://leptos.dev/";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RequestUrl(String);

impl RequestUrl {
    pub fn from_path(path: impl AsRef<str>) -> Self {
        let path = path.as_ref();
        let mut string = String::with_capacity(BASE.len() + path.len());
        string.push_str(BASE);
        string.push_str(path);
        Self(string)
    }
}

impl Default for RequestUrl {
    fn default() -> Self {
        Self(String::from(BASE))
    }
}

impl Location for RequestUrl {
    type Error = url::ParseError;

    fn init(&self) {}

    fn try_to_url(&self) -> Result<Url, Self::Error> {
        let url = url::Url::parse(&self.0)?;
        Ok(Url {
            origin: url.origin().unicode_serialization(),
            pathname: url.path().to_string(),
            search: url.query().unwrap_or_default().to_string(),
            search_params: Default::default(), // TODO
            hash: Default::default(),
        })
    }

    fn set_navigation_hook(&mut self, _cb: impl FnMut(Url) + 'static) {}

    fn navigate(&self, _loc: &LocationChange) {}
}

#[derive(Clone, Default)]
pub struct BrowserUrl {
    navigation_hook: Option<Rc<dyn Fn(Url)>>,
}

impl Debug for BrowserUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BrowserUrl").finish_non_exhaustive()
    }
}

impl BrowserUrl {
    pub fn new() -> Self {
        Self::default()
    }

    fn try_current() -> Result<Url, JsValue> {
        let location = window().location();
        Ok(Url {
            origin: location.origin()?,
            pathname: location.pathname()?,
            search: location
                .search()?
                .strip_prefix('?')
                .map(String::from)
                .unwrap_or_default(),
            search_params: search_params_from_web_url(
                &UrlSearchParams::new_with_str(&location.search()?)?,
            )?,
            hash: location.hash()?,
        })
    }

    fn try_url_from(href: &str) -> Result<Url, JsValue> {
        let location = web_sys::Url::new_with_base(
            &if href.starts_with("//") {
                let mut origin = window().location().origin()?;
                origin.push_str(href);
                origin
            } else {
                href.to_string()
            },
            "http://leptos",
        )?;
        Ok(Url {
            origin: location.origin(),
            pathname: location.pathname(),
            search: location
                .search()
                .strip_prefix('?')
                .map(String::from)
                .unwrap_or_default(),
            search_params: search_params_from_web_url(
                &location.search_params(),
            )?,
            hash: location.hash(),
        })
    }

    fn unescape(s: &str) -> String {
        js_sys::decode_uri(s).unwrap().into()
    }

    fn scroll_to_el(loc_scroll: bool) {
        if let Ok(hash) = window().location().hash() {
            if !hash.is_empty() {
                let hash = js_sys::decode_uri(&hash[1..])
                    .ok()
                    .and_then(|decoded| decoded.as_string())
                    .unwrap_or(hash);
                let el = document().get_element_by_id(&hash);
                if let Some(el) = el {
                    el.scroll_into_view();
                    return;
                }
            }
        }

        // scroll to top
        if loc_scroll {
            window().scroll_to_with_x_and_y(0.0, 0.0);
        }
    }
}

fn search_params_from_web_url(
    params: &web_sys::UrlSearchParams,
) -> Result<Params<String>, JsValue> {
    let mut search_params = Params::new();
    for pair in try_iter(params)?.into_iter().flatten() {
        let row = pair?.unchecked_into::<Array>();
        search_params.push((
            row.get(0).unchecked_into::<JsString>().into(),
            row.get(1).unchecked_into::<JsString>().into(),
        ));
    }
    /* let search_params = try_iter(&location.search_params())?
    .into_iter()
    .flatten()
    .flat_map(|value| {
        Ok((
            value?.dyn_into::<JsString>()?.into(),
            value?.dyn_into::<JsString>()?.into(),
        ))
    })
    .collect::<Params>()?; */
    Ok(search_params)
}

impl Location for BrowserUrl {
    type Error = JsValue;

    fn init(&self) {
        let this = self.clone();
        let handle_anchor_click = move |ev: Event| {
            let ev = ev.unchecked_into::<MouseEvent>();
            if ev.default_prevented()
                || ev.button() != 0
                || ev.meta_key()
                || ev.alt_key()
                || ev.ctrl_key()
                || ev.shift_key()
            {
                return;
            }

            let composed_path = ev.composed_path();
            let mut a: Option<HtmlAnchorElement> = None;
            for i in 0..composed_path.length() {
                if let Ok(el) =
                    composed_path.get(i).dyn_into::<HtmlAnchorElement>()
                {
                    a = Some(el);
                }
            }
            if let Some(a) = a {
                let href = a.href();
                let target = a.target();

                // let browser handle this event if link has target,
                // or if it doesn't have href or state
                // TODO "state" is set as a prop, not an attribute
                if !target.is_empty()
                    || (href.is_empty() && !a.has_attribute("state"))
                {
                    return;
                }

                let rel = a.get_attribute("rel").unwrap_or_default();
                let mut rel = rel.split([' ', '\t']);

                // let browser handle event if it has rel=external or download
                if a.has_attribute("download") || rel.any(|p| p == "external") {
                    return;
                }

                let url = Self::try_url_from(href.as_str()).unwrap();
                let path_name = Self::unescape(&url.pathname);

                // let browser handle this event if it leaves our domain
                // or our base path
                if url.origin
                    != window().location().origin().unwrap_or_default()
                // TODO base path for router
                /* || (true // TODO base_path //!self.base_path.is_empty()
                && !path_name.is_empty()
                && !path_name
                    .to_lowercase()
                    .starts_with(&self.base_path.to_lowercase())) */
                {
                    return;
                }

                let to = path_name
                    + if url.search.is_empty() { "" } else { "?" }
                    + &Self::unescape(&url.search)
                    + &Self::unescape(&url.hash);
                let state = Reflect::get(&a, &JsValue::from_str("state"))
                    .ok()
                    .and_then(|value| {
                        if value == JsValue::UNDEFINED {
                            None
                        } else {
                            Some(value)
                        }
                    });

                ev.prevent_default();

                let replace = Reflect::get(&a, &JsValue::from_str("replace"))
                    .ok()
                    .and_then(|value| value.as_bool())
                    .unwrap_or(false);

                let change = LocationChange {
                    value: to,
                    replace,
                    scroll: true,
                    state: State(state),
                };

                // run any router-specific hook
                if let Some(navigate_hook) = &this.navigation_hook {
                    navigate_hook(url);
                }

                // complete navigation
                this.navigate(&change);
            }
        };

        let closure = Closure::wrap(
            Box::new(handle_anchor_click) as Box<dyn FnMut(Event)>
        )
        .into_js_value();
        window()
            .add_event_listener_with_callback(
                "click",
                closure.as_ref().unchecked_ref(),
            )
            .expect(
                "couldn't add `click` listener to `window` to handle `<a>` \
                 clicks",
            );

        // handle popstate event (forward/back navigation)
        if let Some(navigation_hook) = self.navigation_hook.clone() {
            let cb = move || match Self::try_current() {
                Ok(url) => navigation_hook(url),
                Err(e) => {
                    #[cfg(debug_assertions)]
                    web_sys::console::error_1(&e);
                    _ = e;
                }
            };
            let closure =
                Closure::wrap(Box::new(cb) as Box<dyn Fn()>).into_js_value();
            window()
                .add_event_listener_with_callback(
                    "popstate",
                    closure.as_ref().unchecked_ref(),
                )
                .expect("couldn't add `popstate` listener to `window`");
        }
    }

    fn try_to_url(&self) -> Result<Url, Self::Error> {
        Self::try_current()
    }

    fn set_navigation_hook(&mut self, cb: impl Fn(Url) + 'static) {
        self.navigation_hook = Some(Rc::new(cb));
    }

    fn navigate(&self, loc: &LocationChange) {
        let history = window().history().unwrap();

        if loc.replace {
            history
                .replace_state_with_url(
                    &loc.state.to_js_value(),
                    "",
                    Some(&loc.value),
                )
                .unwrap();
        } else {
            // push the "forward direction" marker
            let state = &loc.state.to_js_value();
            history
                .push_state_with_url(state, "", Some(&loc.value))
                .unwrap();
        }
        // scroll to el
        Self::scroll_to_el(loc.scroll);
    }
}
