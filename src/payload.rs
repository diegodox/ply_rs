use std::ops::{Deref, DerefMut};

use crate::PLYValue;

#[derive(Debug, Clone, PartialEq)]
pub struct Payload(pub(crate) Vec<PLYValue>);

impl Payload {
    pub fn new(v: Vec<PLYValue>) -> Self {
        Self(v)
    }
}

impl IntoIterator for Payload {
    type Item = PLYValue;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Payload {
    pub fn push_value(&mut self, v: PLYValue) {
        self.0.push(v);
    }
}

impl From<Vec<PLYValue>> for Payload {
    fn from(v: Vec<PLYValue>) -> Self {
        Self(v)
    }
}

impl Deref for Payload {
    type Target = [PLYValue];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Payload {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
