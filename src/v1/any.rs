// Copyright 2022 The Engula Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::v1::*;

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Self::default()
    }
}

impl From<Value> for () {
    fn from(_: Value) -> Self {
        ()
    }
}

impl TryFrom<Value> for value::Value {
    type Error = Value;

    fn try_from(v: Value) -> Result<Self, Self::Error> {
        if let Some(v) = v.value {
            Ok(v)
        } else {
            Err(v)
        }
    }
}

impl<T: Into<value::Value>> From<T> for Value {
    fn from(v: T) -> Self {
        Self {
            value: Some(v.into()),
        }
    }
}

impl From<Value> for Option<value::Value> {
    fn from(v: Value) -> Self {
        v.value
    }
}

impl<T: Into<value::Value>> From<Option<T>> for Value {
    fn from(v: Option<T>) -> Self {
        Self {
            value: v.map(|v| v.into()),
        }
    }
}

macro_rules! impl_type {
    ($rust_type:ty, $value_type:path) => {
        impl From<$rust_type> for value::Value {
            fn from(v: $rust_type) -> Self {
                $value_type(v)
            }
        }

        impl TryFrom<value::Value> for $rust_type {
            type Error = value::Value;

            fn try_from(v: value::Value) -> Result<Self, Self::Error> {
                if let $value_type(v) = v {
                    Ok(v)
                } else {
                    Err(v)
                }
            }
        }

        impl TryFrom<Value> for $rust_type {
            type Error = Value;

            fn try_from(v: Value) -> Result<Self, Self::Error> {
                if let Some($value_type(v)) = v.value {
                    Ok(v)
                } else {
                    Err(v)
                }
            }
        }

        impl TryFrom<Value> for Option<$rust_type> {
            type Error = Value;

            fn try_from(v: Value) -> Result<Self, Self::Error> {
                if let Some(v) = v.value {
                    if let $value_type(v) = v {
                        Ok(Some(v))
                    } else {
                        Err(v.into())
                    }
                } else {
                    Ok(None)
                }
            }
        }
    };
}

impl_type!(i64, value::Value::I64Value);
impl_type!(f64, value::Value::F64Value);
impl_type!(Vec<u8>, value::Value::BlobValue);
impl_type!(String, value::Value::TextValue);
impl_type!(ListValue, value::Value::ListValue);
impl_type!(MapValue, value::Value::MapValue);
impl_type!(SetValue, value::Value::SetValue);
impl_type!(RangeValue, value::Value::RangeValue);

impl From<&[u8]> for value::Value {
    fn from(v: &[u8]) -> Self {
        Vec::from(v).into()
    }
}

impl<const N: usize> From<[u8; N]> for value::Value {
    fn from(v: [u8; N]) -> Self {
        Vec::from(v).into()
    }
}

impl<const N: usize> From<&'_ [u8; N]> for value::Value {
    fn from(v: &'_ [u8; N]) -> Self {
        v.as_slice().into()
    }
}

impl From<&str> for value::Value {
    fn from(v: &str) -> Self {
        Self::TextValue(v.to_owned())
    }
}
