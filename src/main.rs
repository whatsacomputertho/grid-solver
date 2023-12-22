mod gridgraph;
mod gridpath;
mod gridproblem;

use crate::gridgraph::GridGraph;
use crate::gridpath::GridPath;
use crate::gridproblem::GridProblem;

fn main() {
    //Initialize a possibly prime grid graph and print it
    let prime_grid_graph: GridGraph = GridGraph::new(4, 5);
    println!("Dimension 4 x 5 grid graph (possibly prime)");
    println!("{}", prime_grid_graph);

    //Initialize a prime grid problem from the grid graph
    let prime_grid_problem: GridProblem = GridProblem::from_grid_graph(
        prime_grid_graph,
        [1, 1], [0, 1]
    );

    //Solve the prime grid problem and print the solution
    let prime_solution: GridPath = prime_grid_problem.solve().unwrap();
    println!("Solution");
    println!("{}", prime_solution);
}