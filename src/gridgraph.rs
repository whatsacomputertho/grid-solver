use std::fmt;
use petgraph::Undirected;
use petgraph::graph::Graph;
use petgraph::visit::NodeIndexable;

/// # GridGraph struct
///
/// A `GridGraph` is an n by m grid of vertices where each
/// (x, y) is adjacent to (x+/-1, y) and (x, y+/-1) if they
/// belong to the graph.
pub struct GridGraph {
    n: usize,
    m: usize,
    graph: Graph<String, String, Undirected>
}

impl GridGraph {
    /// Initialize a GridGraph given its dimensions (n by m)
    ///
    /// ### Example
    ///
    /// ```rust
    /// let my_grid_graph: GridGraph = GridGraph::new(4_usize, 3_usize);
    /// ```
    pub fn new(n: usize, m: usize) -> GridGraph {
        //Initialize the graph
        let mut graph = Graph::new_undirected();

        //Add nodes to the graph
        for i in 0..m {
            for j in 0..n {
                //Add the node
                graph.add_node(format!("({},{})",i,j));

                //Draw an edge in the left direction if node to the left
                if j > 0 {
                    graph.add_edge(
                        NodeIndexable::from_index(&graph, (i*n) + j),
                        NodeIndexable::from_index(&graph, (i*n) + j - 1),
                        String::from("")
                    );
                }

                //Draw an edge in the up direction if node above
                if i > 0 {
                    graph.add_edge(
                        NodeIndexable::from_index(&graph, (i*n) + j),
                        NodeIndexable::from_index(&graph, ((i-1)*n) + j),
                        String::from("")
                    );
                }
            }
        }

        //Initialize the GridGraph
        GridGraph {
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

    /// Determine whether two vertices are color compatible
    pub fn are_color_compatible(&self, v_coords: [usize; 2], w_coords: [usize; 2]) -> bool {
        //Sanity check on the input parameters
        if v_coords[0] >= self.n || v_coords[1] >= self.m ||
           w_coords[0] >= self.n || w_coords[1] >= self.m {
            panic!(
                "Coordinates out of bounds: ({},{}), ({},{})",
                v_coords[0], v_coords[1],
                w_coords[0], w_coords[1]
            );
        }

        //Determine if the graph is even or odd
        let graph_is_odd: bool = ((self.n*self.m) & 1) == 1;

        //If the graph is odd then the majority color has even parity
        if graph_is_odd {
            //We therefore check if v and w both have even parity
            return ((w_coords[0]+w_coords[1]) & 1 == 0) && ((v_coords[0]+v_coords[1]) & 1 == 0);
        }

        //If the graph is even then the vertices must share parity
        return (w_coords[0]+w_coords[1]) & 1 != (v_coords[0]+v_coords[1]) & 1;
    }

    /// Determine whether the vertex at the given coordinates
    /// is a corner vertex
    pub fn is_corner_vertex(&self, v_coords: [usize; 2]) -> bool {
        //Sanity check on the input parameters
        if v_coords[0] >= self.n || v_coords[1] >= self.m {
            panic!(
                "Coordinate out of bounds: ({},{})",
                v_coords[0], v_coords[1]
            );
        }

        //Initialize the corner vertex coords
        let c1: [usize; 2] = [0, 0];
        let c2: [usize; 2] = [self.n - 1, 0];
        let c3: [usize; 2] = [0, self.m - 1];
        let c4: [usize; 2] = [self.n - 1, self.m - 1];

        //Check if the vertex coords matches one of the corners
        return if v_coords == c1 || v_coords == c2 || v_coords == c3 || v_coords == c4 {
            true
        } else {
            false
        }
    }

    /// Determine whether the Hamiltonian path problem over this
    /// grid grpah is forbidden when either m or n is 1
    fn is_forbidden_case_1(&self, v_coords: [usize; 2], w_coords: [usize; 2]) -> bool {
        //Return true if neither v or w are the origin vertex
        if v_coords != [0, 0] && w_coords != [0, 0] {
            return true;
        }

        //Determine which dimension is 1 and capture the opposite
        let is_n: bool = self.n == 1;
        let bound: usize = match is_n {
            true => self.m,
            false => self.n
        };

        //Return true if neither v or w are the opposite corner vertex
        if is_n && (v_coords != [0, bound - 1] && w_coords != [0, bound - 1]) {
            return true;
        } else if !is_n && (v_coords != [bound - 1, 0] && w_coords != [bound - 1, 0]) {
            return true;
        }

        //Return true if both v and w are corner vertices
        return false;
    }

    /// Determine whether the Hamiltonian path problem over this
    /// grid grpah is forbidden when either m or n is 2
    fn is_forbidden_case_2(&self, v_coords: [usize; 2], w_coords: [usize; 2]) -> bool {
        //Break if v or w is a corner vertex, as the edge between them
        //cannot be a nonboundary edge in this case
        if self.is_corner_vertex(v_coords) || self.is_corner_vertex(w_coords) {
            return false;
        }

        //Determine which dimension is 2
        let is_n: bool = self.n == 2;

        //If n is 2 then check if the vertices share a y coord
        if is_n && (v_coords[1] == w_coords[1]) {
            return true;
        }

        //If m is 2 then check if the vertices share an x coord
        if !is_n && (v_coords[0] == w_coords[0]) {
            return true;
        }

        //Return false if v and w lack a nonboundary edge between them
        return false;
    }

    /// Determine whether the Hamiltonian path problem over this
    /// grid grpah is forbidden when either m or n is 3
    fn is_forbidden_case_3(&self, v_coords: [usize; 2], w_coords: [usize; 2]) -> bool {
        //Determine which dimension is 3 and capture the opposite
        let is_n: bool = self.n == 3;
        let opp_dim: usize = match is_n {
            true => self.m,
            false => self.n
        };

        //Check if the opposite dimension is odd, if so then break
        if opp_dim & 1 == 1 {
            return false;
        }

        //Check if v has the same color as w, if they share the same
        //color then break
        if (w_coords[0]+w_coords[1]) & 1 == (v_coords[0]+v_coords[1]) & 1 {
            return false;
        }

        //Check if v's position in relation to that of w satisfies the
        //required conditions
        let comp_coords: [usize; 2] = if is_n { [v_coords[1], w_coords[1]] } else { [v_coords[0], w_coords[0]] };
        let opp_coord: usize = if is_n { v_coords[0] } else { v_coords[1] };
        let is_greater: bool = comp_coords[0] > comp_coords[1];
        let distance: usize = if is_greater { comp_coords[0] - comp_coords[1] } else { comp_coords[1] - comp_coords[0] };
        let is_dst_sat: bool = if opp_coord == 1 { distance > 0 } else { distance > 1 };
        
        //Break if the distance condition is not satisfied
        if !is_dst_sat {
            return false;
        }

        //If the distance condition is satisfied then check if the
        //vertex matches the parity of the far corner vertices if
        //v is greater than w, or the near corner vertices otherwise
        if is_greater && ((v_coords[0]+v_coords[1]) & 1 == (1 & 1)) {
            return false; //v shares color with far corner vertices
        } else if !is_greater && ((v_coords[0]+v_coords[1]) & 1 == (0 & 1)) {
            return false; //v shares color with near corner vertices
        }

        //If we satisfy all of the conditions then the problem is forbidden
        true
    }

    /// Determine whether the Hamiltonian path problem over this
    /// grid graph is forbidden
    pub fn is_forbidden(&self, v_coords: [usize; 2], w_coords: [usize; 2]) -> bool {
        //Sanity check on the input parameters
        if v_coords[0] >= self.n || v_coords[1] >= self.m ||
           w_coords[0] >= self.n || w_coords[1] >= self.m {
            panic!(
                "Coordinates out of bounds: ({},{}), ({},{})",
                v_coords[0], v_coords[1],
                w_coords[0], w_coords[1]
            )
        }

        //Check if either m or n is 1, if so then check the forbidden
        //conditions for this case
        if self.n == 1 || self.m == 1 {
            return self.is_forbidden_case_1(v_coords, w_coords);
        }

        //Check if either m or n is 2, if so then check the forbidden
        //conditions for this case
        if self.n == 2 || self.m == 2 {
            return self.is_forbidden_case_2(v_coords, w_coords);
        }

        //Check if either m or n is 3, if so then check the forbidden
        //conditions for this case
        if self.n == 3 || self.m == 3 {
            return self.is_forbidden_case_3(v_coords, w_coords);
        }

        //If none of the forbidden cases are satisfied then return false
        false
    }
}

impl fmt::Display for GridGraph {
    /// Format a GridGraph as a string
    ///
    /// For example, for a 3 by 2 grid graph:
    /// ```rust
    /// let my_grid_graph: GridGraph = GridGraph::new(3, 2);
    /// println!("{}", my_grid_graph);
    /// ```
    ///
    /// Yields the following
    /// ```
    /// o---o---o
    /// |   |   |
    /// o---o---o
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //Initialize a string for the graph display
        let mut graph_display: String = String::from("");

        //Add nodes to the graph
        for i in 0..self.m {
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
                graph_display += &format!("\n{}\n{}", inter_row_display, row_display);
            } else {
                graph_display += &row_display;
            }
        }

        //Write the graph display
        f.write_str(&graph_display)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn color_comp_odd_min() {
        //Initialize an odd grid graph and check if two vertices
        //which belong to the minority color and thus should not
        //be color-compatible are color-compatible
        let my_grid_graph: GridGraph = GridGraph::new(5, 7);

        //Two odd parity coordinates belonging to the minority color
        let v_coords: [usize; 2] = [3, 4];
        let w_coords: [usize; 2] = [1, 6];

        //Assert that the color compatibility of these vertices
        //comes back as false
        assert_eq!(
            my_grid_graph.are_color_compatible(v_coords, w_coords),
            false
        );
    }
    
    #[test]
    fn color_comp_odd_dif() {
        //Initialize an odd grid graph and check if two vertices
        //which belong to the different colors and thus should not
        //be color-compatible are color-compatible
        let my_grid_graph: GridGraph = GridGraph::new(5, 7);

        //Two different parity coordinates
        let v_coords: [usize; 2] = [2, 3];
        let w_coords: [usize; 2] = [1, 5];

        //Assert that the color compatibility of these vertices
        //comes back as false
        assert_eq!(
            my_grid_graph.are_color_compatible(v_coords, w_coords),
            false
        );
    }
    
    #[test]
    fn color_comp_odd_maj() {
        //Initialize an odd grid graph and check if two vertices
        //which belong to the majority colors and thus should be
        //color-compatible are color-compatible
        let my_grid_graph: GridGraph = GridGraph::new(5, 7);

        //Two even parity coordinates belonging to the majority color
        let v_coords: [usize; 2] = [2, 2];
        let w_coords: [usize; 2] = [4, 6];

        //Assert that the color compatibility of these vertices
        //comes back as true
        assert_eq!(
            my_grid_graph.are_color_compatible(v_coords, w_coords),
            true
        );
    }
    
    #[test]
    fn color_comp_even_even() {
        //Initialize an even grid graph and check if two vertices
        //which have even parity are color compatible
        let my_grid_graph: GridGraph = GridGraph::new(5, 8);

        //Two even parity coordinates
        let v_coords: [usize; 2] = [2, 6];
        let w_coords: [usize; 2] = [1, 7];

        //Assert that the color compatibility of these vertices
        //comes back as false
        assert_eq!(
            my_grid_graph.are_color_compatible(v_coords, w_coords),
            false
        );
    }
    
    #[test]
    fn color_comp_even_dif() {
        //Initialize an even grid graph and check if two vertices
        //which have different parity are color compatible
        let my_grid_graph: GridGraph = GridGraph::new(5, 8);

        //Two different parity coordinates
        let v_coords: [usize; 2] = [2, 3];
        let w_coords: [usize; 2] = [1, 5];

        //Assert that the color compatibility of these vertices
        //comes back as true
        assert_eq!(
            my_grid_graph.are_color_compatible(v_coords, w_coords),
            true
        );
    }
    
    #[test]
    fn color_comp_even_odd() {
        //Initialize an even grid graph and check if two vertices
        //which have odd parity are color compatible
        let my_grid_graph: GridGraph = GridGraph::new(6, 8);

        //Two odd parity coordinates
        let v_coords: [usize; 2] = [3, 2];
        let w_coords: [usize; 2] = [5, 6];

        //Assert that the color compatibility of these vertices
        //comes back as false
        assert_eq!(
            my_grid_graph.are_color_compatible(v_coords, w_coords),
            false
        );
    }

    #[test]
    fn forbidden_case_1_width_part_forb() {
        //Initialize a width 1 grid graph
        let my_grid_graph: GridGraph = GridGraph::new(1, 7);

        //Initialize invalid start and end vertices, one is a
        //corner but the other is not
        let v_coords: [usize; 2] = [0, 0];
        let w_coords: [usize; 2] = [0, 4];

        //The problem should be forbidden
        assert_eq!(
            my_grid_graph.is_forbidden(v_coords, w_coords),
            true
        )
    }

    #[test]
    fn forbidden_case_1_width_full_forb() {
        //Initialize a width 1 grid graph
        let my_grid_graph: GridGraph = GridGraph::new(1, 9);

        //Initialize invalid start and end vertices, neither are
        //corner vertices
        let v_coords: [usize; 2] = [0, 5];
        let w_coords: [usize; 2] = [0, 2];

        //The problem should be forbidden
        assert_eq!(
            my_grid_graph.is_forbidden(v_coords, w_coords),
            true
        )
    }

    #[test]
    fn forbidden_case_1_width_valid() {
        //Initialize a width 1 grid graph
        let my_grid_graph: GridGraph = GridGraph::new(1, 10);

        //Initialize invalid start and end vertices, both are
        //corner vertices
        let v_coords: [usize; 2] = [0, 0];
        let w_coords: [usize; 2] = [0, 9];

        //The problem should be valid
        assert_eq!(
            my_grid_graph.is_forbidden(v_coords, w_coords),
            false
        )
    }

    #[test]
    fn forbidden_case_1_height_part_forb() {
        //Initialize a width 1 grid graph
        let my_grid_graph: GridGraph = GridGraph::new(7, 1);

        //Initialize invalid start and end vertices, one is a
        //corner but the other is not
        let v_coords: [usize; 2] = [4, 0];
        let w_coords: [usize; 2] = [0, 0];

        //The problem should be forbidden
        assert_eq!(
            my_grid_graph.is_forbidden(v_coords, w_coords),
            true
        )
    }

    #[test]
    fn forbidden_case_1_height_full_forb() {
        //Initialize a width 1 grid graph
        let my_grid_graph: GridGraph = GridGraph::new(9, 1);

        //Initialize invalid start and end vertices, neither are
        //corner vertices
        let v_coords: [usize; 2] = [5, 0];
        let w_coords: [usize; 2] = [2, 0];

        //The problem should be forbidden
        assert_eq!(
            my_grid_graph.is_forbidden(v_coords, w_coords),
            true
        )
    }

    #[test]
    fn forbidden_case_1_height_valid() {
        //Initialize a width 1 grid graph
        let my_grid_graph: GridGraph = GridGraph::new(10, 1);

        //Initialize invalid start and end vertices, both are
        //corner vertices
        let v_coords: [usize; 2] = [0, 0];
        let w_coords: [usize; 2] = [9, 0];

        //The problem should be valid
        assert_eq!(
            my_grid_graph.is_forbidden(v_coords, w_coords),
            false
        )
    }

    #[test]
    fn forbidden_case_2_width_valid() {
        //Initialize a width 2 grid graph
        let my_grid_graph: GridGraph = GridGraph::new(2, 8);

        //Initialize invalid start and end vertices between which
        //there is no nonboundary edge
        let v_coords: [usize; 2] = [0, 7];
        let w_coords: [usize; 2] = [1, 2];

        //The problem should be valid
        assert_eq!(
            my_grid_graph.is_forbidden(v_coords, w_coords),
            false
        )
    }

    #[test]
    fn forbidden_case_2_width_forb() {
        //Initialize a width 2 grid graph
        let my_grid_graph: GridGraph = GridGraph::new(2, 12);

        //Initialize invalid start and end vertices between which
        //there is a nonboundary edge
        let v_coords: [usize; 2] = [0, 5];
        let w_coords: [usize; 2] = [1, 5];

        //The problem should be forbidden
        assert_eq!(
            my_grid_graph.is_forbidden(v_coords, w_coords),
            true
        )
    }

    #[test]
    fn forbidden_case_2_height_valid() {
        //Initialize a height 2 grid graph
        let my_grid_graph: GridGraph = GridGraph::new(11, 2);

        //Initialize invalid start and end vertices between which
        //there is no nonboundary edge
        let v_coords: [usize; 2] = [8, 1];
        let w_coords: [usize; 2] = [6, 1];

        //The problem should be valid
        assert_eq!(
            my_grid_graph.is_forbidden(v_coords, w_coords),
            false
        )
    }

    #[test]
    fn forbidden_case_2_height_forb() {
        //Initialize a height 2 grid graph
        let my_grid_graph: GridGraph = GridGraph::new(7, 2);

        //Initialize invalid start and end vertices between which
        //there is a nonboundary edge
        let v_coords: [usize; 2] = [3, 1];
        let w_coords: [usize; 2] = [3, 0];

        //The problem should be forbidden
        assert_eq!(
            my_grid_graph.is_forbidden(v_coords, w_coords),
            true
        )
    }

    #[test]
    fn forbidden_case_3_width_valid() {
        //Initialize a width 3 grid graph
        let my_grid_graph: GridGraph = GridGraph::new(3, 12);

        //Initialize valid start and end vertices
        let v_coords: [usize; 2] = [0, 2];
        let w_coords: [usize; 2] = [1, 6];

        //The problem should be valid
        assert_eq!(
            my_grid_graph.is_forbidden(v_coords, w_coords),
            false
        )
    }

    #[test]
    fn forbidden_case_3_width_forb() {
        //Initialize a width 3 grid graph
        let my_grid_graph: GridGraph = GridGraph::new(3, 12);

        //Initialize invalid start and end vertices
        let v_coords: [usize; 2] = [0, 3];
        let w_coords: [usize; 2] = [2, 6];

        //The problem should be forbidden
        assert_eq!(
            my_grid_graph.is_forbidden(v_coords, w_coords),
            true
        )
    }

    #[test]
    fn forbidden_case_3_height_valid() {
        //Initialize a height 3 grid graph
        let my_grid_graph: GridGraph = GridGraph::new(8, 3);

        //Initialize valid start and end vertices
        let v_coords: [usize; 2] = [4, 2];
        let w_coords: [usize; 2] = [6, 1];

        //The problem should be valid
        assert_eq!(
            my_grid_graph.is_forbidden(v_coords, w_coords),
            false
        )
    }

    #[test]
    fn forbidden_case_3_height_forb() {
        //Initialize a height 3 grid graph
        let my_grid_graph: GridGraph = GridGraph::new(8, 3);

        //Initialize invalid start and end vertices
        let v_coords: [usize; 2] = [5, 1];
        let w_coords: [usize; 2] = [4, 1];

        //The problem should be forbidden
        assert_eq!(
            my_grid_graph.is_forbidden(v_coords, w_coords),
            true
        )
    }
}