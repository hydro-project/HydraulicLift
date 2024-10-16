use std::collections::{BTreeSet, HashSet};

use syn::Ident;

use crate::r_ast::*;
use crate::io::IO;

/// Pair of expression and inputs needed by expression
type Tagged<T> = (T, HashSet<Ident>);

/// defer_tagging!(value { Enum: Variant, Variant, ... })
/// will match value to the variants, and return the result of converting the value in the variant
macro_rules! defer_tagging {
    ($item:ident { $enum:ident :: $( $variant:ident ),* }) => {
        match $item {
            $($enum :: $variant (inner) => {
                let (new_value, variables) = inner.into();
                ($enum :: $variant (new_value), variables)
            }),*
        }
    };
}

/// Takes in an unlabeled AST nodes
/// Returns the same node with IO labels, and passes the required inputs up

impl From<RExpr<()>> for Tagged<RExpr<IO>> {
    fn from(value: RExpr<()>) -> Self {
        defer_tagging!(value { RExpr :: If, Block, Raw})
    }
}

impl From<RExprIf<()>> for Tagged<RExprIf<IO>> {
    fn from(RExprIf { condition, then_block, else_block }: RExprIf<()>) -> Self {
        
    }
}

impl From<RExprBlock<()>> for Tagged<RExprBlock<IO>> {
    fn from(value: RExprBlock<()>) -> Self {

    }
}

impl From<RStmt<()>> for Tagged<RStmt<IO>> {
    fn from(value: RStmt<()>) -> Self {
        defer_tagging!(value {RStmt :: LetAwait, Return, Expr, Raw})
    }
}

impl From<RStmtLetAwait<()>> for Tagged<RStmtLetAwait<IO>> {
    fn from(RStmtLetAwait { definition, box future }: RStmtLetAwait<()>) -> Self {
        let (tagged_future, inputs) = future.into();
        let tagged_stmt = RStmtLetAwait {
            definition,
            future: Box::new(tagged_future),
        };
        inputs.remove()
        (tagged_stmt, inputs)
    }
}

impl From<RReturn<()>> for Tagged<RReturn<IO>> {
    fn from(value: RReturn<()>) -> Self {
        value.into()
    }
}

impl From<Raw<syn::Expr, ()>> for Tagged<Raw<syn::Expr, IO>> {
    fn from(value: Raw<syn::Expr, ()>) -> Self {
        
    }
}

impl From<Raw<syn::Stmt, ()>> for Tagged<Raw<syn::Stmt, IO>> {
    fn from(value: Raw<syn::Stmt, ()>) -> Self {
        
    }
}