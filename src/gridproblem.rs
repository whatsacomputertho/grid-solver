use crate::gridgraph::GridGraph;
use crate::gridpath::GridPath;

/// # GridProblem struct
///
/// A `GridProblem` is initialized with a `GridGraph`, and
/// start and end vertex coordinates.  It may optionally be
/// initialized with grid graph dimensions (given as usizes).
///
/// It provides the necessary functionality for deconstructing
/// and reconstructing the grid graph into a Hamiltonian path
/// between its vertices from the specified start vertex and
/// to the specified end vertex.
pub struct GridProblem {
    grid_graph: GridGraph,
    start_coords: [usize; 2],
    end_coords: [usize; 2]
}

impl GridProblem {
    /// Initialize a `GridProblem` given grid dimensions and
    /// start and end vertex coordinates.
    pub fn new(width: usize, height: usize, start_coords: [usize; 2], end_coords: [usize; 2]) -> GridProblem {
        //Sanity check the grid graph coordinates against the given
        //start and end vertex coordinates
        if start_coords[0] >= width - 1 || end_coords[1] >= width - 1 ||
           start_coords[1] >= height - 1 || end_coords[1] >= height - 1 {
            panic!(
                "Vertex coordinates out of bounds of {} x {}: ({}, {}), ({}, {})",
                width, height, start_coords[0], start_coords[1],
                end_coords[0], end_coords[1]
            );
        }

        //Initialize a new grid graph
        let grid_graph: GridGraph = GridGraph::new(width, height);

        //Initialize the grid problem
        GridProblem {
            grid_graph,
            start_coords,
            end_coords
        }
    }

    /// Initialize a `GridProblem` given a `GridGraph` and
    /// start and end vertex coordinates.
    pub fn from_grid_graph(grid_graph: GridGraph, start_coords: [usize; 2], end_coords: [usize; 2]) -> GridProblem {
        //Sanity check the grid graph coordinates against the given
        //start and end vertex coordinates
        let width: usize = grid_graph.get_width();
        let height: usize = grid_graph.get_height();
        if start_coords[0] >= width - 1 || end_coords[1] >= width - 1 ||
           start_coords[1] >= height - 1 || end_coords[1] >= height - 1 {
            panic!(
                "Vertex coordinates out of bounds of {} x {}: ({}, {}), ({}, {})",
                width, height, start_coords[0], start_coords[1],
                end_coords[0], end_coords[1]
            );
        }

        //Initialize the grid problem
        GridProblem {
            grid_graph,
            start_coords,
            end_coords
        }
    }

    /// Solve the grid problem by stripping and splitting it
    /// into sub-problems
    pub fn solve(&self) -> Option<GridPath> {
        //Check if the problem is acceptable
        let are_color_compatible: bool = self.grid_graph.are_color_compatible(self.start_coords, self.end_coords);
        let is_forbidden: bool = self.grid_graph.is_forbidden(self.start_coords, self.end_coords);
        if !are_color_compatible || is_forbidden {
            //If the problem is not acceptable, then there is no solution
            return None;
        }

        //Get the width and height of the grid graph
        let width: usize = self.grid_graph.get_width();
        let height: usize = self.grid_graph.get_height();

        //Check if either of the dimensions of the grid graph is 1
        if width == 1 || height == 1 {
            //Return the obvious path between the start and end vertices
            let is_width: bool = width == 1;
            let path: Vec<[usize; 2]> = {
                let mut path_vec: Vec<[usize; 2]> = Vec::new();
                let bound: usize = if is_width { height } else { width };
                for i in 0..bound {
                    let vertex_coords: [usize; 2] = if is_width { [0, i] } else { [i, 0] };
                    path_vec.push(vertex_coords);
                }
                path_vec
            };
            return Some(GridPath::new(width, height, path));
        }

        //Check if this is a prime problem, if so then return it
        if GridPath::is_prime(width, height, self.start_coords, self.end_coords) {
            return GridPath::get_prime(width, height, self.start_coords, self.end_coords);
        }

        //TODO: Strip and split non prime paths until all are prime
        //then solve and reconstruct
        return None;
    }
}