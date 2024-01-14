use crate::gridextension::GridExtension;

use std::fmt;
use petgraph::Undirected;
use petgraph::graph::Graph;
use petgraph::visit::NodeIndexable;
use lazy_static::lazy_static;
use json::JsonValue;

/// # GridPath struct
///
/// A `GridPath` is an n by m grid of vertices joined by
/// edges forming a path over the grid
pub struct GridPath {
    n: usize,
    m: usize,
    vertex_order: Vec<[usize; 2]>,
    graph: Graph<String, String, Undirected>
}

impl GridPath {
    /// Initialize a GridPath given its dimensions (n by m)
    ///
    /// ### Example
    ///
    /// ```rust
    /// let my_grid_graph: GridPath = GridPath::new(4_usize, 3_usize);
    /// ```
    pub fn new(n: usize, m: usize, vertex_order: Vec<[usize; 2]>) -> GridPath {
        //Get the graph given the vertex order
        let graph = GridPath::get_graph_from_vertex_order(n, m, &vertex_order);

        //Initialize the GridPath
        GridPath {
            n: n,
            m: m,
            vertex_order: vertex_order,
            graph: graph
        }
    }

    /// Given dimensions and a vertext order, get a grid-shaped petgraph graph
    /// structure with edges forming the path given by the vertex order.
    fn get_graph_from_vertex_order(n: usize, m: usize, vertex_order: &Vec<[usize; 2]>) -> Graph<String, String, Undirected> {
        //Initialize the graph
        let mut graph = Graph::new_undirected();

        //Add nodes to the graph
        for i in 0..m {
            for j in 0..n {
                //Add the node
                graph.add_node(format!("({},{})",i,j));
            }
        }

        //Add edges to the graph
        for i in 1..vertex_order.len() {
            //Determine the nodes at the ith and i-1th coordinate pairs
            let n1_x: usize = vertex_order[i-1][0];
            let n1_y: usize = vertex_order[i-1][1];
            let n2_x: usize = vertex_order[i][0];
            let n2_y: usize = vertex_order[i][1];
            let n1_index: usize = (n1_y * n) + n1_x;
            let n2_index: usize = (n2_y * n) + n2_x;
            let n1 = NodeIndexable::from_index(&graph, n1_index);
            let n2 = NodeIndexable::from_index(&graph, n2_index);

            //Draw an edge between them
            graph.add_edge(n1, n2, String::from(""));
        }

        //Return the graph
        graph
    }

    /// Check if there exists a prime solution for the given
    /// dimensions and start and end coordinates
    pub fn is_prime(width: usize, height: usize, start: [usize; 2], end: [usize; 2]) -> bool {
        //Get the static ref to the prime solutions JSON
        let prime_solution_json_ref = &*PRIME_SOLUTION_JSON;

        //Loop through dimension-specific solution objects
        for graph_dimension_solutions in prime_solution_json_ref.members() {
            //If the dimensions do not match those given then continue
            if graph_dimension_solutions["n"] != width || graph_dimension_solutions["m"] != height {
                continue;
            }

            //If the dimensions match then loop through its paths
            for prime_path in graph_dimension_solutions["paths"].members() {
                //If the start and end vertices match those given then return true
                if prime_path[0][0] == start[0] && prime_path[0][1] == start[1] &&
                   prime_path[(width * height) - 1][0] == end[0] && prime_path[(width * height) - 1][1] == end[1] {
                    return true;
                }
            }

            //If the dimensions match but no matching start & end vertex paths were
            //found then return 
            return false;
        }

        //If we make it out of the loop then no solution was found, return false
        return false;
    }

