use crate::{
    error::{PLYError, PLYResult},
    IntoPlyElement, Payload, Property, PropertyList,
};

#[derive(Debug, Clone, PartialEq)]
/// Struct represent Generic PLY Element
///
/// Note: generic type P expect [Property] or [PropertyList].
pub struct GenericElement<P> {
    pub(crate) count: usize,
    pub(crate) props: P,
    pub(crate) payloads: Vec<Payload>,
}

impl<P> GenericElement<P> {
    pub fn new(property: P) -> GenericElement<P> {
        Self {
            count: 0,
            props: property,
            payloads: Vec::new(),
        }
    }

    pub fn into_ploperty(self) -> P {
        self.props
    }

    pub fn property(&self) -> &P {
        &self.props
    }

    pub fn property_mut(&mut self) -> &mut P {
        &mut self.props
    }

    pub fn into_payload(self) -> Vec<Payload> {
        self.payloads
    }

    pub fn payload(&self) -> &[Payload] {
        &self.payloads
    }

    pub fn payload_mut(&mut self) -> &mut [Payload] {
        &mut self.payloads
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

impl GenericElement<Property> {
    pub fn push_payload(&mut self, payload: Payload) -> PLYResult<()> {
        if self.property().len() != payload.len() {
            return Err(PLYError::PropertyLengthErr);
        }
        if !payload
            .iter()
            .zip(self.property().props.iter())
            .all(|(v, t)| v.value_type() == *t)
        {
            return Err(PLYError::MissmatchDataType);
        }

        self.count += 1;
        self.payloads.push(payload);
        Ok(())
    }
}

impl GenericElement<PropertyList> {
    pub fn push_payload(&mut self, payload: Payload) -> PLYResult<()> {
        if !(payload
            .iter()
            .all(|v| v.value_type() == self.property().prop))
        {
            return Err(PLYError::MissmatchDataType);
        }

        self.count += 1;
        self.payloads.push(payload);
        Ok(())
    }
}

impl<Item: IntoPlyElement> std::iter::FromIterator<Item> for GenericElement<Property> {
    fn from_iter<T: IntoIterator<Item = Item>>(iter: T) -> Self {
        iter.into_iter().map(|x| x.into_payload()).fold(
            GenericElement::new(Item::property()),
            |mut acc, x| {
                acc.push_payload(x).unwrap();
                acc
            },
        )
    }
}
