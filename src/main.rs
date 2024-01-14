mod gridgraph;
mod gridpath;
mod gridproblem;
mod gridextension;

use crate::gridgraph::GridGraph;
use crate::gridpath::GridPath;
use crate::gridproblem::GridProblem;
use crate::gridextension::GridExtension;

fn main() {
    //Initialize a prime grid graph which is extended in every
    //direction.  This is intended to test our stripping functionality.
    //The original prime grid graph is 4 x 5 with start and end coords
    //(1, 1) and (0, 1) respectively.  The resulting extended graph is
    //8 x 9 with start and end coords (3, 3) and (2, 3) respectively.
    let prime_grid_graph: GridGraph = GridGraph::new(8, 9);
    println!("Dimension 4 x 5 grid graph (possibly prime)");
    println!("{}", prime_grid_graph);
    let mut prime_grid_problem: GridProblem = GridProblem::from_grid_graph(
        prime_grid_graph,
        [3, 3], [2, 3]
    );

    //Check if the prime grid problem can be stripped (should be true)
    println!("Can the problem be stripped? {}", prime_grid_problem.can_be_stripped());

    //Solve the prime grid problem and print the solution.  The problem
    //should have been stripped down until the prime 4 x 5 problem was encountered
    //at which point it should have retrieved the prime solution and then extended
    //the solution back up to the 8 x 9 case.
    let mut prime_solution: GridPath = prime_grid_problem.solve().unwrap();
    println!("Solution");
    println!("{}", prime_solution);
}