#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use tachy_route::{
    location::{BrowserUrl, RequestUrl},
    matching::{ParamSegment, StaticSegment},
    reactive::{reactive_route, ReactiveRouter},
    route::{PossibleRoutes, RouteDefinition},
    router::FallbackOrView,
};
use tachys::{island, prelude::*, tachydom::dom::body};
mod api {
    use serde::{de::DeserializeOwned, Deserialize, Serialize};
    pub fn story(path: &str) -> String {
        {
            let res = ::alloc::fmt::format(
                format_args!("https://node-hnapi.herokuapp.com/{0}", path),
            );
            res
        }
    }
    pub fn user(path: &str) -> String {
        {
            let res = ::alloc::fmt::format(
                format_args!("https://hacker-news.firebaseio.com/v0/user/{0}.json", path),
            );
            res
        }
    }
    #[cfg(not(feature = "ssr"))]
    pub async fn fetch_api<T>(path: &str) -> Option<T>
    where
        T: DeserializeOwned + Default,
    {
        let abort_controller = web_sys::AbortController::new().ok();
        let abort_signal = abort_controller.as_ref().map(|a| a.signal());
        gloo_net::http::Request::get(path)
            .abort_signal(abort_signal.as_ref())
            .send()
            .await
            .map_err(|e| {
                let lvl = ::log::Level::Error;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api::log(
                        format_args!("{0}", e),
                        lvl,
                        &("hackernews::api", "hackernews::api", "src/api.rs"),
                        31u32,
                        ::log::__private_api::Option::None,
                    );
                }
            })
            .ok()?
            .json()
            .await
            .ok()
    }
    pub struct Story {
        pub id: usize,
        pub title: String,
        pub points: Option<i32>,
        pub user: Option<String>,
        pub time: usize,
        pub time_ago: String,
        #[serde(alias = "type")]
        pub story_type: String,
        pub url: String,
        #[serde(default)]
        pub domain: String,
        #[serde(default)]
        pub comments: Option<Vec<Comment>>,
        pub comments_count: Option<usize>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Story {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "id",
                "title",
                "points",
                "user",
                "time",
                "time_ago",
                "story_type",
                "url",
                "domain",
                "comments",
                "comments_count",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.id,
                &self.title,
                &self.points,
                &self.user,
                &self.time,
                &self.time_ago,
                &self.story_type,
                &self.url,
                &self.domain,
                &self.comments,
                &&self.comments_count,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(f, "Story", names, values)
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Story {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __field8,
                    __field9,
                    __field10,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            5u64 => _serde::__private::Ok(__Field::__field5),
                            6u64 => _serde::__private::Ok(__Field::__field6),
                            7u64 => _serde::__private::Ok(__Field::__field7),
                            8u64 => _serde::__private::Ok(__Field::__field8),
                            9u64 => _serde::__private::Ok(__Field::__field9),
                            10u64 => _serde::__private::Ok(__Field::__field10),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "id" => _serde::__private::Ok(__Field::__field0),
                            "title" => _serde::__private::Ok(__Field::__field1),
                            "points" => _serde::__private::Ok(__Field::__field2),
                            "user" => _serde::__private::Ok(__Field::__field3),
                            "time" => _serde::__private::Ok(__Field::__field4),
                            "time_ago" => _serde::__private::Ok(__Field::__field5),
                            "story_type" | "type" => {
                                _serde::__private::Ok(__Field::__field6)
                            }
                            "url" => _serde::__private::Ok(__Field::__field7),
                            "domain" => _serde::__private::Ok(__Field::__field8),
                            "comments" => _serde::__private::Ok(__Field::__field9),
                            "comments_count" => _serde::__private::Ok(__Field::__field10),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"id" => _serde::__private::Ok(__Field::__field0),
                            b"title" => _serde::__private::Ok(__Field::__field1),
                            b"points" => _serde::__private::Ok(__Field::__field2),
                            b"user" => _serde::__private::Ok(__Field::__field3),
                            b"time" => _serde::__private::Ok(__Field::__field4),
                            b"time_ago" => _serde::__private::Ok(__Field::__field5),
                            b"story_type" | b"type" => {
                                _serde::__private::Ok(__Field::__field6)
                            }
                            b"url" => _serde::__private::Ok(__Field::__field7),
                            b"domain" => _serde::__private::Ok(__Field::__field8),
                            b"comments" => _serde::__private::Ok(__Field::__field9),
                            b"comments_count" => {
                                _serde::__private::Ok(__Field::__field10)
                            }
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Story>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Story;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Story",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            usize,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Story with 11 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Story with 11 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            Option<i32>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct Story with 11 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct Story with 11 elements",
                                    ),
                                );
                            }
                        };
                        let __field4 = match _serde::de::SeqAccess::next_element::<
                            usize,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct Story with 11 elements",
                                    ),
                                );
                            }
                        };
                        let __field5 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        5usize,
                                        &"struct Story with 11 elements",
                                    ),
                                );
                            }
                        };
                        let __field6 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        6usize,
                                        &"struct Story with 11 elements",
                                    ),
                                );
                            }
                        };
                        let __field7 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        7usize,
                                        &"struct Story with 11 elements",
                                    ),
                                );
                            }
                        };
                        let __field8 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                _serde::__private::Default::default()
                            }
                        };
                        let __field9 = match _serde::de::SeqAccess::next_element::<
                            Option<Vec<Comment>>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                _serde::__private::Default::default()
                            }
                        };
                        let __field10 = match _serde::de::SeqAccess::next_element::<
                            Option<usize>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        10usize,
                                        &"struct Story with 11 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Story {
                            id: __field0,
                            title: __field1,
                            points: __field2,
                            user: __field3,
                            time: __field4,
                            time_ago: __field5,
                            story_type: __field6,
                            url: __field7,
                            domain: __field8,
                            comments: __field9,
                            comments_count: __field10,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<usize> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Option<i32>> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<Option<String>> = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<usize> = _serde::__private::None;
                        let mut __field5: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field6: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field7: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field8: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field9: _serde::__private::Option<
                            Option<Vec<Comment>>,
                        > = _serde::__private::None;
                        let mut __field10: _serde::__private::Option<Option<usize>> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<usize>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("title"),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("points"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<i32>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("user"),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<String>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("time"),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<usize>(&mut __map)?,
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::__private::Option::is_some(&__field5) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "time_ago",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field6 => {
                                    if _serde::__private::Option::is_some(&__field6) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "story_type",
                                            ),
                                        );
                                    }
                                    __field6 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field7 => {
                                    if _serde::__private::Option::is_some(&__field7) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("url"),
                                        );
                                    }
                                    __field7 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field8 => {
                                    if _serde::__private::Option::is_some(&__field8) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("domain"),
                                        );
                                    }
                                    __field8 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field9 => {
                                    if _serde::__private::Option::is_some(&__field9) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "comments",
                                            ),
                                        );
                                    }
                                    __field9 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<Vec<Comment>>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field10 => {
                                    if _serde::__private::Option::is_some(&__field10) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "comments_count",
                                            ),
                                        );
                                    }
                                    __field10 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<usize>,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("title")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("points")?
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("user")?
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("time")?
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::__private::Some(__field5) => __field5,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("time_ago")?
                            }
                        };
                        let __field6 = match __field6 {
                            _serde::__private::Some(__field6) => __field6,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("story_type")?
                            }
                        };
                        let __field7 = match __field7 {
                            _serde::__private::Some(__field7) => __field7,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("url")?
                            }
                        };
                        let __field8 = match __field8 {
                            _serde::__private::Some(__field8) => __field8,
                            _serde::__private::None => {
                                _serde::__private::Default::default()
                            }
                        };
                        let __field9 = match __field9 {
                            _serde::__private::Some(__field9) => __field9,
                            _serde::__private::None => {
                                _serde::__private::Default::default()
                            }
                        };
                        let __field10 = match __field10 {
                            _serde::__private::Some(__field10) => __field10,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("comments_count")?
                            }
                        };
                        _serde::__private::Ok(Story {
                            id: __field0,
                            title: __field1,
                            points: __field2,
                            user: __field3,
                            time: __field4,
                            time_ago: __field5,
                            story_type: __field6,
                            url: __field7,
                            domain: __field8,
                            comments: __field9,
                            comments_count: __field10,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "id",
                    "title",
                    "points",
                    "user",
                    "time",
                    "time_ago",
                    "story_type",
                    "type",
                    "url",
                    "domain",
                    "comments",
                    "comments_count",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Story",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Story>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Story {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Story",
                    false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "title",
                    &self.title,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "points",
                    &self.points,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "user",
                    &self.user,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "time",
                    &self.time,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "time_ago",
                    &self.time_ago,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "story_type",
                    &self.story_type,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "url",
                    &self.url,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "domain",
                    &self.domain,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "comments",
                    &self.comments,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "comments_count",
                    &self.comments_count,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Story {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Story {
        #[inline]
        fn eq(&self, other: &Story) -> bool {
            self.id == other.id && self.title == other.title
                && self.points == other.points && self.user == other.user
                && self.time == other.time && self.time_ago == other.time_ago
                && self.story_type == other.story_type && self.url == other.url
                && self.domain == other.domain && self.comments == other.comments
                && self.comments_count == other.comments_count
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Story {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Story {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<usize>;
            let _: ::core::cmp::AssertParamIsEq<String>;
            let _: ::core::cmp::AssertParamIsEq<Option<i32>>;
            let _: ::core::cmp::AssertParamIsEq<Option<String>>;
            let _: ::core::cmp::AssertParamIsEq<Option<Vec<Comment>>>;
            let _: ::core::cmp::AssertParamIsEq<Option<usize>>;
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Story {
        #[inline]
        fn clone(&self) -> Story {
            Story {
                id: ::core::clone::Clone::clone(&self.id),
                title: ::core::clone::Clone::clone(&self.title),
                points: ::core::clone::Clone::clone(&self.points),
                user: ::core::clone::Clone::clone(&self.user),
                time: ::core::clone::Clone::clone(&self.time),
                time_ago: ::core::clone::Clone::clone(&self.time_ago),
                story_type: ::core::clone::Clone::clone(&self.story_type),
                url: ::core::clone::Clone::clone(&self.url),
                domain: ::core::clone::Clone::clone(&self.domain),
                comments: ::core::clone::Clone::clone(&self.comments),
                comments_count: ::core::clone::Clone::clone(&self.comments_count),
            }
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for Story {
        #[inline]
        fn default() -> Story {
            Story {
                id: ::core::default::Default::default(),
                title: ::core::default::Default::default(),
                points: ::core::default::Default::default(),
                user: ::core::default::Default::default(),
                time: ::core::default::Default::default(),
                time_ago: ::core::default::Default::default(),
                story_type: ::core::default::Default::default(),
                url: ::core::default::Default::default(),
                domain: ::core::default::Default::default(),
                comments: ::core::default::Default::default(),
                comments_count: ::core::default::Default::default(),
            }
        }
    }
    pub struct Comment {
        pub id: usize,
        pub level: usize,
        pub user: Option<String>,
        pub time: usize,
        pub time_ago: String,
        pub content: Option<String>,
        pub comments: Vec<Comment>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Comment {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "id",
                "level",
                "user",
                "time",
                "time_ago",
                "content",
                "comments",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.id,
                &self.level,
                &self.user,
                &self.time,
                &self.time_ago,
                &self.content,
                &&self.comments,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "Comment",
                names,
                values,
            )
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for Comment {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            5u64 => _serde::__private::Ok(__Field::__field5),
                            6u64 => _serde::__private::Ok(__Field::__field6),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "id" => _serde::__private::Ok(__Field::__field0),
                            "level" => _serde::__private::Ok(__Field::__field1),
                            "user" => _serde::__private::Ok(__Field::__field2),
                            "time" => _serde::__private::Ok(__Field::__field3),
                            "time_ago" => _serde::__private::Ok(__Field::__field4),
                            "content" => _serde::__private::Ok(__Field::__field5),
                            "comments" => _serde::__private::Ok(__Field::__field6),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"id" => _serde::__private::Ok(__Field::__field0),
                            b"level" => _serde::__private::Ok(__Field::__field1),
                            b"user" => _serde::__private::Ok(__Field::__field2),
                            b"time" => _serde::__private::Ok(__Field::__field3),
                            b"time_ago" => _serde::__private::Ok(__Field::__field4),
                            b"content" => _serde::__private::Ok(__Field::__field5),
                            b"comments" => _serde::__private::Ok(__Field::__field6),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<Comment>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = Comment;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct Comment",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            usize,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct Comment with 7 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            usize,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct Comment with 7 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct Comment with 7 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            usize,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct Comment with 7 elements",
                                    ),
                                );
                            }
                        };
                        let __field4 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct Comment with 7 elements",
                                    ),
                                );
                            }
                        };
                        let __field5 = match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        5usize,
                                        &"struct Comment with 7 elements",
                                    ),
                                );
                            }
                        };
                        let __field6 = match _serde::de::SeqAccess::next_element::<
                            Vec<Comment>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        6usize,
                                        &"struct Comment with 7 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(Comment {
                            id: __field0,
                            level: __field1,
                            user: __field2,
                            time: __field3,
                            time_ago: __field4,
                            content: __field5,
                            comments: __field6,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<usize> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<usize> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Option<String>> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<usize> = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field5: _serde::__private::Option<Option<String>> = _serde::__private::None;
                        let mut __field6: _serde::__private::Option<Vec<Comment>> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<usize>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("level"),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<usize>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("user"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<String>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("time"),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<usize>(&mut __map)?,
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "time_ago",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::__private::Option::is_some(&__field5) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "content",
                                            ),
                                        );
                                    }
                                    __field5 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<String>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field6 => {
                                    if _serde::__private::Option::is_some(&__field6) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "comments",
                                            ),
                                        );
                                    }
                                    __field6 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Vec<Comment>,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("level")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("user")?
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("time")?
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("time_ago")?
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::__private::Some(__field5) => __field5,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("content")?
                            }
                        };
                        let __field6 = match __field6 {
                            _serde::__private::Some(__field6) => __field6,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("comments")?
                            }
                        };
                        _serde::__private::Ok(Comment {
                            id: __field0,
                            level: __field1,
                            user: __field2,
                            time: __field3,
                            time_ago: __field4,
                            content: __field5,
                            comments: __field6,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "id",
                    "level",
                    "user",
                    "time",
                    "time_ago",
                    "content",
                    "comments",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "Comment",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<Comment>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for Comment {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "Comment",
                    false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "level",
                    &self.level,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "user",
                    &self.user,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "time",
                    &self.time,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "time_ago",
                    &self.time_ago,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "content",
                    &self.content,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "comments",
                    &self.comments,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Comment {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Comment {
        #[inline]
        fn eq(&self, other: &Comment) -> bool {
            self.id == other.id && self.level == other.level && self.user == other.user
                && self.time == other.time && self.time_ago == other.time_ago
                && self.content == other.content && self.comments == other.comments
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for Comment {}
    #[automatically_derived]
    impl ::core::cmp::Eq for Comment {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<usize>;
            let _: ::core::cmp::AssertParamIsEq<Option<String>>;
            let _: ::core::cmp::AssertParamIsEq<String>;
            let _: ::core::cmp::AssertParamIsEq<Option<String>>;
            let _: ::core::cmp::AssertParamIsEq<Vec<Comment>>;
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Comment {
        #[inline]
        fn clone(&self) -> Comment {
            Comment {
                id: ::core::clone::Clone::clone(&self.id),
                level: ::core::clone::Clone::clone(&self.level),
                user: ::core::clone::Clone::clone(&self.user),
                time: ::core::clone::Clone::clone(&self.time),
                time_ago: ::core::clone::Clone::clone(&self.time_ago),
                content: ::core::clone::Clone::clone(&self.content),
                comments: ::core::clone::Clone::clone(&self.comments),
            }
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for Comment {
        #[inline]
        fn default() -> Comment {
            Comment {
                id: ::core::default::Default::default(),
                level: ::core::default::Default::default(),
                user: ::core::default::Default::default(),
                time: ::core::default::Default::default(),
                time_ago: ::core::default::Default::default(),
                content: ::core::default::Default::default(),
                comments: ::core::default::Default::default(),
            }
        }
    }
    pub struct User {
        pub created: usize,
        pub id: String,
        pub karma: i32,
        pub about: Option<String>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for User {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "User",
                "created",
                &self.created,
                "id",
                &self.id,
                "karma",
                &self.karma,
                "about",
                &&self.about,
            )
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for User {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "created" => _serde::__private::Ok(__Field::__field0),
                            "id" => _serde::__private::Ok(__Field::__field1),
                            "karma" => _serde::__private::Ok(__Field::__field2),
                            "about" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"created" => _serde::__private::Ok(__Field::__field0),
                            b"id" => _serde::__private::Ok(__Field::__field1),
                            b"karma" => _serde::__private::Ok(__Field::__field2),
                            b"about" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<User>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = User;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct User",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            usize,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct User with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct User with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            i32,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct User with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct User with 4 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(User {
                            created: __field0,
                            id: __field1,
                            karma: __field2,
                            about: __field3,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<usize> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<i32> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<Option<String>> = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "created",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<usize>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("karma"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("about"),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<String>,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("created")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("karma")?
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("about")?
                            }
                        };
                        _serde::__private::Ok(User {
                            created: __field0,
                            id: __field1,
                            karma: __field2,
                            about: __field3,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "created",
                    "id",
                    "karma",
                    "about",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "User",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<User>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for User {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "User",
                    false as usize + 1 + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "created",
                    &self.created,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "karma",
                    &self.karma,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "about",
                    &self.about,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for User {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for User {
        #[inline]
        fn eq(&self, other: &User) -> bool {
            self.created == other.created && self.id == other.id
                && self.karma == other.karma && self.about == other.about
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for User {}
    #[automatically_derived]
    impl ::core::cmp::Eq for User {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<usize>;
            let _: ::core::cmp::AssertParamIsEq<String>;
            let _: ::core::cmp::AssertParamIsEq<i32>;
            let _: ::core::cmp::AssertParamIsEq<Option<String>>;
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for User {
        #[inline]
        fn clone(&self) -> User {
            User {
                created: ::core::clone::Clone::clone(&self.created),
                id: ::core::clone::Clone::clone(&self.id),
                karma: ::core::clone::Clone::clone(&self.karma),
                about: ::core::clone::Clone::clone(&self.about),
            }
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for User {
        #[inline]
        fn default() -> User {
            User {
                created: ::core::default::Default::default(),
                id: ::core::default::Default::default(),
                karma: ::core::default::Default::default(),
                about: ::core::default::Default::default(),
            }
        }
    }
}
mod routes {
    pub mod nav {
        use tachys::prelude::*;
        /// Props for the [`Nav`] component.
        ///
        ///
        #[builder(crate_module_path = ::tachys::typed_builder)]
        #[allow(non_snake_case)]
        pub struct NavProps {}
        #[automatically_derived]
        impl NavProps {
            /**
                Create a builder for building `NavProps`.
                On the builder, call  to set the values of the fields.
                Finally, call `.build()` to create the instance of `NavProps`.
                */
            #[allow(dead_code, clippy::default_trait_access)]
            pub fn builder() -> NavPropsBuilder<()> {
                NavPropsBuilder {
                    fields: (),
                    phantom: ::core::default::Default::default(),
                }
            }
        }
        #[must_use]
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        pub struct NavPropsBuilder<TypedBuilderFields = ()> {
            fields: TypedBuilderFields,
            phantom: ::core::marker::PhantomData<()>,
        }
        #[automatically_derived]
        impl<TypedBuilderFields> Clone for NavPropsBuilder<TypedBuilderFields>
        where
            TypedBuilderFields: Clone,
        {
            #[allow(clippy::default_trait_access)]
            fn clone(&self) -> Self {
                Self {
                    fields: self.fields.clone(),
                    phantom: ::core::default::Default::default(),
                }
            }
        }
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        #[automatically_derived]
        impl NavPropsBuilder<()> {
            #[allow(clippy::default_trait_access, clippy::used_underscore_binding)]
            pub fn build(self) -> NavProps {
                let () = self.fields;
                #[allow(deprecated)] NavProps {}.into()
            }
        }
        #[allow(missing_docs)]
        impl ::tachys::component::Props for NavProps {
            type Builder = NavPropsBuilder;
            fn builder() -> Self::Builder {
                NavProps::builder()
            }
        }
        ///
        #[allow(non_snake_case, clippy::too_many_arguments)]
        #[allow(clippy::needless_lifetimes)]
        pub fn Nav() -> impl RenderHtml<Dom> {
            ::tachys::tachy_reaccy::untrack(move || { __Nav() })
        }
        #[doc(hidden)]
        #[allow(non_snake_case, dead_code, clippy::too_many_arguments)]
        pub fn __Nav() -> impl RenderHtml<Dom> {
            {
                ::tachys::tachydom::html::element::header()
                    .class(::tachys::tachydom::view::static_types::Static::<"header">)
            }
        }
    }
    pub mod stories {
        use crate::api;
        use tachy_reaccy::async_signal::AsyncState;
        use tachy_route::reactive::ReactiveMatchedRoute;
        use tachys::{prelude::*, tachydom::view::either::Either, Show};
        fn category(from: &str) -> &'static str {
            match from {
                "new" => "newest",
                "show" => "show",
                "ask" => "ask",
                "job" => "jobs",
                _ => "news",
            }
        }
        pub fn Stories(matched: &ReactiveMatchedRoute) -> impl RenderHtml<Dom> {
            {
                ::std::io::_print(format_args!("Stories\n"));
            };
            let page = matched.search("page");
            let story_type = matched.param("stories");
            let page = move || {
                page.get().and_then(|page| page.parse::<usize>().ok()).unwrap_or(1)
            };
            let story_type = move || {
                story_type.get().unwrap_or_else(|| "top".to_string())
            };
            let stories = AsyncDerived::new_unsync(move || {
                let page = page();
                let story_type = story_type();
                {
                    ::std::io::_print(format_args!("starting to load stories\n"));
                };
                async move {
                    {
                        ::std::io::_print(
                            format_args!("inside async to load stories\n"),
                        );
                    };
                    let path = {
                        let res = ::alloc::fmt::format(
                            format_args!("{0}?page={1}", category(& story_type), page),
                        );
                        res
                    };
                    api::fetch_api::<Vec<api::Story>>(&api::story(&path)).await
                }
            });
            let pending = move || stories.with(AsyncState::loading);
            let hide_more_link = move || {
                stories
                    .get()
                    .current_value()
                    .and_then(|value| value.as_ref().map(|value| value.len()))
                    .unwrap_or_default() < 28 || pending()
            };
            let stories = move || {
                async move {
                    {
                        ::std::io::_print(format_args!("Loading stories here\n"));
                    };
                    match stories.await {
                        None => {
                            Either::Left({
                                ::tachys::tachydom::html::element::p()
                                    .child(
                                        #[allow(unused_braces)]
                                        {
                                            ::tachys::tachydom::view::static_types::Static::<
                                                "Error loading stories.",
                                            >
                                        },
                                    )
                            })
                        }
                        Some(stories) => {
                            Either::Right({
                                ::tachys::tachydom::html::element::ul()
                                    .child(
                                        #[allow(unused_braces)]
                                        {
                                            {
                                                stories
                                                    .into_iter()
                                                    .map(|story| {
                                                        {
                                                            ::tachys::component::component_view(
                                                                &Story,
                                                                ::tachys::component::component_props_builder(&Story)
                                                                    .story(#[allow(unused_braces)] { story })
                                                                    .build(),
                                                            )
                                                        }
                                                    })
                                                    .collect::<Vec<_>>()
                                            }
                                        },
                                    )
                            })
                        }
                    }
                }
                    .suspend()
                    .track()
                    .transition()
                    .with_fallback("Loading...")
            };
            {
                ::tachys::tachydom::html::element::div()
                    .child(
                        #[allow(unused_braces)]
                        {
                            ::tachys::tachydom::html::element::div()
                                .child(
                                    #[allow(unused_braces)]
                                    {
                                        ::tachys::tachydom::html::element::span()
                                            .child(
                                                #[allow(unused_braces)]
                                                {
                                                    ::tachys::component::component_view(
                                                        &Show,
                                                        ::tachys::component::component_props_builder(&Show)
                                                            .when(#[allow(unused_braces)] { move || { page() > 1 } })
                                                            .fallback(
                                                                #[allow(unused_braces)]
                                                                {
                                                                    || {
                                                                        ::tachys::tachydom::html::element::span()
                                                                            .child(
                                                                                #[allow(unused_braces)]
                                                                                {
                                                                                    ::tachys::tachydom::view::static_types::Static::<"< prev">
                                                                                },
                                                                            )
                                                                            .class(
                                                                                ::tachys::tachydom::view::static_types::Static::<
                                                                                    "page-link disabled",
                                                                                >,
                                                                            )
                                                                            .attr(
                                                                                "aria-hidden",
                                                                                ::tachys::tachydom::view::static_types::Static::<"true">,
                                                                            )
                                                                    }
                                                                },
                                                            )
                                                            .children({
                                                                ::tachys::children::ToChildren::to_children(move || {
                                                                    ::tachys::tachydom::html::element::a()
                                                                        .child(
                                                                            #[allow(unused_braces)]
                                                                            {
                                                                                ::tachys::tachydom::view::static_types::Static::<"< prev">
                                                                            },
                                                                        )
                                                                        .class(
                                                                            ::tachys::tachydom::view::static_types::Static::<
                                                                                "page-link",
                                                                            >,
                                                                        )
                                                                        .href(move || {
                                                                            let res = ::alloc::fmt::format(
                                                                                format_args!("/{0}?page={1}", story_type(), page() - 1),
                                                                            );
                                                                            res
                                                                        })
                                                                        .attr(
                                                                            "aria-label",
                                                                            ::tachys::tachydom::view::static_types::Static::<
                                                                                "Previous Page",
                                                                            >,
                                                                        )
                                                                })
                                                            })
                                                            .build(),
                                                    )
                                                },
                                            )
                                    },
                                )
                                .child(
                                    #[allow(unused_braces)]
                                    {
                                        ::tachys::tachydom::html::element::span()
                                            .child(
                                                #[allow(unused_braces)]
                                                {
                                                    ::tachys::tachydom::view::static_types::Static::<"page ">
                                                },
                                            )
                                            .child(#[allow(unused_braces)] { { page } })
                                    },
                                )
                                .child(
                                    #[allow(unused_braces)]
                                    {
                                        ::tachys::tachydom::html::element::span()
                                            .child(
                                                #[allow(unused_braces)]
                                                {
                                                    ::tachys::tachydom::html::element::a()
                                                        .child(
                                                            #[allow(unused_braces)]
                                                            {
                                                                ::tachys::tachydom::view::static_types::Static::<"more >">
                                                            },
                                                        )
                                                        .href(move || {
                                                            let res = ::alloc::fmt::format(
                                                                format_args!("/{0}?page={1}", story_type(), page() + 1),
                                                            );
                                                            res
                                                        })
                                                        .attr(
                                                            "aria-label",
                                                            ::tachys::tachydom::view::static_types::Static::<
                                                                "Next Page",
                                                            >,
                                                        )
                                                },
                                            )
                                            .class(
                                                ::tachys::tachydom::view::static_types::Static::<
                                                    "page-link",
                                                >,
                                            )
                                            .class(("disabled", hide_more_link))
                                            .attr("aria-hidden", hide_more_link)
                                    },
                                )
                                .class(
                                    ::tachys::tachydom::view::static_types::Static::<
                                        "news-list-nav",
                                    >,
                                )
                        },
                    )
                    .child(
                        #[allow(unused_braces)]
                        {
                            ::tachys::tachydom::html::element::main()
                                .child(
                                    #[allow(unused_braces)]
                                    {
                                        ::tachys::tachydom::html::element::div()
                                            .child(#[allow(unused_braces)] { { stories } })
                                    },
                                )
                                .class(
                                    ::tachys::tachydom::view::static_types::Static::<
                                        "news-list",
                                    >,
                                )
                        },
                    )
                    .class(::tachys::tachydom::view::static_types::Static::<"news-view">)
            }
        }
        /// Props for the [`Story`] component.
        ///
        ///
        /// # Required Props
        /// - **story**: [`api::Story`]
        #[builder(crate_module_path = ::tachys::typed_builder)]
        #[allow(non_snake_case)]
        struct StoryProps {
            #[builder(setter(doc = "**story**: [`api::Story`]"))]
            story: api::Story,
        }
        #[automatically_derived]
        impl StoryProps {
            /**
                Create a builder for building `StoryProps`.
                On the builder, call `.story(...)` to set the values of the fields.
                Finally, call `.build()` to create the instance of `StoryProps`.
                */
            #[allow(dead_code, clippy::default_trait_access)]
            fn builder() -> StoryPropsBuilder<((),)> {
                StoryPropsBuilder {
                    fields: ((),),
                    phantom: ::core::default::Default::default(),
                }
            }
        }
        #[must_use]
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        struct StoryPropsBuilder<TypedBuilderFields = ((),)> {
            fields: TypedBuilderFields,
            phantom: ::core::marker::PhantomData<()>,
        }
        #[automatically_derived]
        impl<TypedBuilderFields> Clone for StoryPropsBuilder<TypedBuilderFields>
        where
            TypedBuilderFields: Clone,
        {
            #[allow(clippy::default_trait_access)]
            fn clone(&self) -> Self {
                Self {
                    fields: self.fields.clone(),
                    phantom: ::core::default::Default::default(),
                }
            }
        }
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        #[automatically_derived]
        impl StoryPropsBuilder<((),)> {
            ///**story**: [`api::Story`]
            #[allow(clippy::used_underscore_binding)]
            pub fn story(
                self,
                story: api::Story,
            ) -> StoryPropsBuilder<((api::Story,),)> {
                let story = (story,);
                let ((),) = self.fields;
                StoryPropsBuilder {
                    fields: (story,),
                    phantom: self.phantom,
                }
            }
        }
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        #[allow(clippy::exhaustive_enums)]
        pub enum StoryPropsBuilder_Error_Repeated_field_story {}
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        #[automatically_derived]
        impl StoryPropsBuilder<((api::Story,),)> {
            #[deprecated(note = "Repeated field story")]
            pub fn story(
                self,
                _: StoryPropsBuilder_Error_Repeated_field_story,
            ) -> StoryPropsBuilder<((api::Story,),)> {
                self
            }
        }
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        #[allow(clippy::exhaustive_enums)]
        pub enum StoryPropsBuilder_Error_Missing_required_field_story {}
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, missing_docs, clippy::panic)]
        #[automatically_derived]
        impl StoryPropsBuilder<((),)> {
            #[deprecated(note = "Missing required field story")]
            pub fn build(
                self,
                _: StoryPropsBuilder_Error_Missing_required_field_story,
            ) -> ! {
                {
                    #[cold]
                    #[track_caller]
                    #[inline(never)]
                    const fn panic_cold_explicit() -> ! {
                        ::core::panicking::panic_explicit()
                    }
                    panic_cold_explicit();
                }
            }
        }
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        #[automatically_derived]
        impl StoryPropsBuilder<((api::Story,),)> {
            #[allow(clippy::default_trait_access, clippy::used_underscore_binding)]
            pub fn build(self) -> StoryProps {
                let (story,) = self.fields;
                let story = story.0;
                #[allow(deprecated)] StoryProps { story }.into()
            }
        }
        #[allow(missing_docs)]
        impl ::tachys::component::Props for StoryProps {
            type Builder = StoryPropsBuilder;
            fn builder() -> Self::Builder {
                StoryProps::builder()
            }
        }
        ///
        /// # Required Props
        /// - **story**: [`api::Story`]
        #[allow(non_snake_case, clippy::too_many_arguments)]
        #[allow(clippy::needless_lifetimes)]
        fn Story(props: StoryProps) -> impl RenderHtml<Dom> {
            let StoryProps { story } = props;
            ::tachys::tachy_reaccy::untrack(move || { __Story(story) })
        }
        #[doc(hidden)]
        #[allow(non_snake_case, dead_code, clippy::too_many_arguments)]
        pub fn __Story(story: api::Story) -> impl RenderHtml<Dom> {
            {
                ::tachys::tachydom::html::element::li()
                    .child(
                        #[allow(unused_braces)]
                        {
                            ::tachys::tachydom::html::element::span()
                                .child(#[allow(unused_braces)] { { story.points } })
                                .class(
                                    ::tachys::tachydom::view::static_types::Static::<"score">,
                                )
                        },
                    )
                    .child(
                        #[allow(unused_braces)]
                        {
                            ::tachys::tachydom::html::element::span()
                                .child(
                                    #[allow(unused_braces)]
                                    {
                                        {
                                            if !story.url.starts_with("item?id=") {
                                                Either::Left({
                                                    ::tachys::tachydom::html::element::span()
                                                        .child(
                                                            #[allow(unused_braces)]
                                                            {
                                                                ::tachys::tachydom::html::element::a()
                                                                    .child(#[allow(unused_braces)] { { story.title.clone() } })
                                                                    .href(story.url)
                                                                    .target(
                                                                        ::tachys::tachydom::view::static_types::Static::<"_blank">,
                                                                    )
                                                                    .rel(
                                                                        ::tachys::tachydom::view::static_types::Static::<
                                                                            "noreferrer",
                                                                        >,
                                                                    )
                                                            },
                                                        )
                                                        .child(
                                                            #[allow(unused_braces)]
                                                            {
                                                                ::tachys::tachydom::html::element::span()
                                                                    .child(
                                                                        #[allow(unused_braces)]
                                                                        { ::tachys::tachydom::view::static_types::Static::<"("> },
                                                                    )
                                                                    .child(#[allow(unused_braces)] { { story.domain } })
                                                                    .child(
                                                                        #[allow(unused_braces)]
                                                                        { ::tachys::tachydom::view::static_types::Static::<")"> },
                                                                    )
                                                                    .class(
                                                                        ::tachys::tachydom::view::static_types::Static::<"host">,
                                                                    )
                                                            },
                                                        )
                                                })
                                            } else {
                                                let title = story.title.clone();
                                                Either::Right({
                                                    ::tachys::tachydom::html::element::a()
                                                        .child(#[allow(unused_braces)] { { title.clone() } })
                                                        .href({
                                                            let res = ::alloc::fmt::format(
                                                                format_args!("/stories/{0}", story.id),
                                                            );
                                                            res
                                                        })
                                                })
                                            }
                                        }
                                    },
                                )
                                .class(
                                    ::tachys::tachydom::view::static_types::Static::<"title">,
                                )
                        },
                    )
                    .child(
                        #[allow(unused_braces)]
                        { ::tachys::tachydom::html::element::br() },
                    )
                    .child(
                        #[allow(unused_braces)]
                        {
                            ::tachys::tachydom::html::element::span()
                                .child(
                                    #[allow(unused_braces)]
                                    {
                                        {
                                            if story.story_type != "job" {
                                                Either::Left({
                                                    ::tachys::tachydom::html::element::span()
                                                        .child(#[allow(unused_braces)] { { "by " } })
                                                        .child(
                                                            #[allow(unused_braces)]
                                                            {
                                                                {
                                                                    story
                                                                        .user
                                                                        .map(|user| {
                                                                            ::tachys::tachydom::html::element::a()
                                                                                .child(#[allow(unused_braces)] { { user.clone() } })
                                                                                .href({
                                                                                    let res = ::alloc::fmt::format(
                                                                                        format_args!("/users/{0}", user),
                                                                                    );
                                                                                    res
                                                                                })
                                                                        })
                                                                }
                                                            },
                                                        )
                                                        .child(
                                                            #[allow(unused_braces)]
                                                            {
                                                                {
                                                                    {
                                                                        let res = ::alloc::fmt::format(
                                                                            format_args!(" {0} | ", story.time_ago),
                                                                        );
                                                                        res
                                                                    }
                                                                }
                                                            },
                                                        )
                                                        .child(
                                                            #[allow(unused_braces)]
                                                            {
                                                                ::tachys::tachydom::html::element::a()
                                                                    .child(
                                                                        #[allow(unused_braces)]
                                                                        {
                                                                            {
                                                                                if story.comments_count.unwrap_or_default() > 0 {
                                                                                    {
                                                                                        let res = ::alloc::fmt::format(
                                                                                            format_args!(
                                                                                                "{0} comments", story.comments_count.unwrap_or_default()
                                                                                            ),
                                                                                        );
                                                                                        res
                                                                                    }
                                                                                } else {
                                                                                    "discuss".into()
                                                                                }
                                                                            }
                                                                        },
                                                                    )
                                                                    .href({
                                                                        let res = ::alloc::fmt::format(
                                                                            format_args!("/stories/{0}", story.id),
                                                                        );
                                                                        res
                                                                    })
                                                            },
                                                        )
                                                })
                                            } else {
                                                let title = story.title.clone();
                                                Either::Right({
                                                    ::tachys::tachydom::html::element::a()
                                                        .child(#[allow(unused_braces)] { { title.clone() } })
                                                        .href({
                                                            let res = ::alloc::fmt::format(
                                                                format_args!("/item/{0}", story.id),
                                                            );
                                                            res
                                                        })
                                                })
                                            }
                                        }
                                    },
                                )
                                .class(
                                    ::tachys::tachydom::view::static_types::Static::<"meta">,
                                )
                        },
                    )
                    .child(
                        #[allow(unused_braces)]
                        {
                            {
                                (story.story_type != "link")
                                    .then(|| {
                                        (
                                            ::tachys::tachydom::view::static_types::Static::<" ">,
                                            ::tachys::tachydom::html::element::span()
                                                .child(#[allow(unused_braces)] { { story.story_type } })
                                                .class(
                                                    ::tachys::tachydom::view::static_types::Static::<"label">,
                                                ),
                                        )
                                    })
                            }
                        },
                    )
                    .class(::tachys::tachydom::view::static_types::Static::<"news-item">)
            }
        }
    }
    pub mod story {
        use crate::api;
        use send_wrapper::SendWrapper;
        use tachy_route::{reactive::ReactiveMatchedRoute, route::MatchedRoute};
        use tachys::{
            prelude::*, island, children::Children,
            tachydom::view::{any_view::IntoAny, either::Either},
        };
        pub fn Story(matched: MatchedRoute) -> impl RenderHtml<Dom> {
            let mut path = String::from("item/");
            let id = matched.param("id").unwrap_or_default();
            let id_is_empty = id.is_empty();
            path.push_str(id);
            let story = async move {
                if id_is_empty {
                    None
                } else {
                    api::fetch_api::<api::Story>(&api::story(&path)).await
                }
            };
            SendWrapper::new(async move {
                    match story.await {
                        None => {
                            Either::Left({
                                ::tachys::tachydom::html::element::div()
                                    .child(
                                        #[allow(unused_braces)]
                                        {
                                            ::tachys::tachydom::view::static_types::Static::<
                                                "Error loading this story.",
                                            >
                                        },
                                    )
                                    .class(
                                        ::tachys::tachydom::view::static_types::Static::<
                                            "item-view",
                                        >,
                                    )
                            })
                        }
                        Some(story) => {
                            Either::Right({
                                ::tachys::tachydom::html::element::div()
                                    .child(
                                        #[allow(unused_braces)]
                                        {
                                            ::tachys::tachydom::html::element::div()
                                                .child(
                                                    #[allow(unused_braces)]
                                                    {
                                                        ::tachys::tachydom::html::element::a()
                                                            .child(
                                                                #[allow(unused_braces)]
                                                                {
                                                                    ::tachys::tachydom::html::element::h1()
                                                                        .child(#[allow(unused_braces)] { { story.title } })
                                                                },
                                                            )
                                                            .href(story.url)
                                                            .target(
                                                                ::tachys::tachydom::view::static_types::Static::<"_blank">,
                                                            )
                                                    },
                                                )
                                                .child(
                                                    #[allow(unused_braces)]
                                                    {
                                                        ::tachys::tachydom::html::element::span()
                                                            .child(
                                                                #[allow(unused_braces)]
                                                                { ::tachys::tachydom::view::static_types::Static::<"("> },
                                                            )
                                                            .child(#[allow(unused_braces)] { { story.domain } })
                                                            .child(
                                                                #[allow(unused_braces)]
                                                                { ::tachys::tachydom::view::static_types::Static::<")"> },
                                                            )
                                                            .class(
                                                                ::tachys::tachydom::view::static_types::Static::<"host">,
                                                            )
                                                    },
                                                )
                                                .child(
                                                    #[allow(unused_braces)]
                                                    {
                                                        {
                                                            story
                                                                .user
                                                                .map(|user| {
                                                                    ::tachys::tachydom::html::element::p()
                                                                        .child(#[allow(unused_braces)] { { story.points } })
                                                                        .child(
                                                                            #[allow(unused_braces)]
                                                                            {
                                                                                ::tachys::tachydom::view::static_types::Static::<
                                                                                    " points | by ",
                                                                                >
                                                                            },
                                                                        )
                                                                        .child(
                                                                            #[allow(unused_braces)]
                                                                            {
                                                                                ::tachys::tachydom::html::element::a()
                                                                                    .child(#[allow(unused_braces)] { { user.clone() } })
                                                                                    .href({
                                                                                        let res = ::alloc::fmt::format(
                                                                                            format_args!("/users/{0}", user),
                                                                                        );
                                                                                        res
                                                                                    })
                                                                            },
                                                                        )
                                                                        .child(
                                                                            #[allow(unused_braces)]
                                                                            {
                                                                                {
                                                                                    {
                                                                                        let res = ::alloc::fmt::format(
                                                                                            format_args!(" {0}", story.time_ago),
                                                                                        );
                                                                                        res
                                                                                    }
                                                                                }
                                                                            },
                                                                        )
                                                                        .class(
                                                                            ::tachys::tachydom::view::static_types::Static::<"meta">,
                                                                        )
                                                                })
                                                        }
                                                    },
                                                )
                                                .class(
                                                    ::tachys::tachydom::view::static_types::Static::<
                                                        "item-view-header",
                                                    >,
                                                )
                                        },
                                    )
                                    .child(
                                        #[allow(unused_braces)]
                                        {
                                            ::tachys::tachydom::html::element::div()
                                                .child(
                                                    #[allow(unused_braces)]
                                                    {
                                                        ::tachys::tachydom::html::element::p()
                                                            .child(
                                                                #[allow(unused_braces)]
                                                                {
                                                                    {
                                                                        if story.comments_count.unwrap_or_default() > 0 {
                                                                            {
                                                                                let res = ::alloc::fmt::format(
                                                                                    format_args!(
                                                                                        "{0} comments", story.comments_count.unwrap_or_default()
                                                                                    ),
                                                                                );
                                                                                res
                                                                            }
                                                                        } else {
                                                                            "No comments yet.".into()
                                                                        }
                                                                    }
                                                                },
                                                            )
                                                            .class(
                                                                ::tachys::tachydom::view::static_types::Static::<
                                                                    "item-view-comments-header",
                                                                >,
                                                            )
                                                    },
                                                )
                                                .child(
                                                    #[allow(unused_braces)]
                                                    {
                                                        ::tachys::tachydom::html::element::ul()
                                                            .child(
                                                                #[allow(unused_braces)]
                                                                {
                                                                    {
                                                                        story
                                                                            .comments
                                                                            .into_iter()
                                                                            .flatten()
                                                                            .map(|comment| {
                                                                                ::tachys::component::component_view(
                                                                                    &Comment,
                                                                                    ::tachys::component::component_props_builder(&Comment)
                                                                                        .comment(#[allow(unused_braces)] { comment })
                                                                                        .build(),
                                                                                )
                                                                            })
                                                                            .collect::<Vec<_>>()
                                                                    }
                                                                },
                                                            )
                                                            .class(
                                                                ::tachys::tachydom::view::static_types::Static::<
                                                                    "comment-children",
                                                                >,
                                                            )
                                                    },
                                                )
                                                .class(
                                                    ::tachys::tachydom::view::static_types::Static::<
                                                        "item-view-comments",
                                                    >,
                                                )
                                        },
                                    )
                                    .class(
                                        ::tachys::tachydom::view::static_types::Static::<
                                            "item-view",
                                        >,
                                    )
                            })
                        }
                    }
                })
                .suspend()
                .with_fallback("Loading...")
        }
        /// Props for the [`Toggle`] component.
        ///
        ///
        /// # Required Props
        /// - **children**: [`Children`]
        #[builder(crate_module_path = ::tachys::typed_builder)]
        #[allow(non_snake_case)]
        pub struct ToggleProps {
            #[builder(setter(doc = "**children**: [`Children`]"))]
            #[allow(missing_docs)]
            pub children: Children,
        }
        #[automatically_derived]
        impl ToggleProps {
            /**
                Create a builder for building `ToggleProps`.
                On the builder, call `.children(...)` to set the values of the fields.
                Finally, call `.build()` to create the instance of `ToggleProps`.
                */
            #[allow(dead_code, clippy::default_trait_access)]
            pub fn builder() -> TogglePropsBuilder<((),)> {
                TogglePropsBuilder {
                    fields: ((),),
                    phantom: ::core::default::Default::default(),
                }
            }
        }
        #[must_use]
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        pub struct TogglePropsBuilder<TypedBuilderFields = ((),)> {
            fields: TypedBuilderFields,
            phantom: ::core::marker::PhantomData<()>,
        }
        #[automatically_derived]
        impl<TypedBuilderFields> Clone for TogglePropsBuilder<TypedBuilderFields>
        where
            TypedBuilderFields: Clone,
        {
            #[allow(clippy::default_trait_access)]
            fn clone(&self) -> Self {
                Self {
                    fields: self.fields.clone(),
                    phantom: ::core::default::Default::default(),
                }
            }
        }
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        #[automatically_derived]
        impl TogglePropsBuilder<((),)> {
            ///**children**: [`Children`]
            #[allow(clippy::used_underscore_binding)]
            pub fn children(
                self,
                children: Children,
            ) -> TogglePropsBuilder<((Children,),)> {
                let children = (children,);
                let ((),) = self.fields;
                TogglePropsBuilder {
                    fields: (children,),
                    phantom: self.phantom,
                }
            }
        }
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        #[allow(clippy::exhaustive_enums)]
        pub enum TogglePropsBuilder_Error_Repeated_field_children {}
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        #[automatically_derived]
        impl TogglePropsBuilder<((Children,),)> {
            #[deprecated(note = "Repeated field children")]
            pub fn children(
                self,
                _: TogglePropsBuilder_Error_Repeated_field_children,
            ) -> TogglePropsBuilder<((Children,),)> {
                self
            }
        }
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        #[allow(clippy::exhaustive_enums)]
        pub enum TogglePropsBuilder_Error_Missing_required_field_children {}
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, missing_docs, clippy::panic)]
        #[automatically_derived]
        impl TogglePropsBuilder<((),)> {
            #[deprecated(note = "Missing required field children")]
            pub fn build(
                self,
                _: TogglePropsBuilder_Error_Missing_required_field_children,
            ) -> ! {
                {
                    #[cold]
                    #[track_caller]
                    #[inline(never)]
                    const fn panic_cold_explicit() -> ! {
                        ::core::panicking::panic_explicit()
                    }
                    panic_cold_explicit();
                }
            }
        }
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        #[automatically_derived]
        impl TogglePropsBuilder<((Children,),)> {
            #[allow(clippy::default_trait_access, clippy::used_underscore_binding)]
            pub fn build(self) -> ToggleProps {
                let (children,) = self.fields;
                let children = children.0;
                #[allow(deprecated)] ToggleProps { children }.into()
            }
        }
        #[allow(dead_code)]
        #[allow(missing_docs)]
        #[allow(non_snake_case)]
        pub fn _island_Toggle(el: ::tachys::tachydom::web_sys::HtmlElement) {
            ::tachys::tachydom::web_sys::console::log_2(
                &tachys::tachydom::wasm_bindgen::JsValue::from_str("island is"),
                &el,
            );
            let island = Toggle({ ToggleProps::builder().children(children).build() });
            let state = island
                .hydrate_from_position::<
                    true,
                >(&el, ::tachys::tachydom::view::Position::Current);
            std::mem::forget(state);
        }
        #[automatically_derived]
        const _: () = {
            #[allow(missing_docs)]
            #[allow(non_snake_case)]
            #[export_name = "_island_Toggle"]
            pub unsafe extern "C" fn __wasm_bindgen_generated__island_Toggle(
                arg0_1: <<::tachys::tachydom::web_sys::HtmlElement as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
                arg0_2: <<::tachys::tachydom::web_sys::HtmlElement as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
                arg0_3: <<::tachys::tachydom::web_sys::HtmlElement as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
                arg0_4: <<::tachys::tachydom::web_sys::HtmlElement as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
            ) -> wasm_bindgen::convert::WasmRet<
                <() as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
            > {
                let _ret = {
                    let arg0 = unsafe {
                        <::tachys::tachydom::web_sys::HtmlElement as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                            <<::tachys::tachydom::web_sys::HtmlElement as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                                arg0_1,
                                arg0_2,
                                arg0_3,
                                arg0_4,
                            ),
                        )
                    };
                    let _ret = _island_Toggle(arg0);
                    _ret
                };
                <() as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret).into()
            }
        };
        #[cfg(
            all(
                target_arch = "wasm32",
                not(any(target_os = "emscripten", target_os = "wasi"))
            )
        )]
        #[automatically_derived]
        const _: () = {
            #[allow(missing_docs)]
            #[allow(non_snake_case)]
            #[no_mangle]
            #[doc(hidden)]
            pub extern "C" fn __wbindgen_describe__island_Toggle() {
                use wasm_bindgen::describe::*;
                wasm_bindgen::__rt::link_mem_intrinsics();
                inform(FUNCTION);
                inform(0);
                inform(1u32);
                <::tachys::tachydom::web_sys::HtmlElement as WasmDescribe>::describe();
                <() as WasmDescribe>::describe();
                <() as WasmDescribe>::describe();
            }
        };
        #[cfg(target_arch = "wasm32")]
        #[automatically_derived]
        const _: () = {
            static _INCLUDED_FILES: &[&str] = &[];
            #[link_section = "__wasm_bindgen_unstable"]
            pub static _GENERATED: [u8; 121usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.89\"}C\x00\x00\x00\x01\x00\x00\x00\x01\x02el\x00\x0e_island_Toggle\x01\x01\x00\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x1bhackernews-2cb78fdf9862f9e8\x00\x00";
        };
        impl ::tachys::component::Props for ToggleProps {
            type Builder = TogglePropsBuilder;
            fn builder() -> Self::Builder {
                ToggleProps::builder()
            }
        }
        ///
        /// # Required Props
        /// - **children**: [`Children`]
        #[allow(non_snake_case, clippy::too_many_arguments)]
        #[allow(clippy::needless_lifetimes)]
        pub fn Toggle(props: ToggleProps) -> impl RenderHtml<Dom> {
            let ToggleProps { children } = props;
            {
                ::tachys::tachydom::html::islands::Island::new(
                    "Toggle",
                    ::tachys::tachy_reaccy::untrack(move || {
                        ::tachys::tachy_reaccy::Owner::with_hydration(move || {
                            __Toggle(children)
                        })
                    }),
                )
            }
        }
        #[doc(hidden)]
        #[allow(non_snake_case, dead_code, clippy::too_many_arguments)]
        pub fn __Toggle(children: Children) -> impl RenderHtml<Dom> {
            let open = ArcRwSignal::new(true);
            {
                (
                    ::tachys::tachydom::html::element::div()
                        .child(
                            #[allow(unused_braces)]
                            {
                                ::tachys::tachydom::html::element::a()
                                    .child(
                                        #[allow(unused_braces)]
                                        {
                                            {
                                                let open = open.clone();
                                                move || {
                                                    if open.get() { "[-]" } else { "[+] comments collapsed" }
                                                }
                                            }
                                        },
                                    )
                                    .on(
                                        ::tachys::tachydom::html::event::click,
                                        {
                                            let open = open.clone();
                                            move |_| open.update(|n| *n = !*n)
                                        },
                                    )
                            },
                        )
                        .class(
                            ::tachys::tachydom::view::static_types::Static::<"toggle">,
                        )
                        .class((
                            "open",
                            {
                                let open = open.clone();
                                move || open.get()
                            },
                        )),
                    ::tachys::tachydom::html::element::ul()
                        .child(#[allow(unused_braces)] { { children() } })
                        .class(
                            ::tachys::tachydom::view::static_types::Static::<
                                "comment-children",
                            >,
                        )
                        .style((
                            "display",
                            move || if open.get() { "block" } else { "none" },
                        )),
                )
            }
        }
        /// Props for the [`Comment`] component.
        ///
        ///
        /// # Required Props
        /// - **comment**: [`api::Comment`]
        #[builder(crate_module_path = ::tachys::typed_builder)]
        #[allow(non_snake_case)]
        pub struct CommentProps {
            #[builder(setter(doc = "**comment**: [`api::Comment`]"))]
            pub comment: api::Comment,
        }
        #[automatically_derived]
        impl CommentProps {
            /**
                Create a builder for building `CommentProps`.
                On the builder, call `.comment(...)` to set the values of the fields.
                Finally, call `.build()` to create the instance of `CommentProps`.
                */
            #[allow(dead_code, clippy::default_trait_access)]
            pub fn builder() -> CommentPropsBuilder<((),)> {
                CommentPropsBuilder {
                    fields: ((),),
                    phantom: ::core::default::Default::default(),
                }
            }
        }
        #[must_use]
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        pub struct CommentPropsBuilder<TypedBuilderFields = ((),)> {
            fields: TypedBuilderFields,
            phantom: ::core::marker::PhantomData<()>,
        }
        #[automatically_derived]
        impl<TypedBuilderFields> Clone for CommentPropsBuilder<TypedBuilderFields>
        where
            TypedBuilderFields: Clone,
        {
            #[allow(clippy::default_trait_access)]
            fn clone(&self) -> Self {
                Self {
                    fields: self.fields.clone(),
                    phantom: ::core::default::Default::default(),
                }
            }
        }
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        #[automatically_derived]
        impl CommentPropsBuilder<((),)> {
            ///**comment**: [`api::Comment`]
            #[allow(clippy::used_underscore_binding)]
            pub fn comment(
                self,
                comment: api::Comment,
            ) -> CommentPropsBuilder<((api::Comment,),)> {
                let comment = (comment,);
                let ((),) = self.fields;
                CommentPropsBuilder {
                    fields: (comment,),
                    phantom: self.phantom,
                }
            }
        }
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        #[allow(clippy::exhaustive_enums)]
        pub enum CommentPropsBuilder_Error_Repeated_field_comment {}
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        #[automatically_derived]
        impl CommentPropsBuilder<((api::Comment,),)> {
            #[deprecated(note = "Repeated field comment")]
            pub fn comment(
                self,
                _: CommentPropsBuilder_Error_Repeated_field_comment,
            ) -> CommentPropsBuilder<((api::Comment,),)> {
                self
            }
        }
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, non_snake_case)]
        #[allow(clippy::exhaustive_enums)]
        pub enum CommentPropsBuilder_Error_Missing_required_field_comment {}
        #[doc(hidden)]
        #[allow(dead_code, non_camel_case_types, missing_docs, clippy::panic)]
        #[automatically_derived]
        impl CommentPropsBuilder<((),)> {
            #[deprecated(note = "Missing required field comment")]
            pub fn build(
                self,
                _: CommentPropsBuilder_Error_Missing_required_field_comment,
            ) -> ! {
                {
                    #[cold]
                    #[track_caller]
                    #[inline(never)]
                    const fn panic_cold_explicit() -> ! {
                        ::core::panicking::panic_explicit()
                    }
                    panic_cold_explicit();
                }
            }
        }
        #[allow(dead_code, non_camel_case_types, missing_docs)]
        #[automatically_derived]
        impl CommentPropsBuilder<((api::Comment,),)> {
            #[allow(clippy::default_trait_access, clippy::used_underscore_binding)]
            pub fn build(self) -> CommentProps {
                let (comment,) = self.fields;
                let comment = comment.0;
                #[allow(deprecated)] CommentProps { comment }.into()
            }
        }
        #[allow(missing_docs)]
        impl ::tachys::component::Props for CommentProps {
            type Builder = CommentPropsBuilder;
            fn builder() -> Self::Builder {
                CommentProps::builder()
            }
        }
        ///
        /// # Required Props
        /// - **comment**: [`api::Comment`]
        #[allow(non_snake_case, clippy::too_many_arguments)]
        #[allow(clippy::needless_lifetimes)]
        pub fn Comment(props: CommentProps) -> impl RenderHtml<Dom> {
            let CommentProps { comment } = props;
            ::tachys::tachy_reaccy::untrack(move || { __Comment(comment) })
        }
        #[doc(hidden)]
        #[allow(non_snake_case, dead_code, clippy::too_many_arguments)]
        pub fn __Comment(comment: api::Comment) -> impl RenderHtml<Dom> {
            {
                ::tachys::tachydom::html::element::li()
                    .child(
                        #[allow(unused_braces)]
                        {
                            ::tachys::tachydom::html::element::div()
                                .child(
                                    #[allow(unused_braces)]
                                    {
                                        ::tachys::tachydom::html::element::a()
                                            .child(#[allow(unused_braces)] { { comment.user.clone() } })
                                            .href({
                                                let res = ::alloc::fmt::format(
                                                    format_args!(
                                                        "/users/{0}", comment.user.clone().unwrap_or_default()
                                                    ),
                                                );
                                                res
                                            })
                                    },
                                )
                                .child(
                                    #[allow(unused_braces)]
                                    {
                                        {
                                            {
                                                let res = ::alloc::fmt::format(
                                                    format_args!(" {0}", comment.time_ago),
                                                );
                                                res
                                            }
                                        }
                                    },
                                )
                                .class(
                                    ::tachys::tachydom::view::static_types::Static::<"by">,
                                )
                        },
                    )
                    .child(
                        #[allow(unused_braces)]
                        {
                            ::tachys::tachydom::html::element::div()
                                .class(
                                    ::tachys::tachydom::view::static_types::Static::<"text">,
                                )
                                .inner_html(comment.content.unwrap_or_default())
                        },
                    )
                    .child(
                        #[allow(unused_braces)]
                        {
                            {
                                (!comment.comments.is_empty())
                                    .then(|| {
                                        {
                                            ::tachys::component::component_view(
                                                &Toggle,
                                                ::tachys::component::component_props_builder(&Toggle)
                                                    .children({
                                                        ::tachys::children::ToChildren::to_children(move || {
                                                            comment
                                                                .comments
                                                                .into_iter()
                                                                .map(|comment: api::Comment| {
                                                                    ::tachys::component::component_view(
                                                                        &Comment,
                                                                        ::tachys::component::component_props_builder(&Comment)
                                                                            .comment(#[allow(unused_braces)] { comment })
                                                                            .build(),
                                                                    )
                                                                })
                                                                .collect::<Vec<_>>()
                                                        })
                                                    })
                                                    .build(),
                                            )
                                        }
                                    })
                            }
                        },
                    )
                    .class(::tachys::tachydom::view::static_types::Static::<"comment">)
            }
        }
        fn pluralize(n: usize) -> &'static str {
            if n == 1 { " reply" } else { " replies" }
        }
    }
    pub mod users {
        use crate::api::{self, User};
        use send_wrapper::SendWrapper;
        use std::{collections::HashMap, future::IntoFuture};
        use tachy_reaccy::async_signal::ArcAsyncDerived;
        use tachy_route::{reactive::ReactiveMatchedRoute, route::MatchedRoute};
        use tachys::{
            prelude::*,
            tachydom::view::{any_view::IntoAny, either::Either, template::ViewTemplate},
        };
        pub fn User(matched: MatchedRoute) -> impl RenderHtml<Dom> {
            let id = matched.param("id").unwrap_or_default().to_owned();
            let user = async move {
                if id.is_empty() {
                    None
                } else {
                    api::fetch_api::<User>(&api::user(&id)).await
                }
            };
            let user_view = SendWrapper::new(async move {
                    match user.await {
                        None => {
                            Either::Left({
                                ::tachys::tachydom::html::element::h1()
                                    .child(
                                        #[allow(unused_braces)]
                                        {
                                            ::tachys::tachydom::view::static_types::Static::<
                                                "User not found.",
                                            >
                                        },
                                    )
                            })
                        }
                        Some(user) => {
                            Either::Right({
                                ::tachys::tachydom::html::element::div()
                                    .child(
                                        #[allow(unused_braces)]
                                        {
                                            ::tachys::tachydom::html::element::h1()
                                                .child(
                                                    #[allow(unused_braces)]
                                                    {
                                                        ::tachys::tachydom::view::static_types::Static::<"User: ">
                                                    },
                                                )
                                                .child(#[allow(unused_braces)] { { user.id.clone() } })
                                        },
                                    )
                                    .child(
                                        #[allow(unused_braces)]
                                        {
                                            ::tachys::tachydom::html::element::ul()
                                                .child(
                                                    #[allow(unused_braces)]
                                                    {
                                                        ::tachys::tachydom::html::element::li()
                                                            .child(
                                                                #[allow(unused_braces)]
                                                                {
                                                                    ::tachys::tachydom::html::element::span()
                                                                        .child(
                                                                            #[allow(unused_braces)]
                                                                            {
                                                                                ::tachys::tachydom::view::static_types::Static::<
                                                                                    "Created: ",
                                                                                >
                                                                            },
                                                                        )
                                                                        .class(
                                                                            ::tachys::tachydom::view::static_types::Static::<"label">,
                                                                        )
                                                                },
                                                            )
                                                            .child(#[allow(unused_braces)] { { user.created } })
                                                    },
                                                )
                                                .child(
                                                    #[allow(unused_braces)]
                                                    {
                                                        ::tachys::tachydom::html::element::li()
                                                            .child(
                                                                #[allow(unused_braces)]
                                                                {
                                                                    ::tachys::tachydom::html::element::span()
                                                                        .child(
                                                                            #[allow(unused_braces)]
                                                                            {
                                                                                ::tachys::tachydom::view::static_types::Static::<"Karma: ">
                                                                            },
                                                                        )
                                                                        .class(
                                                                            ::tachys::tachydom::view::static_types::Static::<"label">,
                                                                        )
                                                                },
                                                            )
                                                            .child(#[allow(unused_braces)] { { user.karma } })
                                                    },
                                                )
                                                .child(
                                                    #[allow(unused_braces)]
                                                    {
                                                        ::tachys::tachydom::html::element::li()
                                                            .inner_html({ user.about.unwrap_or_default() })
                                                            .class(
                                                                ::tachys::tachydom::view::static_types::Static::<"about">,
                                                            )
                                                    },
                                                )
                                                .class(
                                                    ::tachys::tachydom::view::static_types::Static::<"meta">,
                                                )
                                        },
                                    )
                                    .child(
                                        #[allow(unused_braces)]
                                        {
                                            ::tachys::tachydom::html::element::p()
                                                .child(
                                                    #[allow(unused_braces)]
                                                    {
                                                        ::tachys::tachydom::html::element::a()
                                                            .child(
                                                                #[allow(unused_braces)]
                                                                {
                                                                    ::tachys::tachydom::view::static_types::Static::<
                                                                        "submissions",
                                                                    >
                                                                },
                                                            )
                                                            .href({
                                                                let res = ::alloc::fmt::format(
                                                                    format_args!(
                                                                        "https://news.ycombinator.com/submitted?id={0}", user.id
                                                                    ),
                                                                );
                                                                res
                                                            })
                                                    },
                                                )
                                                .child(
                                                    #[allow(unused_braces)]
                                                    { ::tachys::tachydom::view::static_types::Static::<" | "> },
                                                )
                                                .child(
                                                    #[allow(unused_braces)]
                                                    {
                                                        ::tachys::tachydom::html::element::a()
                                                            .child(
                                                                #[allow(unused_braces)]
                                                                {
                                                                    ::tachys::tachydom::view::static_types::Static::<"comments">
                                                                },
                                                            )
                                                            .href({
                                                                let res = ::alloc::fmt::format(
                                                                    format_args!(
                                                                        "https://news.ycombinator.com/threads?id={0}", user.id
                                                                    ),
                                                                );
                                                                res
                                                            })
                                                    },
                                                )
                                                .class(
                                                    ::tachys::tachydom::view::static_types::Static::<"links">,
                                                )
                                        },
                                    )
                            })
                        }
                    }
                })
                .suspend()
                .with_fallback("Loading...");
            {
                ::tachys::tachydom::html::element::div()
                    .child(#[allow(unused_braces)] { { user_view } })
                    .class(::tachys::tachydom::view::static_types::Static::<"user-view">)
            }
        }
    }
}
use routes::{nav::Nav, stories::Stories, story::Story, users::User};
use tachys::children::Children;
/// Props for the [`App`] component.
///
///
#[builder(crate_module_path = ::tachys::typed_builder)]
#[allow(non_snake_case)]
pub struct AppProps {}
#[automatically_derived]
impl AppProps {
    /**
                Create a builder for building `AppProps`.
                On the builder, call  to set the values of the fields.
                Finally, call `.build()` to create the instance of `AppProps`.
                */
    #[allow(dead_code, clippy::default_trait_access)]
    pub fn builder() -> AppPropsBuilder<()> {
        AppPropsBuilder {
            fields: (),
            phantom: ::core::default::Default::default(),
        }
    }
}
#[must_use]
#[doc(hidden)]
#[allow(dead_code, non_camel_case_types, non_snake_case)]
pub struct AppPropsBuilder<TypedBuilderFields = ()> {
    fields: TypedBuilderFields,
    phantom: ::core::marker::PhantomData<()>,
}
#[automatically_derived]
impl<TypedBuilderFields> Clone for AppPropsBuilder<TypedBuilderFields>
where
    TypedBuilderFields: Clone,
{
    #[allow(clippy::default_trait_access)]
    fn clone(&self) -> Self {
        Self {
            fields: self.fields.clone(),
            phantom: ::core::default::Default::default(),
        }
    }
}
#[allow(dead_code, non_camel_case_types, missing_docs)]
#[automatically_derived]
impl AppPropsBuilder<()> {
    #[allow(clippy::default_trait_access, clippy::used_underscore_binding)]
    pub fn build(self) -> AppProps {
        let () = self.fields;
        #[allow(deprecated)] AppProps {}.into()
    }
}
#[allow(missing_docs)]
impl ::tachys::component::Props for AppProps {
    type Builder = AppPropsBuilder;
    fn builder() -> Self::Builder {
        AppProps::builder()
    }
}
///
#[allow(non_snake_case, clippy::too_many_arguments)]
#[allow(clippy::needless_lifetimes)]
pub fn App() -> impl RenderHtml<Dom> {
    ::tachys::tachy_reaccy::untrack(move || { __App() })
}
#[doc(hidden)]
#[allow(non_snake_case, dead_code, clippy::too_many_arguments)]
pub fn __App() -> impl RenderHtml<Dom> {
    let (is_routing, set_is_routing) = signal(false);
    let router = ReactiveRouter(
        { #[cfg(not(feature = "ssr"))] { BrowserUrl::new() } },
        || {
            (
                RouteDefinition::new(
                    (StaticSegment("users"), ParamSegment("id")),
                    (),
                    User,
                ),
                RouteDefinition::new(
                    (StaticSegment("stories"), ParamSegment("id")),
                    (),
                    Story,
                ),
                RouteDefinition::new(
                    ParamSegment("stories"),
                    (),
                    reactive_route(Stories),
                ),
            )
        },
        || "Not Found",
    );
    {
        (
            ::tachys::component::component_view(
                &Nav,
                ::tachys::component::component_props_builder(&Nav).build(),
            ),
            ::tachys::tachydom::html::element::main()
                .child(#[allow(unused_braces)] { { router } }),
        )
    }
}
#[allow(dead_code)]
#[cfg(feature = "hydrate")]
pub fn hydrate() {
    Root::global_islands(|| ());
}
#[automatically_derived]
const _: () = {
    #[cfg(feature = "hydrate")]
    #[export_name = "hydrate"]
    pub unsafe extern "C" fn __wasm_bindgen_generated_hydrate() -> wasm_bindgen::convert::WasmRet<
        <() as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
    > {
        let _ret = {
            let _ret = hydrate();
            _ret
        };
        <() as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret).into()
    }
};
#[cfg(
    all(target_arch = "wasm32", not(any(target_os = "emscripten", target_os = "wasi")))
)]
#[automatically_derived]
const _: () = {
    #[cfg(feature = "hydrate")]
    #[no_mangle]
    #[doc(hidden)]
    pub extern "C" fn __wbindgen_describe_hydrate() {
        use wasm_bindgen::describe::*;
        wasm_bindgen::__rt::link_mem_intrinsics();
        inform(FUNCTION);
        inform(0);
        inform(0u32);
        <() as WasmDescribe>::describe();
        <() as WasmDescribe>::describe();
    }
};
#[cfg(target_arch = "wasm32")]
#[automatically_derived]
const _: () = {
    static _INCLUDED_FILES: &[&str] = &[];
    #[link_section = "__wasm_bindgen_unstable"]
    pub static _GENERATED: [u8; 111usize] = *b".\x00\x00\x00{\"schema_version\":\"0.2.88\",\"version\":\"0.2.89\"}9\x00\x00\x00\x01\x00\x00\x00\x00\x00\x07hydrate\x01\x01\x00\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x1bhackernews-2cb78fdf9862f9e8\x00\x00";
};
