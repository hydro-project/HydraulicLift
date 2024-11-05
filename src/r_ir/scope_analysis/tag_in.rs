
use crate::utils::{debug::DebugStr, functional::State, scope::Scope, tagged::Tagged};

use super::super::ir::*;

/// Tags nodes with their possible input scopes by traversing forwards.
/// This is neccessary so we can ignore globals on the second pass of finding output scopes.
pub fn tag_possible_inputs(untagged: RExpr, input: Scope) -> RExpr<Scope> {
    TS::from(untagged).eval(input)
}

// :: r () -> scope -> r scope

/// State monad of type ScopeDef
pub type TS<T> = State<'static, Scope, T>;

impl From<RExpr> for TS<RExpr<Scope>> {
    fn from(untagged: RExpr) -> Self {
        match untagged {
            RExpr::If(s) => TS::from(s).map(RExpr::If),
            RExpr::Block(s) => TS::from(s).map(RExpr::Block),
            RExpr::Await(s) => TS::from(s).map(RExpr::Await),
            RExpr::Raw(s) => TS::from(s).map(RExpr::Raw),
        }
    }
}

impl From<RExprIf> for TS<RExprIf<Scope>> {
    fn from(
        RExprIf {
            box cond_expr,
            box then_expr,
            box else_expr,
        }: RExprIf,
    ) -> Self {
        TS::from(cond_expr).and_then(|cond_expr| {
            TS::from(then_expr)
                .zip(TS::from(else_expr))
                .map(|(then_expr, else_expr)| RExprIf::new(cond_expr, then_expr, else_expr))
        })
    }
}

impl From<RExprBlock> for TS<RExprBlock<Scope>> {
    fn from(RExprBlock { stmt, box expr }: RExprBlock) -> Self {
        TS::from(stmt).and_then(|stmt| TS::from(expr).map(|expr| RExprBlock::new(stmt, expr)))
    }
}

impl From<RExprAwait> for TS<RExprAwait<Scope>> {
    fn from(RExprAwait(box inner): RExprAwait) -> Self {
        TS::from(inner).map(RExprAwait::new)
    }
}

impl From<RExprRaw> for TS<RExprRaw<Scope>> {
    fn from(
        RExprRaw {
            expr: DebugStr(expr),
            scope: (),
        }: RExprRaw,
    ) -> Self {
        TS::get().map(|input| RExprRaw::new(expr, input))
    }
}

impl From<RStmt> for TS<RStmt<Scope>> {
    fn from(value: RStmt) -> Self {
        match value {
            RStmt::Let(s) => TS::from(s).map(RStmt::Let),
            RStmt::Return(s) => TS::from(s).map(RStmt::Return),
        }
    }
}

impl From<RStmtLet> for TS<RStmtLet<Scope>> {
    fn from(RStmtLet { id, box value }: RStmtLet) -> Self {
        let id2 = id.clone();
        TS::from(value).and_then(|value| {
            TS::modify(|input| input.with(id2)).map_const(RStmtLet::new(id, value))
        })
    }
}

impl From<RStmtReturn> for TS<RStmtReturn<Scope>> {
    fn from(RStmtReturn { box value }: RStmtReturn) -> Self {
        TS::from(value).map(|value| RStmtReturn::new(value))
    }
}

impl<U1, U2> From<Tagged<U1>> for TS<Tagged<U2, Scope>>
where
    U1: 'static,
    U2: 'static,
    TS<U2>: From<U1>,
{
    fn from(Tagged(inner, ()): Tagged<U1>) -> Self {
        // Store the needed input scope of inner in the Tagged structure
        TS::from(inner).and_then(|inner| TS::get().map(|input| Tagged(inner, input)))
    }
}