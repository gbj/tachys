use super::params_map::ParamsMap;
use std::{borrow::Cow, str::FromStr, vec::IntoIter};

pub trait RoutePath {
    type Params;

    fn exhaustive_match(&self, path: &str) -> Option<PathMatch> {
        let mut segments = path.split('/').filter(|n| !n.is_empty());
        let matched = self.test(&mut segments)?;
        segments.next().is_none().then_some(matched)
    }

    fn test<'a, I>(&self, path: &mut I) -> Option<PathMatch>
    where
        I: Iterator<Item = &'a str>;
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct PathMatch {
    pub path: Option<Cow<'static, str>>,
    pub params: ParamsMap,
}

trait InitialOrMiddleSegment {}

impl InitialOrMiddleSegment for () {}
impl RoutePath for () {
    type Params = ();

    fn test<'a, I>(&self, path: &mut I) -> Option<PathMatch>
    where
        I: Iterator<Item = &'a str>,
    {
        Some(Default::default())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct StaticSegment(pub &'static str);

impl InitialOrMiddleSegment for StaticSegment {}

impl RoutePath for StaticSegment {
    type Params = ();
    fn test<'a, I>(&self, path: &mut I) -> Option<PathMatch>
    where
        I: Iterator<Item = &'a str>,
    {
        if self.0.is_empty() {
            Some(Default::default())
        } else {
            let to_match = path.next()?;
            (self.0 == to_match).then(|| PathMatch {
                path: Some(self.0.into()),
                params: Default::default(),
            })
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ParamSegment<T>
where
    T: FromStr,
{
    field_name: &'static str,
    validator: for<'a> fn(&'a str) -> Option<T>,
}

impl<T> ParamSegment<T>
where
    T: FromStr + 'static,
{
    pub fn new(field_name: &'static str) -> ParamSegment<T> {
        ParamSegment {
            field_name,
            validator: |segment| T::from_str(segment).ok(),
        }
    }
}

impl<T> InitialOrMiddleSegment for ParamSegment<T> where T: FromStr {}

impl<T> RoutePath for ParamSegment<T>
where
    T: FromStr + 'static,
{
    type Params = (&'static str, T);

    fn test<'a, I>(&self, path: &mut I) -> Option<PathMatch>
    where
        I: Iterator<Item = &'a str>,
    {
        let segment = path.next()?;
        let value = (self.validator)(segment)?;
        Some(PathMatch {
            path: Some(segment.to_string().into()),
            params: ParamsMap::single(self.field_name, segment),
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SplatSegment {
    field_name: &'static str,
}
impl RoutePath for SplatSegment {
    type Params = (&'static str, String);

    fn test<'a, I>(&self, path: &mut I) -> Option<PathMatch>
    where
        I: Iterator<Item = &'a str>,
    {
        let mut matched = String::new();
        if let Some(next) = path.next() {
            matched.push_str(next);
        }
        for next in path {
            matched.push('/');
            matched.push_str(next);
        }
        Some(PathMatch {
            path: Some(matched.clone().into()),
            params: ParamsMap::single(self.field_name, matched),
        })
    }
}

impl<A> RoutePath for (A,)
where
    A: RoutePath,
{
    type Params = A::Params;

    fn test<'a, I>(&self, path: &mut I) -> Option<PathMatch>
    where
        I: Iterator<Item = &'a str>,
    {
        self.0.test(path)
    }
}

macro_rules! impl_route_path_tuple {
    ($($ty:ident),* => $last:ident) => {
        impl<$($ty),*, $last> InitialOrMiddleSegment for ($($ty,)* $last)
        where
			$($ty: InitialOrMiddleSegment),*,
			$last: InitialOrMiddleSegment
        {}

		impl<$($ty),*, $last> RoutePath for ($($ty,)* $last)
		where
			$($ty: InitialOrMiddleSegment + RoutePath),*,
			$last: RoutePath
		{
			type Params = ($($ty::Params,)* $last::Params);

			fn test<'a, It>(&self, path: &mut It) -> Option<PathMatch>
			where
				It: Iterator<Item = &'a str>,
			{
				paste::paste! {
                    // destructure into path segment in tuple
					let ($([<$ty:lower>],)* [<$last:lower>]) = self;

                    // test each segment of path to see if it was matched
                    // if any weren't matched, just return
					$(let [<$ty:lower _match>] = [<$ty:lower>].test(path)?);*;
					let [<$last:lower _match>] = [<$last:lower>].test(path)?;

                    // join all paths, prepending `/` to actual matched segments
                    let path = [
                        // if path is None, it means it didn't correspond to any
                        // actual text: for example, it was () or a "" segment
                        // it it is Some, it needs to have its `/` inserted again
                        $([<$ty:lower _match>].path.map(|n| ["/".into(), n].into_iter()),)*
                        [<$last:lower _match>].path.map(|n| ["/".into(), n].into_iter())
                    ]
                    .into_iter()
                    .flatten()
                    .flatten()
                    .collect::<String>();
                    // if this path is still empty, though (i.e., ((), ()))
                    // then we should still ignore it
                    let path = if path.is_empty() || path == "/" {
                        None
                    } else {
                        Some(Cow::Owned(path))
                    };

                    // join all params
                    let params = [
                        $([<$ty:lower _match>].params.into_iter(),)*
                        [<$last:lower _match>].params.into_iter()
                    ]
                    .into_iter()
                    .flatten()
                    .collect();

					Some(PathMatch { path, params })
				}
			}
		}
	};
}

impl_route_path_tuple!(A => B);
impl_route_path_tuple!(A, B => C);
impl_route_path_tuple!(A, B, C => D);
impl_route_path_tuple!(A, B, C, D => E);
impl_route_path_tuple!(A, B, C, D, E => F);
impl_route_path_tuple!(A, B, C, D, E, F => G);
impl_route_path_tuple!(A, B, C, D, E, F, G => H);
impl_route_path_tuple!(A, B, C, D, E, F, G, H => I);
impl_route_path_tuple!(A, B, C, D, E, F, G, H, I => J);
impl_route_path_tuple!(A, B, C, D, E, F, G, H, I, J => K);
impl_route_path_tuple!(A, B, C, D, E, F, G, H, I, J, K => L);
impl_route_path_tuple!(A, B, C, D, E, F, G, H, I, J, K, L => M);
impl_route_path_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M => N);
impl_route_path_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N => O);
impl_route_path_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O => P);
impl_route_path_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P => Q);
impl_route_path_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q => R);
impl_route_path_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R => S);
impl_route_path_tuple!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S => T
);
impl_route_path_tuple!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T => U
);
impl_route_path_tuple!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U => V
);
impl_route_path_tuple!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V => W
);
impl_route_path_tuple!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W => X
);
impl_route_path_tuple!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X => Y
);
impl_route_path_tuple!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y =>
    Z
);

