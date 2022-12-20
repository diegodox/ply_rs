use std::io::{BufWriter, Write};

use crate::{Comment, Element, Format, PLYFile, Property, PropertyList};

const MAGIC_NUMBER: &str = "ply";
const END_HEADER: &str = "end_header";

pub(crate) trait PlyWriteHeader<T: Write> {
    fn write_header(&self, writer: &mut BufWriter<T>) -> std::io::Result<()>;
}

impl<T: Write> PlyWriteHeader<T> for Comment {
    fn write_header(&self, writer: &mut BufWriter<T>) -> std::io::Result<()> {
        writeln!(writer, "comment {}", self.0.join(" "))
    }
}
#[test]
fn test_write_comment() {
    let mut writer = BufWriter::new(Vec::new());
    let comment = Comment(vec!["test".to_string(), "comment".to_string()]);
    comment.write_header(&mut writer).unwrap();
    assert_eq!(
        writer.into_inner().unwrap(),
        "comment test comment\n".as_bytes(),
    )
}

impl<T: Write> PlyWriteHeader<T> for Element {
    fn write_header(&self, writer: &mut BufWriter<T>) -> std::io::Result<()> {
        match self {
            Element::Element { name, elements } => {
                writeln!(writer, "element {} {}", name, elements.count)?;
                elements.property().write_header(writer)
            }

            Element::ListElement { name, elements } => {
                writeln!(writer, "element {} {}", name, elements.count)?;
                elements.property().write_header(writer)
            }
        }
    }
}

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
