use syn::{visit::Visit, Expr, Ident};

use crate::utils::{
    debug::DebugStr,
    functional::State,
    scope::{ScopeDef, Scope},
    tagged::TagOut,
};

use super::super::ir::*;

/// Tags nodes with their output scopes by traversing backwards
/// and only marking needed outputs which are possible inputs.
/// Treats existing tags as possible inputs.
pub fn tag_outputs(untagged: RExpr<ScopeDef>) -> TagOut<RExpr<ScopeDef, Scope>, Scope> {
    let (scope, expr) = TS::from(untagged).run(Scope::empty());
    assert!(scope.0.len() <= 1, "Needed input scope must be exactly the singular hf+ input or empty, but instead found {:?}", scope);
    TagOut(expr, scope)
}

/// State monad of type Scope
pub type TS<T> = State<'static, Scope, T>;

impl From<RExpr<ScopeDef>> for TS<RExpr<ScopeDef, Scope>> {
    fn from(value: RExpr<ScopeDef>) -> Self {
        match value {
            RExpr::If(s) => TS::from(s).map(RExpr::If),
            RExpr::Block(s) => TS::from(s).map(RExpr::Block),
            RExpr::Await(s) => TS::from(s).map(RExpr::Await),
            RExpr::Raw(s) => TS::from(s).map(RExpr::Raw),
        }
    }
}

impl From<RExprIf<ScopeDef>> for TS<RExprIf<ScopeDef, Scope>> {
    fn from(
        RExprIf {
            box cond_expr,
            box then_expr,
            box else_expr,
        }: RExprIf<ScopeDef>,
    ) -> Self {
        TS::from(then_expr)
            .zip(TS::from(else_expr))
            .and_then(|(then_expr, else_expr)| {
                TS::from(cond_expr).map(|cond_expr| RExprIf::new(cond_expr, then_expr, else_expr))
            })
    }
}

impl From<RExprBlock<ScopeDef>> for TS<RExprBlock<ScopeDef, Scope>> {
    fn from(RExprBlock { stmt, box expr }: RExprBlock<ScopeDef>) -> Self {
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

impl From<RExprAwait<ScopeDef>> for TS<RExprAwait<ScopeDef, Scope>> {
    fn from(RExprAwait(box inner): RExprAwait<ScopeDef>) -> Self {
        TS::from(inner).map(RExprAwait::new)
    }
}

impl From<RExprRaw<ScopeDef>> for TS<RExprRaw<ScopeDef>> {
    fn from(
        RExprRaw {
            expr: DebugStr(expr),
            scope: possible_inputs,
        }: RExprRaw<ScopeDef>,
    ) -> Self {
        TS::state(|output| {
            // Visit the underlying expression backwards.
            // Transform the needed output scope into the needed input scope.
            let needed_input = ScopeVisitor::visit(output, &expr);
            // only the intersection of needed and possible inputs matter.
            // Anything else is either a global or already dropped.
            let input = possible_inputs.intersect(needed_input);
            (input.clone().lower(), RExprRaw::new(expr, input))
        })
    }
}

impl From<RStmt<ScopeDef>> for TS<RStmt<ScopeDef, Scope>> {
    fn from(value: RStmt<ScopeDef>) -> Self {
        match value {
            RStmt::Let(s) => TS::from(s).map(RStmt::Let),
            RStmt::Return(s) => TS::from(s).map(RStmt::Return),
        }
    }
}

impl From<RStmtLet<ScopeDef>> for TS<RStmtLet<ScopeDef, Scope>> {
    fn from(
        RStmtLet {
            id,
            is_mut,
            box value,
        }: RStmtLet<ScopeDef>,
    ) -> Self {
        let id2 = id.clone();
        TS::modify(move |output| output.without(&id2))
            .and(TS::from(value).map(move |value| RStmtLet::new(id, is_mut, value)))
    }
}

impl From<RStmtReturn<ScopeDef>> for TS<RStmtReturn<ScopeDef, Scope>> {
    fn from(RStmtReturn { box value }: RStmtReturn<ScopeDef>) -> Self {
        TS::put(Scope::empty()).and(TS::from(value).map(|value| RStmtReturn::new(value)))
    }
}

impl<T1, T2> From<TagOut<T1>> for TS<TagOut<T2, Scope>>
where
    T1: 'static,
    T2: 'static,
    TS<T2>: From<T1>,
{
    fn from(TagOut(inner, ()): TagOut<T1>) -> Self {
        // Store the needed output scope of inner in the Tagged structure
        TS::get().and_then(|output| TS::from(inner).map(|inner| TagOut(inner, output)))
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
        self.0 = self.0.clone().wth(ident.clone())
    }

    fn visit_pat_ident(&mut self, syn::PatIdent { ident, .. }: &'ast syn::PatIdent) {
        // TODO: support mutability
        self.0 = self.0.clone().without(&ident)
    }
}
