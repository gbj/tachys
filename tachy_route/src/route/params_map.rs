use std::vec::IntoIter;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ParamsMap(Vec<(&'static str, String)>);

impl ParamsMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn single(key: &'static str, value: impl ToString) -> Self {
        Self(vec![(key, value.to_string())])
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.0
            .iter()
            .find(|(k, _)| *k == key)
            .map(|(_, v)| v.as_str())
    }
}

impl IntoIterator for ParamsMap {
    type Item = (&'static str, String);
    type IntoIter = IntoIter<(&'static str, String)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<(&'static str, String)> for ParamsMap {
    fn from_iter<T: IntoIterator<Item = (&'static str, String)>>(
        iter: T,
    ) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl Extend<(&'static str, String)> for ParamsMap {
    fn extend<T: IntoIterator<Item = (&'static str, String)>>(
        &mut self,
        iter: T,
    ) {
        self.0.extend(iter);
    }
}
