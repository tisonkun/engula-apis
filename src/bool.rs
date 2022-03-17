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

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Self::I64Value(v as i64)
    }
}

impl TryFrom<Value> for bool {
    type Error = Value;

    fn try_from(v: Value) -> Result<Self, Self::Error> {
        if let Value::I64Value(v) = v {
            Ok(v != 0)
        } else {
            Err(v)
        }
    }
}

impl TryFrom<TypedValue> for bool {
    type Error = TypedValue;

    fn try_from(v: TypedValue) -> Result<Self, Self::Error> {
        if let Some(Value::I64Value(v)) = v.value {
            Ok(v != 0)
        } else {
            Err(v)
        }
    }
}

impl TryFrom<TypedValue> for Option<bool> {
    type Error = TypedValue;

    fn try_from(v: TypedValue) -> Result<Self, Self::Error> {
        if let Some(v) = v.value {
            v.try_into().map(Some).map_err(|v| TypedValue::from(v))
        } else {
            Ok(None)
        }
    }
}

impl From<Vec<bool>> for ListValue {
    fn from(v: Vec<bool>) -> Self {
        let v: Vec<i64> = v.into_iter().map(|v| v as i64).collect();
        v.into()
    }
}

impl TryFrom<ListValue> for Vec<bool> {
    type Error = ListValue;

    fn try_from(v: ListValue) -> Result<Self, Self::Error> {
        let v: Vec<i64> = v.try_into()?;
        Ok(v.into_iter().map(|v| v != 0).collect())
    }
}