    /// Check if there exists a prime solution for the given
    /// dimensions and start and end coordinates
    pub fn get_prime(width: usize, height: usize, start: [usize; 2], end: [usize; 2]) -> Option<GridPath> {
        //Get the static ref to the prime solutions JSON
        let prime_solution_json_ref = &*PRIME_SOLUTION_JSON;

        //Loop through dimension-specific solution objects
        for graph_dimension_solutions in prime_solution_json_ref.members() {
            //If the dimensions do not match those given then continue
            if graph_dimension_solutions["n"] != width || graph_dimension_solutions["m"] != height {
                continue;
            }

            //If the dimensions match then loop through its paths
            for prime_path in graph_dimension_solutions["paths"].members() {
                //If the start and end vertices match those given then instantiate
                //and return the path
                if prime_path[0][0] == start[0] && prime_path[0][1] == start[1] &&
                   prime_path[(width * height) - 1][0] == end[0] && prime_path[(width * height) - 1][1] == end[1] {
                    return Some(
                        GridPath::new(
                            width, height,
                            prime_path.members().map(|v| [v[0].as_usize().unwrap(), v[1].as_usize().unwrap()]).collect()
                        )
                    );
                }
            }

            //If the dimensions match but no matching start & end vertex paths were
            //found then return None
            return None;
        }

        //If we make it out of the loop then no solution was found, return None
        return None;
    }

    /// Get the width of a grid graph
    pub fn get_width(&self) -> usize {
        self.n
    }

    /// Get the height of a grid graph
    pub fn get_height(&self) -> usize {
        self.m
    }

    /// Increment the x coordinate of all vertices by a usize
    fn get_right_shift_vertex_order(&self, shift: usize) -> Vec<[usize; 2]> {
        //Initialize a new vertex order vec
        let mut new_vertex_order: Vec<[usize; 2]> = Vec::new();

        //Loop through the current vertex order vec and populate the new
        //vertex order vec with vertices shifted n to the right
        for vertex in self.vertex_order.iter() {
            new_vertex_order.push([vertex[0] + shift, vertex[1]]);
        }

        //Return the new vertex order
        new_vertex_order
    }

    /// Increment the x coordinate of all vertices by a usize
    fn get_up_shift_vertex_order(&self, shift: usize) -> Vec<[usize; 2]> {
        //Initialize a new vertex order vec
        let mut new_vertex_order: Vec<[usize; 2]> = Vec::new();

        //Loop through the current vertex order vec and populate the new
        //vertex order vec with vertices shifted n above
        for vertex in self.vertex_order.iter() {
            new_vertex_order.push([vertex[0], vertex[1] + shift]);
        }
        
        //Return the new vertex order
        new_vertex_order
    }

    /// Extend the GridPath with a height-2 strip in the upward direction
    fn extend_up(&mut self) {
        //Loop through the vertices in the vertex order until vertices are
        //found forming an edge on the upper boundary of the grid.  Once
        //found extend the grid path along that edge.
        for i in 1..self.vertex_order.len() {
            //Check if the ith and i-1th vertices are on the upper boundary
            let bound: usize = self.m - 1;
            if self.vertex_order[i][1] != bound || self.vertex_order[i-1][1] != bound {
                continue;
            }

            //If they are then decide which direction to move first and
            //construct the loop ranges accordingly
            let left_first: bool = self.vertex_order[i-1][0] < self.vertex_order[i][0];
            let start_range = if left_first { (0..self.vertex_order[i-1][0] + 1).rev().collect::<Vec<_>>() } else { ((self.vertex_order[i-1][0])..self.n).collect::<Vec<_>>() };
            let mid_range = if left_first { (0..self.n).collect::<Vec<_>>() } else { ((0..self.n).rev()).collect::<Vec<_>>() };
            let end_range = if left_first { (self.vertex_order[i][0]..self.n).rev().collect::<Vec<_>>() } else { (0..self.vertex_order[i][0]).collect::<Vec<_>>() };

            //Initialize a Vec<[usize; 2]> containing the path to add
            let mut ext_path: Vec<[usize; 2]> = Vec::new();

            //Extend the GridPath up by 2
            for j in start_range {
                let next_vertex: [usize; 2] = [j, self.m];
                ext_path.push(next_vertex);
            }
            for j in mid_range {
                let next_vertex: [usize; 2] = [j, self.m + 1];
                ext_path.push(next_vertex);
            }
            for j in end_range {
                let next_vertex: [usize; 2] = [j, self.m];
                ext_path.push(next_vertex);
            }

            //Insert the newly constructed path into the existing vertex order
            //between the i and i-1 vertices
            self.vertex_order.splice(i..i, ext_path);

            //Initialize a new petgraph graph for display of the path and return
            let new_graph = GridPath::get_graph_from_vertex_order(self.n, self.m + 2, &self.vertex_order);
            self.graph = new_graph;

            //Update the vertical dimension of the graph and return
            self.m += 2;
            return;
        }

        //If we reach this point then panic, the graph cannot be extended up
        panic!("No edges on upper boundary of the grid, cannot extend upward");
    }

