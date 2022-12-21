use std::{fmt::Display, io::Write};

use crate::writer::header::PlyWriteHeader;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Format of PLY file
pub enum Format {
    Ascii { version: String },
    BinaryBigEndian { version: String },
    BinaryLittleEndian { version: String },
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Format::Ascii { version } => write!(f, "format ascii {version}"),
            Format::BinaryBigEndian { version } => {
                write!(f, "format binary_big_endian {version}")
            }
            Format::BinaryLittleEndian { version } => {
                write!(f, "format binary_little_endian {version}")
            }
        }
    }
}

impl<T: Write> PlyWriteHeader<T> for Format {
    fn write_header(&self, writer: &mut T) -> std::io::Result<()> {
        match self {
            crate::Format::Ascii { version } => writeln!(writer, "format ascii {version}"),
            crate::Format::BinaryBigEndian { version } => {
                writeln!(writer, "format binary_big_endian {version}")
            }
            crate::Format::BinaryLittleEndian { version } => {
                writeln!(writer, "format binary_little_endian {version}")
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::io::BufWriter;

    use crate::{writer::header::PlyWriteHeader, Format};

    #[test]
    fn test_write_format() {
        let mut writer = BufWriter::new(Vec::new());
        let format = Format::Ascii {
            version: "1.0".to_string(),
        };
        format.write_header(&mut writer).unwrap();
        assert_eq!(
            writer.into_inner().unwrap(),
            "format ascii 1.0\n".as_bytes(),
        )
    }
}
