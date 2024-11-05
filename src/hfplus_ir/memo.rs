use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

use hydroflow_plus::ir::HfPlusNode;

use crate::h_ir::ir::{HExpr, HInput};

/// Memoized conversions from Rc<K: HNode> to Rc<HfPlusNode>.
/// This allowes for constructing tees off of shared inputs
#[derive(Clone, Debug)]
pub struct HfMemos<'a> {
    exprs: HfMemo<'a, HExpr>,
    inputs: HfMemo<'a, HInput>,
}

/// Memoized conversion from K to a HF+ node.
#[derive(Clone, Debug)]
struct HfMemo<'a, K> {
    map: HashMap<Rc<K>, Rc<RefCell<HfPlusNode<'a>>>>,
}

pub trait HfMemoize<'a, K> 
where K: Hash + Eq + Clone {
    /// Get a clone of the corresponding value's rc pointer if present.
    fn get(&self, key: &Rc<K>) -> Option<Rc<RefCell<HfPlusNode<'a>>>>;

    /// Memoize a mapping.
    fn with(self, key: Rc<K>, value: Rc<RefCell<HfPlusNode<'a>>>) -> Self;
}

impl<'a> HfMemos<'a> {
    pub fn new() -> Self {
        Self {
            exprs: HfMemo::new(),
            inputs: HfMemo::new(),
        }
    }
}

impl<'a, K> HfMemo<'a, K> where K: Hash + Eq + Clone {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

impl<'a, K> HfMemoize<'a, K> for HfMemo<'a, K> 
where K: Hash + Eq + Clone {
    fn get(&self, key: &Rc<K>) -> Option<Rc<RefCell<HfPlusNode<'a>>>> {
        self.map.get(key).map(Clone::clone)
    }

    fn with(mut self, key: Rc<K>, value: Rc<RefCell<HfPlusNode<'a>>>) -> Self {
        self.map
            .insert(key, value);
        self
    }
}

impl<'a> HfMemoize<'a, HExpr> for HfMemos<'a> {
    fn get(&self, key: &Rc<HExpr>) -> Option<Rc<RefCell<HfPlusNode<'a>>>> {
        self.exprs.get(key)
    }

    fn with(mut self, key: Rc<HExpr>, value: Rc<RefCell<HfPlusNode<'a>>>) -> Self {
        self.exprs = self.exprs.with(key, value);
        self
    }
}

impl<'a> HfMemoize<'a, HInput> for HfMemos<'a> {
    fn get(&self, key: &Rc<HInput>) -> Option<Rc<RefCell<HfPlusNode<'a>>>> {
        self.inputs.get(key)
    }

    fn with(mut self, key: Rc<HInput>, value: Rc<RefCell<HfPlusNode<'a>>>) -> Self {
        self.inputs = self.inputs.with(key, value);
        self
    }
}