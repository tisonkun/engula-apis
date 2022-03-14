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

use std::ops::{Bound, RangeBounds};

use prost::Message;

use crate::v1::*;

impl From<RangeValue> for Value {
    fn from(v: RangeValue) -> Self {
        Self::RangeValue(v)
    }
}

impl TryFrom<TypedValue> for RangeValue {
    type Error = TypedValue;

    fn try_from(v: TypedValue) -> Result<Self, Self::Error> {
        if let Some(Value::RangeValue(v)) = v.value {
            Ok(v)
        } else {
            Err(v)
        }
    }
}

impl TryFrom<TypedValue> for Option<RangeValue> {
    type Error = TypedValue;

    fn try_from(v: TypedValue) -> Result<Self, Self::Error> {
        if let Some(v) = v.value {
            if let Value::RangeValue(v) = v {
                Ok(Some(v))
            } else {
                Err(v.into())
            }
        } else {
            Ok(None)
        }
    }
}

pub fn range_value<T>(r: impl RangeBounds<T>) -> RangeValue
where
    T: Clone + Into<TypedValue>,
{
    let mut v = RangeValue::default();
    match r.start_bound().cloned() {
        Bound::Included(start) => {
            v.start = start.into().encode_to_vec();
            v.start_bound = RangeBound::Included as i32;
        }
        Bound::Excluded(start) => {
            v.start = start.into().encode_to_vec();
            v.start_bound = RangeBound::Excluded as i32;
        }
        Bound::Unbounded => {
            v.start_bound = RangeBound::Unbounded as i32;
        }
    }
    match r.end_bound().cloned() {
        Bound::Included(end) => {
            v.end = end.into().encode_to_vec();
            v.end_bound = RangeBound::Included as i32;
        }
        Bound::Excluded(end) => {
            v.end = end.into().encode_to_vec();
            v.end_bound = RangeBound::Excluded as i32;
        }
        Bound::Unbounded => {
            v.end_bound = RangeBound::Unbounded as i32;
        }
    }
    v
}
