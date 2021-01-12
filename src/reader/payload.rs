use std::convert::TryInto;

use crate::{GenericElement, Payload, Property, PropertyList};

pub(crate) fn read_elemet_payload_ascii<P, I>(element: &mut GenericElement<P>, lines: &mut I)
where
    P: ReadPayload<Payload = Payload>,
    I: Iterator<Item = String>,
{
    for _ in 0..element.count {
        let line = lines.next().unwrap();
        let payload = element.props.read_as_ascii(line);
        element.payloads.push(payload);
    }
}
pub(crate) fn read_element_payload_be_bytes<P, I>(element: &mut GenericElement<P>, bytes: &mut I)
where
    P: ReadPayload<Payload = Payload>,
    I: Iterator<Item = u8>,
{
    for _ in 0..element.count {
        let payload = element.props.read_as_be(bytes);
        element.payloads.push(payload);
    }
}

pub(crate) fn read_element_payload_le_bytes<P, I>(element: &mut GenericElement<P>, bytes: &mut I)
where
    P: ReadPayload<Payload = Payload>,
    I: Iterator<Item = u8>,
{
    for _ in 0..element.count {
        let payload = element.props.read_as_le(bytes);
        element.payloads.push(payload);
    }
}

#[test]
fn test_read_element_payload_ascii() {
    use crate::*;
    let mut element = GenericElement {
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
        payloads: Vec::<Payload>::with_capacity(8),
    };
    let lines = r#"0 0 0
0 0 1
0 1 1
0 1 0
1 0 0
1 0 1
1 1 1
1 1 0
"#;
    read_elemet_payload_ascii(&mut element, &mut lines.lines().map(|e| e.to_string()));
    assert_eq!(
        element.payloads,
        vec![
            Payload(vec![
                PLYValue::Uchar(0),
                PLYValue::Uchar(0),
                PLYValue::Uchar(0)
            ],),
            Payload(vec![
                PLYValue::Uchar(0),
                PLYValue::Uchar(0),
                PLYValue::Uchar(1)
            ],),
            Payload(vec![
                PLYValue::Uchar(0),
                PLYValue::Uchar(1),
                PLYValue::Uchar(1)
            ],),
            Payload(vec![
                PLYValue::Uchar(0),
                PLYValue::Uchar(1),
                PLYValue::Uchar(0)
            ],),
            Payload(vec![
                PLYValue::Uchar(1),
                PLYValue::Uchar(0),
                PLYValue::Uchar(0)
            ],),
            Payload(vec![
                PLYValue::Uchar(1),
                PLYValue::Uchar(0),
                PLYValue::Uchar(1)
            ],),
            Payload(vec![
                PLYValue::Uchar(1),
                PLYValue::Uchar(1),
                PLYValue::Uchar(1)
            ],),
            Payload(vec![
                PLYValue::Uchar(1),
                PLYValue::Uchar(1),
                PLYValue::Uchar(0)
            ],),
        ]
    )
}