    /// Extend the GridPath with a height-2 strip in the downward direction
    fn extend_down(&mut self) {
        //Loop through the vertices in the vertex order until vertices are
        //found forming an edge on the upper boundary of the grid.  Once
        //found extend the grid path along that edge.
        for i in 1..self.vertex_order.len() {
            //Check if the ith and i-1th vertices are on the lower boundary
            if self.vertex_order[i][1] != 0 || self.vertex_order[i-1][1] != 0 {
                continue;
            }

            //If found then shift the grid path upward by 2
            let mut new_vertex_order: Vec<[usize; 2]> = self.get_up_shift_vertex_order(2);

            //Decide which direction to move first and construct the loop ranges accordingly
            let left_first: bool = new_vertex_order[i-1][0] < new_vertex_order[i][0];
            let start_range = if left_first { (0..new_vertex_order[i-1][0] + 1).rev().collect::<Vec<_>>() } else { ((new_vertex_order[i-1][0])..self.n).collect::<Vec<_>>() };
            let mid_range = if left_first { (0..self.n).collect::<Vec<_>>() } else { (0..self.n).rev().collect::<Vec<_>>() };
            let end_range = if left_first { (new_vertex_order[i][0]..self.n).rev().collect::<Vec<_>>() } else { (0..new_vertex_order[i][0] + 1).collect::<Vec<_>>() };

            //Initialize a Vec<[usize; 2]> containing the path to add
            let mut ext_path: Vec<[usize; 2]> = Vec::new();

            //Extend the GridPath up by 2
            for j in start_range {
                let next_vertex: [usize; 2] = [j, 1];
                ext_path.push(next_vertex);
            }
            for j in mid_range {
                let next_vertex: [usize; 2] = [j, 0];
                ext_path.push(next_vertex);
            }
            for j in end_range {
                let next_vertex: [usize; 2] = [j, 1];
                ext_path.push(next_vertex);
            }

            //Insert the newly constructed path into the new vertex order
            //between the i and i-1 vertices and overwrite the current vertex order
            new_vertex_order.splice(i..i, ext_path);
            self.vertex_order = new_vertex_order;

            //Initialize a new petgraph graph for display of the path and return
            let new_graph = GridPath::get_graph_from_vertex_order(self.n, self.m + 2, &self.vertex_order);
            self.graph = new_graph;

            //Update the vertical dimension of the graph and return
            self.m += 2;
            return;
        }

        //If we reach this point then panic, the graph cannot be extended down
        panic!("No edges on lower boundary of the grid, cannot extend downward");
    }

