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

        //Initialize the GridPath
        GridPath {
            n: n,
            m: m,
            graph: graph
        }
    }

    /// Get the width of a grid graph
    pub fn get_width(&self) -> usize {
        self.n
    }

    /// Get the height of a grid graph
    pub fn get_height(&self) -> usize {
        self.m
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