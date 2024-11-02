use std::{hash::Hash, rc::Rc};

use hydroflow_plus::ir::HfPlusNode;
use syn::Expr;

use crate::{h_ir::node::HNode, utils::{functional::FakeFunctor, pattern::Pattern}};

use super::{
    func::{FilterMapFunc, MapFunc},
    memo::{HfMemoGen, HfMemoize, HfMemos},
};

/// Implement this for an HNode to convert it to hydroflow+.
/// Additional traits will automatically be implemented,
/// allowing type-safe (with respect to node output patterns)
/// hydroflow+ nodes to be generated.
pub trait HfGen<'a>: HNode {
    /// Generates Hydroflow+ from h_node. memos must contain inputs.
    fn gen(h_node: Self, memos: HfMemos<'a>) -> (HfMemos<'a>, Box<HfPlusNode<'a>>);
}

// The rest of this file contains generator traits which will allow
// specific HfPlusNodes to be created safely.

pub trait HfGenMap<'a, O>: HfGen<'a> + HNode<O = O>
where
    O: Pattern,
{
    /// Generate a node which maps over h_node.
    fn gen_map<U, I: Pattern>(
        h_node: U,
        memos: HfMemos<'a>,
        func: MapFunc<I, O>,
    ) -> (HfMemos<'a>, Box<HfPlusNode<'a>>)
    where
        U: HfGen<'a> + HNode<O = I>;
}

impl<'a, T, O> HfGenMap<'a, O> for T
where
    O: Pattern,
    T: HfGen<'a> + HNode<O = O>,
{
    fn gen_map<U, I: Pattern>(
        h_node: U,
        memos: HfMemos<'a>,
        func: MapFunc<I, O>,
    ) -> (HfMemos<'a>, Box<HfPlusNode<'a>>)
    where
        U: HfGen<'a> + HNode<O = I>,
    {
        U::gen(h_node, memos)
            .map(|hf_node| HfPlusNode::Map {
                f: Expr::from(func).into(),
                input: hf_node,
            })
            .map(Box::new)
    }
}

pub trait HfGenFilterMap<'a, O>: HfGen<'a> + HNode<O = O>
where
    O: Pattern,
{
    /// Generate a node which filter maps over h_node.
    /// The body of func should handle returning None.
    fn gen_filter_map<U, I: Pattern>(
        h_node: U,
        memos: HfMemos<'a>,
        func: FilterMapFunc<I, O>,
    ) -> (HfMemos<'a>, Box<HfPlusNode<'a>>)
    where
        U: HfGen<'a> + HNode<O = I>;
}

impl<'a, T, O> HfGenFilterMap<'a, O> for T
where
    O: Pattern,
    T: HfGen<'a> + HNode<O = O>,
{
    fn gen_filter_map<U, I: Pattern>(
        h_node: U,
        memos: HfMemos<'a>,
        func: FilterMapFunc<I, O>,
    ) -> (HfMemos<'a>, Box<HfPlusNode<'a>>)
    where
        U: HfGen<'a> + HNode<O = I>,
    {
        U::gen(h_node, memos)
            .map(|hf_node| HfPlusNode::FilterMap {
                f: Expr::from(func).into(),
                input: hf_node,
            })
            .map(Box::new)
    }
}

pub trait HfGenTee<'a, O>: HfGen<'a> + HNode<O = O>
where
    O: Pattern,
{
    /// Generate a node which tees over a shared node.
    /// Does this by either
    /// 1) getting the generated node from memos,
    /// or 2) generating it and memoizing it into memos.
    fn gen_tee<U>(h_node: Rc<U>, memos: HfMemos<'a>) -> (HfMemos<'a>, Box<HfPlusNode<'a>>)
    where
        U: HfGen<'a> + HNode<O = O> + Eq + Hash + Clone,
        HfMemos<'a>: HfMemoize<'a, U>;
}

impl<'a, T, O> HfGenTee<'a, O> for T
where
    O: Pattern,
    T: HfGen<'a> + HNode<O = O>,
{
    fn gen_tee<U>(h_node: Rc<U>, memos: HfMemos<'a>) -> (HfMemos<'a>, Box<HfPlusNode<'a>>)
    where
        U: HfGen<'a> + HNode<O = O> + Eq + Hash + Clone,
        HfMemos<'a>: HfMemoize<'a, U>,
    {
        memos
            .get_or_gen(h_node)
            .map(|inner| Box::new(HfPlusNode::Tee { inner }))
    }
}

pub trait HfGenUnion<'a, O>: HfGen<'a> + HNode<O = O>
where
    O: Pattern,
{
    /// Generate a node merges two input streams.
    fn gen_union<U1, U2, I: Pattern>(
        h_node1: U1,
        h_node2: U2,
        memos: HfMemos<'a>,
    ) -> (HfMemos<'a>, Box<HfPlusNode<'a>>)
    where
        U1: HfGen<'a> + HNode<O = I>,
        U2: HfGen<'a> + HNode<O = I>;
}

impl<'a, T, O> HfGenUnion<'a, O> for T
where
    O: Pattern,
    T: HfGen<'a> + HNode<O = O>,
{
    fn gen_union<U1, U2, I: Pattern>(
        h_node1: U1,
        h_node2: U2,
        memos: HfMemos<'a>,
    ) -> (HfMemos<'a>, Box<HfPlusNode<'a>>)
    where
        U1: HfGen<'a> + HNode<O = I>,
        U2: HfGen<'a> + HNode<O = I>,
    {
        let (memos, hf_node1) = U1::gen(h_node1, memos);
        let (memos, hf_node2) = U2::gen(h_node2, memos);
        (memos, Box::new(HfPlusNode::Union(hf_node1, hf_node2)))
    }
}
