//Import library modules
use clap::{Parser};

/** GridCli struct schema
 *
 * The GridCli struct is used to store the command line
 * arguments passed into the application
 */
#[derive(Parser)]
#[command(name="Grid Solver")]
#[command(author="whatsacomputertho")]
#[command(version="0.1.0")]
#[command(about="Draw a Hamiltonian path between two vertices in a grid graph G(n, m)")]
pub struct GridCli {
    /// Width of the grid
    #[arg(long="width")]
    pub width: Option<usize>,

    /// Height of the grid
    #[arg(long="height")]
    pub height: Option<usize>,

    /// Start vertex x coordinate
    #[arg(long="start-x")]
    pub start_x: Option<usize>,

    /// Start vertex y coordinate
    #[arg(long="start-y")]
    pub start_y: Option<usize>,

    /// End vertex x coordinate
    #[arg(long="end-x")]
    pub end_x: Option<usize>,

    /// End vertex y coordinate
    #[arg(long="end-y")]
    pub end_y: Option<usize>
}