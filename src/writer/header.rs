use std::io::{BufWriter, Write};

use crate::{Comment, Element, Format, PLYFile, Property, PropertyList};

const MAGIC_NUMBER: &str = "ply";
const END_HEADER: &str = "end_header";

pub(crate) trait PlyWriteHeader<T: Write> {
    fn write_header(&self, writer: &mut BufWriter<T>) -> std::io::Result<()>;
}
