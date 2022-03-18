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

use crate::v1::*;

pub type BoundValue = range_bound::Value;

macro_rules! impl_bound {
    ($rust_type:ty, $value_type:path) => {
        impl From<$rust_type> for BoundValue {
            fn from(v: $rust_type) -> Self {
                $value_type(v)
            }
        }

        impl TryFrom<BoundValue> for $rust_type {
            type Error = BoundValue;

            fn try_from(v: BoundValue) -> Result<Self, Self::Error> {
                if let $value_type(v) = v {
                    Ok(v)
                } else {
                    Err(v)
                }
            }
        }
    };
}

impl_bound!(i64, BoundValue::I64Value);
impl_bound!(f64, BoundValue::F64Value);
impl_bound!(Vec<u8>, BoundValue::BlobValue);
impl_bound!(String, BoundValue::TextValue);

impl<T: Into<BoundValue>> From<Bound<T>> for RangeBound {
    fn from(v: Bound<T>) -> Self {
        match v {
            Bound::Included(v) => Self {
                value: Some(v.into()),
                included: true,
            },
            Bound::Excluded(v) => Self {
                value: Some(v.into()),
                included: false,
            },
            Bound::Unbounded => Self::default(),
        }
    }
}

impl<T> TryFrom<RangeBound> for Bound<T>
where
    T: TryFrom<BoundValue, Error = BoundValue>,
{
    type Error = RangeBound;

    fn try_from(v: RangeBound) -> Result<Self, Self::Error> {
        let included = v.included;
        if let Some(v) = v.value {
            if included {
                match v.try_into() {
                    Ok(v) => Ok(Bound::Included(v)),
                    Err(v) => Err(Bound::Included(v).into()),
                }
            } else {
                match v.try_into() {
                    Ok(v) => Ok(Bound::Excluded(v)),
                    Err(v) => Err(Bound::Excluded(v).into()),
                }
            }
        } else {
            Ok(Bound::Unbounded)
        }
    }
}

impl<T: Into<RangeBound>> From<(T, T)> for RangeValue {
    fn from(v: (T, T)) -> Self {
        Self {
            start: Some(v.0.into()),
            end: Some(v.1.into()),
        }
    }
}

impl<T> TryFrom<RangeValue> for (Bound<T>, Bound<T>)
where
    Bound<T>: TryFrom<RangeBound, Error = RangeBound> + Into<RangeBound>,
{
    type Error = RangeValue;

    fn try_from(v: RangeValue) -> Result<Self, Self::Error> {
        match (v.start, v.end) {
            (Some(start), Some(end)) => match (start.try_into(), end.try_into()) {
                (Ok(start), Ok(end)) => Ok((start, end)),
                (Ok(start), Err(end)) => Err((start.into(), end).into()),
                (Err(start), Ok(end)) => Err((start, end.into()).into()),
                (Err(start), Err(end)) => Err((start, end).into()),
            },
            (Some(start), None) => match start.try_into() {
                Ok(start) => Ok((start, Bound::Unbounded)),
                Err(start) => Err((start, RangeBound::default()).into()),
            },
            (None, Some(end)) => match end.try_into() {
                Ok(end) => Ok((Bound::Unbounded, end)),
                Err(end) => Err((RangeBound::default(), end).into()),
            },
            (None, None) => Ok((Bound::Unbounded, Bound::Unbounded)),
        }
    }
}

impl<T> TryFrom<TypedValue> for (Bound<T>, Bound<T>)
where
    (Bound<T>, Bound<T>): TryFrom<RangeValue, Error = RangeValue>,
{
    type Error = TypedValue;

    fn try_from(v: TypedValue) -> Result<Self, Self::Error> {
        if let Some(Value::RangeValue(v)) = v.value {
            v.try_into().map_err(|v: RangeValue| v.into())
        } else {
            Err(v)
        }
    }
}

pub fn range_bounds<T>(r: impl RangeBounds<T>) -> RangeValue
where
    T: Clone + Into<BoundValue>,
{
    (r.start_bound().cloned(), r.end_bound().cloned()).into()
}
