use std::io::BufWriter;

use ply::{
    Comment, Element, Format, GenericElement, PLYFile, PLYValue, PLYValueTypeName, Payload,
    Property, PropertyList,
};

const PLY: &str = "\
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
";

fn main() {
    let mut ply = PLYFile::new(Format::Ascii {
        version: "1.0".to_string(),
    });
    ply.comments.push(Comment::new("test data"));
    ply.elements.push(Element::Element({
        let mut element = GenericElement::new(
            "vertex",
            Property::from(vec![
                ("x", PLYValueTypeName::Float),
                ("y", PLYValueTypeName::Float),
                ("z", PLYValueTypeName::Float),
            ]),
        );
        element
            .push_payload(Payload::from(vec![
                PLYValue::Float(0f32),
                PLYValue::Float(0f32),
                PLYValue::Float(0f32),
            ]))
            .unwrap();
        element
            .push_payload(Payload::from(vec![
                PLYValue::Float(0f32),
                PLYValue::Float(0f32),
                PLYValue::Float(1f32),
            ]))
            .unwrap();
        element
            .push_payload(Payload::from(vec![
                PLYValue::Float(0f32),
                PLYValue::Float(1f32),
                PLYValue::Float(1f32),
            ]))
            .unwrap();
        element
            .push_payload(Payload::from(vec![
                PLYValue::Float(0f32),
                PLYValue::Float(1f32),
                PLYValue::Float(0f32),
            ]))
            .unwrap();
        element
            .push_payload(Payload::from(vec![
                PLYValue::Float(1f32),
                PLYValue::Float(0f32),
                PLYValue::Float(0f32),
            ]))
            .unwrap();
        element
            .push_payload(Payload::from(vec![
                PLYValue::Float(1f32),
                PLYValue::Float(0f32),
                PLYValue::Float(1f32),
            ]))
            .unwrap();
        element
            .push_payload(Payload::from(vec![
                PLYValue::Float(1f32),
                PLYValue::Float(1f32),
                PLYValue::Float(1f32),
            ]))
            .unwrap();
        element
            .push_payload(Payload::from(vec![
                PLYValue::Float(1f32),
                PLYValue::Float(1f32),
                PLYValue::Float(0f32),
            ]))
            .unwrap();
        element
    }));
    ply.elements.push(Element::ListElement({
        let mut element = GenericElement::new(
            "list",
            PropertyList::new("vertex_id", PLYValueTypeName::Uchar, PLYValueTypeName::Char),
        );
        element
            .push_payload(Payload::from(vec![PLYValue::Char(3)]))
            .unwrap();
        element
            .push_payload(Payload::from(vec![PLYValue::Char(3), PLYValue::Char(3)]))
            .unwrap();
        element
            .push_payload(Payload::from(vec![
                PLYValue::Char(3),
                PLYValue::Char(3),
                PLYValue::Char(3),
            ]))
            .unwrap();
        element
    }));

    // write ply
    let mut writer = BufWriter::new(Vec::new());
    ply.write(&mut writer).unwrap();
    let string = String::from_utf8(writer.into_inner().unwrap()).unwrap();

    // we created same ply file as PLY
    assert_eq!(string.as_str(), PLY);

    // print ply
    print!("{ply:#?}");
}
