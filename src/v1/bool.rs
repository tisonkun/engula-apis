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

impl From<bool> for value::Value {
    fn from(v: bool) -> Self {
        Self::I64Value(v as i64)
    }
}

impl TryFrom<value::Value> for bool {
    type Error = value::Value;

    fn try_from(v: value::Value) -> Result<Self, Self::Error> {
        if let value::Value::I64Value(v) = v {
            Ok(v != 0)
        } else {
            Err(v)
        }
    }
}

impl TryFrom<Value> for bool {
    type Error = Value;

    fn try_from(v: Value) -> Result<Self, Self::Error> {
        if let Some(value::Value::I64Value(v)) = v.value {
            Ok(v != 0)
        } else {
            Err(v)
        }
    }
}

impl TryFrom<Value> for Option<bool> {
    type Error = Value;

    fn try_from(v: Value) -> Result<Self, Self::Error> {
        if let Some(v) = v.value {
            if let value::Value::I64Value(v) = v {
                Ok(Some(v != 0))
            } else {
                Err(v.into())
            }
        } else {
            Ok(None)
        }
    }
}
