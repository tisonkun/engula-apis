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

use std::ops::{
    Bound, Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};

use crate::v1::*;

impl RangeValue {
    pub fn from_bounds<T>(range: impl RangeBounds<T>) -> Self
    where
        T: Clone,
        (Bound<T>, Bound<T>): Into<RangeValue>,
    {
        (range.start_bound().cloned(), range.end_bound().cloned())
            .into()
            .into()
    }
}

impl From<RangeFull> for RangeValue {
    fn from(_: RangeFull) -> Self {
        Self::default()
    }
}

macro_rules! impl_type {
    ($rust_type:ty, $range_type:path) => {
        impl From<$rust_type> for range_bound::Value {
            fn from(v: $rust_type) -> Self {
                $range_type(v)
            }
        }

        impl TryFrom<range_bound::Value> for $rust_type {
            type Error = range_bound::Value;

            fn try_from(v: range_bound::Value) -> Result<Self, Self::Error> {
                if let $range_type(v) = v {
                    Ok(v)
                } else {
                    Err(v)
                }
            }
        }

        impl From<Bound<$rust_type>> for RangeBound {
            fn from(v: Bound<$rust_type>) -> Self {
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

        impl TryFrom<RangeBound> for Bound<$rust_type> {
            type Error = RangeBound;

            fn try_from(b: RangeBound) -> Result<Self, Self::Error> {
                match b.value {
                    Some($range_type(v)) => {
                        if b.included {
                            Ok(Bound::Included(v))
                        } else {
                            Ok(Bound::Excluded(v))
                        }
                    }
                    None => Ok(Bound::Unbounded),
                    _ => Err(b),
                }
            }
        }

        impl From<Range<$rust_type>> for RangeValue {
            fn from(r: Range<$rust_type>) -> Self {
                (Bound::Included(r.start), Bound::Excluded(r.end)).into()
            }
        }

        impl From<Range<$rust_type>> for value::Value {
            fn from(r: Range<$rust_type>) -> Self {
                RangeValue::from(r).into()
            }
        }

        impl From<RangeFrom<$rust_type>> for RangeValue {
            fn from(r: RangeFrom<$rust_type>) -> Self {
                (Bound::Included(r.start), Bound::Unbounded).into()
            }
        }

        impl From<RangeFrom<$rust_type>> for value::Value {
            fn from(r: RangeFrom<$rust_type>) -> Self {
                RangeValue::from(r).into()
            }
        }

        impl From<RangeInclusive<$rust_type>> for RangeValue {
            fn from(r: RangeInclusive<$rust_type>) -> Self {
                (
                    Bound::Included(r.start().clone()),
                    Bound::Included(r.end().clone()),
                )
                    .into()
            }
        }

        impl From<RangeInclusive<$rust_type>> for value::Value {
            fn from(r: RangeInclusive<$rust_type>) -> Self {
                RangeValue::from(r).into()
            }
        }

        impl From<RangeTo<$rust_type>> for RangeValue {
            fn from(r: RangeTo<$rust_type>) -> Self {
                (Bound::Unbounded, Bound::Excluded(r.end)).into()
            }
        }

        impl From<RangeTo<$rust_type>> for value::Value {
            fn from(r: RangeTo<$rust_type>) -> Self {
                RangeValue::from(r).into()
            }
        }

        impl From<RangeToInclusive<$rust_type>> for RangeValue {
            fn from(r: RangeToInclusive<$rust_type>) -> Self {
                (Bound::Unbounded, Bound::Included(r.end)).into()
            }
        }

        impl From<RangeToInclusive<$rust_type>> for value::Value {
            fn from(r: RangeToInclusive<$rust_type>) -> Self {
                RangeValue::from(r).into()
            }
        }

        impl From<(Bound<$rust_type>, Bound<$rust_type>)> for RangeValue {
            fn from(r: (Bound<$rust_type>, Bound<$rust_type>)) -> Self {
                Self {
                    start: Some(r.0.into()),
                    end: Some(r.1.into()),
                }
            }
        }

        impl From<(Bound<$rust_type>, Bound<$rust_type>)> for value::Value {
            fn from(r: (Bound<$rust_type>, Bound<$rust_type>)) -> Self {
                RangeValue::from(r).into()
            }
        }

        impl TryFrom<RangeValue> for (Bound<$rust_type>, Bound<$rust_type>) {
            type Error = ();

            fn try_from(r: RangeValue) -> Result<Self, Self::Error> {
                let start = if let Some(v) = r.start {
                    v.try_into().map_err(|_| ())?
                } else {
                    Bound::Unbounded
                };
                let end = if let Some(v) = r.end {
                    v.try_into().map_err(|_| ())?
                } else {
                    Bound::Unbounded
                };
                Ok((start, end))
            }
        }

        impl TryFrom<value::Value> for (Bound<$rust_type>, Bound<$rust_type>) {
            type Error = ();

            fn try_from(v: value::Value) -> Result<Self, Self::Error> {
                if let value::Value::RangeValue(v) = v {
                    v.try_into()
                } else {
                    Err(())
                }
            }
        }

        impl TryFrom<Value> for (Bound<$rust_type>, Bound<$rust_type>) {
            type Error = ();

            fn try_from(v: Value) -> Result<Self, Self::Error> {
                if let Some(v) = v.value {
                    v.try_into()
                } else {
                    Err(())
                }
            }
        }
    };
}

impl_type!(i64, range_bound::Value::I64Value);
impl_type!(Vec<u8>, range_bound::Value::BlobValue);
impl_type!(String, range_bound::Value::TextValue);
