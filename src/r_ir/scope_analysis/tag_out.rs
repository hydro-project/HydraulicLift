use syn::{visit::Visit, Expr, Ident};

use crate::utils::{
    debug::DebugStr,
    scope::Scope,
    tagged::Tagged,
};

use super::{super::ir::*, tag_in::TS};

/// Tags nodes with their output scopes by traversing backwards 
/// and only marking needed outputs which are possible inputs.
/// Treats existing tags as possible inputs.
pub fn tag_outputs(untagged: RExpr<Scope>) -> Tagged<RExpr<Scope>, Scope> {
    let (scope, expr) = Tag::tag(untagged).run(Scope::empty());
    assert!(scope.0.len() <= 1, "Needed input scope must be exactly the singular hf+ input or empty, but instead found {:?}", scope);
    Tagged(expr, scope)
}

trait Tag: Sized {
    fn tag(untagged: Self) -> TS<Self>; 
}

impl Tag for RExpr<Scope> {
    fn tag(untagged: Self) -> TS<Self> {
        match untagged {
            RExpr::If(s) => Tag::tag(s).map(RExpr::If),
            RExpr::Block(s) => Tag::tag(s).map(RExpr::Block),
            RExpr::Await(s) => Tag::tag(s).map(RExpr::Await),
            RExpr::Raw(s) => Tag::tag(s).map(RExpr::Raw),
        }
    }
}

impl Tag for RExprIf<Scope> {    
    fn tag(Self{ box cond_expr, box then_expr, box else_expr }: Self) -> TS<Self> {
        Tag::tag(then_expr)
            .zip(Tag::tag(else_expr))
            .and_then(|(then_expr, else_expr)| {
                Tag::tag(cond_expr).map(|cond_expr| RExprIf::new(cond_expr, then_expr, else_expr))
            })
    }
}

impl Tag for RExprBlock<Scope> {
    fn tag(Self{ stmt, box expr }: Self) -> TS<Self> {
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

impl Tag for RExprAwait<Scope> {
    
    fn tag(Self(box inner): Self) -> TS<Self> {
        Tag::tag(inner).map(RExprAwait::new)
    }
}

impl Tag for RExprRaw<Scope> {
    fn tag(Self{ expr: DebugStr(expr), scope: possible_inputs }: Self) -> TS<Self> {
        TS::state(|output| {
            // Visit the underlying expression backwards.
            // Transform the needed output scope into the needed input scope.
            let needed_input = ScopeVisitor::visit(output, &expr);
            // only the intersection of needed and possible inputs matter.
            // Anything else is either a global or already dropped.
            let input = needed_input.intersect(possible_inputs);
            (input.clone(), RExprRaw::new(expr, input))
        })
    }
}

impl Tag for RStmt<Scope> {
    fn tag(untagged: Self) -> TS<Self> {
        match untagged {
            RStmt::Let(s) => Tag::tag(s).map(RStmt::Let),
            RStmt::Return(s) => Tag::tag(s).map(RStmt::Return),
        }
    }
}

impl Tag for RStmtLet<Scope> {    
    fn tag(Self { id, box value }: Self) -> TS<Self> {
        let id2 = id.clone();
        TS::modify(move |output| output.without(&id2))
            .and(Tag::tag(value).map(|value| RStmtLet::new(id, value)))
    }
}

impl Tag for RStmtReturn<Scope> {
    fn tag(Self { box value }: Self) -> TS<Self> {
        TS::put(Scope::empty()).and(Tag::tag(value).map(|value| RStmtReturn::new(value)))
    }
}

impl<T> Tag for Tagged<T, Scope> where T: 'static + Tag{
    fn tag(Self(inner, _): Self) -> TS<Self> {
        // Store the needed output scope of inner in the Tagged structure
        TS::get().and_then(|output| Tag::tag(inner).map(|inner| Tagged(inner, output)))
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
