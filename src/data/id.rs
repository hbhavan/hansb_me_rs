use std::sync::Arc;

#[derive(Clone, PartialEq)]
pub struct Id(Arc<str>);

#[allow(dead_code)]
impl Id {
    pub fn new(id: &str) -> Self {
        Id(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
