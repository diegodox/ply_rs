use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use crate::PLYFile;

pub(crate) mod header;
use header::{from_header_lines, read_header_lines};

pub(crate) mod payload;
use payload::{
    read_element_payload_be_bytes, read_element_payload_le_bytes, read_elemet_payload_ascii,
};

impl PLYFile {
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<PLYFile> {
        let mut lines = BufReader::new(File::open(path).unwrap())
            .lines()
            .map(|e| e.unwrap());
        Ok(PLYFile::from_lines(&mut lines))
    }
    pub fn from_lines<I: Iterator<Item = String>>(lines: &mut I) -> PLYFile {
        let header_lines = read_header_lines(lines);
        let mut ply = {
            let mut iter = header_lines.into_iter();
            from_header_lines(&mut iter)
        };
        match ply.format {
            crate::Format::Ascii { .. } => {
                for element in &mut ply.elements {
                    match element {
                        crate::Element::Element(element) => {
                            read_elemet_payload_ascii(element, lines)
                        }
                        crate::Element::ListElement(element) => {
                            read_elemet_payload_ascii(element, lines)
                        }
                    }
                }
            }
            crate::Format::BinaryBigEndian { .. } => {
                let s = lines.next().unwrap();
                let mut bytes = s.as_bytes().iter().copied();
                for element in &mut ply.elements {
                    match element {
                        crate::Element::Element(element) => {
                            read_element_payload_be_bytes(element, &mut bytes)
                        }
                        crate::Element::ListElement(element) => {
                            read_element_payload_be_bytes(element, &mut bytes)
                        }
                    }
                }
            }
            crate::Format::BinaryLittleEndian { .. } => {
                let s = lines.next().unwrap();
                let mut bytes = s.as_bytes().iter().copied();
                for element in &mut ply.elements {
                    match element {
                        crate::Element::Element(element) => {
                            read_element_payload_le_bytes(element, &mut bytes)
                        }
                        crate::Element::ListElement(element) => {
                            read_element_payload_le_bytes(element, &mut bytes)
                        }
                    }
                }
            }
        }
        ply
    }
}

#[test]
fn test_read_ply_file_ascii() {
    use crate::*;
    let data = r#"ply
format ascii 1.0
comment test data
element vertex 3
property float x
property float y
property float z
element face 3
property list uchar uint vertex_list
end_header
0 0 0
0 0 1
0 1 1
1 1
2 1 2
3 1 2 3
"#;
    let mut lines = data.lines().map(|s| s.to_string());
    let ply = PLYFile::from_lines(&mut lines);
    assert_eq!(
        ply,
        PLYFile {
            format: Format::Ascii {
                version: "1.0".to_string()
            },
            comments: vec![Comment(vec!["test".to_string(), "data".to_string()])],
            elements: vec![
                Element::Element(GenericElement {
                    name: "vertex".to_string(),
                    count: 3,
                    props: Property {
                        props: vec![
                            PLYValueTypeName::Float,
                            PLYValueTypeName::Float,
                            PLYValueTypeName::Float,
                        ],
                        names: vec!["x".to_string(), "y".to_string(), "z".to_string()],
                    },
                    payloads: vec![
                        Payload(vec![
                            PLYValue::Float(0f32),
                            PLYValue::Float(0f32),
                            PLYValue::Float(0f32)
                        ],),
                        Payload(vec![
                            PLYValue::Float(0f32),
                            PLYValue::Float(0f32),
                            PLYValue::Float(1f32)
                        ],),
                        Payload(vec![
                            PLYValue::Float(0f32),
                            PLYValue::Float(1f32),
                            PLYValue::Float(1f32)
                        ],)
                    ]
                }),
                Element::ListElement(GenericElement {
                    name: "face".to_string(),
                    count: 3,
                    props: PropertyList {
                        prop: crate::PLYValueTypeName::Uint,
                        name: "vertex_list".to_string(),
                        count: crate::PLYValueTypeName::Uchar
                    },
                    payloads: vec![
                        Payload(vec![PLYValue::Uint(1)]),
                        Payload(vec![PLYValue::Uint(1), PLYValue::Uint(2)]),
                        Payload(vec![
                            PLYValue::Uint(1),
                            PLYValue::Uint(2),
                            PLYValue::Uint(3)
                        ])
                    ]
                })
            ]
        }
    )
}
