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

use std::{
    collections::HashMap,
    ops::{Bound, RangeBounds},
};

use prost::Message;

pub type Value = typed_value::Value;

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

impl From<Value> for () {
    fn from(_: Value) -> Self {
        ()
    }
}

impl From<TypedValue> for () {
    fn from(_: TypedValue) -> Self {
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

impl TryFrom<TypedValue> for i64 {
    type Error = TypedValue;

    fn try_from(v: TypedValue) -> Result<Self, Self::Error> {
        if let Some(v) = v.value {
            v.try_into().map_err(|v| TypedValue { value: Some(v) })
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

impl TryFrom<Value> for Vec<u8> {
    type Error = Value;

    fn try_from(v: Value) -> Result<Self, Self::Error> {
        if let Value::BlobValue(v) = v {
            Ok(v)
        } else {
            Err(v)
        }
    }
}

impl TryFrom<TypedValue> for Vec<u8> {
    type Error = TypedValue;

    fn try_from(v: TypedValue) -> Result<Self, Self::Error> {
        if let Some(v) = v.value {
            v.try_into().map_err(|v| TypedValue { value: Some(v) })
        } else {
            Err(v)
        }
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

impl<K, V> From<(K, V)> for MapValue
where
    K: Into<ListValue>,
    V: Into<ListValue>,
{
    fn from((keys, values): (K, V)) -> Self {
        Self {
            keys: Some(keys.into()),
            values: Some(values.into()),
        }
    }
}

impl From<HashMap<Vec<u8>, i64>> for MapValue {
    fn from(map: HashMap<Vec<u8>, i64>) -> Self {
        let (keys, values) = map.into_iter().fold(
            (Vec::new(), Vec::new()),
            |(mut keys, mut values), (k, v)| {
                keys.push(k);
                values.push(v);
                (keys, values)
            },
        );
        (keys, values).into()
    }
}

impl From<ListValue> for Value {
    fn from(v: ListValue) -> Self {
        Value::ListValue(v)
    }
}

impl From<Vec<i64>> for ListValue {
    fn from(v: Vec<i64>) -> Self {
        Self {
            i64_value: v,
            ..Default::default()
        }
    }
}

impl TryFrom<Value> for Vec<i64> {
    type Error = Value;

    fn try_from(v: Value) -> Result<Self, Self::Error> {
        if let Value::ListValue(v) = v {
            if !v.i64_value.is_empty() || v.encoded_len() == 0 {
                Ok(v.i64_value)
            } else {
                Err(Value::ListValue(v))
            }
        } else {
            Err(v)
        }
    }
}

impl TryFrom<TypedValue> for Vec<i64> {
    type Error = TypedValue;

    fn try_from(v: TypedValue) -> Result<Self, Self::Error> {
        if let Some(v) = v.value {
            v.try_into().map_err(|v| TypedValue { value: Some(v) })
        } else {
            Err(v)
        }
    }
}

impl From<Vec<Vec<u8>>> for ListValue {
    fn from(v: Vec<Vec<u8>>) -> Self {
        Self {
            blob_value: v,
            ..Default::default()
        }
    }
}

impl TryFrom<Value> for Vec<Vec<u8>> {
    type Error = Value;

    fn try_from(v: Value) -> Result<Self, Self::Error> {
        if let Value::ListValue(v) = v {
            if !v.blob_value.is_empty() || v.encoded_len() == 0 {
                Ok(v.blob_value)
            } else {
                Err(Value::ListValue(v))
            }
        } else {
            Err(v)
        }
    }
}

impl TryFrom<TypedValue> for Vec<Vec<u8>> {
    type Error = TypedValue;

    fn try_from(v: TypedValue) -> Result<Self, Self::Error> {
        if let Some(v) = v.value {
            v.try_into().map_err(|v| TypedValue { value: Some(v) })
        } else {
            Err(v)
        }
    }
}

impl From<typed_expr::Expr> for TypedExpr {
    fn from(expr: typed_expr::Expr) -> Self {
        Self { expr: Some(expr) }
    }
}

impl From<AnyExpr> for TypedExpr {
    fn from(expr: AnyExpr) -> Self {
        typed_expr::Expr::AnyExpr(expr).into()
    }
}

impl From<I64Expr> for TypedExpr {
    fn from(expr: I64Expr) -> Self {
        typed_expr::Expr::I64Expr(expr).into()
    }
}

impl From<BlobExpr> for TypedExpr {
    fn from(expr: BlobExpr) -> Self {
        typed_expr::Expr::BlobExpr(expr).into()
    }
}

impl From<ListExpr> for TypedExpr {
    fn from(expr: ListExpr) -> Self {
        typed_expr::Expr::ListExpr(expr).into()
    }
}

impl<R: RangeBounds<i64>> From<R> for RangeExpr {
    fn from(range: R) -> Self {
        let mut expr = Self::default();
        match range.start_bound() {
            Bound::Included(start) => {
                expr.start = Some(start.to_owned().into());
                expr.start_bound = RangeBound::Included as i32;
            }
            Bound::Excluded(end) => {
                expr.start = Some(end.to_owned().into());
                expr.start_bound = RangeBound::Excluded as i32;
            }
            Bound::Unbounded => {
                expr.start_bound = RangeBound::Unbounded as i32;
            }
        }
        match range.end_bound() {
            Bound::Included(end) => {
                expr.end = Some(end.to_owned().into());
                expr.end_bound = RangeBound::Included as i32;
            }
            Bound::Excluded(end) => {
                expr.end = Some(end.to_owned().into());
                expr.end_bound = RangeBound::Excluded as i32;
            }
            Bound::Unbounded => {
                expr.end_bound = RangeBound::Unbounded as i32;
            }
        }
        expr
    }
}
