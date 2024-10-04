use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug)]
struct Node {
    id: usize,
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Element {
    id: usize,
    nodes: [usize; 4],
}

#[derive(Debug)]
struct GlobalData {
    simulation_time: f64,
    simulation_step_time: f64,
    conductivity: f64,
    alfa: f64,
    tot: f64,
    initial_temp: f64,
    density: f64,
    specific_heat: f64,
    n_n: usize, // Number of nodes
    n_e: usize, // Number of elements
}

// Function to parse a file and extract the data
fn parse_file(filename: &str) -> (GlobalData, Vec<Node>, Vec<Element>, Vec<usize>) {
    let mut global_data = GlobalData {
        simulation_time: 0.0,
        simulation_step_time: 0.0,
        conductivity: 0.0,
        alfa: 0.0,
        tot: 0.0,
        initial_temp: 0.0,
        density: 0.0,
        specific_heat: 0.0,
        n_e: 0,
        n_n: 0,
    };

    let mut nodes: Vec<Node> = Vec::new();
    let mut elements: Vec<Element> = Vec::new();
    let mut boundary_conditions: Vec<usize> = Vec::new();
    
    let file = File::open(filename).expect("Unable to open the file");
    let reader = io::BufReader::new(file);
    
    let mut section = String::new();
    
    for line in reader.lines() {
        let line = line.unwrap();
        
        // Skip empty lines
        if line.trim().is_empty() {
            continue;
        }
        
        // Switch between sections in the input file
        if line.starts_with("*Node") {
            section = "Node".to_string();
            continue;
        } else if line.starts_with("*Element") {
            section = "Element".to_string();
            continue;
        } else if line.starts_with("*BC") {
            section = "BC".to_string();
            continue;
        }
        
        // Read global data parameters
        if section.is_empty() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            match parts[0] {
                "SimulationTime" => global_data.simulation_time = f64::from_str(parts[1]).unwrap(),
                "SimulationStepTime" => global_data.simulation_step_time = f64::from_str(parts[1]).unwrap(),
                "Conductivity" => global_data.conductivity = f64::from_str(parts[1]).unwrap(),
                "Alfa" => global_data.alfa = f64::from_str(parts[1]).unwrap(),
                "Tot" => global_data.tot = f64::from_str(parts[1]).unwrap(),
                "InitialTemp" => global_data.initial_temp = f64::from_str(parts[1]).unwrap(),
                "Density" => global_data.density = f64::from_str(parts[1]).unwrap(),
                "SpecificHeat" => global_data.specific_heat = f64::from_str(parts[1]).unwrap(),
                "Nodes" => global_data.n_n = usize::from_str(parts[2]).unwrap(),
                "Elements" => global_data.n_e = usize::from_str(parts[2]).unwrap(),
                _ => {}
            }
        }
        
        // Parse nodes
        if section == "Node" {
            let parts: Vec<&str> = line.split(',').collect();
            let node = Node {
                id: usize::from_str(parts[0].trim()).unwrap(),
                x: f64::from_str(parts[1].trim()).unwrap(),
                y: f64::from_str(parts[2].trim()).unwrap(),
            };
            nodes.push(node);
        }
        
        // Parse elements
        if section == "Element" {
            let parts: Vec<&str> = line.split(',').collect();
            let element = Element {
                id: usize::from_str(parts[0].trim()).unwrap(),
                nodes: [
                    usize::from_str(parts[1].trim()).unwrap(),
                    usize::from_str(parts[2].trim()).unwrap(),
                    usize::from_str(parts[3].trim()).unwrap(),
                    usize::from_str(parts[4].trim()).unwrap(),
                ],
            };
            elements.push(element);
        }
        
        // Parse boundary conditions
        if section == "BC" {
            let parts: Vec<&str> = line.split(',').collect();
            for part in parts {
                let node_id = usize::from_str(part.trim()).unwrap();
                boundary_conditions.push(node_id);
            }
        }
    }
    
    (global_data, nodes, elements, boundary_conditions)
}

fn main() {
    let (global_data, nodes, elements, boundary_conditions) = parse_file("data.txt");
    
    // Display Global Data
    println!("=== Global Data ===");
    println!("Simulation Time:        {}", global_data.simulation_time);
    println!("Simulation Step Time:   {}", global_data.simulation_step_time);
    println!("Conductivity:           {}", global_data.conductivity);
    println!("Alfa:                   {}", global_data.alfa);
    println!("Tot:                    {}", global_data.tot);
    println!("Initial Temperature:    {}", global_data.initial_temp);
    println!("Density:                {}", global_data.density);
    println!("Specific Heat:          {}", global_data.specific_heat);
    println!("Number of Nodes:        {}", global_data.n_n);
    println!("Number of Elements:     {}", global_data.n_e);

    // Display Nodes
    println!("\n=== Nodes ===");
    for node in &nodes {
        println!(
            "Node {}: x = {:>10.9}, y = {:>10.9}",
            node.id, node.x, node.y
        );
    }

    // Display Elements
    println!("\n=== Elements ===");
    for element in &elements {
        println!(
            "Element {}: Nodes = [{}, {}, {}, {}]",
            element.id, element.nodes[0], element.nodes[1], element.nodes[2], element.nodes[3]
        );
    }

    // Display Boundary Conditions
    println!("\n=== Boundary Conditions ===");
    println!("Nodes with boundary conditions: {:?}", boundary_conditions);
}

