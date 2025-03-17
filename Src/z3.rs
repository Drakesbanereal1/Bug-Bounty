use z3::{Config, Context, Solver};
use z3::ast::{Int, Ast};
use std::ops::{Add, Sub}; // Import the Add and Sub traits

pub fn run_z3() -> Result<(), Box<dyn std::error::Error>> {
    // Setting up the Z3 context and solver
    let config = Config::new();
    let ctx = Context::new(&config);

    // Create integer constants using Int::new_const
    let x = Int::new_const(&ctx, "x");
    let y = Int::new_const(&ctx, "y");

    // Perform addition and subtraction directly on `Int` objects
    let sum = x.clone().add(&y);
    let difference = x.sub(&y);

    // Create constraints using Z3's `Int` expressions
    let constraint1 = sum._eq(&Int::from_i64(&ctx, 10));  // x + y == 10
    let constraint2 = difference._eq(&Int::from_i64(&ctx, 2));  // x - y == 2

    // Add constraints to the solver
    let solver = Solver::new(&ctx);
    solver.assert(&constraint1);
    solver.assert(&constraint2);

    // Check satisfiability
    if solver.check() == z3::SatResult::Sat {
        println!("Satisfiable!");
    } else {
        println!("Unsatisfiable.");
    }

    Ok(())
}

fn main() {
    // Running the Z3 solver
    if let Err(e) = run_z3() {
        eprintln!("Error: {}", e);
    }
}
