use std::io::{BufWriter, Write};

use crate::{Comment, Element, Format, GenericElement, PLYFile, Property, PropertyList};

const MAGIC_NUMBER: &str = "ply";
const END_HEADER: &str = "end_header";

pub(crate) trait PlyWriteHeader<T: Write> {
    fn write_header(&self, writer: &mut BufWriter<T>) -> std::io::Result<()>;
}

impl<T: Write> PlyWriteHeader<T> for PLYFile {
    fn write_header(&self, writer: &mut BufWriter<T>) -> std::io::Result<()> {
        writeln!(writer, "{}", MAGIC_NUMBER)?;
        self.format.write_header(writer)?;
        for comment in self.comments.iter() {
            comment.write_header(writer)?;
        }
        for element in self.elements.iter() {
            match element {
                Element::Element(e) => e.write_header(writer),
                Element::ListElement(e) => e.write_header(writer),
            }?;
        }
        writeln!(writer, "{}", END_HEADER)?;
        Ok(())
    }
}

impl<T: Write> PlyWriteHeader<T> for Format {
    fn write_header(&self, writer: &mut BufWriter<T>) -> std::io::Result<()> {
        match self {
            crate::Format::Ascii { version } => writeln!(writer, "format ascii {}", version),
            crate::Format::BinaryBigEndian { version } => {
                writeln!(writer, "format binary_big_endian {}", version)
            }
            crate::Format::BinaryLittleEndian { version } => {
                writeln!(writer, "format binary_little_endian {}", version)
            }
        }
    }
}
#[test]
fn test_write_format() {
    let mut writer = BufWriter::new(Vec::new());
    let format = Format::Ascii {
        version: "1.0".to_string(),
    };
    format.write_header(&mut writer).unwrap();
    assert_eq!(
        writer.into_inner().unwrap(),
        r#"format ascii 1.0
"#
        .as_bytes(),
    )
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
        r#"comment test comment
"#
        .as_bytes(),
    )
}

impl<T: Write, P: PlyWriteHeader<T>> PlyWriteHeader<T> for GenericElement<P> {
    fn write_header(&self, writer: &mut BufWriter<T>) -> std::io::Result<()> {
        writeln!(writer, "element {} {}", self.name, self.count)?;
        self.property().write_header(writer)
    }
}
#[test]
fn test_write_element_header() {
    use crate::*;
    let mut writer = BufWriter::new(Vec::new());
    let element = GenericElement {
        name: "vertex".to_string(),
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
    };
    element.write_header(&mut writer).unwrap();
    assert_eq!(
        writer.into_inner().unwrap(),
        r#"element vertex 20
property float x
property float y
property float z
property uchar red
property uchar green
property uchar blue
"#
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
        r#"property float x
property float y
property float z
property uchar red
property uchar green
property uchar blue
"#
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
        r#"property list uchar float vertex
"#
        .as_bytes(),
    )
}
