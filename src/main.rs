mod gridgraph;
mod gridpath;
mod gridproblem;
mod gridextension;
mod gridcli;

use std::process;
use clap::Parser;
use crate::gridcli::GridCli;
use crate::gridpath::GridPath;
use crate::gridproblem::GridProblem;

fn main() {
    //Parse the command line args
    let cli_args = GridCli::parse();
    let width: usize = match cli_args.width {
        Some(x) => x as usize,
        None => {
            eprintln!("Please specify the width of the grid using the --width argument");
            process::exit(1);
        }
    };
    let height: usize = match cli_args.height {
        Some(x) => x as usize,
        None => {
            eprintln!("Please specify the height of the grid using the --height argument");
            process::exit(1);
        }
    };
    let start_x: usize = match cli_args.start_x {
        Some(x) => x as usize,
        None => {
            eprintln!("Please specify the x coordinate of the start vertex using the --start-x argument");
            process::exit(1);
        }
    };
    let start_y: usize = match cli_args.start_y {
        Some(x) => x as usize,
        None => {
            eprintln!("Please specify the y coordinate of the start vertex using the --start-y argument");
            process::exit(1);
        }
    };
    let end_x: usize = match cli_args.end_x {
        Some(x) => x as usize,
        None => {
            eprintln!("Please specify the x coordinate of the end vertex using the --end-x argument");
            process::exit(1);
        }
    };
    let end_y: usize = match cli_args.end_y {
        Some(x) => x as usize,
        None => {
            eprintln!("Please specify the y coordinate of the end vertex using the --end-x argument");
            process::exit(1);
        }
    };

    //Initialize a grid problem given the dimensions of the grid graph
    //and the start and end coordinates
    let mut problem: GridProblem = GridProblem::new(width, height, [start_x, start_y], [end_x, end_y]);
    let solution: GridPath = match problem.solve() {
        Some(x) => x,
        None => {
            eprintln!(
                "The grid problem was not acceptable, either:
    - Its start coordinates were not color compatible, or
    - It was a forbidden problem"
            );
            process::exit(1);
        }
    };
    println!("{}", solution);
}