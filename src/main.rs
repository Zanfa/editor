extern crate cassowary;

use cassowary::{Solver, Variable};
use cassowary::WeightedRelation::*;
use cassowary::strength::{MEDIUM, REQUIRED, STRONG, WEAK};

use std::collections::HashMap;

struct Element {
    left: Variable,
    right: Variable,
}

fn print_changes(names: &HashMap<Variable, &'static str>, changes: &[(Variable, f64)]) {
    println!("Changed:");
    for &(ref var, ref val) in changes {
        println!("{}: {}", names[var], val);
    }
}

fn main() {
    let mut names = HashMap::new();

    let window_width = Variable::new();
    names.insert(window_width, "window_width");
    let box1 = Element {
        left: Variable::new(),
        right: Variable::new(),
    };

    names.insert(box1.left, "box1.left");
    names.insert(box1.right, "box1.right");

    let box2 = Element {
        left: Variable::new(),
        right: Variable::new(),
    };
    names.insert(box2.left, "box2.left");
    names.insert(box2.right, "box2.right");

    let mut solver = Solver::new();
    solver
        .add_constraints(&[
            window_width | GE(REQUIRED) | 0.0,        // positive window width
            box1.left | EQ(REQUIRED) | 0.0,           // left align
            box2.right | EQ(REQUIRED) | window_width, // right align
            box2.left | GE(REQUIRED) | box1.right,    // no overlap
            // positive widths
            box1.left | LE(REQUIRED) | box1.right,
            box2.left | LE(REQUIRED) | box2.right,
            // preferred widths:
            box1.right - box1.left | EQ(WEAK) | 50.0,
            box2.right - box2.left | EQ(WEAK) | 100.0,
        ])
        .unwrap();

    solver.add_edit_variable(window_width, STRONG).unwrap();
    solver.suggest_value(window_width, 300.0).unwrap();

    print_changes(&names, solver.fetch_changes());

    solver.suggest_value(window_width, 500.0).unwrap();
    print_changes(&names, solver.fetch_changes());
}
