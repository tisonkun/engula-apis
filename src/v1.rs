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

pub use crate::range::range_value;

pub type Expr = typed_expr::Expr;
pub type Value = typed_value::Value;

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

impl From<bool> for TypedValue {
    fn from(v: bool) -> Self {
        if v { Some(Value::I64Value(1)) } else { None }.into()
    }
}

impl From<TypedValue> for bool {
    fn from(v: TypedValue) -> Self {
        v.value.is_some()
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
