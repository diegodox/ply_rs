//! # ply
//!
//! [PLY (Polygon File Format)](http://paulbourke.net/dataformats/ply/) file parser for Rust
//!
use std::{
    convert::TryInto,
    fmt::{Debug, Display},
    ops::Deref,
};

pub mod error;
use error::{PLYError, PLYResult};

pub(crate) mod ply_value;
pub use ply_value::{PLYValue, PLYValueTypeName};

pub(crate) mod reader;
pub(crate) mod writer;

#[derive(Debug, Clone, PartialEq)]
/// Struct represent PLY File
pub struct PLYFile {
    pub format: Format,
    pub comments: Vec<Comment>,
    pub elements: Vec<Element>,
}

impl PLYFile {
    pub fn new(format: Format) -> Self {
        Self {
            format,
            comments: Vec::new(),
            elements: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Format of PLY file
pub enum Format {
    Ascii { version: String },
    BinaryBigEndian { version: String },
    BinaryLittleEndian { version: String },
}
impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Format::Ascii { version } => write!(f, "format ascii {}", version),
            Format::BinaryBigEndian { version } => {
                write!(f, "format binary_big_endian {}", version)
            }
            Format::BinaryLittleEndian { version } => {
                write!(f, "format binary_little_endian {}", version)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
/// Struct represent Comment
pub struct Comment(Vec<String>);
impl Comment {
    pub fn new<S: Into<String>>(comment: S) -> Comment {
        Comment(
            comment
                .into()
                .split_whitespace()
                .map(|v| v.to_string())
                .collect(),
        )
    }
}
impl Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "comment {}", self.0.join(" "))
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Enum represent PLY Element
pub enum Element {
    Element(GenericElement<Property>),
    ListElement(GenericElement<PropertyList>),
}
impl TryInto<GenericElement<Property>> for Element {
    type Error = PLYError;

    fn try_into(self) -> Result<GenericElement<Property>, Self::Error> {
        match self {
            Element::Element(e) => Ok(e),
            Element::ListElement(_) => Err(PLYError::MissmatchDataType),
        }
    }
}
impl TryInto<GenericElement<PropertyList>> for Element {
    type Error = PLYError;

    fn try_into(self) -> Result<GenericElement<PropertyList>, Self::Error> {
        match self {
            Element::ListElement(e) => Ok(e),
            Element::Element(_) => Err(PLYError::MissmatchDataType),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Struct represent Generic PLY Element
pub struct GenericElement<P> {
    pub name: String,
    count: usize,
    props: P,
    payloads: Vec<Payload>,
}
impl<P> GenericElement<P> {
    pub fn new<S: Into<String>>(name: S, property: P) -> GenericElement<P> {
        Self {
            name: name.into(),
            count: 0,
            props: property,
            payloads: Vec::new(),
        }
    }
    pub fn property(&self) -> &P {
        &self.props
    }
    pub fn property_mut(&mut self) -> &mut P {
        &mut self.props
    }
    pub fn payload(&self) -> &[Payload] {
        &self.payloads
    }
    pub fn payload_mut(&mut self) -> &mut [Payload] {
        &mut self.payloads
    }
}
impl GenericElement<Property> {
    pub fn push_payload(&mut self, payload: Payload) -> PLYResult<()> {
        if self.property().len() != payload.len() {
            return Err(PLYError::PropertyLengthErr);
        }
        if !payload
            .iter()
            .zip(self.property().props.iter())
            .all(|(v, t)| v.value_type() == *t)
        {
            return Err(PLYError::MissmatchDataType);
        }

        self.count += 1;
        self.payloads.push(payload);
        Ok(())
    }
}
#[test]
fn test_push_payload() {
    let mut element = {
        let property = {
            let mut props = Property::new();
            props.push_prop("x", PLYValueTypeName::Float);
            props.push_prop("y", PLYValueTypeName::Float);
            props.push_prop("z", PLYValueTypeName::Float);
            props
        };
        GenericElement::new("test_element", property)
    };

    let result = element.push_payload(Payload(vec![
        PLYValue::Float(1f32),
        PLYValue::Float(2f32),
        PLYValue::Float(3f32),
    ]));
    assert!(result.is_ok());
    assert!(element.count == 1);

    let result = element.push_payload(Payload(vec![
        PLYValue::Double(1f64),
        PLYValue::Double(2f64),
        PLYValue::Double(3f64),
    ]));
    assert_eq!(result, Err(PLYError::MissmatchDataType));
    assert!(element.count == 1);

    let result = element.push_payload(Payload(vec![
        PLYValue::Float(1f32),
        PLYValue::Float(2f32),
        PLYValue::Float(3f32),
        PLYValue::Float(4f32),
    ]));
    assert_eq!(result, Err(PLYError::PropertyLengthErr));
    assert!(element.count == 1);
}

impl GenericElement<PropertyList> {
    pub fn push_payload(&mut self, payload: Payload) -> PLYResult<()> {
        if !(payload
            .iter()
            .all(|v| v.value_type() == self.property().prop))
        {
            return Err(PLYError::MissmatchDataType);
        }

        self.count += 1;
        self.payloads.push(payload);
        Ok(())
    }
}
#[test]
fn test_push_list_payload() {
    let mut element = GenericElement::new(
        "test_element",
        PropertyList::new(
            "list_name",
            PLYValueTypeName::Uchar,
            PLYValueTypeName::Float,
        ),
    );

    let result = element.push_payload(Payload(vec![
        PLYValue::Float(1f32),
        PLYValue::Float(2f32),
        PLYValue::Float(3f32),
    ]));
    assert!(result.is_ok());
    assert!(element.count == 1);

    let result = element.push_payload(Payload(vec![
        PLYValue::Float(1f32),
        PLYValue::Float(2f32),
        PLYValue::Float(3f32),
        PLYValue::Float(4f32),
    ]));
    assert!(result.is_ok());
    assert!(element.count == 2);

    let result = element.push_payload(Payload(vec![
        PLYValue::Double(1f64),
        PLYValue::Double(2f64),
        PLYValue::Double(3f64),
    ]));
    assert_eq!(result, Err(PLYError::MissmatchDataType));
    assert!(element.count == 2);
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
/// property "prop" "name"
pub struct Property {
    pub(crate) props: Vec<PLYValueTypeName>,
    pub(crate) names: Vec<String>,
}
impl Property {
    pub fn new() -> Property {
        Self::default()
    }
    pub fn push_prop<S: Into<String>>(&mut self, name: S, property: PLYValueTypeName) {
        self.props.push(property);
        self.names.push(name.into());
    }
    pub fn is_empty(&self) -> bool {
        debug_assert_eq!(self.props.is_empty(), self.names.is_empty());
        self.props.is_empty()
    }
    pub fn len(&self) -> usize {
        debug_assert_eq!(self.props.len(), self.names.len());
        self.props.len()
    }
    /// Iterator over element property (name, prop)
    pub fn iter(&self) -> impl Iterator<Item = (&str, PLYValueTypeName)> {
        self.names
            .iter()
            .map(|x| x.as_str())
            .zip(self.props.iter().copied())
    }
    /// Iterator over element property (name, prop)
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&mut str, &mut PLYValueTypeName)> {
        self.names
            .iter_mut()
            .map(|x| x.as_mut_str())
            .zip(self.props.iter_mut())
    }
}
impl<S: Into<String>> From<Vec<(S, PLYValueTypeName)>> for Property {
    fn from(v: Vec<(S, PLYValueTypeName)>) -> Self {
        let (names, props) = v.into_iter().map(|(s, p)| (s.into(), p)).unzip();
        Self { names, props }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// property list "length-type" "prop-type" "name"
pub struct PropertyList {
    pub(crate) count: PLYValueTypeName,
    pub(crate) prop: PLYValueTypeName,
    pub(crate) name: String,
}
impl PropertyList {
    pub fn new<S: Into<String>>(
        name: S,
        count: PLYValueTypeName,
        prop: PLYValueTypeName,
    ) -> PropertyList {
        Self {
            count,
            prop,
            name: name.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Payload(Vec<PLYValue>);
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
