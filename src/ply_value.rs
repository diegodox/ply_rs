use std::{convert::TryInto, fmt::Display, str::FromStr};

use crate::error::{PLYError, PLYResult};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Scalar data types a property may have (without value)
pub enum PLYValueTypeName {
    Char,
    Uchar,
    Short,
    Ushort,
    Int,
    Uint,
    Float,
    Double,
}
impl PLYValueTypeName {
    /// Returns bytes length of type
    pub fn bytes_len(&self) -> usize {
        match self {
            PLYValueTypeName::Char => 1,
            PLYValueTypeName::Uchar => 1,
            PLYValueTypeName::Short => 2,
            PLYValueTypeName::Ushort => 2,
            PLYValueTypeName::Int => 4,
            PLYValueTypeName::Uint => 4,
            PLYValueTypeName::Float => 4,
            PLYValueTypeName::Double => 8,
        }
    }
    /// Returns type name
    pub fn to_str(&self) -> &'static str {
        match self {
            PLYValueTypeName::Char => r#"char"#,
            PLYValueTypeName::Uchar => r#"uchar"#,
            PLYValueTypeName::Short => r#"short"#,
            PLYValueTypeName::Ushort => r#"ushort"#,
            PLYValueTypeName::Int => r#"int"#,
            PLYValueTypeName::Uint => r#"uinit"#,
            PLYValueTypeName::Float => r#"float"#,
            PLYValueTypeName::Double => r#"double"#,
        }
    }
    /// parse `&str` into [PLYValue]
    pub fn parse(&self, value: &str) -> PLYResult<PLYValue> {
        match self {
            PLYValueTypeName::Char => Ok(PLYValue::Char(
                value.parse().or(Err(PLYError::ParseFromStrErr))?,
            )),
            PLYValueTypeName::Uchar => Ok(PLYValue::Uchar(
                value.parse().or(Err(PLYError::ParseFromStrErr))?,
            )),
            PLYValueTypeName::Short => Ok(PLYValue::Short(
                value.parse().or(Err(PLYError::ParseFromStrErr))?,
            )),
            PLYValueTypeName::Ushort => Ok(PLYValue::Ushort(
                value.parse().or(Err(PLYError::ParseFromStrErr))?,
            )),
            PLYValueTypeName::Int => Ok(PLYValue::Int(
                value.parse().or(Err(PLYError::ParseFromStrErr))?,
            )),
            PLYValueTypeName::Uint => Ok(PLYValue::Uint(
                value.parse().or(Err(PLYError::ParseFromStrErr))?,
            )),
            PLYValueTypeName::Float => Ok(PLYValue::Float(
                value.parse().or(Err(PLYError::ParseFromStrErr))?,
            )),
            PLYValueTypeName::Double => Ok(PLYValue::Double(
                value.parse().or(Err(PLYError::ParseFromStrErr))?,
            )),
        }
    }
    /// from big-endian
    pub fn from_be_bytes<I: Iterator<Item = u8>>(&self, bytes: &mut I) -> PLYValue {
        match self {
            PLYValueTypeName::Char => PLYValue::Char(i8::from_be_bytes([bytes.next().unwrap()])),
            PLYValueTypeName::Uchar => PLYValue::Uchar(u8::from_be_bytes([bytes.next().unwrap()])),
            PLYValueTypeName::Short => PLYValue::Short(i16::from_be_bytes([
                bytes.next().unwrap(),
                bytes.next().unwrap(),
            ])),
            PLYValueTypeName::Ushort => PLYValue::Ushort(u16::from_be_bytes([
                bytes.next().unwrap(),
                bytes.next().unwrap(),
            ])),
            PLYValueTypeName::Int => PLYValue::Int(i32::from_be_bytes([
                bytes.next().unwrap(),
                bytes.next().unwrap(),
                bytes.next().unwrap(),
                bytes.next().unwrap(),
            ])),
            PLYValueTypeName::Uint => PLYValue::Uint(u32::from_be_bytes([
                bytes.next().unwrap(),
                bytes.next().unwrap(),
                bytes.next().unwrap(),
                bytes.next().unwrap(),
            ])),
            PLYValueTypeName::Float => PLYValue::Float(f32::from_be_bytes([
                bytes.next().unwrap(),
                bytes.next().unwrap(),
                bytes.next().unwrap(),
                bytes.next().unwrap(),
            ])),
            PLYValueTypeName::Double => PLYValue::Double(f64::from_be_bytes([
                bytes.next().unwrap(), //1
                bytes.next().unwrap(), //2
                bytes.next().unwrap(), //3
                bytes.next().unwrap(), //4
                bytes.next().unwrap(), //5
                bytes.next().unwrap(), //6
                bytes.next().unwrap(), //7
                bytes.next().unwrap(), //8
            ])),
        }
    }
    /// from little-endian
    pub fn from_le_bytes<I: Iterator<Item = u8>>(&self, bytes: &mut I) -> PLYValue {
        match self {
            PLYValueTypeName::Char => PLYValue::Char(i8::from_le_bytes([bytes.next().unwrap()])),
            PLYValueTypeName::Uchar => PLYValue::Uchar(u8::from_le_bytes([bytes.next().unwrap()])),
            PLYValueTypeName::Short => PLYValue::Short(i16::from_le_bytes([
                bytes.next().unwrap(),
                bytes.next().unwrap(),
            ])),
            PLYValueTypeName::Ushort => PLYValue::Ushort(u16::from_le_bytes([
                bytes.next().unwrap(),
                bytes.next().unwrap(),
            ])),
            PLYValueTypeName::Int => PLYValue::Int(i32::from_le_bytes([
                bytes.next().unwrap(),
                bytes.next().unwrap(),
                bytes.next().unwrap(),
                bytes.next().unwrap(),
            ])),
            PLYValueTypeName::Uint => PLYValue::Uint(u32::from_le_bytes([
                bytes.next().unwrap(),
                bytes.next().unwrap(),
                bytes.next().unwrap(),
                bytes.next().unwrap(),
            ])),
            PLYValueTypeName::Float => PLYValue::Float(f32::from_le_bytes([
                bytes.next().unwrap(),
                bytes.next().unwrap(),
                bytes.next().unwrap(),
                bytes.next().unwrap(),
            ])),
            PLYValueTypeName::Double => PLYValue::Double(f64::from_le_bytes([
                bytes.next().unwrap(), //1
                bytes.next().unwrap(), //2
                bytes.next().unwrap(), //3
                bytes.next().unwrap(), //4
                bytes.next().unwrap(), //5
                bytes.next().unwrap(), //6
                bytes.next().unwrap(), //7
                bytes.next().unwrap(), //8
            ])),
        }
    }
    pub fn zero(&self) -> PLYValue {
        match self {
            PLYValueTypeName::Char => PLYValue::Char(0),
            PLYValueTypeName::Uchar => PLYValue::Uchar(0),
            PLYValueTypeName::Short => PLYValue::Short(0),
            PLYValueTypeName::Ushort => PLYValue::Ushort(0),
            PLYValueTypeName::Int => PLYValue::Int(0),
            PLYValueTypeName::Uint => PLYValue::Uint(0),
            PLYValueTypeName::Float => PLYValue::Float(0f32),
            PLYValueTypeName::Double => PLYValue::Double(0f64),
        }
    }
}
pub(crate) trait PlyTryFrom<T> {
    fn try_from(&self, v: T) -> PLYResult<PLYValue>;
}
impl PlyTryFrom<usize> for PLYValueTypeName {
    fn try_from(&self, v: usize) -> PLYResult<PLYValue> {
        match self {
            PLYValueTypeName::Char => Ok(PLYValue::Char(
                v.try_into().map_err(|_| PLYError::TypeConversionFail)?,
            )),
            PLYValueTypeName::Uchar => Ok(PLYValue::Uchar(
                v.try_into().map_err(|_| PLYError::TypeConversionFail)?,
            )),
            PLYValueTypeName::Short => Ok(PLYValue::Short(
                v.try_into().map_err(|_| PLYError::TypeConversionFail)?,
            )),
            PLYValueTypeName::Ushort => Ok(PLYValue::Ushort(
                v.try_into().map_err(|_| PLYError::TypeConversionFail)?,
            )),
            PLYValueTypeName::Int => Ok(PLYValue::Int(
                v.try_into().map_err(|_| PLYError::TypeConversionFail)?,
            )),
            PLYValueTypeName::Uint => Ok(PLYValue::Uint(
                v.try_into().map_err(|_| PLYError::TypeConversionFail)?,
            )),
            PLYValueTypeName::Float => Err(PLYError::TypeConversionFail),
            PLYValueTypeName::Double => Err(PLYError::TypeConversionFail),
        }
    }
}
#[test]
fn test_from_be_bytes() {
    assert_eq!(
        PLYValueTypeName::Char.from_be_bytes(&mut vec![8u8].into_iter()),
        PLYValue::Char(8)
    );
}
impl FromStr for PLYValueTypeName {
    type Err = PLYError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "char" => Ok(Self::Char),
            "uchar" => Ok(Self::Uchar),
            "short" => Ok(Self::Short),
            "ushort" => Ok(Self::Ushort),
            "int" => Ok(Self::Int),
            "uint" => Ok(Self::Uint),
            "float" => Ok(Self::Float),
            "double" => Ok(Self::Double),
            _ => Err(PLYError::UnknownPLYTypeIdentifier),
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
/// Scalar data types a property may have (with value)
pub enum PLYValue {
    Char(i8),
    Uchar(u8),
    Short(i16),
    Ushort(u16),
    Int(i32),
    Uint(u32),
    Float(f32),
    Double(f64),
}

impl PLYValue {
    /// Returns [PLYValueTypeName] collespond to `self`
    pub fn value_type(&self) -> PLYValueTypeName {
        match self {
            PLYValue::Char(_) => PLYValueTypeName::Char,
            PLYValue::Uchar(_) => PLYValueTypeName::Uchar,
            PLYValue::Short(_) => PLYValueTypeName::Short,
            PLYValue::Ushort(_) => PLYValueTypeName::Ushort,
            PLYValue::Int(_) => PLYValueTypeName::Int,
            PLYValue::Uint(_) => PLYValueTypeName::Uint,
            PLYValue::Float(_) => PLYValueTypeName::Float,
            PLYValue::Double(_) => PLYValueTypeName::Double,
        }
    }
}

impl Display for PLYValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PLYValue::Char(v) => write!(f, "{}", v),
            PLYValue::Uchar(v) => write!(f, "{}", v),
            PLYValue::Short(v) => write!(f, "{}", v),
            PLYValue::Ushort(v) => write!(f, "{}", v),
            PLYValue::Int(v) => write!(f, "{}", v),
            PLYValue::Uint(v) => write!(f, "{}", v),
            PLYValue::Float(v) => write!(f, "{}", v),
            PLYValue::Double(v) => write!(f, "{}", v),
        }
    }
}

