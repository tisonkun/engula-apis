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

impl<T: Into<Expr>> From<T> for TypedExpr {
    fn from(expr: T) -> Self {
        Self {
            expr: Some(expr.into()),
        }
    }
}

impl From<I64Expr> for Expr {
    fn from(expr: I64Expr) -> Self {
        Expr::I64Expr(expr)
    }
}

impl From<F64Expr> for Expr {
    fn from(expr: F64Expr) -> Self {
        Expr::F64Expr(expr)
    }
}

impl From<BlobExpr> for Expr {
    fn from(expr: BlobExpr) -> Self {
        Expr::BlobExpr(expr)
    }
}

impl From<TextExpr> for Expr {
    fn from(expr: TextExpr) -> Self {
        Expr::TextExpr(expr)
    }
}

impl From<ListExpr> for Expr {
    fn from(expr: ListExpr) -> Self {
        Expr::ListExpr(expr)
    }
}

impl From<MapExpr> for Expr {
    fn from(expr: MapExpr) -> Self {
        Expr::MapExpr(expr)
    }
}

impl From<SetExpr> for Expr {
    fn from(expr: SetExpr) -> Self {
        Expr::SetExpr(expr)
    }
}

impl From<AnyExpr> for Expr {
    fn from(expr: AnyExpr) -> Self {
        Expr::AnyExpr(expr)
    }
}
