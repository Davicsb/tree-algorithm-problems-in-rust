mod structs;
mod readmap;
mod prm_generator;

use readmap::OccupancyMap;
use prm_generator::{generate_random_graph, save_graph_to_csv};

fn main() {
    let map_file_path = "data/map.jpg";
    let map = OccupancyMap::new(map_file_path);

    let num_vertices = 250;   // Quantos nós aleatórios gerar
    let connection_radius = 60.0; // Distância máx. para tentar conectar (em pixels)
    
    // --- 3. Gerar o Grafo Aleatório ---
    let random_graph = generate_random_graph(&map, num_vertices, connection_radius);
    println!("Grafo aleatório gerado com {} vértices.", random_graph.vertices.len());

    let graph_csv_path = "data/graph.csv";
    match save_graph_to_csv(&random_graph, graph_csv_path) {
        Ok(_) => println!("Grafo aleatório completo salvo em {}", graph_csv_path),
        Err(e) => eprintln!("Erro ao salvar o grafo completo em CSV: {}", e),
    }
}