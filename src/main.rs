#![feature(box_patterns)]


//steps:
// 1) Encapsulate special-cased rust logic, pulling all dataflow-relevant operations above the barrier
// 2) Traverse tree backwards, annotating all scoping information
// 3) Produce hydroflow from the annotated tree

//old
// 1) transform into subset of rust with simple statements and expressions
// 2) transform these statements into dataflowy structure?
// 3) output hydroflow

use hfplus_ir::visualize::visualize;
use hydroflow_plus::ir::HfPlusNode;


mod r_ir;
mod h_ir;
mod hfplus_ir;
mod compile;
mod tests;
mod utils;

pub fn main() {
    let hf = compile!(let hf_in = HfPlusNode::Placeholder => {
        let x = hf_in + 1;
        return x;
        x + 2 // this doesn't show up in the resulting HF+!
    });

    println!("fn HFPlus() {{{}\n}}", visualize(hf));
}
