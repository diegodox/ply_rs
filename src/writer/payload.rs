use std::io::Write;

use crate::{Format, GenericElement, Payload};

pub(crate) fn write_element_payload<T: Write, P: WritePayload<T, Payload = Payload>>(
    element: &GenericElement<P>,
    writer: &mut T,
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

#[cfg(test)]
mod test {
    use std::io::BufWriter;

    use crate::writer::payload::write_element_payload;

    #[test]
    fn test_write_element_payload_ascii() {
        use crate::*;
        let mut writer = BufWriter::new(Vec::new());
        let element = GenericElement {
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
}

pub(crate) trait WritePayload<T: Write> {
    type Payload;

    fn write_payload_ascii(&self, payload: &Self::Payload, writer: &mut T) -> std::io::Result<()>;

    fn write_payload_be(&self, payload: &Self::Payload, writer: &mut T) -> std::io::Result<()>;

    fn write_payload_le(&self, payload: &Self::Payload, writer: &mut T) -> std::io::Result<()>;
}
