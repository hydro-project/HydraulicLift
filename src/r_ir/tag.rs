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
        Tag::tag(untagged).eval(Scope::empty())
    }
}

/// State monad of type Scope
type TState<T> = State<Scope, T>;

/// Adds scope tags to raw expressions.
/// Records the needed output scope for any Tagged item.
trait Tag {
    type Out;
    /// tag<T> :: T<()> -> needed_output -> (T<Scope>, needed_input)
    fn tag(untagged: Self) -> TState<Self::Out>;
}

impl Tag for RExpr {
    type Out = RExpr<Scope>;

    fn tag(untagged: Self) -> TState<Self::Out> {
        match untagged {
            RExpr::If(s) => Tag::tag(s).map(RExpr::If),
            RExpr::Block(s) => Tag::tag(s).map(RExpr::Block),
            RExpr::Raw(s) => Tag::tag(s).map(RExpr::Raw),
        }
    }
}

impl Tag for RExprIf {
    type Out = RExprIf<Scope>;

    fn tag(
        Self {
            box cond_expr,
            box then_expr,
            box else_expr,
        }: Self,
    ) -> TState<RExprIf<Scope>> {
        Tag::tag(then_expr)
            .zip(Tag::tag(else_expr))
            .and_then(|(then_expr, else_expr)| {
                Tag::tag(cond_expr).map(|cond_expr| RExprIf::new(cond_expr, then_expr, else_expr))
            })
    }
}

impl Tag for RExprBlock {
    type Out = RExprBlock<Scope>;

    fn tag(Self { stmt, box expr }: Self) -> TState<RExprBlock<Scope>> {
        // TODO: this doesnt include nested scopes!!!
        // let x = 5;
        // {
        //    let x = 3;
        // }
        // x
        // Solution: Maybe disallow re-using identifiers anywhere?
        Tag::tag(expr).and_then(|expr| Tag::tag(stmt).map(|stmt| RExprBlock::new(stmt, expr)))
    }
}

impl Tag for RExprRaw {
    type Out = RExprRaw<Scope>;

    fn tag(
        Self {
            expr: DebugStr(expr),
            scope: (),
        }: Self,
    ) -> TState<RExprRaw<Scope>> {
        TState::state(|output| {
            // Visit the underlying expression backwards.
            // Transform the needed output scope into the needed input scope.
            let input = ScopeVisitor::visit(output, &expr);
            (RExprRaw::new(expr, input.clone()), input)
        })
    }
}

impl Tag for RStmt {
    type Out = RStmt<Scope>;

    fn tag(untagged: Self) -> TState<RStmt<Scope>> {
        match untagged {
            RStmt::Let(s) => Tag::tag(s).map(RStmt::Let),
            RStmt::Return(s) => Tag::tag(s).map(RStmt::Return),
        }
    }
}

impl Tag for RStmtLet {
    type Out = RStmtLet<Scope>;

    fn tag(Self { id, box value }: Self) -> TState<RStmtLet<Scope>> {
        let id2 = id.clone();
        TState::modify(move |output| output.without(&id2))
            .and(Tag::tag(value).map(|value| RStmtLet::new(id, value)))
    }
}

impl Tag for RStmtReturn {
    type Out = RStmtReturn<Scope>;

    fn tag(Self { box value }: Self) -> TState<RStmtReturn<Scope>> {
        TState::put(Scope::empty()).and(Tag::tag(value).map(|value| RStmtReturn::new(value)))
    }
}

impl<U1, U2> Tag for Tagged<U1>
where
    U1: 'static + Tag<Out = U2>,
    U2: 'static
{
    type Out = Tagged<U2, Scope>;

    fn tag(Self(inner, ()): Self) -> TState<Tagged<U2, Scope>> {
        // Store the needed output scope of inner in the Tagged structure
        TState::get().and_then(|output| Tag::tag(inner).map(|inner| Tagged(inner, output)))
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
