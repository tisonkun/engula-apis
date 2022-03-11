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

tonic::include_proto!("engula.v1alpha");

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

impl From<Value> for () {
    fn from(_: Value) -> Self {
        ()
    }
}

impl From<ValueUnion> for () {
    fn from(_: ValueUnion) -> Self {
        ()
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

impl TryFrom<ValueUnion> for i64 {
    type Error = ValueUnion;

    fn try_from(v: ValueUnion) -> Result<Self, Self::Error> {
        if let Some(v) = v.value {
            v.try_into().map_err(|v| ValueUnion { value: Some(v) })
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

impl From<HashMap<Vec<u8>, i64>> for Value {
    fn from(map: HashMap<Vec<u8>, i64>) -> Self {
        let (keys, values) = map.into_iter().fold(
            (Vec::new(), Vec::new()),
            |(mut keys, mut values), (k, v)| {
                keys.push(k);
                values.push(v);
                (keys, values)
            },
        );
        MapValue {
            keys: Some(keys.into()),
            values: Some(values.into()),
        }
        .into()
    }
}

impl<T: Into<ListValue>> From<T> for Value {
    fn from(v: T) -> Self {
        Self::ListValue(v.into())
    }
}

impl From<Vec<i64>> for ListValue {
    fn from(v: Vec<i64>) -> Self {
        ListValue {
            i64_value: v,
            ..Default::default()
        }
    }
}

impl From<Vec<Vec<u8>>> for ListValue {
    fn from(v: Vec<Vec<u8>>) -> Self {
        ListValue {
            blob_value: v,
            ..Default::default()
        }
    }
}

impl From<Option<select_expr::Expr>> for SelectExpr {
    fn from(expr: Option<select_expr::Expr>) -> Self {
        Self { expr }
    }
}

impl From<Option<mutate_expr::Expr>> for MutateExpr {
    fn from(expr: Option<mutate_expr::Expr>) -> Self {
        Self { expr }
    }
}
