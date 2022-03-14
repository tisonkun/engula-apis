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

#![allow(clippy::all)]

tonic::include_proto!("engula.v1");

mod blob;
mod f64;
mod i64;
mod list;
mod map;
mod range;
mod set;
mod text;
mod typed_expr;
pub mod v1;

use std::collections::HashMap;

pub type Value = value_union::Value;

impl<T: Into<Value>> From<T> for ValueUnion {
    fn from(v: T) -> Self {
        Self {
            value: Some(v.into()),
        }
    }
}

impl From<Option<Value>> for ValueUnion {
    fn from(v: Option<Value>) -> Self {
        Self { value: v }
    }
}

impl From<ValueUnion> for Option<Value> {
    fn from(v: ValueUnion) -> Self {
        v.value
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Self::I64Value(v)
    }
}

impl TryFrom<Value> for i64 {
    type Error = Value;

    fn try_from(v: Value) -> Result<Self, Self::Error> {
        if let Value::I64Value(v) = v {
            Ok(v)
        } else {
            Err(v)
        }
    }
}

impl From<&[u8]> for Value {
    fn from(v: &[u8]) -> Self {
        Self::BlobValue(v.to_owned())
    }
}

impl From<Vec<u8>> for Value {
    fn from(v: Vec<u8>) -> Self {
        Self::BlobValue(v)
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Self::TextValue(v.to_owned())
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Self::TextValue(v)
    }
}

impl From<MapValue> for Value {
    fn from(v: MapValue) -> Self {
        Value::MapValue(v)
    }
}

impl<K, V> From<HashMap<K, V>> for Value
where
    K: Into<Value> + Ord,
    V: Into<Value>,
{
    fn from(map: HashMap<K, V>) -> Self {
        let (keys, values) = map.into_iter().fold(
            (Vec::new(), Vec::new()),
            |(mut keys, mut values), (k, v)| {
                keys.push(k.into().into());
                values.push(v.into().into());
                (keys, values)
            },
        );
        MapValue { keys, values }.into()
    }
}

impl From<ListValue> for Value {
    fn from(v: ListValue) -> Self {
        Self::ListValue(v)
    }
}

impl<T> From<Vec<T>> for Value
where
    T: Into<Value>,
{
    fn from(values: Vec<T>) -> Self {
        ListValue {
            values: values.into_iter().map(|v| v.into().into()).collect(),
        }
        .into()
    }
}
