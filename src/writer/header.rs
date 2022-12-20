use std::io::{BufWriter, Write};

pub(crate) trait PlyWriteHeader<T: Write> {
    fn write_header(&self, writer: &mut BufWriter<T>) -> std::io::Result<()>;
}
