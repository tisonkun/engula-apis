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

use std::collections::{BTreeMap, HashMap};

use crate::v1::*;

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

impl TryFrom<MapValue> for (ListValue, ListValue) {
    type Error = MapValue;

    fn try_from(v: MapValue) -> Result<Self, Self::Error> {
        match (v.keys, v.values) {
            (Some(keys), Some(values)) => Ok((keys, values)),
            (Some(keys), None) => Err(MapValue {
                keys: Some(keys),
                values: None,
            }),
            (None, Some(values)) => Err(MapValue {
                keys: None,
                values: Some(values),
            }),
            (None, None) => Err(MapValue::default()),
        }
    }
}

macro_rules! impl_map {
    ($map_type:ty, $key_type:ty, $value_type:ty) => {
        impl From<$map_type> for MapValue {
            fn from(map: $map_type) -> Self {
                let mut keys = Vec::with_capacity(map.len());
                let mut values = Vec::with_capacity(map.len());
                for (k, v) in map {
                    keys.push(k);
                    values.push(v);
                }
                (keys, values).into()
            }
        }

        impl TryFrom<MapValue> for $map_type {
            type Error = MapValue;

            fn try_from(v: MapValue) -> Result<Self, Self::Error> {
                match (v.keys, v.values) {
                    (Some(keys), Some(values)) => {
                        let keys: Result<Vec<$key_type>, _> = keys.try_into();
                        let values: Result<Vec<$value_type>, _> = values.try_into();
                        match (keys, values) {
                            (Ok(keys), Ok(values)) => {
                                if keys.len() == values.len() {
                                    Ok(keys.into_iter().zip(values.into_iter()).collect())
                                } else {
                                    Err((keys, values).into())
                                }
                            }
                            (Ok(keys), Err(values)) => Err((keys, values).into()),
                            (Err(keys), Ok(values)) => Err((keys, values).into()),
                            (Err(keys), Err(values)) => Err((keys, values).into()),
                        }
                    }
                    (Some(keys), None) => Err(MapValue {
                        keys: Some(keys),
                        values: None,
                    }
                    .into()),
                    (None, Some(values)) => Err(MapValue {
                        keys: None,
                        values: Some(values),
                    }
                    .into()),
                    (None, None) => Err(MapValue::default().into()),
                }
            }
        }

        impl TryFrom<Value> for $map_type {
            type Error = Value;

            fn try_from(v: Value) -> Result<Self, Self::Error> {
                if let Value::MapValue(v) = v {
                    v.try_into().map_err(|v: MapValue| v.into())
                } else {
                    Err(v)
                }
            }
        }

        impl TryFrom<TypedValue> for $map_type {
            type Error = TypedValue;

            fn try_from(v: TypedValue) -> Result<Self, Self::Error> {
                if let Some(v) = v.value {
                    v.try_into().map_err(|v: Value| v.into())
                } else {
                    Err(v)
                }
            }
        }

        impl TryFrom<TypedValue> for Option<$map_type> {
            type Error = TypedValue;

            fn try_from(v: TypedValue) -> Result<Self, Self::Error> {
                if let Some(v) = v.value {
                    v.try_into().map(Some).map_err(|v: Value| v.into())
                } else {
                    Ok(None)
                }
            }
        }
    };
}

macro_rules! impl_map_type {
    ($key_type:ty, $value_type:ty) => {
        impl_map!(HashMap<$key_type, $value_type>, $key_type, $value_type);
        impl_map!(BTreeMap<$key_type, $value_type>, $key_type, $value_type);

        impl<const N: usize> From<[($key_type, $value_type); N]> for MapValue {
            fn from(map: [($key_type, $value_type); N]) -> Self {
                HashMap::from(map).into()
            }
        }
    }
}

impl_map_type!(i64, i64);
impl_map_type!(i64, bool);
impl_map_type!(i64, Vec<u8>);
impl_map_type!(i64, String);
impl_map_type!(Vec<u8>, i64);
impl_map_type!(Vec<u8>, bool);
impl_map_type!(Vec<u8>, Vec<u8>);
impl_map_type!(Vec<u8>, String);
impl_map_type!(String, i64);
impl_map_type!(String, bool);
impl_map_type!(String, Vec<u8>);
impl_map_type!(String, String);
