use std::io::{BufWriter, Write};

use crate::{
    ply_value::PlyTryFrom, Format, GenericElement, PLYValue, Payload, Property, PropertyList,
};

pub(crate) fn write_element_payload<T: Write, P: WritePayload<T, Payload = Payload>>(
    element: &GenericElement<P>,
    writer: &mut BufWriter<T>,
    format: &Format,
) -> std::io::Result<()> {
    match format {
        Format::Ascii { .. } => {
            for payload in element.payload() {
                element.property().write_payload_ascii(payload, writer)?;
            }
        }

        Format::BinaryBigEndian { .. } => {
            for payload in element.payload() {
                element.property().write_payload_be(payload, writer)?;
            }
        }
        Format::BinaryLittleEndian { .. } => {
            for payload in element.payload() {
                element.property().write_payload_le(payload, writer)?;
            }
        }
    };
    Ok(())
}
#[test]
fn test_write_element_payload_ascii() {
    use crate::*;
    let mut writer = BufWriter::new(Vec::new());
    let element = GenericElement {
        name: "vertex".to_string(),
        count: 8,
        props: Property {
            props: vec![
                PLYValueTypeName::Uchar,
                PLYValueTypeName::Uchar,
                PLYValueTypeName::Uchar,
            ],
            names: vec!["red".to_string(), "green".to_string(), "blue".to_string()],
        },
        payloads: vec![
            Payload(vec![
                PLYValue::Uchar(0),
                PLYValue::Uchar(0),
                PLYValue::Uchar(0),
            ]),
            Payload(vec![
                PLYValue::Uchar(0),
                PLYValue::Uchar(0),
                PLYValue::Uchar(1),
            ]),
            Payload(vec![
                PLYValue::Uchar(0),
                PLYValue::Uchar(1),
                PLYValue::Uchar(1),
            ]),
            Payload(vec![
                PLYValue::Uchar(0),
                PLYValue::Uchar(1),
                PLYValue::Uchar(0),
            ]),
            Payload(vec![
                PLYValue::Uchar(1),
                PLYValue::Uchar(0),
                PLYValue::Uchar(0),
            ]),
            Payload(vec![
                PLYValue::Uchar(1),
                PLYValue::Uchar(0),
                PLYValue::Uchar(1),
            ]),
            Payload(vec![
                PLYValue::Uchar(1),
                PLYValue::Uchar(1),
                PLYValue::Uchar(1),
            ]),
            Payload(vec![
                PLYValue::Uchar(1),
                PLYValue::Uchar(1),
                PLYValue::Uchar(0),
            ]),
        ],
    };
    write_element_payload(
        &element,
        &mut writer,
        &Format::Ascii {
            version: "1.0".to_string(),
        },
    )
    .unwrap();
    assert_eq!(
        "\
0 0 0
0 0 1
0 1 1
0 1 0
1 0 0
1 0 1
1 1 1
1 1 0
"
        .as_bytes()
        .to_vec(),
        writer.into_inner().unwrap(),
    );
}

pub(crate) trait WritePayload<T: Write> {
    type Payload;

    fn write_payload_ascii(
        &self,
        payload: &Self::Payload,
        writer: &mut BufWriter<T>,
    ) -> std::io::Result<()>;
    fn write_payload_be(
        &self,
        payload: &Self::Payload,
        writer: &mut BufWriter<T>,
    ) -> std::io::Result<()>;
    fn write_payload_le(
        &self,
        payload: &Self::Payload,
        writer: &mut BufWriter<T>,
    ) -> std::io::Result<()>;
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
            .map(|v| format!("{}", v))
            .collect::<Vec<_>>()
            .join(" ");
        writeln!(writer, "{}", line)
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

impl<T: Write> WritePayload<T> for PropertyList {
    type Payload = Payload;

    fn write_payload_ascii(
        &self,
        payload: &Payload,
        writer: &mut BufWriter<T>,
    ) -> std::io::Result<()> {
        let line = payload
            .iter()
            .map(|v| format!("{}", v))
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
