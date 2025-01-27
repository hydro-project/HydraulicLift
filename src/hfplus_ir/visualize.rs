use std::collections::HashMap;

use hydroflow_plus::ir::{HfPlusLeaf, HfPlusNode};
use quote::{quote, ToTokens};


// maybe rename to decompile
pub fn visualize((leaves, node): (Vec<HfPlusLeaf>, HfPlusNode)) -> String {
    let mut memo = NodeMapping { ids: HashMap::new(), defs: Vec::new(), i: 0 };
    let leaves_out:Vec<_> = leaves.iter().map(|leaf| to_vis_leaf(leaf, &mut memo)).collect();
    let out = to_vis(&node, &mut memo, 1);
    let mut out_str = String::new();
    for (i, def) in memo.defs.iter().enumerate() {
        out_str.push_str(&format!("\n\nnode{}={};", i, def));
    }
    for leaf_out in leaves_out {
        out_str.push_str(&format!("\n\n{leaf_out};"));
    }
    out_str.push_str(&format!("\n\n{}", out));
    out_str
}

/// Quick and dirty hack:
/// Mapping from address of HfNode to node id, and node id is index to definition string
struct NodeMapping {
    ids: HashMap<usize, u32>,
    defs: Vec<String>,
    i: u32
} 

impl NodeMapping {
    fn insert(&mut self, addr: usize, vis_string: String) {
        self.ids.insert(addr, self.i);
        self.defs.push(vis_string);
        self.i += 1;
    }
}

fn to_vis_leaf(node: &HfPlusLeaf, memo: &mut NodeMapping) -> String {
    match node {
        HfPlusLeaf::CycleSink { ident, input, .. } => {
            let x = to_vis(&input, memo, 1);
            format!("{} \n\t.cycle_sink(\"{}\")", x, ident.to_token_stream())
        },
        _ => panic!("Visualizer doesn't support this hf+ leaf node yet.")
    }
}

fn to_vis(node: &HfPlusNode, memo: &mut NodeMapping, tab: usize) -> String {
    let tabs = "\t".repeat(tab);
    match node {
        HfPlusNode::Placeholder => "Placeholder".to_string(),
        HfPlusNode::Tee { inner } => {
            let addr = (*inner).as_ptr() as usize;
            if let None = memo.ids.get(&addr) {
                let x = to_vis(&*inner.borrow_mut(), memo, 1);
                memo.insert(addr, x);
            }
            let id = memo.ids.get(&addr).unwrap().clone();
            format!("node{id}.tee()")
        },
        HfPlusNode::Union(n1, n2) =>  {
            let x1 = to_vis(n1, memo, tab);
            let x2 = to_vis(n2, memo, tab+1);
            format!("{x1} \n{tabs}.union(\n{tabs}\t{x2}\n{tabs})")
        },
        HfPlusNode::Map { f, input } => {
            let x = to_vis(&input, memo, tab);
            let f = f.to_token_stream();
            format!("{} \n{tabs}.map({})", x, f)
        },
        HfPlusNode::FilterMap { f, input } => {
            let x = to_vis(&input, memo, tab);
            let f = f.to_token_stream();
            format!("{} \n{tabs}.filter_map({})", x, f)
        },
        HfPlusNode::Persist(input) => {
            // TODO: this is used to represent poll_futures while that is not in hf+
            let x = to_vis(&input, memo, tab);
            format!("{} \n{tabs}.poll_futures()", x)
        },
        HfPlusNode::CycleSource { ident, .. } => {
            format!("cycle_source(\"{}\")", ident.to_token_stream())
        }
        _ => panic!("Visualizer doesn't support this hf+ node yet.")
    }
}