    /// Extend the GridPath with a width-2 strip in the rightward direction
    fn extend_right(&mut self) {
        //Loop through the vertices in the vertex order until vertices are
        //found forming an edge on the right boundary of the grid.  Once found
        //extend the grid path along that edge.
        for i in 1..self.vertex_order.len() {
            //Check if the ith and i-1th vertices are on the right boundary
            let bound: usize = self.n - 1;
            if self.vertex_order[i][0] != bound || self.vertex_order[i-1][0] != bound {
                continue;
            }

            //Decide which direction to move first and construct the loop ranges accordingly
            let down_first: bool = self.vertex_order[i-1][1] < self.vertex_order[i][1];
            let start_range = if down_first { (0..self.vertex_order[i-1][1] + 1).rev().collect::<Vec<_>>() } else { ((self.vertex_order[i-1][1])..self.m).collect::<Vec<_>>() };
            let mid_range = if down_first { (0..self.m).collect::<Vec<_>>() } else { (0..self.m).rev().collect::<Vec<_>>() };
            let end_range = if down_first { (self.vertex_order[i][1]..self.m).rev().collect::<Vec<_>>() } else { (0..self.vertex_order[i][1] + 1).collect::<Vec<_>>() };

            //Initialize a Vec<[usize; 2]> containing the path to add
            let mut ext_path: Vec<[usize; 2]> = Vec::new();

            //Extend the GridPath to the right by 2
            for j in start_range {
                let next_vertex: [usize; 2] = [self.n, j];
                ext_path.push(next_vertex);
            }
            for j in mid_range {
                let next_vertex: [usize; 2] = [self.n + 1, j];
                ext_path.push(next_vertex);
            }
            for j in end_range {
                let next_vertex: [usize; 2] = [self.n, j];
                ext_path.push(next_vertex);
            }

            //Insert the newly constructed path into the new vertex order
            //between the i and i-1 vertices and overwrite the current vertex order
            self.vertex_order.splice(i..i, ext_path);

            //Initialize a new petgraph graph for display of the path and return
            let new_graph = GridPath::get_graph_from_vertex_order(self.n + 2, self.m, &self.vertex_order);
            self.graph = new_graph;

            //Update the horizontal dimension of the graph and return
            self.n += 2;
            return;
        }

        //If we reach this point then panic, the graph cannot be extended to the right
        panic!("No edges on right boundary of the grid, cannot extend to the right");
    }
    
    /// Extend the GridPath with a width-2 strip in the leftward direction
    fn extend_left(&mut self) {
        //Loop through the vertices in the vertex order until vertices are
        //found forming an edge on the left boundary of the grid.  Once found
        //extend the grid path along that edge.
        for i in 1..self.vertex_order.len() {
            //Check if the ith and i-1th vertices are on the left boundary
            if self.vertex_order[i][0] != 0 || self.vertex_order[i-1][0] != 0 {
                continue;
            }

            //If found then shift the grid path to the right by 2
            let mut new_vertex_order: Vec<[usize; 2]> = self.get_right_shift_vertex_order(2);

            //Decide which direction to move first and construct the loop ranges accordingly
            let down_first: bool = new_vertex_order[i-1][1] < new_vertex_order[i][1];
            let start_range = if down_first { (0..new_vertex_order[i-1][1] + 1).rev().collect::<Vec<_>>() } else { ((new_vertex_order[i-1][1])..self.m).collect::<Vec<_>>() };
            let mid_range = if down_first { (0..self.m).collect::<Vec<_>>() } else { (0..self.m).rev().collect::<Vec<_>>() };
            let end_range = if down_first { (new_vertex_order[i][1]..self.m).rev().collect::<Vec<_>>() } else { (0..new_vertex_order[i][1] + 1).collect::<Vec<_>>() };

            //Initialize a Vec<[usize; 2]> containing the path to add
            let mut ext_path: Vec<[usize; 2]> = Vec::new();

            //Extend the GridPath to the right by 2
            for j in start_range {
                let next_vertex: [usize; 2] = [1, j];
                ext_path.push(next_vertex);
            }
            for j in mid_range {
                let next_vertex: [usize; 2] = [0, j];
                ext_path.push(next_vertex);
            }
            for j in end_range {
                let next_vertex: [usize; 2] = [1, j];
                ext_path.push(next_vertex);
            }

            //Insert the newly constructed path into the new vertex order
            //between the i and i-1 vertices and overwrite the current vertex order
            new_vertex_order.splice(i..i, ext_path);
            self.vertex_order = new_vertex_order;

            //Initialize a new petgraph graph for display of the path and return
            let new_graph = GridPath::get_graph_from_vertex_order(self.n + 2, self.m, &self.vertex_order);
            self.graph = new_graph;

            //Update the horizontal dimension of the graph and return
            self.n += 2;
            return;
        }

        //If we reach this point then panic, the graph cannot be extended to the right
        panic!("No edges on right boundary of the grid, cannot extend to the right");
    }

