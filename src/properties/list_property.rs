use std::io::{BufWriter, Write};

use crate::{
    payload::Payload,
    ply_value::PlyTryFrom,
    writer::{header::PlyWriteHeader, payload::WritePayload},
    PLYValue, PLYValueTypeName,
};

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

#[cfg(test)]
mod test {
    use crate::{
        error::PLYError, GenericElement, PLYValue, PLYValueTypeName, Payload, PropertyList,
    };

    #[test]
    fn test_push_list_payload() {
        let mut element = GenericElement::new(PropertyList::new(
            "list_name",
            PLYValueTypeName::Uchar,
            PLYValueTypeName::Float,
        ));

        let result = element.push_payload(Payload::new(vec![
            PLYValue::Float(1f32),
            PLYValue::Float(2f32),
            PLYValue::Float(3f32),
        ]));
        assert!(result.is_ok());
        assert!(element.count() == 1);

        let result = element.push_payload(Payload::new(vec![
            PLYValue::Float(1f32),
            PLYValue::Float(2f32),
            PLYValue::Float(3f32),
            PLYValue::Float(4f32),
        ]));
        assert!(result.is_ok());
        assert!(element.count() == 2);

        let result = element.push_payload(Payload::new(vec![
            PLYValue::Double(1f64),
            PLYValue::Double(2f64),
            PLYValue::Double(3f64),
        ]));
        assert_eq!(result, Err(PLYError::MissmatchDataType));
        assert!(element.count() == 2);
    }
}

impl<T: Write> PlyWriteHeader<T> for PropertyList {
    fn write_header(&self, writer: &mut BufWriter<T>) -> std::io::Result<()> {
        writeln!(
            writer,
            "property list {} {} {}",
            self.count.to_str(),
            self.prop.to_str(),
            self.name
        )
    }
}

#[test]
fn test_write_property_list() {
    use crate::*;
    let mut writer = BufWriter::new(Vec::new());
    let property = PropertyList {
        name: "vertex".to_string(),
        count: PLYValueTypeName::Uchar,
        prop: PLYValueTypeName::Float,
    };
    property.write_header(&mut writer).unwrap();
    assert_eq!(
        writer.into_inner().unwrap(),
        "property list uchar float vertex\n".as_bytes(),
    )
}

impl<T: Write> WritePayload<T> for PropertyList {
    type Payload = Payload;

    fn write_payload_ascii(
        &self,
        payload: &Payload,
        writer: &mut BufWriter<T>,
    ) -> std::io::Result<()> {
        let line = payload
            .iter()
            .map(|v| format!("{v}"))
            .collect::<Vec<_>>()
            .join(" ");
        writeln!(writer, "{} {}", payload.len(), line)
    }

    fn write_payload_be(
        &self,
        payload: &Payload,
        writer: &mut BufWriter<T>,
    ) -> std::io::Result<()> {
        match self.count.try_from(payload.len()).unwrap() {
            PLYValue::Char(v) => writer.write(&v.to_be_bytes())?,
            PLYValue::Uchar(v) => writer.write(&v.to_be_bytes())?,
            PLYValue::Short(v) => writer.write(&v.to_be_bytes())?,
            PLYValue::Ushort(v) => writer.write(&v.to_be_bytes())?,
            PLYValue::Int(v) => writer.write(&v.to_be_bytes())?,
            PLYValue::Uint(v) => writer.write(&v.to_be_bytes())?,
            PLYValue::Float(v) => writer.write(&v.to_be_bytes())?,
            PLYValue::Double(v) => writer.write(&v.to_be_bytes())?,
        };
        for v in payload.iter() {
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
        payload: &Self::Payload,
        writer: &mut BufWriter<T>,
    ) -> std::io::Result<()> {
        match self.count.try_from(payload.len()).unwrap() {
            PLYValue::Char(v) => writer.write(&v.to_le_bytes())?,
            PLYValue::Uchar(v) => writer.write(&v.to_le_bytes())?,
            PLYValue::Short(v) => writer.write(&v.to_le_bytes())?,
            PLYValue::Ushort(v) => writer.write(&v.to_le_bytes())?,
            PLYValue::Int(v) => writer.write(&v.to_le_bytes())?,
            PLYValue::Uint(v) => writer.write(&v.to_le_bytes())?,
            PLYValue::Float(v) => writer.write(&v.to_le_bytes())?,
            PLYValue::Double(v) => writer.write(&v.to_le_bytes())?,
        };
        for v in payload.iter() {
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
