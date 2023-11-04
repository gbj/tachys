use super::{PartialPathMatch, RouteMatch};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ParamSegment(pub &'static str);

impl RouteMatch for ParamSegment {
    fn test<'a>(&self, path: &'a str) -> Option<PartialPathMatch<'a>> {
        let mut matched = String::new();
        let mut param_value = String::new();
        let mut test = path.chars();

        // match an initial /
        if let Some('/') = test.next() {
            matched.push('/');
        }
        for char in test {
            // when we get a closing /, stop matching
            if char == '/' {
                break;
            }
            // otherwise, push into the matched param
            else {
                matched.push(char);
                param_value.push(char);
            }
        }

        let next_index = matched.len();
        Some(PartialPathMatch::new(
            &path[next_index..],
            vec![(self.0, param_value)],
            matched,
        ))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct WildcardSegment(pub &'static str);

impl RouteMatch for WildcardSegment {
    fn test<'a>(&self, path: &'a str) -> Option<PartialPathMatch<'a>> {
        let mut matched = String::new();
        let mut param_value = String::new();
        let mut test = path.chars();

        // match an initial /
        if let Some('/') = test.next() {
            matched.push('/');
        }
        for char in test {
            matched.push(char);
            param_value.push(char);
        }

        let next_index = matched.len();
        Some(PartialPathMatch::new(
            &path[next_index..],
            vec![(self.0, param_value)],
            matched,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::RouteMatch;
    use crate::route2::{ParamSegment, StaticSegment, WildcardSegment};

    #[test]
    fn single_param_match() {
        let path = "/foo";
        let def = ParamSegment("a");
        let matched = def.test(path).expect("couldn't match route");
        assert_eq!(matched.matched(), "/foo");
        assert_eq!(matched.remaining(), "");
        assert_eq!(matched.params()[0], ("a", "foo".to_string()));
    }

    #[test]
    fn single_param_match_with_trailing_slash() {
        let path = "/foo/";
        let def = ParamSegment("a");
        let matched = def.test(path).expect("couldn't match route");
        assert_eq!(matched.matched(), "/foo");
        assert_eq!(matched.remaining(), "/");
        assert_eq!(matched.params()[0], ("a", "foo".to_string()));
    }

    #[test]
    fn tuple_of_param_matches() {
        let path = "/foo/bar";
        let def = (ParamSegment("a"), ParamSegment("b"));
        let matched = def.test(path).expect("couldn't match route");
        assert_eq!(matched.matched(), "/foo/bar");
        assert_eq!(matched.remaining(), "");
        assert_eq!(matched.params()[0], ("a", "foo".to_string()));
        assert_eq!(matched.params()[1], ("b", "bar".to_string()));
    }

    #[test]
    fn splat_should_match_all() {
        let path = "/foo/bar/////";
        let def = (
            StaticSegment("foo"),
            StaticSegment("bar"),
            WildcardSegment("rest"),
        );
        let matched = def.test(path).expect("couldn't match route");
        assert_eq!(matched.matched(), "/foo/bar/////");
        assert_eq!(matched.remaining(), "");
        assert_eq!(matched.params()[0], ("rest", "////".to_string()));
    }
}
