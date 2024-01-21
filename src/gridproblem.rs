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
        if start_coords[0] >= width || end_coords[0] >= width ||
           start_coords[1] >= height || end_coords[1] >= height {
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

    /// Check if the grid problem is acceptable
    pub fn is_acceptable(&self) -> bool {
        let are_color_compatible: bool = self.grid_graph.are_color_compatible(self.start_coords, self.end_coords);
        let is_forbidden: bool = self.grid_graph.is_forbidden(self.start_coords, self.end_coords);
        if are_color_compatible && !is_forbidden {
            return true;
        }
        return false;
    }

    /// Strip the grid problem to the right if it can be stripped
    fn strip_right(&mut self) -> bool {
        //Check if either the start vertex or the end vertex is less than
        //two units away from the right boundary
        let bound: usize = self.grid_graph.get_width();
        let start_diff: usize = bound - self.start_coords[0];
        let end_diff: usize = bound - self.end_coords[0];
        if start_diff <= 2 || end_diff <= 2 {
            return false;
        }

        //If not then create a new GridProblem with width decreased by 2
        //and check if it is acceptable, if not then exit early
        let stripped_grid_problem: GridProblem = GridProblem::new(
            self.grid_graph.get_width() - 2,
            self.grid_graph.get_height(),
            self.start_coords,
            self.end_coords
        );
        if !stripped_grid_problem.is_acceptable() {
            return false;
        }

        //If it can be stripped to the right then strip it to the right
        //and return true to signify that the problem was stripped
        self.grid_graph = GridGraph::new(
            self.grid_graph.get_width() - 2,
            self.grid_graph.get_height()
        );
        self.extensions.push(GridExtension::Right);
        true
    }

    /// Strip the grid problem above if it can be stripped
    fn strip_up(&mut self) -> bool {
        //Check if either the start vertex or the end vertex is less than
        //two units away from the upper boundary
        let bound: usize = self.grid_graph.get_height();
        let start_diff: usize = bound - self.start_coords[1];
        let end_diff: usize = bound - self.end_coords[1];
        if start_diff <= 2 || end_diff <= 2 {
            return false;
        }

        //If not then create a new GridProblem with height decreased by 2
        //and check if it is acceptable, if not then exit early
        let stripped_grid_problem: GridProblem = GridProblem::new(
            self.grid_graph.get_width(),
            self.grid_graph.get_height() - 2,
            self.start_coords,
            self.end_coords
        );
        if !stripped_grid_problem.is_acceptable() {
            return false;
        }

        //If it can be stripped to the right then strip it above and return
        //true to signify that the problem was stripped
        self.grid_graph = GridGraph::new(
            self.grid_graph.get_width(),
            self.grid_graph.get_height() - 2
        );
        self.extensions.push(GridExtension::Up);
        true
    }

    /// Strip the grid problem to the left if it can be stripped
    fn strip_left(&mut self) -> bool {
        //Check if either the start vertex or the end vertex is less than
        //two units away from the left boundary, if so then exit early
        if self.start_coords[0] < 2 || self.end_coords[0] < 2 {
            return false;
        }

        //If not then create a new GridProblem with width decreased by 2
        //and check if it is acceptable, if not then exit early
        let stripped_start_coords: [usize; 2] = [
            self.start_coords[0] - 2,
            self.start_coords[1]
        ];
        let stripped_end_coords: [usize; 2] = [
            self.end_coords[0] - 2,
            self.end_coords[1]
        ];
        let stripped_grid_problem: GridProblem = GridProblem::new(
            self.grid_graph.get_width() - 2,
            self.grid_graph.get_height(),
            stripped_start_coords,
            stripped_end_coords
        );
        if !stripped_grid_problem.is_acceptable() {
            return false;
        }

        //If it can be stripped to the left then strip it to the left
        //and return true to signify that the problem was stripped
        self.grid_graph = GridGraph::new(
            self.grid_graph.get_width() - 2,
            self.grid_graph.get_height()
        );
        self.start_coords = stripped_start_coords;
        self.end_coords = stripped_end_coords;
        self.extensions.push(GridExtension::Left);
        true
    }

    /// Strip the grid problem below if it can be stripped
    fn strip_down(&mut self) -> bool {
        //Check if either the start vertex or the end vertex is less than
        //two units away from the lower boundary, if so then exit early
        if self.start_coords[1] < 2 || self.end_coords[1] < 2 {
            return false;
        }

        //If not then create a new GridProblem with height decreased by 2
        //and check if it is acceptable, if not then exit early
        let stripped_start_coords: [usize; 2] = [
            self.start_coords[0],
            self.start_coords[1] - 2
        ];
        let stripped_end_coords: [usize; 2] = [
            self.end_coords[0],
            self.end_coords[1] - 2
        ];
        let stripped_grid_problem: GridProblem = GridProblem::new(
            self.grid_graph.get_width(),
            self.grid_graph.get_height() - 2,
            stripped_start_coords,
            stripped_end_coords
        );
        if !stripped_grid_problem.is_acceptable() {
            return false;
        }

        //If it can be stripped below then strip it below and return true
        //to signify that the problem was stripped
        self.grid_graph = GridGraph::new(
            self.grid_graph.get_width(),
            self.grid_graph.get_height() - 2
        );
        self.start_coords = stripped_start_coords;
        self.end_coords = stripped_end_coords;
        self.extensions.push(GridExtension::Down);
        true
    }

    /// Strip the grid problem if it can be stripped
    pub fn strip(&mut self) -> bool {
        if self.strip_right() {
            return true;
        } else if self.strip_up() {
            return true;
        } else if self.strip_left() {
            return true;
        } else if self.strip_down() {
            return true;
        }
        return false;
    }

    /// Check if the grid problem can be split horizontally
    pub fn can_be_split_horizontally(&self) -> bool {
        //Check if the start and end vertex share a y coordinate, if so
        //then return false
        if self.start_coords[1] == self.end_coords[1] {
            return false;
        }

        //If they do not share a y coordinate, then loop through the
        //vertices of the grid graph starting at the lesser y coordinate
        //of the start and end vertices and looping until we reach one
        //less than the greater y coordinate of the two
        let is_start_coords_below: bool = self.start_coords[1] < self.end_coords[1];
        let outer_range_start = if is_start_coords_below { self.start_coords[1] } else { self.end_coords[1] };
        let outer_range_end = if is_start_coords_below { self.end_coords[1] } else { self.start_coords[1] };
        let outer_range = outer_range_start..outer_range_end;
        for i in outer_range {
            for j in 0..self.grid_graph.get_width() {
                //Continue if either the upper or lower vertices are either
                //the start or end vertices
                let lower_vertex_coords: [usize; 2] = [j, i];
                let upper_vertex_coords: [usize; 2] = [j, i+1];
                if lower_vertex_coords == self.start_coords || upper_vertex_coords == self.start_coords ||
                   lower_vertex_coords == self.end_coords || upper_vertex_coords == self.end_coords {
                    continue;
                }

                //Initialize two sub GridProblems with the upper vertex coords
                //and the lower vertex coords inserted as new start/end vertices
                let lower_sub_problem: GridProblem = if is_start_coords_below {
                    GridProblem::new(
                        self.grid_graph.get_width(),
                        upper_vertex_coords[1],
                        self.start_coords,
                        lower_vertex_coords
                    )
                } else {
                    GridProblem::new(
                        self.grid_graph.get_width(),
                        upper_vertex_coords[1],
                        lower_vertex_coords,
                        self.end_coords
                    )
                };
                let upper_sub_problem: GridProblem = if is_start_coords_below {
                    GridProblem::new(
                        self.grid_graph.get_width(),
                        self.grid_graph.get_height() - upper_vertex_coords[1],
                        [upper_vertex_coords[0], 0],
                        [self.end_coords[0], self.end_coords[1] - upper_vertex_coords[1]]
                    )
                } else {
                    GridProblem::new(
                        self.grid_graph.get_width(),
                        self.grid_graph.get_height() - upper_vertex_coords[1],
                        [self.start_coords[0], self.start_coords[1] - upper_vertex_coords[1]],
                        [upper_vertex_coords[0], 0]
                    )
                };
                
                //If the left and right sub problems are both acceptable then
                //return true, otherwise continue
                if lower_sub_problem.is_acceptable() && upper_sub_problem.is_acceptable() {
                    return true;
                }
            }
        }

        //If no split is found such that both sub problems are acceptable, return false
        false
    }

    /// Check if the grid problem can be split vertically
    pub fn can_be_split_vertically(&self) -> bool {
        //Check if the start and end vertex share an x coordinate, if so
        //then return false
        if self.start_coords[0] == self.end_coords[0] {
            return false;
        }

        //If they do not share an x coordinate, then loop through the
        //vertices of the grid graph starting at the lesser x coordinate
        //of the start and end vertices and looping until we reach one
        //less than the greater x coordinate of the two
        let is_start_coords_left: bool = self.start_coords[0] < self.end_coords[0];
        let outer_range_start = if is_start_coords_left { self.start_coords[0] } else { self.end_coords[0] };
        let outer_range_end = if is_start_coords_left { self.end_coords[0] } else { self.start_coords[0] };
        let outer_range = outer_range_start..outer_range_end;
        for i in outer_range {
            for j in 0..self.grid_graph.get_height() {
                //Continue if either the left or right vertices are either
                //the start or end vertices
                let left_vertex_coords: [usize; 2] = [i, j];
                let right_vertex_coords: [usize; 2] = [i+1, j];
                if left_vertex_coords == self.start_coords || right_vertex_coords == self.start_coords ||
                   left_vertex_coords == self.end_coords || right_vertex_coords == self.end_coords {
                    continue;
                }

                //Initialize two sub GridProblems with the left vertex coords
                //and the right vertex coords inserted as new start/end vertices
                let left_sub_problem: GridProblem = if is_start_coords_left {
                    GridProblem::new(
                        right_vertex_coords[0],
                        self.grid_graph.get_height(),
                        self.start_coords,
                        left_vertex_coords
                    )
                } else {
                    GridProblem::new(
                        right_vertex_coords[0],
                        self.grid_graph.get_height(),
                        left_vertex_coords,
                        self.end_coords
                    )
                };
                let right_sub_problem: GridProblem = if is_start_coords_left {
                    GridProblem::new(
                        self.grid_graph.get_width() - right_vertex_coords[0],
                        self.grid_graph.get_height(),
                        [0, right_vertex_coords[1]],
                        [self.end_coords[0] - right_vertex_coords[0], self.end_coords[1]]
                    )
                } else {
                    GridProblem::new(
                        self.grid_graph.get_width() - right_vertex_coords[0],
                        self.grid_graph.get_height(),
                        [self.start_coords[0] - right_vertex_coords[0], self.start_coords[1]],
                        [0, right_vertex_coords[1]]
                    )
                };
                
                //If the left and right sub problems are both acceptable then
                //return true, otherwise continue
                if left_sub_problem.is_acceptable() && right_sub_problem.is_acceptable() {
                    return true;
                }
            }
        }

        //If no split is found such that both sides are acceptable, return false
        false
    }

    /// Split the grid problem horizontally and return the subproblems
    pub fn split_horizontally(&self) -> Option<(GridProblem, GridProblem)> {
        //Check if the start and end vertex share a y coordinate, if so
        //then return None
        if self.start_coords[1] == self.end_coords[1] {
            return None;
        }

        //If they do not share a y coordinate, then loop through the
        //vertices of the grid graph starting at the lesser y coordinate
        //of the start and end vertices and looping until we reach one
        //less than the greater y coordinate of the two
        let is_start_coords_below: bool = self.start_coords[1] < self.end_coords[1];
        let outer_range_start = if is_start_coords_below { self.start_coords[1] } else { self.end_coords[1] };
        let outer_range_end = if is_start_coords_below { self.end_coords[1] } else { self.start_coords[1] };
        let outer_range = outer_range_start..outer_range_end;
        for i in outer_range {
            for j in 0..self.grid_graph.get_width() {
                //Continue if either the upper or lower vertices are either
                //the start or end vertices
                let lower_vertex_coords: [usize; 2] = [j, i];
                let upper_vertex_coords: [usize; 2] = [j, i+1];
                if lower_vertex_coords == self.start_coords || upper_vertex_coords == self.start_coords ||
                   lower_vertex_coords == self.end_coords || upper_vertex_coords == self.end_coords {
                    continue;
                }

                //Initialize two sub GridProblems with the upper vertex coords
                //and the lower vertex coords inserted as new start/end vertices
                let lower_sub_problem: GridProblem = if is_start_coords_below {
                    GridProblem::new(
                        self.grid_graph.get_width(),
                        upper_vertex_coords[1],
                        self.start_coords,
                        lower_vertex_coords
                    )
                } else {
                    GridProblem::new(
                        self.grid_graph.get_width(),
                        upper_vertex_coords[1],
                        lower_vertex_coords,
                        self.end_coords
                    )
                };
                let upper_sub_problem: GridProblem = if is_start_coords_below {
                    GridProblem::new(
                        self.grid_graph.get_width(),
                        self.grid_graph.get_height() - upper_vertex_coords[1],
                        [upper_vertex_coords[0], 0],
                        [self.end_coords[0], self.end_coords[1] - upper_vertex_coords[1]]
                    )
                } else {
                    GridProblem::new(
                        self.grid_graph.get_width(),
                        self.grid_graph.get_height() - upper_vertex_coords[1],
                        [self.start_coords[0], self.start_coords[1] - upper_vertex_coords[1]],
                        [upper_vertex_coords[0], 0]
                    )
                };
                
                //If the left and right sub problems are both acceptable then
                //return them, otherwise continue
                if lower_sub_problem.is_acceptable() && upper_sub_problem.is_acceptable() {
                    return Some((lower_sub_problem, upper_sub_problem));
                }
            }
        }

        //If no split is found such that both sub problems are acceptable, return None
        None
    }

    /// Split the grid problem vertically and return the subproblems
    pub fn split_vertically(&self) -> Option<(GridProblem, GridProblem)> {
        //Check if the start and end vertex share an x coordinate, if so
        //then return None
        if self.start_coords[0] == self.end_coords[0] {
            return None;
        }

        //If they do not share an x coordinate, then loop through the
        //vertices of the grid graph starting at the lesser x coordinate
        //of the start and end vertices and looping until we reach one
        //less than the greater x coordinate of the two
        let is_start_coords_left: bool = self.start_coords[0] < self.end_coords[0];
        let outer_range_start = if is_start_coords_left { self.start_coords[0] } else { self.end_coords[0] };
        let outer_range_end = if is_start_coords_left { self.end_coords[0] } else { self.start_coords[0] };
        let outer_range = outer_range_start..outer_range_end;
        for i in outer_range {
            for j in 0..self.grid_graph.get_height() {
                //Continue if either the left or right vertices are either
                //the start or end vertices
                let left_vertex_coords: [usize; 2] = [i, j];
                let right_vertex_coords: [usize; 2] = [i+1, j];
                if left_vertex_coords == self.start_coords || right_vertex_coords == self.start_coords ||
                   left_vertex_coords == self.end_coords || right_vertex_coords == self.end_coords {
                    continue;
                }

                //Initialize two sub GridProblems with the left vertex coords
                //and the right vertex coords inserted as new start/end vertices
                let left_sub_problem: GridProblem = if is_start_coords_left {
                    GridProblem::new(
                        right_vertex_coords[0],
                        self.grid_graph.get_height(),
                        self.start_coords,
                        left_vertex_coords
                    )
                } else {
                    GridProblem::new(
                        right_vertex_coords[0],
                        self.grid_graph.get_height(),
                        left_vertex_coords,
                        self.end_coords
                    )
                };
                let right_sub_problem: GridProblem = if is_start_coords_left {
                    GridProblem::new(
                        self.grid_graph.get_width() - right_vertex_coords[0],
                        self.grid_graph.get_height(),
                        [0, right_vertex_coords[1]],
                        [self.end_coords[0] - right_vertex_coords[0], self.end_coords[1]]
                    )
                } else {
                    GridProblem::new(
                        self.grid_graph.get_width() - right_vertex_coords[0],
                        self.grid_graph.get_height(),
                        [self.start_coords[0] - right_vertex_coords[0], self.start_coords[1]],
                        [0, right_vertex_coords[1]]
                    )
                };
                
                //If the left and right sub problems are both acceptable then
                //return them, otherwise continue
                if left_sub_problem.is_acceptable() && right_sub_problem.is_acceptable() {
                    return Some((left_sub_problem, right_sub_problem));
                }
            }
        }

        //If no split is found such that both sides are acceptable, return None
        None
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

            //If there is no solution then first strip the problem as much as possible
            loop {
                if !self.strip() {
                    break;
                }
            }

            //Get the width and height of the grid graph
            let width: usize = self.grid_graph.get_width();
            let height: usize = self.grid_graph.get_height();

            //After stripping is complete, check if the problem is prime.  If
            //so then lookup its solution and continue.
            if GridPath::is_prime(width, height, self.start_coords, self.end_coords) {
                solution = GridPath::get_prime(width, height, self.start_coords, self.end_coords);
                continue;
            }

            //If the GridProblem is not prime, break it into subproblems by splitting it
            if self.can_be_split_horizontally() {
                let (mut p_below, mut p_above): (GridProblem, GridProblem) = self.split_horizontally().unwrap();
                let p_below_solution: GridPath = p_below.solve().unwrap();
                let p_above_solution: GridPath = p_above.solve().unwrap();
                let mut vertex_order: Vec<[usize; 2]> = p_below_solution.vertex_order;
                vertex_order.extend(p_above_solution.get_up_shift_vertex_order(p_below.grid_graph.get_height()));
                let solution_path = GridPath::new(
                    p_below.grid_graph.get_width(),
                    p_below.grid_graph.get_height() + p_above.grid_graph.get_height(),
                    vertex_order
                );
                solution = Some(solution_path);
                continue;
            }
            if self.can_be_split_vertically() {
                let (mut p_left, mut p_right): (GridProblem, GridProblem) = self.split_vertically().unwrap();
                let p_left_solution: GridPath = p_left.solve().unwrap();
                let p_right_solution: GridPath = p_right.solve().unwrap();
                let mut vertex_order: Vec<[usize; 2]> = p_left_solution.vertex_order;
                vertex_order.extend(p_right_solution.get_right_shift_vertex_order(p_left.grid_graph.get_width()));
                let solution_path = GridPath::new(
                    p_left.grid_graph.get_width() + p_right.grid_graph.get_width(),
                    p_left.grid_graph.get_height(),
                    vertex_order
                );
                solution = Some(solution_path);
                continue;
            }

            //Check if either of the dimensions of the grid graph is 1, if so then solve it
            //and set the solution path
            if width == 1 || height == 1 {
                let is_width: bool = width == 1;
                let path: Vec<[usize; 2]> = {
                    let mut path_vec: Vec<[usize; 2]> = Vec::new();
                    let bound: usize = if is_width { height } else { width };
                    let range = if is_width && self.start_coords[1] != 0 { (0..bound).rev().collect::<Vec<_>>() }
                                else if !is_width && self.start_coords[0] != 0 { (0..bound).rev().collect::<Vec<_>>() }
                                else { (0..bound).collect::<Vec<_>>() };
                    for i in range {
                        let vertex_coords: [usize; 2] = if is_width { [0, i] } else { [i, 0] };
                        path_vec.push(vertex_coords);
                    }
                    path_vec
                };
                solution = Some(GridPath::new(width, height, path));
                continue;
            }

            //This point should be unreachable, to avoid an infinite loop here we panic
            panic!("Grid problem was acceptable but had no solution, could not be stripped, split, or solved.");
        }
    }
}