use std::rc::Rc;

use syn::{Expr, Ident};

use crate::{derive_hnode, utils::{
    debug::DebugStr, functional::Semigroup, pattern::{ExprPat, ScopePat}, scope::Scope, tagged::Tagged
}};


// :: value
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HOutput {
    pub input: HReturn,
    pub other: Option<Box<HOutput>>,
}
derive_hnode!(HOutput: Ident);

/// :: value
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HReturn {
    pub value: HExpr,
}
derive_hnode!(HReturn: Ident);

/// :: (value, scope)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HExpr {
    Raw(Tagged<HExprRaw, Scope>),
    Await(HExprAwait),
    // A merge point
    Union(HExprUnion),
    /// A branch point
    Shared(HExprShared),
}
derive_hnode!(HExpr: ExprPat);


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HExprAwait(pub Box<HExpr>);
derive_hnode!(HExprAwait: ExprPat);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HExprRaw {
    pub expr: DebugStr<Expr>,
    pub input: HScope,
    pub scope_def: Scope,
}
derive_hnode!(HExprRaw: ExprPat);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HExprUnion(pub Box<HExpr>, pub Box<HExpr>);
derive_hnode!(HExprUnion: ExprPat);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HExprShared(pub Rc<HExpr>);
derive_hnode!(HExprShared: ExprPat);

/// :: scope
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HScope {
    Bind(Tagged<HBind, Scope>),
    Filter(HFilter),
    Input(Tagged<HInput, Scope>),
}
derive_hnode!(HScope: ScopePat);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HBind {
    pub id: Ident,
    pub value: Box<HExpr>,
}
derive_hnode!(HBind: ScopePat);

/// Filters for cond == expectation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HFilter {
    pub expectation: bool,
    pub cond: Box<HExpr>,
}
derive_hnode!(HFilter: ScopePat);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HInput;
derive_hnode!(HInput: ScopePat);

impl HOutput {
    /// Creates a new output from the return value
    pub fn new(input: HReturn) -> Self {
        Self { input, other: None }
    }

    /// Creates a new output returning value
    pub fn ret(value: HExpr) -> Self {
        Self::new(HReturn::new(value))
    }
}

impl Semigroup for HOutput {
    fn concat(self, Self { input, other }: Self) -> Self {
        let move_one = Self {
            input,
            other: Some(Box::new(self)),
        };
        match other {
            Some(box rest) => move_one.concat(rest),
            None => move_one,
        }
    }
}

impl HReturn {
    pub fn new(value: HExpr) -> Self {
        Self { value }
    }
}

impl Semigroup for HExpr {
    fn concat(self, other: Self) -> Self {
        Self::Union(HExprUnion::new(self, other))
    }
}

impl HExprRaw {
    pub fn new(expr: Expr, input: HScope, scope_def: Scope) -> Self {
        Self {
            expr: expr.into(),
            input,
            scope_def,
        }
    }
}

impl HExprAwait {
    pub fn new(inner: HExpr) -> Self {
        Self(Box::new(inner))
    }
}

impl HExprUnion {
    pub fn new(e1: HExpr, e2: HExpr) -> Self {
        Self(Box::new(e1), Box::new(e2))
    }
}

impl HExprShared {
    pub fn new(inner: Rc<HExpr>) -> Self {
        Self(inner)
    }
}

impl HBind {
    pub fn new(id: Ident, value: HExpr) -> Self {
        Self {
            id,
            value: Box::new(value),
        }
    }
}

impl HFilter {
    pub fn new(expectation: bool, cond: HExpr) -> Self {
        Self {
            expectation,
            cond: Box::new(cond),
        }
    }
}