    /// Given a GridExtension, extend the GridPath in that direction
    pub fn extend(&mut self, direction: GridExtension) {
        match direction {
            GridExtension::Right => self.extend_right(),
            GridExtension::Up    => self.extend_up(),
            GridExtension::Left  => self.extend_left(),
            GridExtension::Down  => self.extend_down()
        }
    }

    /// Given a Vec<GridExtension>, extend the GridPath in those directions
    pub fn extend_many(&mut self, extensions: &Vec<GridExtension>) {
        for direction in extensions.iter() {
            self.extend(*direction);
        }
    }
}

impl fmt::Display for GridPath {
    /// Format a GridPath as a string
    ///
    /// For example, for a 3 by 2 grid graph:
    /// ```rust
    /// let my_vertex_order: Vec<[usize; 2]> = vec![
    ///     [0, 0], [0, 1], [1, 1],
    ///     [2, 1], [2, 0], [1, 0]
    /// ];
    /// let my_grid_path: GridPath = GridPath::new(3, 2, my_vertex_order);
    /// println!("{}", my_grid_graph);
    /// ```
    ///
    /// Yields the following
    /// ```
    /// o---o---o
    /// |       |
    /// o   o---o
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //Initialize a string for the graph display
        let mut graph_display: String = String::from("");

        //Add nodes to the graph
        for i in (0..self.m).rev() {
            //Initialize strings for the row and inter-row display
            let mut row_display: String = String::from("");
            let mut inter_row_display: String = String::from("");

            //Loop through the nodes in this row
            for j in 0..self.n {
                //Initialize strings for the node and inter node display
                let mut node_display: String = String::from("");
                let mut inter_node_display: String = String::from("");

                //Get the node index
                let node_index = NodeIndexable::from_index(&self.graph, (i*self.n) + j);

                //Draw an edge in the left direction if node to the left
                if j > 0 {
                    inter_node_display += "   ";
                    if self.graph.contains_edge(node_index, NodeIndexable::from_index(&self.graph, (i*self.n) + j - 1)) {
                        node_display += "---o";
                    } else {
                        node_display += "   o";
                    }
                } else {
                    node_display += "o"
                }

                //Draw an edge in the up direction if node above
                if i > 0 {
                    if self.graph.contains_edge(node_index, NodeIndexable::from_index(&self.graph, ((i-1)*self.n) + j)) {
                        inter_node_display += "|";
                    } else {
                        inter_node_display += " ";
                    }
                }

                //Add the node displays to the row displays
                row_display += &node_display;
                inter_row_display += &inter_node_display;
            }

            //Add the row and inter-row display to the graph display
            if i > 0 {
                graph_display += &format!("{}\n{}\n", row_display, inter_row_display);
            } else {
                graph_display += &row_display;
            }
        }

        //Write the graph display
        f.write_str(&graph_display)
    }
}