#[test]
fn test_read_element_payload_be() {
    use crate::*;
    let mut element = GenericElement {
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
        payloads: Vec::with_capacity(8),
    };
    let bytes = [
        //0
        0u8.to_be(),
        0u8.to_be(),
        0u8.to_be(),
        //1
        0u8.to_be(),
        0u8.to_be(),
        1u8.to_be(),
        //2
        0u8.to_be(),
        1u8.to_be(),
        1u8.to_be(),
        //3
        0u8.to_be(),
        1u8.to_be(),
        0u8.to_be(),
        //4
        1u8.to_be(),
        0u8.to_be(),
        0u8.to_be(),
        //5
        1u8.to_be(),
        0u8.to_be(),
        1u8.to_be(),
        //6
        1u8.to_be(),
        1u8.to_be(),
        1u8.to_be(),
        //7
        1u8.to_be(),
        1u8.to_be(),
        0u8.to_be(),
    ];
    read_element_payload_be_bytes(&mut element, &mut bytes.iter().copied());
    assert_eq!(
        element.payloads,
        vec![
            Payload(vec![
                PLYValue::Uchar(0),
                PLYValue::Uchar(0),
                PLYValue::Uchar(0)
            ],),
            Payload(vec![
                PLYValue::Uchar(0),
                PLYValue::Uchar(0),
                PLYValue::Uchar(1)
            ],),
            Payload(vec![
                PLYValue::Uchar(0),
                PLYValue::Uchar(1),
                PLYValue::Uchar(1)
            ],),
            Payload(vec![
                PLYValue::Uchar(0),
                PLYValue::Uchar(1),
                PLYValue::Uchar(0)
            ],),
            Payload(vec![
                PLYValue::Uchar(1),
                PLYValue::Uchar(0),
                PLYValue::Uchar(0)
            ],),
            Payload(vec![
                PLYValue::Uchar(1),
                PLYValue::Uchar(0),
                PLYValue::Uchar(1)
            ],),
            Payload(vec![
                PLYValue::Uchar(1),
                PLYValue::Uchar(1),
                PLYValue::Uchar(1)
            ],),
            Payload(vec![
                PLYValue::Uchar(1),
                PLYValue::Uchar(1),
                PLYValue::Uchar(0)
            ],),
        ]
    )
}

#[test]
fn test_read_element_payload_le() {
    use crate::*;
    let mut element = GenericElement {
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
        payloads: Vec::with_capacity(8),
    };
    let bytes = [
        //0
        0u8.to_le(),
        0u8.to_le(),
        0u8.to_le(),
        //1
        0u8.to_le(),
        0u8.to_le(),
        1u8.to_le(),
        //2
        0u8.to_le(),
        1u8.to_le(),
        1u8.to_le(),
        //3
        0u8.to_le(),
        1u8.to_le(),
        0u8.to_le(),
        //4
        1u8.to_le(),
        0u8.to_le(),
        0u8.to_le(),
        //5
        1u8.to_le(),
        0u8.to_le(),
        1u8.to_le(),
        //6
        1u8.to_le(),
        1u8.to_le(),
        1u8.to_le(),
        //7
        1u8.to_le(),
        1u8.to_le(),
        0u8.to_le(),
    ];
    read_element_payload_le_bytes(&mut element, &mut bytes.iter().copied());
    assert_eq!(
        element.payloads,
        vec![
            Payload(vec![
                PLYValue::Uchar(0),
                PLYValue::Uchar(0),
                PLYValue::Uchar(0)
            ],),
            Payload(vec![
                PLYValue::Uchar(0),
                PLYValue::Uchar(0),
                PLYValue::Uchar(1)
            ],),
            Payload(vec![
                PLYValue::Uchar(0),
                PLYValue::Uchar(1),
                PLYValue::Uchar(1)
            ],),
            Payload(vec![
                PLYValue::Uchar(0),
                PLYValue::Uchar(1),
                PLYValue::Uchar(0)
            ],),
            Payload(vec![
                PLYValue::Uchar(1),
                PLYValue::Uchar(0),
                PLYValue::Uchar(0)
            ],),
            Payload(vec![
                PLYValue::Uchar(1),
                PLYValue::Uchar(0),
                PLYValue::Uchar(1)
            ],),
            Payload(vec![
                PLYValue::Uchar(1),
                PLYValue::Uchar(1),
                PLYValue::Uchar(1)
            ],),
            Payload(vec![
                PLYValue::Uchar(1),
                PLYValue::Uchar(1),
                PLYValue::Uchar(0)
            ],),
        ]
    )
}

pub(crate) trait ReadPayload {
    type Payload;

    fn read_as_ascii<S: AsRef<str>>(&self, line: S) -> Self::Payload;
    fn read_as_be<I: Iterator<Item = u8>>(&self, bytes: &mut I) -> Self::Payload;
    fn read_as_le<I: Iterator<Item = u8>>(&self, bytes: &mut I) -> Self::Payload;
}

