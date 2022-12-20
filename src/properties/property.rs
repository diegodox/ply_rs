use std::io::{BufWriter, Write};

use crate::{
    payload::Payload,
    writer::{header::PlyWriteHeader, payload::WritePayload},
    PLYValue, PLYValueTypeName,
};

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

#[cfg(test)]
mod test {
    use crate::{error::PLYError, GenericElement, PLYValue, PLYValueTypeName, Payload, Property};

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
            GenericElement::new(property)
        };

        let result = element.push_payload(Payload::new(vec![
            PLYValue::Float(1f32),
            PLYValue::Float(2f32),
            PLYValue::Float(3f32),
        ]));
        assert!(result.is_ok());
        assert!(element.count() == 1);

        let result = element.push_payload(Payload::new(vec![
            PLYValue::Double(1f64),
            PLYValue::Double(2f64),
            PLYValue::Double(3f64),
        ]));
        assert_eq!(result, Err(PLYError::MissmatchDataType));
        assert!(element.count() == 1);

        let result = element.push_payload(Payload::new(vec![
            PLYValue::Float(1f32),
            PLYValue::Float(2f32),
            PLYValue::Float(3f32),
            PLYValue::Float(4f32),
        ]));
        assert_eq!(result, Err(PLYError::PropertyLengthErr));
        assert!(element.count() == 1);
    }
}

impl<T: Write> PlyWriteHeader<T> for Property {
    fn write_header(&self, writer: &mut BufWriter<T>) -> std::io::Result<()> {
        for (name, ply_type) in self.iter() {
            writeln!(writer, "property {} {}", ply_type.to_str(), name)?
        }
        Ok(())
    }
}

#[test]
fn test_write_property() {
    use crate::*;
    let mut writer = BufWriter::new(Vec::new());
    let property = Property {
        props: vec![
            PLYValueTypeName::Float,
            PLYValueTypeName::Float,
            PLYValueTypeName::Float,
            PLYValueTypeName::Uchar,
            PLYValueTypeName::Uchar,
            PLYValueTypeName::Uchar,
        ],
        names: vec![
            "x".to_string(),
            "y".to_string(),
            "z".to_string(),
            "red".to_string(),
            "green".to_string(),
            "blue".to_string(),
        ],
    };
    property.write_header(&mut writer).unwrap();
    assert_eq!(
        writer.into_inner().unwrap(),
        "\
property float x
property float y
property float z
property uchar red
property uchar green
property uchar blue
"
        .as_bytes(),
    )
}

impl<T: Write> WritePayload<T> for Property {
    type Payload = Payload;

    fn write_payload_ascii(
        &self,
        payload: &Payload,
        writer: &mut BufWriter<T>,
    ) -> std::io::Result<()> {
        let line = payload
            .0
            .iter()
            .map(|v| format!("{v}"))
            .collect::<Vec<_>>()
            .join(" ");
        writeln!(writer, "{line}")
    }

    fn write_payload_be(
        &self,
        payload: &Payload,
        writer: &mut BufWriter<T>,
    ) -> std::io::Result<()> {
        for v in payload.0.iter() {
            match v {
                PLYValue::Char(v) => writer.write(&v.to_be_bytes())?,
                PLYValue::Uchar(v) => writer.write(&v.to_be_bytes())?,
                PLYValue::Short(v) => writer.write(&v.to_be_bytes())?,
                PLYValue::Ushort(v) => writer.write(&v.to_be_bytes())?,
                PLYValue::Int(v) => writer.write(&v.to_be_bytes())?,
                PLYValue::Uint(v) => writer.write(&v.to_be_bytes())?,
                PLYValue::Float(v) => writer.write(&v.to_be_bytes())?,
                PLYValue::Double(v) => writer.write(&v.to_be_bytes())?,
            };
        }
        Ok(())
    }

    fn write_payload_le(
        &self,
        payload: &Payload,
        writer: &mut BufWriter<T>,
    ) -> std::io::Result<()> {
        for v in payload.0.iter() {
            match v {
                PLYValue::Char(v) => writer.write(&v.to_le_bytes())?,
                PLYValue::Uchar(v) => writer.write(&v.to_le_bytes())?,
                PLYValue::Short(v) => writer.write(&v.to_le_bytes())?,
                PLYValue::Ushort(v) => writer.write(&v.to_le_bytes())?,
                PLYValue::Int(v) => writer.write(&v.to_le_bytes())?,
                PLYValue::Uint(v) => writer.write(&v.to_le_bytes())?,
                PLYValue::Float(v) => writer.write(&v.to_le_bytes())?,
                PLYValue::Double(v) => writer.write(&v.to_le_bytes())?,
            };
        }
        Ok(())
    }
}