lazy_static!{
    static ref PRIME_SOLUTION_JSON: JsonValue = json::parse(r#"
    [
        {
            "n" : 2,
            "m" : 2,
            "paths" : [
                [ [0, 0], [1, 0], [1, 1], [0, 1] ],
                [ [0, 0], [0, 1], [1, 1], [1, 0] ],
                [ [0, 1], [1, 1], [1, 0], [0, 0] ],
                [ [1, 0], [1, 1], [0, 1], [0, 0] ],
                [ [1, 1], [0, 1], [0, 0], [1, 0] ],
                [ [1, 1], [1, 0], [0, 0], [0, 1] ],
                [ [1, 0], [0, 0], [0, 1], [1, 1] ],
                [ [1, 0], [0, 0], [0, 1], [1, 1] ],
                [ [0, 1], [0, 0], [1, 0], [1, 1] ]
            ]
        },
        {
            "n" : 2,
            "m" : 3,
            "paths" : [
                [ [0, 0], [1, 0], [1, 1], [1, 2], [0, 2], [0, 1] ],
                [ [0, 0], [0, 1], [0, 2], [1, 2], [1, 1], [0, 1] ],
                [ [0, 0], [1, 0], [1, 1], [0, 1], [0, 2], [1, 2] ],
                [ [0, 1], [0, 2], [1, 2], [1, 1], [1, 0], [0, 0] ],
                [ [0, 1], [0, 0], [1, 0], [1, 1], [1, 2], [0, 2] ],
                [ [0, 2], [1, 2], [1, 1], [1, 0], [0, 0], [0, 1] ],
                [ [0, 2], [1, 2], [1, 1], [1, 0], [0, 0], [1, 0] ],
                [ [0, 2], [0, 1], [0, 0], [1, 0], [1, 1], [1, 2] ],
                [ [1, 0], [1, 1], [1, 2], [0, 2], [0, 1], [0, 0] ],
                [ [1, 0], [0, 0], [0, 1], [1, 1], [1, 2], [0, 2] ],
                [ [1, 0], [0, 0], [0, 1], [0, 2], [1, 2], [1, 1] ],
                [ [1, 1], [1, 2], [0, 2], [0, 1], [0, 0], [1, 0] ],
                [ [1, 1], [1, 0], [0, 0], [0, 1], [0, 2], [1, 2] ],
                [ [1, 2], [0, 2], [0, 1], [1, 1], [1, 0], [0, 0] ],
                [ [1, 2], [1, 1], [1, 0], [0, 0], [0, 1], [0, 2] ],
                [ [1, 2], [0, 2], [0, 1], [0, 0], [1, 0], [1, 1] ]
            ]
        },
        {
            "n" : 3,
            "m" : 2,
            "paths" : [
                [ [0, 0], [0, 1], [1, 1], [2, 1], [2, 0], [1, 0] ],
                [ [0, 0], [1, 0], [2, 0], [2, 1], [1, 1], [0, 1] ],
                [ [0, 0], [0, 1], [1, 1], [1, 0], [2, 0], [2, 1] ],
                [ [1, 0], [2, 0], [2, 1], [1, 1], [0, 1], [0, 0] ],
                [ [1, 0], [0, 0], [0, 1], [1, 1], [2, 1], [2, 0] ],
                [ [2, 0], [2, 1], [1, 1], [0, 1], [0, 0], [1, 0] ],
                [ [2, 0], [2, 1], [1, 1], [1, 0], [0, 0], [0, 1] ],
                [ [2, 0], [1, 0], [0, 0], [0, 1], [1, 1], [2, 1] ],
                [ [0, 1], [1, 1], [2, 1], [2, 0], [1, 0], [0, 0] ],
                [ [0, 1], [0, 0], [1, 0], [1, 1], [2, 1], [2, 0] ],
                [ [0, 1], [0, 0], [1, 0], [2, 0], [2, 1], [1, 1] ],
                [ [1, 1], [2, 1], [2, 0], [1, 0], [0, 0], [0, 1] ],
                [ [1, 1], [0, 1], [0, 0], [1, 0], [2, 0], [2, 1] ],
                [ [2, 1], [2, 0], [1, 0], [1, 1], [0, 1], [0, 0] ],
                [ [2, 1], [1, 1], [0, 1], [0, 0], [1, 0], [2, 0] ],
                [ [2, 1], [2, 0], [1, 0], [0, 0], [0, 1], [1, 1] ]
            ]
        },
        {
            "n" : 3,
            "m" : 3,
            "paths" : [
                [ [0, 0], [1, 0], [2, 0], [2, 1], [2, 2], [1, 2], [1, 1], [0, 1], [0, 2] ],
                [ [0, 0], [0, 1], [0, 2], [1, 2], [2, 2], [2, 1], [2, 0], [1, 0], [1, 1] ],
                [ [0, 0], [1, 0], [1, 1], [0, 1], [0, 2], [1, 2], [2, 2], [2, 1], [2, 0] ],
                [ [0, 0], [1, 0], [2, 0], [2, 1], [1, 1], [0, 1], [0, 2], [1, 2], [2, 2] ],
                [ [0, 2], [1, 2], [2, 2], [2, 1], [2, 0], [1, 0], [1, 1], [0, 1], [0, 0] ],
                [ [0, 2], [1, 2], [2, 2], [2, 1], [2, 0], [1, 0], [0, 0], [0, 1], [1, 1] ],
                [ [0, 2], [0, 1], [0, 0], [1, 0], [1, 1], [1, 2], [2, 2], [2, 1], [2, 0] ],
                [ [0, 2], [1, 2], [1, 1], [0, 1], [0, 0], [1, 0], [2, 0], [2, 1], [2, 2] ],
                [ [1, 1], [0, 1], [0, 2], [1, 2], [2, 2], [2, 1], [2, 0], [1, 0], [0, 0] ],
                [ [1, 1], [1, 2], [2, 2], [2, 1], [2, 0], [1, 0], [0, 0], [0, 1], [0, 2] ],
                [ [1, 1], [2, 1], [2, 2], [1, 2], [0, 2], [0, 1], [0, 0], [1, 0], [2, 0] ],
                [ [1, 1], [2, 1], [2, 0], [1, 0], [0, 0], [0, 1], [0, 2], [1, 2], [2, 2] ],
                [ [2, 0], [2, 1], [2, 2], [1, 2], [0, 2], [0, 1], [1, 1], [1, 0], [0, 0] ],
                [ [2, 0], [2, 1], [2, 2], [1, 2], [0, 2], [0, 1], [0, 0], [1, 0], [1, 1] ],
                [ [2, 0], [1, 0], [0, 0], [0, 1], [1, 1], [2, 1], [2, 2], [1, 2], [0, 2] ],
                [ [2, 0], [1, 0], [0, 0], [0, 1], [0, 2], [1, 2], [1, 1], [2, 1], [2, 2] ],
                [ [2, 2], [2, 1], [2, 0], [1, 0], [1, 1], [1, 2], [0, 2], [0, 1], [0, 0] ],
                [ [2, 2], [2, 1], [2, 0], [1, 0], [0, 0], [0, 1], [1, 1], [1, 2], [0, 2] ],
                [ [2, 2], [2, 1], [2, 0], [1, 0], [0, 0], [0, 1], [0, 2], [1, 2], [1, 1] ],
                [ [2, 2], [1, 2], [0, 2], [0, 1], [0, 0], [1, 0], [1, 1], [2, 1], [2, 0] ]
            ]
        },
        {
            "n" : 4,
            "m" : 5,
            "paths" : [
                [ [0, 1], [0, 0], [1, 0], [2, 0], [3, 0], [3, 1], [2, 1], [2, 2], [3, 2], [3, 3], [3, 4], [2, 4], [2, 3], [1, 3], [1, 4], [0, 4], [0, 3], [0, 2], [1, 2], [1, 1] ],
                [ [0, 3], [0, 4], [1, 4], [2, 4], [3, 4], [3, 3], [2, 3], [2, 2], [3, 2], [3, 1], [3, 0], [2, 0], [2, 1], [1, 1], [1, 0], [0, 0], [0, 1], [0, 2], [1, 2], [1, 3] ],
                [ [1, 1], [1, 2], [0, 2], [0, 3], [0, 4], [1, 4], [1, 3], [2, 3], [2, 4], [3, 4], [3, 3], [3, 2], [2, 2], [2, 1], [3, 1], [3, 0], [2, 0], [1, 0], [0, 0], [0, 1] ],
                [ [1, 3], [1, 2], [0, 2], [0, 1], [0, 0], [1, 0], [1, 1], [2, 1], [2, 0], [3, 0], [3, 1], [3, 2], [2, 2], [2, 3], [3, 3], [3, 4], [2, 4], [1, 4], [0, 4], [0, 3] ],
                [ [2, 1], [2, 2], [3, 2], [3, 3], [3, 4], [2, 4], [2, 3], [1, 3], [1, 4], [0, 4], [0, 3], [0, 2], [1, 2], [1, 1], [0, 1], [0, 0], [1, 0], [2, 0], [3, 0], [3, 1] ],
                [ [2, 3], [2, 2], [3, 2], [3, 1], [3, 0], [2, 0], [2, 1], [1, 1], [1, 0], [0, 0], [0, 1], [0, 2], [1, 2], [1, 3], [0, 3], [0, 4], [1, 4], [2, 4], [3, 4], [3, 3] ],
                [ [3, 1], [3, 0], [2, 0], [1, 0], [0, 0], [0, 1], [1, 1], [1, 2], [0, 2], [0, 3], [0, 4], [1, 4], [1, 3], [2, 3], [2, 4], [3, 4], [3, 3], [3, 2], [2, 2], [2, 1] ],
                [ [3, 3], [3, 4], [2, 4], [1, 4], [0, 4], [0, 3], [1, 3], [1, 2], [0, 2], [0, 1], [0, 0], [1, 0], [1, 1], [2, 1], [2, 0], [3, 0], [3, 1], [3, 2], [2, 2], [2, 3] ]
            ]
        },
        {
            "n" : 5,
            "m" : 4,
            "paths" : [
                [ [1, 0], [0, 0], [0, 1], [0, 2], [0, 3], [1, 3], [1, 2], [2, 2], [2, 3], [3, 3], [4, 3], [4, 2], [3, 2], [3, 1], [4, 1], [4, 0], [3, 0], [2, 0], [2, 1], [1, 1] ],
                [ [1, 1], [2, 1], [2, 0], [3, 0], [4, 0], [4, 1], [3, 1], [3, 2], [4, 2], [4, 3], [3, 3], [2, 3], [2, 2], [1, 2], [1, 3], [0, 3], [0, 2], [0, 1], [0, 0], [1, 0] ],
                [ [1, 2], [2, 2], [2, 3], [3, 3], [4, 3], [4, 2], [3, 2], [3, 1], [4, 1], [4, 0], [3, 0], [2, 0], [2, 1], [1, 1], [1, 0], [0, 0], [0, 1], [0, 2], [0, 3], [1, 3] ],
                [ [1, 3], [0, 3], [0, 2], [0, 1], [0, 0], [1, 0], [1, 1], [2, 1], [2, 0], [3, 0], [4, 0], [4, 1], [3, 1], [3, 2], [4, 2], [4, 3], [3, 3], [2, 3], [2, 2], [1, 2] ],
                [ [3, 0], [4, 0], [4, 1], [4, 2], [4, 3], [3, 3], [3, 2], [2, 2], [2, 3], [1, 3], [0, 3], [0, 2], [1, 2], [1, 1], [0, 1], [0, 0], [1, 0], [2, 0], [2, 1], [3, 1] ],
                [ [3, 1], [2, 1], [2, 0], [1, 0], [0, 0], [0, 1], [1, 1], [1, 2], [0, 2], [0, 3], [1, 3], [2, 3], [2, 2], [3, 2], [3, 3], [4, 3], [4, 4], [4, 1], [4, 0], [3, 0] ],
                [ [3, 2], [2, 2], [2, 3], [1, 3], [0, 3], [0, 2], [1, 2], [1, 1], [0, 1], [0, 0], [1, 0], [2, 0], [2, 1], [3, 1], [3, 0], [4, 0], [4, 1], [4, 2], [4, 3], [3, 3] ],
                [ [3, 3], [4, 3], [4, 2], [4, 1], [4, 0], [3, 0], [3, 1], [2, 1], [2, 0], [1, 0], [0, 0], [0, 1], [1, 1], [1, 2], [0, 2], [0, 3], [1, 3], [2, 3], [2, 2], [3, 2] ]
            ]
        }
    ]
    "#).unwrap();
}