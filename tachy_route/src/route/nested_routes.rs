use super::PathMatch;

pub trait RouteChild {
    fn matches_path(&self, path: &str) -> bool {
        let mut path = path.split('/').filter(|n| !n.is_empty());
        self.matches(&mut path)
    }

    fn test_path(&self, path: &str) -> Option<PathMatch> {
        let mut path = path.split('/').filter(|n| !n.is_empty());
        self.test(&mut path)
    }

    fn matches<'a, I>(&self, path: &mut I) -> bool
    where
        I: Iterator<Item = &'a str> + Clone;

    fn test<'a, I>(&self, path: &mut I) -> Option<PathMatch>
    where
        I: Iterator<Item = &'a str> + Clone;
}

impl RouteChild for () {
    fn matches<'a, I>(&self, path: &mut I) -> bool
    where
        I: Iterator<Item = &'a str> + Clone,
    {
        true
    }

    fn test<'a, I>(&self, path: &mut I) -> Option<PathMatch>
    where
        I: Iterator<Item = &'a str> + Clone,
    {
        Some(Default::default())
    }
}

impl<A> RouteChild for (A,)
where
    A: RouteChild,
{
    fn matches<'a, I>(&self, path: &mut I) -> bool
    where
        I: Iterator<Item = &'a str> + Clone,
    {
        self.0.matches(path)
    }

    fn test<'a, I>(&self, path: &mut I) -> Option<PathMatch>
    where
        I: Iterator<Item = &'a str> + Clone,
    {
        self.0.test(path)
    }
}

macro_rules! impl_route_child_for_tuples {
    ($($ty:ident),*) => {
        impl<$($ty),*> RouteChild for ($($ty,)*)
        where
			$($ty: RouteChild),*,
        {
            fn matches<'a, It>(&self, path: &mut It) -> bool
            where
                It: Iterator<Item = &'a str> + Clone,
            {
                paste::paste! {
					let ($([<$ty:lower>],)*) = &self;
                    $([<$ty:lower>].matches(&mut path.clone()) ||)*
                    false
                }
            }

            fn test<'a, It>(&self, path: &mut It) -> Option<PathMatch>
            where
                It: Iterator<Item = &'a str> + Clone,
            {
                paste::paste! {
					let ($([<$ty:lower>],)*) = &self;
                    $(if let Some(matched) = [<$ty:lower>].test(&mut path.clone()) {
                        return Some(matched);
                    })*
                    None
                }
            }
        }
	};
}

impl_route_child_for_tuples!(A, B);
impl_route_child_for_tuples!(A, B, C);
impl_route_child_for_tuples!(A, B, C, D);
impl_route_child_for_tuples!(A, B, C, D, E);
impl_route_child_for_tuples!(A, B, C, D, E, F);
impl_route_child_for_tuples!(A, B, C, D, E, F, G);
impl_route_child_for_tuples!(A, B, C, D, E, F, G, H);
impl_route_child_for_tuples!(A, B, C, D, E, F, G, H, I);
impl_route_child_for_tuples!(A, B, C, D, E, F, G, H, I, J);
impl_route_child_for_tuples!(A, B, C, D, E, F, G, H, I, J, K);
impl_route_child_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_route_child_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_route_child_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_route_child_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
impl_route_child_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
impl_route_child_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
impl_route_child_for_tuples!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R
);
impl_route_child_for_tuples!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S
);
impl_route_child_for_tuples!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T
);
impl_route_child_for_tuples!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U
);
impl_route_child_for_tuples!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V
);
impl_route_child_for_tuples!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W
);
impl_route_child_for_tuples!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X
);
impl_route_child_for_tuples!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y
);
impl_route_child_for_tuples!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y,
    Z
);
