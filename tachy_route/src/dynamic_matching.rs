#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathDefinition {
    segments: Vec<PathDefinitionSegment>,
    has_splat: bool,
}

impl PathDefinition {
    pub fn from_segments(
        segments: impl IntoIterator<Item = PathDefinitionSegment>,
    ) -> Self {
        let itr = segments.into_iter();
        let mut has_splat = false;
        let mut segments = Vec::with_capacity(itr.size_hint().0);
        for segment in itr {
            if matches!(segment, PathDefinitionSegment::Splat { .. }) {
                has_splat = true;
            }
            segments.push(segment);
        }
        Self {
            segments,
            has_splat,
        }
    }

    pub(crate) fn score(&self) -> Score {
        let initial_score = self.segments.len() as i32;
        let initial_score = initial_score - if self.has_splat { 1 } else { 0 };
        Score(self.segments.iter().rev().fold(
            initial_score,
            |score, segment| {
                score
                    + if matches!(segment, PathDefinitionSegment::Param { .. })
                    {
                        2
                    } else {
                        3
                    }
            },
        ))
    }
}

impl<T> From<T> for PathDefinition
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        let mut has_splat = false;
        let segments = value
            .as_ref()
            .split('/')
            .filter(|n| !n.is_empty())
            .map(|segment| {
                if let Some(field_name) = segment.strip_prefix(':') {
                    PathDefinitionSegment::Param {
                        field_name: field_name.to_string(),
                    }
                } else if let Some(field_name) = segment.strip_prefix('*') {
                    has_splat = true;
                    PathDefinitionSegment::Splat {
                        field_name: field_name.to_string(),
                    }
                } else {
                    PathDefinitionSegment::Static {
                        path: segment.to_string(),
                    }
                }
            })
            .collect::<Vec<_>>();
        #[cfg(debug_assertions)]
        {
            let splat = segments
                .iter()
                .enumerate()
                .filter_map(|(idx, seg)| {
                    if matches!(seg, PathDefinitionSegment::Splat { .. }) {
                        Some(idx)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            if splat.len() > 1 {
                panic!(
                    "Route definitions should contain only a single \
                     splat/wildcard param (/*foo)."
                );
            } else if splat.len() == 1 && splat[0] != (segments.len() - 1) {
                panic!(
                    "Splat/wildcard params (/*foo) should come at the end of \
                     the route."
                )
            }
        }

        Self {
            segments,
            has_splat,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PathDefinitionSegment {
    Static { path: String },
    Param { field_name: String },
    Splat { field_name: String },
}

#[cfg(test)]
mod tests {
    use super::{PathDefinition, RouteDefinition};
    use crate::route::PathDefinitionSegment;
    use std::marker::PhantomData;
    use tachydom::renderer::mock_dom::MockDom;

    #[test]
    fn path_definition_should_convert_from_string() {
        let path = PathDefinition::from("/blog/:id/comments/*rest");
        assert_eq!(
            path,
            PathDefinition::from_segments([
                PathDefinitionSegment::Static {
                    path: "blog".to_string()
                },
                PathDefinitionSegment::Param {
                    field_name: "id".to_string()
                },
                PathDefinitionSegment::Static {
                    path: "comments".to_string()
                },
                PathDefinitionSegment::Splat {
                    field_name: "rest".to_string()
                }
            ])
        )
    }

    #[test]
    #[should_panic]
    fn path_definition_should_panic_if_splat_not_last() {
        _ = PathDefinition::from("/blog/:id/*rest/comments");
    }

    #[test]
    #[should_panic]
    fn path_definition_should_panic_if_multiple_splats() {
        _ = PathDefinition::from("/blog/:id/*foo/*bar");
    }

    #[test]
    fn build_test_route() {
        let route = RouteDefinition {
            path: [],
            children: (),
            view: || (),
            rndr: PhantomData::<MockDom>,
        };
    }
}
