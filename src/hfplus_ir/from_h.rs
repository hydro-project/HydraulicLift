
use std::{cell::RefCell, rc::Rc};

use hydroflow_plus::ir::HfPlusNode;
use quote::quote;

use crate::{h_ir::ir::*, utils::{idents::ident, pattern::{ExprPat, ScopePat}, scope::Scope, tagged::Tagged}};

use super::{func::{FilterMapFunc, MapFunc}, gen::*, memo::{HfMemoize, HfMemos}};

/// Generates hydroflow+ node from hnode, which consumes the specified input.
pub fn generate_hf<'a>(h_node: HOutput, input: HfPlusNode<'a>) -> HfPlusNode<'a> {
    let memos = HfMemos::new().with(Rc::new(HInput), Rc::new(RefCell::new(input)));

    *HOutput::gen(h_node).run(memos).1
}



impl<'a> HfGen<'a> for HExpr {
    fn gen(h_node: Self) -> HFS<'a> {
        match h_node {
            HExpr::Raw(s) => HfGen::gen(s),
            HExpr::Union(s) => HfGen::gen(s),
            HExpr::Shared(s) => HfGen::gen(s),
        }
    }
}

impl<'a> HfGen<'a> for Tagged<HExprRaw, Scope> {
    fn gen(Self(HExprRaw { input, expr, scope: in_scope }, out_scope): Self) -> HFS<'a> {
        Self::gen_map(
            input,
            MapFunc::newb(
                ScopePat::Destructured(in_scope),
                ExprPat::Destructured(ident("value"), ScopePat::Destructured(out_scope)),
                quote! { let value = #expr; },
            ),
        )
    }
}

impl<'a> HfGen<'a> for HExprUnion {
    fn gen(
        Self(box input1, box input2): Self
    ) -> HFS<'a> {
        Self::gen_union(input1, input2)
    }
}

impl<'a> HfGen<'a> for HExprShared {
    fn gen(Self(input): Self) -> HFS<'a> {
        Self::gen_tee(input)
    }
}

impl<'a> HfGen<'a> for HScope {
    fn gen(h_node: Self) -> HFS<'a> {
        match h_node {
            HScope::Input(s) => HfGen::gen(s),
            HScope::Bind(s) => HfGen::gen(s),
            HScope::Filter(s) => HfGen::gen(s),
        }
    }
}

impl<'a> HfGen<'a> for HInput {
    fn gen(h_node: Self) -> HFS<'a> {
        HFS::memo(|_| panic!("Did not memoize hydroflow input"), Rc::new(h_node))
    }
}

impl<'a> HfGen<'a> for Tagged<HBind, Scope> {
    fn gen(
        Tagged(HBind { id, box value }, scope): Self,
    ) -> HFS<'a> {
        // Todo: update this to support shadowing
        Self::gen_map(
            value,
            MapFunc::new(
                ExprPat::Destructured(id.clone(), ScopePat::Destructured(scope.clone().without(&id))),
                ScopePat::Destructured(scope),
            ),
        )
    }
}

impl<'a> HfGen<'a> for HFilter {
    fn gen(
        Self { box cond, expectation }: Self,
    ) -> HFS<'a> {
        // Todo: standardize/fix idents
        Self::gen_filter_map(
            cond,
            FilterMapFunc::newb(
                ExprPat::Destructured(ident("cond"), ScopePat::Ident(ident("scope"))),
                ScopePat::Ident(ident("scope")),
                quote! {
                    if cond != #expectation {
                        return None
                    }
                },
            ),
        )
    }
}

impl<'a> HfGen<'a> for HReturn {
    fn gen(Self { value }: Self) -> HFS<'a> {
        Self::gen_map(
            value,
            MapFunc::new(
                ExprPat::Destructured(ident("value"), ScopePat::Ident(ident("_"))),
                ident("value"),
            ),
        )
    }
}

impl<'a> HfGen<'a> for HOutput {
    fn gen(Self { input, other }: Self) -> HFS<'a> {
        match other {
            Some(box input2) => Self::gen_union(input, input2),
            None => HfGen::gen(input),
        }
    }
}
