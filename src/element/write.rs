use std::io::Write;

use crate::{writer::header::PlyWriteHeader, Element};

impl<T: Write> PlyWriteHeader<T> for Element {
    fn write_header(&self, writer: &mut T) -> std::io::Result<()> {
        match self {
            Element::Element { name, elements } => {
                writeln!(writer, "element {} {}", name, elements.count())?;
                elements.property().write_header(writer)
            }

            Element::ListElement { name, elements } => {
                writeln!(writer, "element {} {}", name, elements.count())?;
                elements.property().write_header(writer)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::io::BufWriter;

    use crate::writer::header::PlyWriteHeader;

    #[test]
    fn test_write_element_header() {
        use crate::*;
        let mut writer = BufWriter::new(Vec::new());
        let element = Element::Element {
            name: "vertex".to_string(),
            elements: GenericElement {
                count: 20,
                props: Property {
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
                },
                payloads: Vec::<Payload>::with_capacity(20),
            },
        };
        element.write_header(&mut writer).unwrap();

        assert_eq!(
            writer.into_inner().unwrap(),
            "\
element vertex 20
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
}
