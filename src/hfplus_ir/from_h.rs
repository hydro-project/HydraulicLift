
use std::rc::Rc;

use hydroflow_plus::ir::HfPlusNode;
use quote::quote;

use crate::{h_ir::ir::*, utils::{idents::ident, pattern::{ExprPat, ScopePat}, scope::Scope, tagged::Tagged}};

use super::{func::{FilterMapFunc, MapFunc}, gen::*, memo::{HfMemoize, HfMemos}};

/// Generates hydroflow+ node from hnode, which consumes the specified input.
pub fn generate_hf<'a>(h_node: HOutput, input: HfPlusNode<'a>) -> HfPlusNode<'a> {
    let memos = HfMemos::new().with(Rc::new(HInput), input);
    *HOutput::gen(h_node, memos).1
}

impl<'a> HfGen<'a> for HExpr {
    fn gen(h_node: Self, memos: HfMemos<'a>) -> (HfMemos<'a>, Box<HfPlusNode<'a>>) {
        match h_node {
            HExpr::Raw(s) => HfGen::gen(s, memos),
            HExpr::Union(s) => HfGen::gen(s, memos),
            HExpr::Shared(s) => HfGen::gen(s, memos),
        }
    }
}

impl<'a> HfGen<'a> for Tagged<HExprRaw, Scope> {
    fn gen(
        Self(HExprRaw { input, expr, scope: in_scope }, out_scope): Self,
        memos: HfMemos<'a>,
    ) -> (HfMemos<'a>, Box<HfPlusNode<'a>>) {
        Self::gen_map(
            input,
            memos,
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
        Self(box input1, box input2): Self,
        memos: HfMemos<'a>,
    ) -> (HfMemos<'a>, Box<HfPlusNode<'a>>) {
        Self::gen_union(input1, input2, memos)
    }
}

impl<'a> HfGen<'a> for HExprShared {
    fn gen(Self(input): Self, memos: HfMemos<'a>) -> (HfMemos<'a>, Box<HfPlusNode<'a>>) {
        Self::gen_tee(input, memos)
    }
}

impl<'a> HfGen<'a> for HScope {
    fn gen(h_node: Self, memos: HfMemos<'a>) -> (HfMemos<'a>, Box<HfPlusNode<'a>>) {
        match h_node {
            HScope::Input(s) => HfGen::gen(s, memos),
            HScope::Bind(s) => HfGen::gen(s, memos),
            HScope::Filter(s) => HfGen::gen(s, memos),
        }
    }
}

impl<'a> HfGen<'a> for HInput {
    fn gen(Self: Self, memos: HfMemos<'a>) -> (HfMemos<'a>, Box<HfPlusNode<'a>>) {
        let hf_input = memos.get(&Rc::new(HInput)).unwrap();
        // can't use one of my i/o type-checked functions because this is just a raw input
        (memos, Box::new(HfPlusNode::Tee { inner: hf_input }))
    }
}

impl<'a> HfGen<'a> for Tagged<HBind, Scope> {
    fn gen(
        Tagged(HBind { id, box value }, scope): Self,
        memos: HfMemos<'a>,
    ) -> (HfMemos<'a>, Box<HfPlusNode<'a>>) {
        // Todo: update this to support shadowing
        Self::gen_map(
            value,
            memos,
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
        memos: HfMemos<'a>,
    ) -> (HfMemos<'a>, Box<HfPlusNode<'a>>) {
        // Todo: standardize/fix idents
        Self::gen_filter_map(
            cond,
            memos,
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
    fn gen(Self { value }: Self, memos: HfMemos<'a>) -> (HfMemos<'a>, Box<HfPlusNode<'a>>) {
        Self::gen_map(
            value,
            memos,
            MapFunc::new(
                ExprPat::Destructured(ident("value"), ScopePat::Ident(ident("_"))),
                ident("value"),
            ),
        )
    }
}

impl<'a> HfGen<'a> for HOutput {
    fn gen(Self { input, other }: Self, memos: HfMemos<'a>) -> (HfMemos<'a>, Box<HfPlusNode<'a>>) {
        match other {
            Some(box input2) => Self::gen_union(input, input2, memos),
            None => HfGen::gen(input, memos),
        }
    }
}
