use crate::{
    Comment, Element, Format, GenericElement, PLYFile, PLYValueTypeName, Property, PropertyList,
};
use std::convert::AsRef;
use std::str::FromStr;

/// Read [HeaderLine]s
pub(crate) fn from_header_lines<I: Iterator<Item = HeaderLine>>(lines: &mut I) -> PLYFile {
    // assert magic number
    assert_eq!(
        lines
            .next()
            .expect("Not found: first line, magic number must be ply"),
        HeaderLine::FileIdentifierLine
    );

    // read format of PLY file
    let format = match lines.next().expect("Not found: secound line, format style") {
        HeaderLine::FormatLine(format) => format,
        _ => panic!("Not found: secound line, format style"),
    };

    // read comment and element
    let mut comments = Vec::new();
    let mut elements = Vec::new();
    while let Some(mut next) = read_to_element_line(lines, &mut comments) {
        loop {
            let (element, next_option) = read_element_props(lines, &mut comments, next);
            elements.push(element);
            if let Some(next_some) = next_option {
                next = next_some;
            } else {
                break;
            }
        }
    }

    PLYFile {
        format,
        comments,
        elements,
    }
}

#[test]
fn test_from_header_lines() {
    let lines = vec![
        HeaderLine::FileIdentifierLine,
        HeaderLine::FormatLine(Format::Ascii {
            version: "1.0".to_string(),
        }),
        HeaderLine::CommentLine(Comment(vec!["test".to_string(), "data".to_string()])),
        HeaderLine::ElementLine {
            name: "vertex".to_string(),
            count: 8,
        },
        HeaderLine::PropertyLine {
            prop_type: PLYValueTypeName::Float,
            name: "x".to_string(),
        },
        HeaderLine::PropertyLine {
            prop_type: PLYValueTypeName::Float,
            name: "y".to_string(),
        },
        HeaderLine::PropertyLine {
            prop_type: PLYValueTypeName::Float,
            name: "z".to_string(),
        },
    ];
    let mut iter = lines.into_iter();
    let ply_file = from_header_lines(&mut iter);
    assert_eq!(
        ply_file,
        PLYFile {
            format: Format::Ascii {
                version: "1.0".to_string()
            },
            comments: vec![Comment(vec!["test".to_string(), "data".to_string()])],
            elements: vec![Element::Element(GenericElement {
                name: "vertex".to_string(),
                count: 8,
                props: Property {
                    names: vec!["x".to_string(), "y".to_string(), "z".to_string()],
                    props: vec![
                        PLYValueTypeName::Float,
                        PLYValueTypeName::Float,
                        PLYValueTypeName::Float
                    ],
                },
                payloads: Vec::with_capacity(8)
            })]
        }
    )
}

