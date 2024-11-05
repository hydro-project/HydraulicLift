use syn::{visit::Visit, Expr};

use crate::utils::{
    debug::DebugStr,
    functional::State,
    scope::Scope,
    tagged::Tagged,
};

use super::ir::*;

/// Tags nodes with their output scopes
impl From<RExpr> for RExpr<Scope> {
    fn from(untagged: RExpr) -> Self {
        TS::from(untagged).eval(Scope::empty())
    }
}

/// State monad of type Scope
type TS<T> = State<'static, Scope, T>;

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
    fn from(RExprIf { box cond_expr, box then_expr, box else_expr }: RExprIf) -> Self {
        TS::from(then_expr)
            .zip(TS::from(else_expr))
            .and_then(|(then_expr, else_expr)| {
                TS::from(cond_expr).map(|cond_expr| RExprIf::new(cond_expr, then_expr, else_expr))
            })
    }
}

impl From<RExprBlock> for TS<RExprBlock<Scope>> {
    fn from(RExprBlock { stmt, box expr }: RExprBlock) -> Self {
        // TODO: this doesnt include nested scopes!!!
        // let x = 5;
        // {
        //    let x = 3;
        // }
        // x
        // Solution: Maybe disallow re-using identifiers anywhere?
        TS::from(expr).and_then(|expr| TS::from(stmt).map(|stmt| RExprBlock::new(stmt, expr)))
    }
}

impl From<RExprAwait> for TS<RExprAwait<Scope>> {
    fn from(RExprAwait(box inner): RExprAwait) -> Self {
        TS::from(inner).map(RExprAwait::new)
    }
}

impl From<RExprRaw> for TS<RExprRaw<Scope>> {
    fn from(RExprRaw { expr: DebugStr(expr), scope: () }: RExprRaw) -> Self {
        TS::state(|output| {
            // Visit the underlying expression backwards.
            // Transform the needed output scope into the needed input scope.
            let input = ScopeVisitor::visit(output, &expr);
            (input.clone(), RExprRaw::new(expr, input))
        })
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
        TS::modify(move |output| output.without(&id2))
            .and(TS::from(value).map(|value| RStmtLet::new(id, value)))
    }
}

impl From<RStmtReturn> for TS<RStmtReturn<Scope>> {
    fn from(RStmtReturn { box value }: RStmtReturn) -> Self {
        TS::put(Scope::empty()).and(TS::from(value).map(|value| RStmtReturn::new(value)))
    }
}

impl<U1, U2> From<Tagged<U1>> for TS<Tagged<U2, Scope>>
where
    U1: 'static,
    U2: 'static,
    TS<U2>: From<U1>
{
    fn from(Tagged(inner, ()): Tagged<U1>) -> Self {
        // Store the needed output scope of inner in the Tagged structure
        TS::get().and_then(|output| TS::from(inner).map(|inner| Tagged(inner, output)))
    }
}

struct ScopeVisitor(Scope);

impl ScopeVisitor {
    /// Returns the input scope needed to produce this
    /// output scope after evaluating this expression.
    fn visit(output: Scope, expr: &Expr) -> Scope {
        let mut visitor = ScopeVisitor(output);
        visitor.visit_expr(expr);
        visitor.0
    }
}

// TODO: expand this to actually work for more complex features
impl<'ast> Visit<'ast> for ScopeVisitor {
    fn visit_ident(&mut self, ident: &'ast proc_macro2::Ident) {
        self.0 = self.0.clone().with(ident.clone())
    }

    fn visit_pat_ident(&mut self, syn::PatIdent { ident, .. }: &'ast syn::PatIdent) {
        // TODO: support mutability
        self.0 = self.0.clone().without(&ident)
    }
}
