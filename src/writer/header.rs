use std::io::Write;

pub(crate) trait PlyWriteHeader<T: Write> {
    fn write_header(&self, writer: &mut T) -> std::io::Result<()>;
}
