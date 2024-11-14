use std::rc::Rc;

use syn::{Expr, Ident};

use crate::{derive_hnode, utils::{
    debug::DebugStr, functional::Semigroup, pattern::{ExprPat, ScopePat}, scope::{ScopeDef, Scope}, tagged::TagOut
}};

use super::node::HNode;


// :: value
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HOutput {
    Return(HReturn),
    Union(HUnion<HOutput>)
}
derive_hnode!(HOutput: Ident);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HReturn {
    pub value: HExpr,
}
derive_hnode!(HReturn: Ident);

/// No outputs! This is a sink node which consumes a scope.
/// ident identifies this sink with a HCycleSource.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HCycleSink {
    pub scope: HScope,
    pub ident: Ident
}

/// :: (value, scope)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HExpr {
    Raw(TagOut<HExprRaw, Scope>),
    Await(HExprAwait),
    /// A merge point
    Union(HUnion<HExpr>),
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
    pub scope_def: ScopeDef,
}
derive_hnode!(HExprRaw: ExprPat);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HExprShared(pub Rc<HExpr>);
derive_hnode!(HExprShared: ExprPat);

/// :: scope
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HScope {
    Expr(HDropExpr),
    Bind(TagOut<HBind, Scope>),
    Filter(HFilter),
    CycleSource(HCycleSource),
    Union(HUnion<HScope>),
    Input(TagOut<HInput, Scope>),
}
derive_hnode!(HScope: ScopePat);

/// drops an expression value
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HDropExpr {
    pub expr: Box<HExpr>
}
derive_hnode!(HDropExpr: ScopePat);

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

/// ident identifies this source with a HCycleSink.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HCycleSource(pub Ident);
derive_hnode!(HCycleSource: ScopePat);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HInput;
derive_hnode!(HInput: ScopePat);

/// Represents a merge point on the dataflow graph.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HUnion<T>(pub Box<T>, pub Box<T>);
impl<T> HNode for HUnion<T> 
    where T: HNode
{
    type O = T::O;
}

impl HOutput {
    /// Creates a new output returning value
    pub fn ret(value: HExpr) -> Self {
        Self::Return(HReturn::new(value))
    }
}


impl HReturn {
    pub fn new(value: HExpr) -> Self {
        Self { value }
    }
}


impl HExprRaw {
    pub fn new(expr: Expr, input: HScope, scope_def: ScopeDef) -> Self {
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

impl HExprShared {
    pub fn new(inner: Rc<HExpr>) -> Self {
        Self(inner)
    }
}

impl HDropExpr {
    pub fn new(expr: HExpr) -> Self {
        Self {
            expr: Box::new(expr)
        }
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

impl HCycleSink {
    pub fn new(scope: HScope, ident: Ident) -> Self {
        Self { scope, ident }
    }
}

impl HCycleSource {
    pub fn new(ident: Ident) -> Self {
        Self(ident)
    }
}

impl<T> HUnion<T> {
    pub fn new(t1: T, t2: T) -> Self {
        Self(Box::new(t1), Box::new(t2))
    }
}

impl Semigroup for HOutput {
    fn concat(self, other: Self) -> Self {
        Self::Union(HUnion::new(self, other))
    }
}

impl Semigroup for HExpr {
    fn concat(self, other: Self) -> Self {
        Self::Union(HUnion::new(self, other))
    }
}

impl Semigroup for HScope {
    fn concat(self, other: Self) -> Self {
        Self::Union(HUnion::new(self, other))
    }
}