use syn::visit::Visit;

use crate::utils::{debug::DebugStr, functional::{FakeFunctor, Semigroup}, scope::Scope, tagged::Tagged};

use super::ir::*;

/// Tags nodes with their output scopes
impl From<RExpr> for RExpr<Scope> {
    fn from(untagged: RExpr) -> Self {
        Tag::tag(untagged, Scope::empty()).1
    }
}

/// Adds scope tags to raw expressions.
/// Records the needed output scope for any Tagged item.
trait Tag {
    type Out;
    /// tag<T> :: T<()> -> needed_output -> (T<Scope>, needed_input)
    fn tag(untagged: Self, output: Scope) -> (Scope, Self::Out);
}

impl Tag for RExpr {
    type Out = RExpr<Scope>;

    fn tag(untagged: Self, output: Scope) -> (Scope, RExpr<Scope>) {
        match untagged {
            RExpr::If(s) => Tag::tag(s, output).map(RExpr::If),
            RExpr::Block(s) => Tag::tag(s, output).map(RExpr::Block),
            RExpr::Raw(s) => Tag::tag(s, output).map(RExpr::Raw),
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
        output: Scope,
    ) -> (Scope, RExprIf<Scope>) {
        let (else_input, else_expr) = Tag::tag(else_expr, output.clone());
        let (then_input, then_expr) = Tag::tag(then_expr, output.clone());
        // This probably doesnt work loll then_input and else_input are expecting different things.
        // Maybe they need to be Tagged?
        Tag::tag(cond_expr, then_input.concat(else_input))
            .map(|cond_expr| RExprIf::new(cond_expr, then_expr, else_expr))
    }
}

impl Tag for RExprBlock {
    type Out = RExprBlock<Scope>;

    fn tag(Self { stmt, box expr }: Self, output: Scope) -> (Scope, RExprBlock<Scope>) {
        // TODO: this doesnt include nested scopes!!!
        // let x = 5;
        // {
        //    let x = 3;
        // }
        // x
        // Solution: Maybe disallow re-using identifiers anywhere?
        let (expr_input, expr) = Tag::tag(expr, output);
        let (stmt_input, stmt) = Tag::tag(stmt, expr_input);
        (stmt_input, RExprBlock::new(stmt, expr))
    }
}

impl Tag for RExprRaw {
    type Out = RExprRaw<Scope>;

    fn tag(
        Self {
            expr: DebugStr(expr),
            scope: (),
        }: Self,
        output: Scope,
    ) -> (Scope, RExprRaw<Scope>) {
        // Visit the underlying expression backwards.
        // Transform the needed output scope into the needed input scope.
        let mut visitor = ScopeVisitor(output);
        visitor.visit_expr(&expr);
        let ScopeVisitor(input) = visitor;
        (input.clone(), RExprRaw::new(expr, input))
    }
}

impl Tag for RStmt {
    type Out = RStmt<Scope>;

    fn tag(untagged: Self, output: Scope) -> (Scope, RStmt<Scope>) {
        match untagged {
            RStmt::Let(s) => Tag::tag(s, output).map(RStmt::Let),
            RStmt::Return(s) => Tag::tag(s, output).map(RStmt::Return),
        }
    }
}

impl Tag for RStmtLet {
    type Out = RStmtLet<Scope>;

    fn tag(Self { id, box value }: Self, output: Scope) -> (Scope, RStmtLet<Scope>) {
        Tag::tag(value, output.without(&id)).map(|value| RStmtLet::new(id, value))
    }
}

impl Tag for RStmtReturn {
    type Out = RStmtReturn<Scope>;

    fn tag(Self { box value }: Self, _: Scope) -> (Scope, RStmtReturn<Scope>) {
        Tag::tag(value, Scope::empty()).map(|value| RStmtReturn::new(value))
    }
}

impl<U1, U2> Tag for Tagged<U1>
where
    U1: Tag<Out = U2>,
{
    type Out = Tagged<U2, Scope>;

    fn tag(Self(inner, ()): Self, output: Scope) -> (Scope, Tagged<U2, Scope>) {
        // Store the needed output scope of inner in the Tagged structure
        Tag::tag(inner, output.clone()).map(|inner| Tagged(inner, output))
    }
}

struct ScopeVisitor(Scope);

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