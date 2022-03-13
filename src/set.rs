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

use std::collections::{BTreeSet, HashSet};

use crate::v1::*;

impl From<SetValue> for Value {
    fn from(v: SetValue) -> Self {
        Self::SetValue(v)
    }
}

impl<T: Into<ListValue>> From<T> for SetValue {
    fn from(v: T) -> Self {
        Self {
            keys: Some(v.into()),
        }
    }
}

macro_rules! impl_set {
    ($set_type:ty, $value_type:ty) => {
        impl From<$set_type> for SetValue {
            fn from(set: $set_type) -> Self {
                let keys: Vec<$value_type> = set.into_iter().collect();
                keys.into()
            }
        }

        impl TryFrom<SetValue> for $set_type {
            type Error = SetValue;

            fn try_from(v: SetValue) -> Result<Self, Self::Error> {
                if let Some(keys) = v.keys {
                    keys.try_into().map_err(|v: ListValue| v.into())
                } else {
                    Err(v)
                }
            }
        }

        impl TryFrom<ListValue> for $set_type {
            type Error = ListValue;

            fn try_from(v: ListValue) -> Result<Self, Self::Error> {
                let list: Vec<$value_type> = v.try_into()?;
                Ok(Self::from_iter(list))
            }
        }

        impl TryFrom<TypedValue> for $set_type {
            type Error = TypedValue;

            fn try_from(v: TypedValue) -> Result<Self, Self::Error> {
                if let Some(Value::SetValue(v)) = v.value {
                    v.try_into().map_err(|v| Value::SetValue(v).into())
                } else {
                    Err(v)
                }
            }
        }

        impl TryFrom<TypedValue> for Option<$set_type> {
            type Error = TypedValue;

            fn try_from(v: TypedValue) -> Result<Self, Self::Error> {
                if let Some(v) = v.value {
                    if let Value::SetValue(v) = v {
                        v.try_into()
                            .map(Some)
                            .map_err(|v| Value::SetValue(v).into())
                    } else {
                        Err(v.into())
                    }
                } else {
                    Ok(None)
                }
            }
        }
    };
}

macro_rules! impl_sets {
    ($value_type:ty) => {
        impl_set!(HashSet<$value_type>, $value_type);
        impl_set!(BTreeSet<$value_type>, $value_type);
    };
}

impl_sets!(i64);
impl_sets!(Vec<u8>);
impl_sets!(String);
