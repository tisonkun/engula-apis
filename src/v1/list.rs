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

use prost::Message;

use crate::v1::*;

macro_rules! impl_type {
    ($rust_type:ty, $list_value:ident) => {
        impl From<Vec<$rust_type>> for ListValue {
            fn from(v: Vec<$rust_type>) -> Self {
                Self {
                    $list_value: v,
                    ..Default::default()
                }
            }
        }

        impl From<Vec<$rust_type>> for value::Value {
            fn from(v: Vec<$rust_type>) -> Self {
                ListValue::from(v).into()
            }
        }

        impl From<&'_ [$rust_type]> for ListValue {
            fn from(v: &'_ [$rust_type]) -> Self {
                Vec::from(v).into()
            }
        }

        impl From<&'_ [$rust_type]> for value::Value {
            fn from(v: &'_ [$rust_type]) -> Self {
                ListValue::from(v).into()
            }
        }

        impl<const N: usize> From<[$rust_type; N]> for ListValue {
            fn from(v: [$rust_type; N]) -> Self {
                Vec::from(v).into()
            }
        }

        impl<const N: usize> From<[$rust_type; N]> for value::Value {
            fn from(v: [$rust_type; N]) -> Self {
                ListValue::from(v).into()
            }
        }

        impl<const N: usize> From<&'_ [$rust_type; N]> for ListValue {
            fn from(v: &'_ [$rust_type; N]) -> Self {
                v.as_slice().into()
            }
        }

        impl<const N: usize> From<&'_ [$rust_type; N]> for value::Value {
            fn from(v: &'_ [$rust_type; N]) -> Self {
                ListValue::from(v).into()
            }
        }

        impl TryFrom<ListValue> for Vec<$rust_type> {
            type Error = ListValue;

            fn try_from(v: ListValue) -> Result<Self, Self::Error> {
                if !v.$list_value.is_empty() || v.encoded_len() == 0 {
                    Ok(v.$list_value)
                } else {
                    Err(v)
                }
            }
        }

        impl TryFrom<value::Value> for Vec<$rust_type> {
            type Error = value::Value;

            fn try_from(v: value::Value) -> Result<Self, Self::Error> {
                if let value::Value::ListValue(v) = v {
                    v.try_into().map_err(|v| value::Value::from(v))
                } else {
                    Err(v)
                }
            }
        }

        impl TryFrom<Value> for Vec<$rust_type> {
            type Error = Value;

            fn try_from(v: Value) -> Result<Self, Self::Error> {
                if let Some(v) = v.value {
                    v.try_into().map_err(|v| Value::from(v))
                } else {
                    Err(v)
                }
            }
        }

        impl TryFrom<Value> for Option<Vec<$rust_type>> {
            type Error = Value;

            fn try_from(v: Value) -> Result<Self, Self::Error> {
                if let Some(v) = v.value {
                    v.try_into().map(Some).map_err(|v| Value::from(v))
                } else {
                    Ok(None)
                }
            }
        }
    };
}

impl_type!(i64, i64_value);
impl_type!(f64, f64_value);
impl_type!(Vec<u8>, blob_value);
impl_type!(String, text_value);

impl<const N: usize> From<[i32; N]> for ListValue {
    fn from(v: [i32; N]) -> Self {
        let v: Vec<i64> = v.into_iter().map(|v| v as i64).collect();
        v.into()
    }
}

impl<const N: usize> From<[i32; N]> for value::Value {
    fn from(v: [i32; N]) -> Self {
        ListValue::from(v).into()
    }
}
