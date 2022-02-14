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

impl From<&[u8]> for Value {
    fn from(v: &[u8]) -> Self {
        Self {
            value: Some(value::Value::BlobValue(v.to_owned())),
        }
    }
}

impl From<Vec<u8>> for Value {
    fn from(v: Vec<u8>) -> Self {
        Self {
            value: Some(value::Value::BlobValue(v)),
        }
    }
}

impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Self {
            value: Some(value::Value::TextValue(v.to_owned())),
        }
    }
}

impl From<String> for Value {
    fn from(v: String) -> Self {
        Self {
            value: Some(value::Value::TextValue(v)),
        }
    }
}

impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Self {
            value: Some(value::Value::Int64Value(v)),
        }
    }
}

impl From<ListValue> for Value {
    fn from(v: ListValue) -> Self {
        Self {
            value: Some(value::Value::ListValue(v)),
        }
    }
}

impl From<Vec<Value>> for Value {
    fn from(v: Vec<Value>) -> Self {
        ListValue {
            values: v,
            ..Default::default()
        }
        .into()
    }
}

impl From<Vec<Vec<u8>>> for Value {
    fn from(v: Vec<Vec<u8>>) -> Self {
        ListValue {
            blob_values: v,
            ..Default::default()
        }
        .into()
    }
}

impl From<Vec<String>> for Value {
    fn from(v: Vec<String>) -> Self {
        ListValue {
            text_values: v,
            ..Default::default()
        }
        .into()
    }
}

impl From<Vec<i64>> for Value {
    fn from(v: Vec<i64>) -> Self {
        ListValue {
            int64_values: v,
            ..Default::default()
        }
        .into()
    }
}
