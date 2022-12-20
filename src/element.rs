use crate::{Payload, Property, PropertyList};

use self::generic_element::GenericElement;

mod generic_element;
mod write;

#[derive(Debug, Clone, PartialEq)]
/// Enum represent PLY Element
pub enum Element {
    Element {
        name: String,
        elements: GenericElement<Property>,
    },

    ListElement {
        name: String,
        elements: GenericElement<PropertyList>,
    },
}

/// Trait indicate type can be a [PlyElement]
///
/// Type implements this can be `collect` into [GenericElement]<[Property]>
pub trait IntoPlyElement {
    /// returns assosicated [Property]
    fn property() -> Property;

    /// convert to [Payload]
    fn into_payload(self) -> Payload;
}
