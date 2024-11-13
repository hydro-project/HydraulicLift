use crate::utils::{debug::DebugStr, functional::State, scope::ScopeDef, tagged::TagOut};

use super::super::ir::*;

/// Tags nodes with their possible input scopes by traversing forwards.
/// This is neccessary so we can ignore globals on the second pass of finding output scopes.
pub fn tag_possible_inputs(untagged: RExpr, input: ScopeDef) -> RExpr<ScopeDef> {
    TS::from(untagged).eval(input)
}

// :: r () -> scope -> r scope

/// State monad of type ScopeDef
type TS<T> = State<'static, ScopeDef, T>;

impl From<RExpr> for TS<RExpr<ScopeDef>> {
    fn from(untagged: RExpr) -> Self {
        match untagged {
            RExpr::If(s) => TS::from(s).map(RExpr::If),
            RExpr::Block(s) => TS::from(s).map(RExpr::Block),
            RExpr::Await(s) => TS::from(s).map(RExpr::Await),
            RExpr::Raw(s) => TS::from(s).map(RExpr::Raw),
        }
    }
}

impl From<RExprIf> for TS<RExprIf<ScopeDef>> {
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

impl From<RExprBlock> for TS<RExprBlock<ScopeDef>> {
    fn from(RExprBlock { stmt, box expr }: RExprBlock) -> Self {
        TS::from(stmt).and_then(|stmt| TS::from(expr).map(|expr| RExprBlock::new(stmt, expr)))
    }
}

impl From<RExprAwait> for TS<RExprAwait<ScopeDef>> {
    fn from(RExprAwait(box inner): RExprAwait) -> Self {
        TS::from(inner).map(RExprAwait::new)
    }
}

impl From<RExprRaw> for TS<RExprRaw<ScopeDef>> {
    fn from(
        RExprRaw {
            expr: DebugStr(expr),
            scope: (),
        }: RExprRaw,
    ) -> Self {
        TS::get().map(|input| RExprRaw::new(expr, input))
    }
}

impl From<RStmt> for TS<RStmt<ScopeDef>> {
    fn from(value: RStmt) -> Self {
        match value {
            RStmt::Let(s) => TS::from(s).map(RStmt::Let),
            RStmt::Return(s) => TS::from(s).map(RStmt::Return),
        }
    }
}

impl From<RStmtLet> for TS<RStmtLet<ScopeDef>> {
    fn from(
        RStmtLet {
            id,
            is_mut,
            box value,
        }: RStmtLet,
    ) -> Self {
        let id2 = id.clone();
        TS::from(value).and_then(move |value| {
            TS::modify(move |input| input.with(id2, is_mut))
                .map_const(RStmtLet::new(id, is_mut, value))
        })
    }
}

impl From<RStmtReturn> for TS<RStmtReturn<ScopeDef>> {
    fn from(RStmtReturn { box value }: RStmtReturn) -> Self {
        TS::from(value).map(|value| RStmtReturn::new(value))
    }
}

impl<U1, U2> From<TagOut<U1>> for TS<TagOut<U2>>
where
    U1: 'static,
    U2: 'static,
    TS<U2>: From<U1>,
{
    fn from(TagOut(inner, ()): TagOut<U1>) -> Self {
        TS::from(inner).map(|inner| TagOut(inner, ()))
    }
}
