use super::{route_match::RouteMatch, PartialPathMatch};

/// A set of route definitions that can be nested inside another route definition.
pub trait PossibleRoutes {
    fn choose<'a>(&self, path: &'a str) -> Option<PartialPathMatch<'a>>;
}

impl PossibleRoutes for () {
    fn choose<'a>(&self, _path: &'a str) -> Option<PartialPathMatch<'a>> {
        None
    }
}

impl<A> PossibleRoutes for (A,)
where
    A: RouteMatch,
{
    fn choose<'a>(&self, path: &'a str) -> Option<PartialPathMatch<'a>> {
        self.0.test(path)
    }
}

macro_rules! tuples {
    ($($ty:ident),*) => {
        impl<$($ty),*> PossibleRoutes for ($($ty,)*)
        where
			$($ty: RouteMatch),*,
        {
            fn choose<'a>(&self, path: &'a str) -> Option<PartialPathMatch<'a>>
            {
                paste::paste! {
					let ($([<$ty:lower>],)*) = &self;
                    $(if let Some(matched) = [<$ty:lower>].test(path) {
                        if matched.is_complete() {
                            return Some(matched);
                        }
                    })*
                    None
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
    use crate::route::{
        ParamSegment, PossibleRoutes, RouteMatch, StaticSegment,
    };

    #[test]
    fn mixture_of_static_and_params() {
        /* matches */
        let possible_routes = (
            (StaticSegment("")),
            (StaticSegment("about")),
            (StaticSegment("foo"), ParamSegment("id")),
            (
                StaticSegment("foo"),
                ParamSegment("id"),
                (StaticSegment("bar")),
            ),
        );
        let path = "/foo/123/bar";

        let matched = possible_routes.choose(path);
        panic!("{matched:#?}");
    }
}
