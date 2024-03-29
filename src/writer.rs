//! PLY File Writer

pub(crate) mod header;
pub(crate) mod payload;

#[cfg(test)]
mod test {
    use std::io::BufWriter;

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
                        Payload(vec![
                            PLYValue::Float(0f32),
                            PLYValue::Float(0f32),
                            PLYValue::Float(0f32),
                        ]),
                        Payload(vec![
                            PLYValue::Float(0f32),
                            PLYValue::Float(0f32),
                            PLYValue::Float(1f32),
                        ]),
                        Payload(vec![
                            PLYValue::Float(0f32),
                            PLYValue::Float(1f32),
                            PLYValue::Float(1f32),
                        ]),
                        Payload(vec![
                            PLYValue::Float(0f32),
                            PLYValue::Float(1f32),
                            PLYValue::Float(0f32),
                        ]),
                        Payload(vec![
                            PLYValue::Float(1f32),
                            PLYValue::Float(0f32),
                            PLYValue::Float(0f32),
                        ]),
                        Payload(vec![
                            PLYValue::Float(1f32),
                            PLYValue::Float(0f32),
                            PLYValue::Float(1f32),
                        ]),
                        Payload(vec![
                            PLYValue::Float(1f32),
                            PLYValue::Float(1f32),
                            PLYValue::Float(1f32),
                        ]),
                        Payload(vec![
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
                        Payload(vec![PLYValue::Char(3)]),
                        Payload(vec![PLYValue::Char(3), PLYValue::Char(3)]),
                        Payload(vec![
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
                comments: vec![Comment(vec!["test".to_string(), "data".to_string()])],
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
}
