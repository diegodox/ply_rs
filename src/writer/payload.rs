use std::io::{BufWriter, Write};

use crate::{
    ply_value::PlyTryFrom, Format, GenericElement, PLYValue, Payload, Property, PropertyList,
};

pub(crate) fn write_element_payload<T: Write, P: WritePayload<T, Payload = Payload>>(
    element: &GenericElement<P>,
    writer: &mut BufWriter<T>,
    format: &Format,
) -> std::io::Result<()> {
    match format {
        Format::Ascii { .. } => {
            for payload in element.payload() {
                element.property().write_payload_ascii(payload, writer)?;
            }
        }

        Format::BinaryBigEndian { .. } => {
            for payload in element.payload() {
                element.property().write_payload_be(payload, writer)?;
            }
        }
        Format::BinaryLittleEndian { .. } => {
            for payload in element.payload() {
                element.property().write_payload_le(payload, writer)?;
            }
        }
    };
    Ok(())
}

0 0 0
0 0 1
0 1 1
0 1 0
1 0 0
1 0 1
1 1 1
1 1 0
"
        .as_bytes()
        .to_vec(),
        writer.into_inner().unwrap(),
    );
}

pub(crate) trait WritePayload<T: Write> {
    type Payload;

    fn write_payload_ascii(
        &self,
        payload: &Self::Payload,
        writer: &mut BufWriter<T>,
    ) -> std::io::Result<()>;
    fn write_payload_be(
        &self,
        payload: &Self::Payload,
        writer: &mut BufWriter<T>,
    ) -> std::io::Result<()>;
    fn write_payload_le(
        &self,
        payload: &Self::Payload,
        writer: &mut BufWriter<T>,
    ) -> std::io::Result<()>;
}
