use std::{
    fmt::Display,
    io::{BufWriter, Write},
};

use crate::writer::header::PlyWriteHeader;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
/// Struct represent Comment
///
/// Stored words (splitted by whitespace).
pub struct Comment(pub(crate) Vec<String>);

impl Comment {
    pub fn new(v: Vec<String>) -> Self {
        Self(v)
    }

    pub fn from_string<S: Into<String>>(comment: S) -> Comment {
        Comment(
            comment
                .into()
                .split_whitespace()
                .map(|v| v.to_string())
                .collect(),
        )
    }
}

impl Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "comment {}", self.0.join(" "))
    }
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
