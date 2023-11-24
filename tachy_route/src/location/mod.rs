use core::fmt::Debug;
use tachydom::dom::window;
use wasm_bindgen::JsValue;

pub mod state;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Url {
    pub origin: String,
    pub pathname: String,
    pub search: String,
    pub hash: String,
}

pub trait Location {
    type Error: Debug;

    fn try_into_url(self) -> Result<Url, Self::Error>;
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

    fn try_into_url(self) -> Result<Url, Self::Error> {
        let url = url::Url::parse(&self.0)?;
        Ok(Url {
            origin: url.origin().unicode_serialization(),
            pathname: url.path().to_string(),
            search: url.query().unwrap_or_default().to_string(),
            hash: Default::default(),
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct BrowserUrl;

impl Location for BrowserUrl {
    type Error = JsValue;

    fn try_into_url(self) -> Result<Url, Self::Error> {
        let location = window().location();
        Ok(Url {
            origin: location.origin()?,
            pathname: location.pathname()?,
            search: location
                .search()?
                .strip_prefix('?')
                .map(String::from)
                .unwrap_or_default(),
            hash: location.hash()?,
        })
    }
}