/// Read headers for find line `element (name) (count)`
///
/// Return Some((name, count)) if found, None otherwise
fn read_to_element_line<I: Iterator<Item = HeaderLine>>(
    lines: &mut I,
    comments: &mut Vec<Comment>,
) -> Option<(String, usize)> {
    for line in lines {
        match line {
            HeaderLine::ElementLine { name, count } => {
                return Some((name, count));
            }
            HeaderLine::CommentLine(c) => comments.push(c),
            HeaderLine::EmptyLine => { /* do nothing */ }
            HeaderLine::UnknownLine => { /* do nothing */ }
            HeaderLine::PropertyLine { .. } => panic!(r#"keyword "propety" cannnot use here"#),
            HeaderLine::PropertyListLine(_) => panic!(r#"keyword "propety list" cannnot use here"#),
            HeaderLine::FormatLine(_) => panic!(r#"keyword "format" cannnot use here"#),
            HeaderLine::FileIdentifierLine => panic!(r#"keyword "ply" cannnot use here"#),
            HeaderLine::EndHeader => panic!(r#"keyword end_header is not allowed here"#),
        }
    }
    None
}

#[test]
fn test_read_to_element_line() {
    let lines = vec![
        HeaderLine::CommentLine(Comment(vec![
            "this".to_string(),
            "is".to_string(),
            "a".to_string(),
            "cube".to_string(),
        ])),
        HeaderLine::ElementLine {
            name: "vertex".to_string(),
            count: 8,
        },
    ];
    let mut iter = lines.into_iter();
    let mut comments = Vec::new();
    let next = read_to_element_line(&mut iter, &mut comments);
    assert_eq!(
        comments,
        vec![Comment(vec![
            "this".to_string(),
            "is".to_string(),
            "a".to_string(),
            "cube".to_string()
        ])]
    );
    assert_eq!(next, Some((r#"vertex"#.to_string(), 8)))
}

/// Read element's props, arg `(name, count)` is a name and count of element.
///
/// Return Element and name, usize if they found while reading props.
fn read_element_props<I: Iterator<Item = HeaderLine>>(
    lines: &mut I,
    comments: &mut Vec<Comment>,
    (name, count): (String, usize),
) -> (Element, Option<(String, usize)>) {
    let mut prop = Property {
        props: Vec::new(),
        names: Vec::new(),
    };
    for line in lines {
        match line {
            HeaderLine::PropertyLine { name, prop_type } => {
                prop.props.push(prop_type);
                prop.names.push(name);
            }
            HeaderLine::PropertyListLine(prop_list) => {
                assert!(
                    prop.props.is_empty(),
                    r#""property" and "property lines" cannot be used at same element"#
                );
                return (
                    Element::ListElement(GenericElement {
                        name,
                        count,
                        props: prop_list,
                        payloads: Vec::with_capacity(count),
                    }),
                    None,
                );
            }
            HeaderLine::ElementLine {
                name: next_name,
                count: next_count,
            } => {
                let element = Element::Element({
                    GenericElement {
                        name,
                        count,
                        props: prop,
                        payloads: Vec::with_capacity(count),
                    }
                });
                return (element, Some((next_name, next_count)));
            }
            HeaderLine::CommentLine(c) => comments.push(c),
            HeaderLine::EmptyLine => { /* do nothing */ }
            HeaderLine::FileIdentifierLine => {
                panic!(r#"line "ply" is not allowed here"#)
            }
            HeaderLine::FormatLine(_) => {
                panic!(r#"keyword format is not allowed here"#)
            }
            HeaderLine::EndHeader => {
                panic!(r#"keyword end_header is not allowed here"#)
            }
            HeaderLine::UnknownLine => {}
        }
    }

    let element = Element::Element({
        GenericElement {
            name,
            count,
            props: prop,
            payloads: Vec::with_capacity(count),
        }
    });

    (element, None)
}

#[test]
fn test_read_element_props() {
    let mut lines = r#"property float x
property float y
property float z
comment color
property uchar red
property uchar green
property uchar blue"#
        .lines()
        .map(|line| parse_header_line(line));
    let mut comments = Vec::new();

    let (element, next) = read_element_props(&mut lines, &mut comments, ("vertex".to_string(), 20));

    assert_eq!(
        element,
        Element::Element(GenericElement {
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
                    "blue".to_string()
                ]
            },
            payloads: Vec::with_capacity(20),
        })
    );
    assert_eq!(next, None);
    assert_eq!(comments, vec![Comment(vec!["color".to_string()])]);
}

/// Read from str iter for end_header line
/// EndHeader is not member of header lines
pub(crate) fn read_header_lines<I: Iterator<Item = String>>(lines: &mut I) -> Vec<HeaderLine> {
    lines
        .map(parse_header_line)
        .take_while(|line| !line.is_end_header())
        .collect::<Vec<_>>()
}

#[test]
fn test_read_header_lines() {
    let mut input = r#"ply
format ascii 1.0
comment test data
element vertex 8
property float x
property float y
property float z
end_header
0 0 0
0 0 1
0 1 1
0 1 0
1 0 0
1 0 1
1 1 1
1 1 0
"#
    .lines()
    .map(|e| e.to_string());
    let header_lines = read_header_lines(&mut input);
    assert_eq!(
        header_lines,
        vec![
            HeaderLine::FileIdentifierLine,
            HeaderLine::FormatLine(Format::Ascii {
                version: "1.0".to_string()
            }),
            HeaderLine::CommentLine(Comment(vec!["test".to_string(), "data".to_string()])),
            HeaderLine::ElementLine {
                name: "vertex".to_string(),
                count: 8
            },
            HeaderLine::PropertyLine {
                prop_type: PLYValueTypeName::Float,
                name: "x".to_string()
            },
            HeaderLine::PropertyLine {
                prop_type: PLYValueTypeName::Float,
                name: "y".to_string()
            },
            HeaderLine::PropertyLine {
                prop_type: PLYValueTypeName::Float,
                name: "z".to_string()
            },
            // HeaderLine::EndHeader
            // EndHeader is not member of header lines
        ]
    );
    assert_eq!(input.next(), Some("0 0 0".to_string()));
    assert_eq!(input.next(), Some("0 0 1".to_string()));
    assert_eq!(input.next(), Some("0 1 1".to_string()));
    assert_eq!(input.next(), Some("0 1 0".to_string()));
    assert_eq!(input.next(), Some("1 0 0".to_string()));
    assert_eq!(input.next(), Some("1 0 1".to_string()));
    assert_eq!(input.next(), Some("1 1 1".to_string()));
    assert_eq!(input.next(), Some("1 1 0".to_string()));
    assert_eq!(input.next(), None);
}

/// Possible Lines in PLY Header
#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) enum HeaderLine {
    /// Line just "ply"
    FileIdentifierLine,
    /// Line like "format ascii 1.0"
    FormatLine(Format),
    /// Line like "comment this file is a cube"
    CommentLine(Comment),
    /// Line like "element vertex 8"
    ElementLine { name: String, count: usize },
    /// Line like "property float z"
    PropertyLine {
        name: String,
        prop_type: PLYValueTypeName,
    },
    /// Line like "property list uchar int vertex_index"
    PropertyListLine(PropertyList),
    /// Empty Line (is leagal??)
    EmptyLine,
    /// End Header
    EndHeader,
    /// Line start from unknown identifier
    UnknownLine,
}

impl HeaderLine {
    pub fn is_end_header(&self) -> bool {
        matches!(self, HeaderLine::EndHeader)
    }
}

/// Parse PLY Header Line to [HeaderLine]
pub(crate) fn parse_header_line<S: AsRef<str>>(line: S) -> HeaderLine {
    let mut words = line.as_ref().split_whitespace();
    match words.next() {
        None => HeaderLine::EmptyLine,
        Some(first_token) => match first_token {
            "property" => match words.next().expect(r#"property name or "list" not found"#) {
                "list" => {
                    let count = PLYValueTypeName::from_str(words.next().unwrap()).unwrap();
                    let prop = PLYValueTypeName::from_str(words.next().unwrap()).unwrap();
                    let name = words.next().expect("property name not found").to_string();
                    HeaderLine::PropertyListLine(PropertyList { count, prop, name })
                }
                prop_type_str => HeaderLine::PropertyLine {
                    name: words.next().expect("property name not found").to_string(),
                    prop_type: PLYValueTypeName::from_str(prop_type_str).unwrap(),
                },
            },
            "element" => HeaderLine::ElementLine {
                name: words.next().expect("element name not found").to_string(),
                count: words
                    .next()
                    .expect("element count not found")
                    .parse()
                    .expect("element count must be unsined integer"),
            },
            "format" => {
                HeaderLine::FormatLine(match words.next().expect("format style not found") {
                    "ascii" => Format::Ascii {
                        version: words.next().expect("format version not found").to_string(),
                    },
                    "binary_little_endian" => Format::BinaryLittleEndian {
                        version: words.next().expect("format version not found").to_string(),
                    },
                    "binary_big_endian" => Format::BinaryBigEndian {
                        version: words.next().expect("format version not found").to_string(),
                    },
                    _ => panic!("unknown format style"),
                })
            }
            "ply" => HeaderLine::FileIdentifierLine,
            "comment" => HeaderLine::CommentLine(Comment(words.map(|s| s.to_string()).collect())),
            "end_header" => HeaderLine::EndHeader,
            _ => {
                // eprintln!("unknown line identifier: {}", x);
                HeaderLine::UnknownLine
            }
        },
    }
}

#[test]
fn parse_ply_line() {
    let line = "ply";
    assert_eq!(parse_header_line(line), HeaderLine::FileIdentifierLine);
}
#[test]
fn parse_ascii_format_line() {
    let line = "format ascii 1.0";
    assert_eq!(
        parse_header_line(line),
        HeaderLine::FormatLine(Format::Ascii {
            version: "1.0".to_string()
        })
    );
}
#[test]
fn parse_comment_line() {
    let line = "comment this file is a cube";
    assert_eq!(
        parse_header_line(line),
        HeaderLine::CommentLine(Comment(vec![
            "this".to_string(),
            "file".to_string(),
            "is".to_string(),
            "a".to_string(),
            "cube".to_string()
        ]))
    );
}
#[test]
fn parse_element_line() {
    let line = "element vertex 8";
    assert_eq!(
        parse_header_line(line),
        HeaderLine::ElementLine {
            name: "vertex".to_string(),
            count: 8
        }
    );
}
#[test]
fn parse_property_line() {
    let line = "property float x";
    assert_eq!(
        parse_header_line(line),
        HeaderLine::PropertyLine {
            name: "x".to_string(),
            prop_type: PLYValueTypeName::Float
        }
    );
}
#[test]
fn parse_property_list_line() {
    let line = "property list uchar int vertex_index";
    assert_eq!(
        parse_header_line(line),
        HeaderLine::PropertyListLine(PropertyList {
            name: "vertex_index".to_string(),
            prop: PLYValueTypeName::Int,
            count: PLYValueTypeName::Uchar
        })
    );
}
#[test]
fn parse_empty_line() {
    let line = "";
    assert_eq!(parse_header_line(line), HeaderLine::EmptyLine);
}
