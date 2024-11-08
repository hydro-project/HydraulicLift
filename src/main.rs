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
mod utils;

#[cfg(test)]
mod tests;

pub fn main() {
    let hf = compile_dbg!(let hf_in = HfPlusNode::Placeholder => {
        // Send a message asking if hf_in*2 is a registered id
        let id = hf_in * 2;
        let message = if id == 0 {
            "You asked for 0?".to_string()
        } else if check_exists(id).await {
            format!("Found id: {hf_in}")
        } else {
            return None;
            let abcd = panic!("This panic doesn't show up in the resulting hydroflow+!");
        };
        Some(message)
    });

    println!("fn HFPlus() {{{}\n}}", visualize(hf));
}

