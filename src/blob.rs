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

impl From<&[u8]> for Value {
    fn from(v: &[u8]) -> Self {
        Vec::from(v).into()
    }
}

impl<const N: usize> From<&'_ [u8; N]> for Value {
    fn from(v: &'_ [u8; N]) -> Self {
        v.as_slice().into()
    }
}

impl<const N: usize> From<[u8; N]> for Value {
    fn from(v: [u8; N]) -> Self {
        Vec::from(v).into()
    }
}

impl From<Vec<u8>> for Value {
    fn from(v: Vec<u8>) -> Self {
        Value::BlobValue(v)
    }
}

impl TryFrom<TypedValue> for Vec<u8> {
    type Error = TypedValue;

    fn try_from(v: TypedValue) -> Result<Self, Self::Error> {
        if let Some(Value::BlobValue(v)) = v.value {
            Ok(v)
        } else {
            Err(v)
        }
    }
}

impl TryFrom<TypedValue> for Option<Vec<u8>> {
    type Error = TypedValue;

    fn try_from(v: TypedValue) -> Result<Self, Self::Error> {
        if let Some(v) = v.value {
            if let Value::BlobValue(v) = v {
                Ok(Some(v))
            } else {
                Err(v.into())
            }
        } else {
            Ok(None)
        }
    }
}
