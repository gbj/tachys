mod param_segments;
mod static_segment;
pub use param_segments::*;
pub use static_segment::*;
use std::{borrow::Cow, str::Chars};

pub(crate) type Params<K> = Vec<(K, String)>;

/// Defines a route which may or may not be matched by any given URL,
/// or URL segment.
///
/// This is a "horizontal" matching: i.e., it treats a tuple of route segments
/// as subsequent segments of the URL and tries to match them all. For a "vertical"
/// matching that sees a tuple as alternatives to one another, see [`RouteChild`](super::RouteChild).
pub trait RouteMatch {
    fn matches(&self, path: &str) -> bool {
        self.matches_iter(&mut path.chars())
    }

    fn matches_iter(&self, path: &mut Chars) -> bool;

    fn test<'a>(&self, path: &'a str) -> Option<PartialPathMatch<'a>>;
}

#[derive(Debug)]
pub struct PartialPathMatch<'a> {
    pub(crate) remaining: &'a str,
    pub(crate) params: Params<&'static str>,
    pub(crate) matched: String,
}

impl<'a> PartialPathMatch<'a> {
    pub fn new(
        remaining: &'a str,
        params: impl Into<Params<&'static str>>,
        matched: impl Into<String>,
    ) -> Self {
        Self {
            remaining,
            params: params.into(),
            matched: matched.into(),
        }
    }

    pub fn is_complete(&self) -> bool {
        self.remaining.is_empty() || self.remaining == "/"
    }

    pub fn remaining(&self) -> &str {
        self.remaining
    }

    pub fn params(&self) -> &[(&'static str, String)] {
        &self.params
    }

    pub fn matched(&self) -> &str {
        self.matched.as_str()
    }
}

macro_rules! tuples {
    ($($ty:ident),*) => {
        impl<$($ty),*> RouteMatch for ($($ty,)*)
        where
			$($ty: RouteMatch),*,
        {
            fn matches_iter(&self, path: &mut Chars) -> bool
            {
				paste::paste! {
					let ($([<$ty:lower>],)*) = &self;
                    $([<$ty:lower>].matches_iter(path) &&)* true
                }
            }

            fn test<'a>(&self, path: &'a str) -> Option<PartialPathMatch<'a>>
            {
				let mut full_params = Vec::new();
				let mut full_matched = String::new();
				paste::paste! {
					let ($([<$ty:lower>],)*) = &self;
                    $(
						let PartialPathMatch {
							remaining,
							matched,
							params
						} = [<$ty:lower>].test(path)?;
						let path = remaining;
						full_matched.push_str(&matched);
						full_params.extend(params);
					)*
					Some(PartialPathMatch {
						remaining: path,
						matched: full_matched,
						params: full_params
					})
                }
            }
        }
	};
}

tuples!(A, B);
tuples!(A, B, C);
tuples!(A, B, C, D);
tuples!(A, B, C, D, E);
tuples!(A, B, C, D, E, F);
tuples!(A, B, C, D, E, F, G);
tuples!(A, B, C, D, E, F, G, H);
tuples!(A, B, C, D, E, F, G, H, I);
tuples!(A, B, C, D, E, F, G, H, I, J);
tuples!(A, B, C, D, E, F, G, H, I, J, K);
tuples!(A, B, C, D, E, F, G, H, I, J, K, L);
tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M);
tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);
tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U);
tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V);
tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W);
tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X);
tuples!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y
);
tuples!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y,
    Z
);

#[cfg(test)]
mod tests {
    use crate::matching::{ParamSegment, RouteMatch, StaticSegment};

    #[test]
    fn mixture_of_static_and_params() {
        /* matches */
        let path = "/posts/3/comments/123";
        let def = (
            (StaticSegment("posts"), ParamSegment("id")),
            (StaticSegment("comments"), ParamSegment("comment_id")),
        );
        let matched = def.test(path).expect("couldn't match route");
        assert_eq!(matched.matched, "/posts/3/comments/123");
        assert_eq!(matched.remaining, "");
        assert_eq!(matched.params[0], ("id", "3".to_string()));
        assert_eq!(matched.params[1], ("comment_id", "123".to_string()));

        /* doesn't match */
        let path = "/posts/3/commentary/123";
        assert!(def.test(path).is_none());
    }
}
