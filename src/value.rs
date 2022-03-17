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

impl From<TypedValue> for () {
    fn from(_: TypedValue) -> Self {
        ()
    }
}

impl From<()> for TypedValue {
    fn from(_: ()) -> Self {
        Self::default()
    }
}

impl From<TypedValue> for Option<Value> {
    fn from(v: TypedValue) -> Self {
        v.value
    }
}

impl<T: Into<Value>> From<T> for TypedValue {
    fn from(v: T) -> Self {
        Self {
            value: Some(v.into()),
        }
    }
}

impl<T: Into<Value>> From<Option<T>> for TypedValue {
    fn from(v: Option<T>) -> Self {
        Self {
            value: v.map(|v| v.into()),
        }
    }
}
macro_rules! impl_type {
    ($rust_type:ty, $value_type:path) => {
        impl From<$rust_type> for Value {
            fn from(v: $rust_type) -> Self {
                $value_type(v)
            }
        }

        impl TryFrom<TypedValue> for $rust_type {
            type Error = TypedValue;

            fn try_from(v: TypedValue) -> Result<Self, Self::Error> {
                if let Some(v) = v.value {
                    v.try_into().map_err(|v| TypedValue::from(v))
                } else {
                    Err(v)
                }
            }
        }

        impl TryFrom<TypedValue> for Option<$rust_type> {
            type Error = TypedValue;

            fn try_from(v: TypedValue) -> Result<Self, Self::Error> {
                if let Some(v) = v.value {
                    v.try_into().map(Some).map_err(|v| TypedValue::from(v))
                } else {
                    Ok(None)
                }
            }
        }
    };
}

macro_rules! impl_simple_type {
    ($rust_type:ty, $value_type:path) => {
        impl_type!($rust_type, $value_type);

        impl TryFrom<Value> for $rust_type {
            type Error = Value;

            fn try_from(v: Value) -> Result<Self, Self::Error> {
                match v {
                    $value_type(v) => Ok(v),
                    Value::ListValue(v) => v.try_into().map_err(|v| Value::from(v)),
                    _ => Err(v),
                }
            }
        }
    };
}

macro_rules! impl_complex_type {
    ($rust_type:ty, $value_type:path) => {
        impl_type!($rust_type, $value_type);

        impl TryFrom<Value> for $rust_type {
            type Error = Value;

            fn try_from(v: Value) -> Result<Self, Self::Error> {
                if let $value_type(v) = v {
                    Ok(v)
                } else {
                    Err(v)
                }
            }
        }
    };
}

impl_simple_type!(i64, Value::I64Value);
impl_simple_type!(f64, Value::F64Value);
impl_simple_type!(Vec<u8>, Value::BlobValue);
impl_simple_type!(String, Value::TextValue);
impl_complex_type!(MapValue, Value::MapValue);
impl_complex_type!(SetValue, Value::SetValue);
impl_complex_type!(ListValue, Value::ListValue);
impl_complex_type!(RangeValue, Value::RangeValue);
