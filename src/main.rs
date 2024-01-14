mod gridgraph;
mod gridpath;
mod gridproblem;
mod gridextension;

use crate::gridgraph::GridGraph;
use crate::gridpath::GridPath;
use crate::gridproblem::GridProblem;
use crate::gridextension::GridExtension;

fn main() {
    //Initialize a possibly prime grid graph and print it
    let prime_grid_graph: GridGraph = GridGraph::new(4, 5);
    println!("Dimension 4 x 5 grid graph (possibly prime)");
    println!("{}", prime_grid_graph);

    //Initialize a prime grid problem from the grid graph
    let mut prime_grid_problem: GridProblem = GridProblem::from_grid_graph(
        prime_grid_graph,
        [1, 1], [0, 1]
    );

    //Solve the prime grid problem and print the solution
    let mut prime_solution: GridPath = prime_grid_problem.solve().unwrap();
    println!("Solution");
    println!("{}", prime_solution);

    //Extend the prime grid solution to the right
    prime_solution.extend(GridExtension::Right);
    println!("Right-extended solution");
    println!("{}", prime_solution);

    //Extend the prime grid solution upward
    prime_solution.extend(GridExtension::Up);
    println!("Up-extended solution");
    println!("{}", prime_solution);

    //Extend the prime grid solution to the left
    prime_solution.extend(GridExtension::Left);
    println!("Left-extended solution");
    println!("{}", prime_solution);

    //Extend the prime grid solution downward
    prime_solution.extend(GridExtension::Down);
    println!("Down-extended solution");
    println!("{}", prime_solution);
}