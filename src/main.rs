mod gridgraph;
mod gridpath;
mod gridproblem;
mod gridextension;
mod gridcli;

use clap::Parser;
use crate::gridcli::GridCli;
use crate::gridpath::GridPath;
use crate::gridproblem::GridProblem;

fn main() {
    //Parse the command line args
    let cli_args = GridCli::parse();
    let width: usize = match cli_args.width {
        Some(x) => x as usize,
        None => panic!("Please specify the width of the grid using the --width argument")
    };
    let height: usize = match cli_args.height {
        Some(x) => x as usize,
        None => panic!("Please specify the height of the grid using the --height argument")
    };
    let start_x: usize = match cli_args.start_x {
        Some(x) => x as usize,
        None => panic!("Please specify the x coordinate of the start vertex using the --start-x argument")
    };
    let start_y: usize = match cli_args.start_y {
        Some(x) => x as usize,
        None => panic!("Please specify the y coordinate of the start vertex using the --start-y argument")
    };
    let end_x: usize = match cli_args.end_x {
        Some(x) => x as usize,
        None => panic!("Please specify the x coordinate of the end vertex using the --end-x argument")
    };
    let end_y: usize = match cli_args.end_y {
        Some(x) => x as usize,
        None => panic!("Please specify the y coordinate of the end vertex using the --end-x argument")
    };

    //Initialize a grid problem given the dimensions of the grid graph
    //and the start and end coordinates
    let mut problem: GridProblem = GridProblem::new(width, height, [start_x, start_y], [end_x, end_y]);
    let solution: GridPath = problem.solve().unwrap();
    println!("{}", solution);
}