impl TryInto<usize> for PLYValue {
    type Error = PLYError;

    fn try_into(self) -> Result<usize, Self::Error> {
        match self {
            PLYValue::Char(v) => v.try_into().map_err(|_e| PLYError::TryIntoUsizeEr),
            PLYValue::Uchar(v) => v.try_into().map_err(|_e| PLYError::TryIntoUsizeEr),
            PLYValue::Short(v) => v.try_into().map_err(|_e| PLYError::TryIntoUsizeEr),
            PLYValue::Ushort(v) => v.try_into().map_err(|_e| PLYError::TryIntoUsizeEr),
            PLYValue::Int(v) => v.try_into().map_err(|_e| PLYError::TryIntoUsizeEr),
            PLYValue::Uint(v) => v.try_into().map_err(|_e| PLYError::TryIntoUsizeEr),
            PLYValue::Float(_v) => Err(PLYError::TryIntoUsizeEr),
            PLYValue::Double(_v) => Err(PLYError::TryIntoUsizeEr),
        }
    }
}

impl Into<PLYValue> for i8 {
    fn into(self) -> PLYValue {
        PLYValue::Char(self)
    }
}
#[test]
fn test_from_i8() {
    assert_eq!(PLYValue::Char(3i8), 3i8.into());
}
impl Into<PLYValue> for u8 {
    fn into(self) -> PLYValue {
        PLYValue::Uchar(self)
    }
}
impl Into<PLYValue> for i16 {
    fn into(self) -> PLYValue {
        PLYValue::Short(self)
    }
}
impl Into<PLYValue> for u16 {
    fn into(self) -> PLYValue {
        PLYValue::Ushort(self)
    }
}
impl Into<PLYValue> for i32 {
    fn into(self) -> PLYValue {
        PLYValue::Int(self)
    }
}
impl Into<PLYValue> for u32 {
    fn into(self) -> PLYValue {
        PLYValue::Uint(self)
    }
}
impl Into<PLYValue> for f32 {
    fn into(self) -> PLYValue {
        PLYValue::Float(self)
    }
}
impl Into<PLYValue> for f64 {
    fn into(self) -> PLYValue {
        PLYValue::Double(self)
    }
}
