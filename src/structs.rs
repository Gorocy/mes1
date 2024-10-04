// Structure to hold node data
#[derive(Debug)]
pub struct Node {
    x: f64,
    y: f64,
}

// Structure to hold element data
#[derive(Debug)]
pub struct Element {
    ids: [usize; 4], // IDs of the nodes making up the element
}

// Structure to hold the entire grid data
#[derive(Debug)]
pub struct Grid {
    nodes: Vec<Node>,      // All nodes in the grid
    elements: Vec<Element>, // All elements in the grid
    temperatures: Vec<f64>, // Temperature at each node
}

// Structure to hold global data such as simulation parameters
#[derive(Debug)]
pub struct GlobalData {
    simulation_time: f64,
    simulation_step_time: f64,
    conductivity: f64,
    alfa: f64,
    tot: f64,
    initial_temp: f64,
    density: f64,
    specific_heat: f64,
    nN: usize, // Number of nodes
    nE: usize, // Number of elements
    H: f64,    // Grid height
    W: f64,    // Grid width
}
