use::petgraph::{Direction, graph::{NodeIndex, UnGraph}};
use std::fmt;

#[derive(Debug)]

struct Fighter {
    name: String,
}

impl Fighter {
    fn new(name: &str) -> Self{
        Self {
            name: name.to_string(),
        }
    }
}

impl fmt::Display for Fighter{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn add_edge(graph: &mut UnGraph<&Fighter, f32>, nodes: &[NodeIndex], a: usize, b:usize) {
    graph.add_edge(nodes[a], nodes[b], 1.0);
}

fn main() {
    let mut graph = UnGraph::new_undirected();

    let fighters = [
        Fighter::new("Dustin Poirier"),
        Fighter::new("Justin Gaethje"),
        Fighter::new("Conor McGregor"),
        Fighter::new("Khabib Nurmagomedov"),
        Fighter::new("Max Holloway"),
    ];

    //Need to use Vec<NodeIndex> to store the nodes and fix iter() fixedSize implementation issue.
    let fighter_nodes: Vec<NodeIndex> = fighters
        .iter()
        .map(|fighter| graph.add_node(fighter))
        .collect();
    //Emulating Social Media or Network Analysius like a graph
    add_edge(&mut graph, &fighter_nodes, 0, 1);
    add_edge(&mut graph, &fighter_nodes, 1, 3);
    add_edge(&mut graph, &fighter_nodes, 2, 3);
    add_edge(&mut graph, &fighter_nodes, 1, 4);
    add_edge(&mut graph, &fighter_nodes, 2, 4);
    add_edge(&mut graph, &fighter_nodes, 1, 2);
    add_edge(&mut graph, &fighter_nodes, 3, 4);

    for (i, &node) in fighter_nodes.iter().enumerate() {
        let name = &fighters[i].name;
        let degree = graph.edges_directed(node, Direction::Outgoing).count() as f32;
        let closeness = 1.0/degree;
        println!("The closeness centrality of {} is {:.4}", name, closeness);

        match name.as_str() {
            "Dustin Poirier" => println!("{} is a great fighter with centrality {}", name, closeness),
            "Justin Gaethje" => println!("{} is a great fighter with centrality {}", name, closeness),
            "Conor McGregor" => println!("{} is a great fighter with centrality {}", name, closeness),
            "Khabib Nurmagomedov" => println!("{} is a great fighter with centrality {}", name, closeness),
            "Max Holloway" => println!("{} is a great fighter with centrality {}", name, closeness),
            _ => println!("{} is not a fighter", name),
        }
    }
}
