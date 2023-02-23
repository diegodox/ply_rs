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
            PLYValueTypeName::Char => "char",
            PLYValueTypeName::Uchar => "uchar",
            PLYValueTypeName::Short => "short",
            PLYValueTypeName::Ushort => "ushort",
            PLYValueTypeName::Int => "int",
            PLYValueTypeName::Uint => "uint",
            PLYValueTypeName::Float => "float",
            PLYValueTypeName::Double => "double",
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
            PLYValueTypeName::Short => {
                PLYValue::Short(i16::from_be_bytes(std::array::from_fn(|_| {
                    bytes.next().unwrap()
                })))
            }
            PLYValueTypeName::Ushort => {
                PLYValue::Ushort(u16::from_be_bytes(std::array::from_fn(|_| {
                    bytes.next().unwrap()
                })))
            }
            PLYValueTypeName::Int => PLYValue::Int(i32::from_be_bytes(std::array::from_fn(|_| {
                bytes.next().unwrap()
            }))),
            PLYValueTypeName::Uint => {
                PLYValue::Uint(u32::from_be_bytes(std::array::from_fn(|_| {
                    bytes.next().unwrap()
                })))
            }
            PLYValueTypeName::Float => {
                PLYValue::Float(f32::from_be_bytes(std::array::from_fn(|_| {
                    bytes.next().unwrap()
                })))
            }
            PLYValueTypeName::Double => {
                PLYValue::Double(f64::from_be_bytes(std::array::from_fn(|_| {
                    bytes.next().unwrap()
                })))
            }
        }
    }
    /// from little-endian
    pub fn from_le_bytes<I: Iterator<Item = u8>>(&self, bytes: &mut I) -> PLYValue {
        match self {
            PLYValueTypeName::Char => PLYValue::Char(i8::from_le_bytes([bytes.next().unwrap()])),
            PLYValueTypeName::Uchar => PLYValue::Uchar(u8::from_le_bytes([bytes.next().unwrap()])),
            PLYValueTypeName::Short => {
                PLYValue::Short(i16::from_le_bytes(std::array::from_fn(|_| {
                    bytes.next().unwrap()
                })))
            }
            PLYValueTypeName::Ushort => {
                PLYValue::Ushort(u16::from_le_bytes(std::array::from_fn(|_| {
                    bytes.next().unwrap()
                })))
            }
            PLYValueTypeName::Int => PLYValue::Int(i32::from_le_bytes(std::array::from_fn(|_| {
                bytes.next().unwrap()
            }))),
            PLYValueTypeName::Uint => {
                PLYValue::Uint(u32::from_le_bytes(std::array::from_fn(|_| {
                    bytes.next().unwrap()
                })))
            }
            PLYValueTypeName::Float => {
                PLYValue::Float(f32::from_le_bytes(std::array::from_fn(|_| {
                    bytes.next().unwrap()
                })))
            }
            PLYValueTypeName::Double => {
                PLYValue::Double(f64::from_le_bytes(std::array::from_fn(|_| {
                    bytes.next().unwrap()
                })))
            }
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
    pub fn unwrap_char(self) -> i8 {
        match self {
            PLYValue::Char(v) => v,
            _ => panic!("type mismatch"),
        }
    }
    pub fn unwrap_uchar(self) -> u8 {
        match self {
            PLYValue::Uchar(v) => v,
            _ => panic!("type mismatch"),
        }
    }
    pub fn unwrap_short(self) -> i16 {
        match self {
            PLYValue::Short(v) => v,
            _ => panic!("type mismatch"),
        }
    }
    pub fn unwrap_ushort(self) -> u16 {
        match self {
            PLYValue::Ushort(v) => v,
            _ => panic!("type mismatch"),
        }
    }
    pub fn unwrap_int(self) -> i32 {
        match self {
            PLYValue::Int(v) => v,
            _ => panic!("type mismatch"),
        }
    }
    pub fn unwrap_uint(self) -> u32 {
        match self {
            PLYValue::Uint(v) => v,
            _ => panic!("type mismatch"),
        }
    }
    pub fn unwrap_float(self) -> f32 {
        match self {
            PLYValue::Float(v) => v,
            _ => panic!("type mismatch"),
        }
    }
    pub fn unwrap_double(self) -> f64 {
        match self {
            PLYValue::Double(v) => v,
            _ => panic!("type mismatch"),
        }
    }
}

impl Display for PLYValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PLYValue::Char(v) => write!(f, "{v}"),
            PLYValue::Uchar(v) => write!(f, "{v}"),
            PLYValue::Short(v) => write!(f, "{v}"),
            PLYValue::Ushort(v) => write!(f, "{v}"),
            PLYValue::Int(v) => write!(f, "{v}"),
            PLYValue::Uint(v) => write!(f, "{v}"),
            PLYValue::Float(v) => write!(f, "{v}"),
            PLYValue::Double(v) => write!(f, "{v}"),
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

impl From<i8> for PLYValue {
    fn from(val: i8) -> Self {
        PLYValue::Char(val)
    }
}
#[test]
fn test_from_i8() {
    assert_eq!(PLYValue::Char(3i8), 3i8.into());
}
impl From<u8> for PLYValue {
    fn from(val: u8) -> Self {
        PLYValue::Uchar(val)
    }
}
impl From<i16> for PLYValue {
    fn from(val: i16) -> Self {
        PLYValue::Short(val)
    }
}
impl From<u16> for PLYValue {
    fn from(val: u16) -> Self {
        PLYValue::Ushort(val)
    }
}
impl From<i32> for PLYValue {
    fn from(val: i32) -> Self {
        PLYValue::Int(val)
    }
}
impl From<u32> for PLYValue {
    fn from(val: u32) -> Self {
        PLYValue::Uint(val)
    }
}
impl From<f32> for PLYValue {
    fn from(val: f32) -> Self {
        PLYValue::Float(val)
    }
}
impl From<f64> for PLYValue {
    fn from(val: f64) -> Self {
        PLYValue::Double(val)
    }
}