impl ReadPayload for Property {
    type Payload = Payload;

    fn read_as_ascii<S: AsRef<str>>(&self, line: S) -> Payload {
        let words = line.as_ref().split_ascii_whitespace();
        Payload(
            self.props
                .iter()
                .zip(words)
                .map(|(t, s)| t.parse(s).unwrap())
                .collect(),
        )
    }

    fn read_as_be<I: Iterator<Item = u8>>(&self, bytes: &mut I) -> Payload {
        Payload(self.props.iter().map(|t| t.from_be_bytes(bytes)).collect())
    }

    fn read_as_le<I: Iterator<Item = u8>>(&self, bytes: &mut I) -> Payload {
        Payload(self.props.iter().map(|t| t.from_le_bytes(bytes)).collect())
    }
}

impl ReadPayload for PropertyList {
    type Payload = Payload;

    fn read_as_ascii<S: AsRef<str>>(&self, line: S) -> Payload {
        let mut words = line.as_ref().split_ascii_whitespace();
        let count_usize = words.next().unwrap().parse().unwrap();
        let data = words
            .map(|s| self.prop.parse(s).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(data.len(), count_usize);

        Payload(data)
    }

    fn read_as_be<I: Iterator<Item = u8>>(&self, bytes: &mut I) -> Payload {
        let count: usize = self.count.from_be_bytes(bytes).try_into().unwrap();
        let data = (0..count).map(|_| self.prop.from_be_bytes(bytes)).collect();
        Payload(data)
    }

    fn read_as_le<I: Iterator<Item = u8>>(&self, bytes: &mut I) -> Self::Payload {
        let count: usize = self.count.from_be_bytes(bytes).try_into().unwrap();
        let data = (0..count).map(|_| self.prop.from_le_bytes(bytes)).collect();
        Payload(data)
    }
}
#[test]
fn test_read_as_ascii() {
    use crate::*;

    let line = r#"0 1 0"#;
    let prop = Property {
        props: vec![
            PLYValueTypeName::Uchar,
            PLYValueTypeName::Uchar,
            PLYValueTypeName::Uchar,
        ],
        names: vec!["red".to_string(), "green".to_string(), "blue".to_string()],
    };
    assert_eq!(
        prop.read_as_ascii(line),
        Payload(vec![
            PLYValue::Uchar(0),
            PLYValue::Uchar(1),
            PLYValue::Uchar(0)
        ],)
    )
}

#[test]
fn test_read_as_be() {
    use crate::*;

    let bytes = [0u8.to_be(), 1u8.to_be(), 0u8.to_be()];
    let prop = Property {
        props: vec![
            PLYValueTypeName::Uchar,
            PLYValueTypeName::Uchar,
            PLYValueTypeName::Uchar,
        ],
        names: vec!["red".to_string(), "green".to_string(), "blue".to_string()],
    };
    assert_eq!(
        prop.read_as_be(&mut bytes.iter().copied()),
        Payload(vec![
            PLYValue::Uchar(0),
            PLYValue::Uchar(1),
            PLYValue::Uchar(0)
        ],)
    )
}

#[test]
fn test_read_as_le() {
    use crate::*;

    let bytes = [0u8.to_le(), 1u8.to_le(), 0u8.to_le()];
    let prop = Property {
        props: vec![
            PLYValueTypeName::Uchar,
            PLYValueTypeName::Uchar,
            PLYValueTypeName::Uchar,
        ],
        names: vec!["red".to_string(), "green".to_string(), "blue".to_string()],
    };
    assert_eq!(
        prop.read_as_le(&mut bytes.iter().copied()),
        Payload(vec![
            PLYValue::Uchar(0),
            PLYValue::Uchar(1),
            PLYValue::Uchar(0)
        ],)
    )
}
