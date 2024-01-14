use crate::gridgraph::GridGraph;
use crate::gridpath::GridPath;
use crate::gridextension::GridExtension;

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
    extensions: Vec<GridExtension>,
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

        //Initialize an empty vector of grid extensions
        let grid_extensions: Vec<GridExtension> = Vec::new();

        //Initialize the grid problem
        GridProblem {
            grid_graph: grid_graph,
            extensions: grid_extensions,
            start_coords: start_coords,
            end_coords: end_coords
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

        //Initialize an empty vector of grid extensions
        let grid_extensions: Vec<GridExtension> = Vec::new();

        //Initialize the grid problem
        GridProblem {
            grid_graph: grid_graph,
            extensions: grid_extensions,
            start_coords: start_coords,
            end_coords: end_coords
        }
    }

    /// Check if the grid problem is acceptable
    pub fn is_acceptable(&self) -> bool {
        let are_color_compatible: bool = self.grid_graph.are_color_compatible(self.start_coords, self.end_coords);
        let is_forbidden: bool = self.grid_graph.is_forbidden(self.start_coords, self.end_coords);
        if are_color_compatible && !is_forbidden {
            return true;
        }
        return false;
    }

    /// Reconstruct the original GridGraph and restore the original
    /// coordinates if the GridGraph was stripped during the solution
    /// of the GridProblem.  Clear the GridProblem's list of extensions
    /// in the process.
    pub fn reconstruct(&mut self) {
        //Check if any extensions exist, if not then exit early
        if self.extensions.len() == 0_usize {
            return;
        }

        //Initialize new GridGraph dimensions and new start and end
        //coordinates
        let mut new_width: usize = self.grid_graph.get_width();
        let mut new_height: usize = self.grid_graph.get_height();
        let mut new_start_coords: [usize; 2] = self.start_coords;
        let mut new_end_coords: [usize; 2] = self.end_coords;

        //Loop through the GridProblem's extensions and determine the
        //new GridGraph dimensions as well as the new start and end
        //coordinates
        for extension in self.extensions.iter() {
            match extension {
                GridExtension::Right => new_width += 2_usize,
                GridExtension::Up => new_height += 2_usize,
                GridExtension::Left => {
                    new_width += 2_usize;
                    new_start_coords[0] += 2_usize;
                    new_end_coords[0] += 2_usize;
                },
                GridExtension::Down => {
                    new_height += 2_usize;
                    new_start_coords[1] += 2_usize;
                    new_end_coords[1] += 2_usize;
                }
            }
        }

        //Initialize a new GridGraph using the new dimensions and update it
        let new_grid_graph: GridGraph = GridGraph::new(new_width, new_height);
        self.grid_graph = new_grid_graph;

        //Update the start and end coords using the new coords
        self.start_coords = new_start_coords;
        self.end_coords = new_end_coords;

        //Clear the extensions
        self.extensions.clear();
    }

    /// Solve the grid problem by stripping and splitting it
    /// into sub-problems
    pub fn solve(&mut self) -> Option<GridPath> {
        //If the problem is not acceptable, then there is no solution
        if !self.is_acceptable() {
            return None;
        }

        //Initialize mutable grid graph, solution path, & collection of extensions
        let mut solution: Option<GridPath> = None;
        
        //Loop until solved
        loop {
            //Check if there is a solution path
            let is_solution: bool = match solution {
                Some(ref _x) => true,
                None => false
            };

            //If there is a solution path then extend it as needed and return it
            if is_solution {
                //Unwrap the solution path and extend it if any strips were performed
                let mut solution_path: GridPath = solution.unwrap();
                solution_path.extend_many(&self.extensions);

                //Reconstruct the original GridProblem after having stripped it
                self.reconstruct();
                return Some(solution_path);
            }

            //If there is no solution path then break down the problem
            //TODO: This implies the need for a GridProblem function "can_be_stripped"
            //which assesses whether the grid problem can be stripped in any direction
            //while self.can_be_stripped() {
                //TODO: This implies the need for a GridProblem function "strip" which
                //strips the GridProblem in some direction, following the same order as
                //the aforementioned "can_be_stripped" function.
            //    self.strip();
            //}

            //If the GridProblem can be split, then get its subproblems, solve them, and
            //join their solutions into a solution path for the larger GridProblem.
            //TODO: This implies the need for a GridProblem function "can_be_split_horizontally"
            //which assesses whether the grid problem can be split along any horizontal edge.
            //if self.can_be_split_horizontally() {
                //TODO: This implies the need for a GridProblem function "split_horizontally"
                //which takes a GridProblem and splits it, returning two smaller GridProblems
                //which are the sub problems of the parent.
            //    let (p_above, p_below): (GridProblem, GridProblem) = self.split_horizontally();
            //    let p_above_solution: GridPath = p_above.solve().unwrap();
            //    let p_below_solution: GridPath = p_below.solve().unwrap();
                //TODO: This implies the need for a GridPath function "join_vertically" which
                //accepts two GridPaths and joins them vertically at a shared vertex
            //    let solution_path = GridPath.join_vertically(p_above_solution, p_below_solution);
            //    solution = Some(solution_path);
            //    continue;
            //}

            //TODO: This implies the need for a GridProblem function "can_be_split_vertically"
            //which assesses whether the grid problem can be split along any vertical edge.
            //if self.can_be_split_vertically() {
                //TODO: This implies the need for a GridProblem function "split_vertically"
                //which takes a GridProblem and splits it, returning two smaller GridProblems
                //which are the sub problems of the parent.
            //    let (p_left, p_right): (GridProblem, GridProblem) = self.split_vertically();
            //    let p_left_solution: GridPath = p_left.solve().unwrap();
            //    let p_right_solution: GridPath = p_right.solve().unwrap();
                //TODO: This implies the need for a GridPath function "join_horizontally" which
                //accepts two GridPaths and joins them horizontally at a shared vertex
            //    let solution_path = GridPath.join_horizontally(p_left_solution, p_right_solution);
            //    solution = Some(solution_path);
            //    continue;
            //}

            //Get the width and height of the grid graph
            let width: usize = self.grid_graph.get_width();
            let height: usize = self.grid_graph.get_height();

            //Check if either of the dimensions of the grid graph is 1, if so then solve it
            //and set the solution path
            if width == 1 || height == 1 {
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
                solution = Some(GridPath::new(width, height, path));
                continue;
            }

            //Check if this is a prime problem, if so then solve it and set the solution path
            if GridPath::is_prime(width, height, self.start_coords, self.end_coords) {
                solution = GridPath::get_prime(width, height, self.start_coords, self.end_coords);
                continue;
            }

            //This point should be unreachable, to avoid an infinite loop here we panic
            panic!("Grid problem was acceptable but had no solution, could not be stripped, split, or solved.");
        }
    }
}