#[cfg(test)]
mod tests {
    use super::{RoutePath, StaticSegment};
    use crate::route::{ParamSegment, PathMatch};

    #[test]
    pub fn should_match_one_static() {
        let path = StaticSegment("foo");
        assert!(path.exhaustive_match("/foo").is_some());
    }

    #[test]
    pub fn should_match_multiple_static() {
        let path = (StaticSegment("foo"), StaticSegment("bar"));
        assert!(path.exhaustive_match("/foo/bar").is_some());
    }

    #[test]
    pub fn should_not_match_if_needs_more_segments() {
        let path = (StaticSegment("foo"), StaticSegment("bar"));
        assert!(path.exhaustive_match("/foo").is_none());
    }

    #[test]
    pub fn should_not_exhaustive_match_if_partial() {
        let path = StaticSegment("foo");
        assert!(path.exhaustive_match("/foo/bar").is_none());
    }

    #[test]
    pub fn trailing_unit_is_fine() {
        let path = (StaticSegment(""), ());
        assert!(path.exhaustive_match("/").is_some());
    }

    #[test]
    pub fn should_capture_params() {
        let path = (StaticSegment("foo"), ParamSegment::<i32>::new("bar"));
        let matched = path.exhaustive_match("/foo/42").unwrap();
        assert_eq!(matched.path.as_deref(), Some("/foo/42"));
        assert_eq!(matched.params.get("bar"), Some("42"));
    }

    #[test]
    pub fn should_not_match_if_params_wrong_type() {
        let path = (StaticSegment("foo"), ParamSegment::<i32>::new("bar"));
        let matched = path.exhaustive_match("/foo/bar");
        assert!(matched.is_none())
    }

    #[test]
    pub fn nesting_tuples_arbitrarily_works_fine() {
        let path = (
            (),
            ((), ()),
            StaticSegment("foo"),
            (),
            ((), ParamSegment::<i32>::new("bar"), ()),
        );
        let matched = path.exhaustive_match("/foo/42").unwrap();
        assert_eq!(matched.path.as_deref(), Some("/foo/42"));
        assert_eq!(matched.params.get("bar"), Some("42"));
    }
}
