use std::{borrow::Borrow, collections::HashMap};

use hydroflow_plus::ir::HfPlusNode;
use quote::ToTokens;


// maybe rename to decompile
pub fn visualize(node: HfPlusNode) -> String {
    let mut memo = NodeMapping { map: HashMap::new(), i: 0 };
    let out = to_vis(&node, &mut memo, 0);
    let mut out_str = String::new();
    for (name, val) in memo.map.values() {
        out_str.push_str(&format!("\n\n{}={};", name, val));
    }
    out_str.push_str(&format!("\n\n{}", out));
    out_str
}

/// Quick and dirty hack:
/// Mapping from addrress of HfNode to (node name, node visualization string)
struct NodeMapping {
    map: HashMap<usize, (String, String)>,
    i: u32
} 

impl NodeMapping {
    fn insert(&mut self, addr: usize, vis_string: String) {
        self.map.insert(addr, (format!("node{}", self.i), vis_string));
        self.i += 1;
    }
}

fn to_vis(node: &HfPlusNode, memo: &mut NodeMapping, tab: usize) -> String {
    let tabs = "\t".repeat(tab);
    match node {
        HfPlusNode::Placeholder => "Placeholder".to_string(),
        HfPlusNode::Tee { inner } => {
            let addr = (*inner).as_ptr() as usize;
            if let None = memo.map.get(&addr) {
                let x = to_vis(&*inner.borrow_mut(), memo, 0);
                memo.insert(addr, x);
            }
            let name = memo.map.get(&addr).unwrap().0.clone();
            format!("{name}.tee()")
        },
        HfPlusNode::Union(n1, n2) =>  {
            let x1 = to_vis(n1, memo, tab+1);
            let x2 = to_vis(n2, memo, tab+1);
            format!("UNION(\n{tabs}\t{},\n{tabs}\t{})", x1, x2)
        },
        HfPlusNode::Map { f, input } => {
            let x = to_vis(&input, memo, tab);
            let f = f.to_token_stream();
            format!("{} \n{tabs}.MAP({})", x, f)
        },
        HfPlusNode::FilterMap { f, input } => {
            let x = to_vis(&input, memo, tab);
            let f = f.to_token_stream();
            format!("{} \n{tabs}.FILTER_MAP({})", x, f)
        },
        _ => panic!("Visualizer doesn't support this hf+ node yet.")
    }
}