use std::io::{BufWriter, Write};

use crate::{
    writer::{header::PlyWriteHeader, payload::write_element_payload},
    Comment, Element, Format,
};

pub const MAGIC_NUMBER: &str = "ply";
pub const END_HEADER: &str = "end_header";

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

    pub fn write<T: Write>(&self, writer: &mut BufWriter<T>) -> std::io::Result<()> {
        self.write_header(writer)?;
        for element in self.elements.iter() {
            match element {
                Element::Element { elements: e, .. } => {
                    write_element_payload(e, writer, &self.format)?
                }
                Element::ListElement { elements: e, .. } => {
                    write_element_payload(e, writer, &self.format)?
                }
            };
        }
        Ok(())
    }
}

impl<T: Write> PlyWriteHeader<T> for PLYFile {
    fn write_header(&self, writer: &mut BufWriter<T>) -> std::io::Result<()> {
        writeln!(writer, "{MAGIC_NUMBER}")?;
        self.format.write_header(writer)?;
        for comment in self.comments.iter() {
            comment.write_header(writer)?;
        }
        for element in self.elements.iter() {
            element.write_header(writer)?
        }
        writeln!(writer, "{END_HEADER}")?;
        Ok(())
    }
}

#[test]
fn test_write_ply() {
    use crate::*;
    let ply = {
        let element_vertex = Element::Element {
            name: "vertex".to_string(),
            elements: GenericElement {
                count: 8,
                props: Property {
                    props: vec![
                        PLYValueTypeName::Float,
                        PLYValueTypeName::Float,
                        PLYValueTypeName::Float,
                    ],
                    names: vec!["x".to_string(), "y".to_string(), "z".to_string()],
                },
                payloads: vec![
                    Payload::new(vec![
                        PLYValue::Float(0f32),
                        PLYValue::Float(0f32),
                        PLYValue::Float(0f32),
                    ]),
                    Payload::new(vec![
                        PLYValue::Float(0f32),
                        PLYValue::Float(0f32),
                        PLYValue::Float(1f32),
                    ]),
                    Payload::new(vec![
                        PLYValue::Float(0f32),
                        PLYValue::Float(1f32),
                        PLYValue::Float(1f32),
                    ]),
                    Payload::new(vec![
                        PLYValue::Float(0f32),
                        PLYValue::Float(1f32),
                        PLYValue::Float(0f32),
                    ]),
                    Payload::new(vec![
                        PLYValue::Float(1f32),
                        PLYValue::Float(0f32),
                        PLYValue::Float(0f32),
                    ]),
                    Payload::new(vec![
                        PLYValue::Float(1f32),
                        PLYValue::Float(0f32),
                        PLYValue::Float(1f32),
                    ]),
                    Payload::new(vec![
                        PLYValue::Float(1f32),
                        PLYValue::Float(1f32),
                        PLYValue::Float(1f32),
                    ]),
                    Payload::new(vec![
                        PLYValue::Float(1f32),
                        PLYValue::Float(1f32),
                        PLYValue::Float(0f32),
                    ]),
                ],
            },
        };
        let element_list = Element::ListElement {
            name: "list".to_string(),
            elements: GenericElement {
                count: 3,
                props: PropertyList {
                    count: PLYValueTypeName::Uchar,
                    prop: PLYValueTypeName::Char,
                    name: "vertex_id".to_string(),
                },
                payloads: vec![
                    Payload::new(vec![PLYValue::Char(3)]),
                    Payload::new(vec![PLYValue::Char(3), PLYValue::Char(3)]),
                    Payload::new(vec![
                        PLYValue::Char(3),
                        PLYValue::Char(3),
                        PLYValue::Char(3),
                    ]),
                ],
            },
        };
        PLYFile {
            format: Format::Ascii {
                version: "1.0".to_string(),
            },
            comments: vec![Comment::new(vec!["test".to_string(), "data".to_string()])],
            elements: vec![element_vertex, element_list],
        }
    };
    let mut writer = BufWriter::new(Vec::new());
    ply.write(&mut writer).unwrap();
    assert_eq!(
        "\
ply
format ascii 1.0
comment test data
element vertex 8
property float x
property float y
property float z
element list 3
property list uchar char vertex_id
end_header
0 0 0
0 0 1
0 1 1
0 1 0
1 0 0
1 0 1
1 1 1
1 1 0
1 3
2 3 3
3 3 3 3
",
        &String::from_utf8(writer.into_inner().unwrap()).unwrap()
    )
}
