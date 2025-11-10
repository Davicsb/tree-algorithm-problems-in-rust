//! #  Gera um grafo aleatório para o mapa e sua árvore geradora minima e salva ambos como .csv em /data
//! ## Output esperado
//! Arquivo CSV contendo o grafo e sua arvore geradora mínima em /data. O CSV está estruturado no ponto x y do vértice original, x y do vérfice destino e o peso da aresta.\
//! ## Algortimo utilizado
//! PRM para gerar o grafo aleatório;\
//! Kruskal para gerar a orvore geradora mínima.\
//! ### Motivação
//! Apesar da atividade pedir um grafo de visibilidade, optamos por um PRM para conseguirmos lidar com obstáculos não poligonais e Kruskal para o usuário ter retorno se o grafo é conexo, e se não for gerar outro grafo.

#![allow(warnings)]


pub use crate::structs;
pub use crate::readmap::OccupancyMap;
pub use crate::prm_generator::{generate_random_graph, save_graph_to_csv};
pub use crate::kruskal::kruskal;
use std::io;

/// Gera um grafo aleatório para o mapa e sua árvore geradora minima e salva ambos como .csv em /data
pub fn first_case() {
    let mut input_line = String::new();
    /// Entra em loop caso o grafo não seja conexo
    loop {
        let map_file_path = "data/map.jpg";
        let map = OccupancyMap::new(map_file_path);

        let num_vertices = 300;   // Quantos nós aleatórios gerar, VALOR ORIGINAL = 250
        let connection_radius = 80.0; // Distância máx. para tentar conectar (em pixels), VALOR ORIGINAL = 60
    
        // --- 3. Gerar o Grafo Aleatório ---
        let random_graph = generate_random_graph(&map, num_vertices, connection_radius);
        println!("Grafo aleatório gerado com {} vértices.", random_graph.vertices.len());

        let graph_csv_path = "data/graph.csv";
        match save_graph_to_csv(&random_graph, graph_csv_path) {
            Ok(_) => println!("Grafo aleatório completo salvo em {}", graph_csv_path),
            Err(e) => eprintln!("Erro ao salvar o grafo completo em CSV: {}", e),
        }

        let agm = match kruskal(&random_graph) {
            Ok(g) => g,
            Err(e) => {
                eprintln!("{}", e);
                println!("Você gostaria de gerar outro grafo e tentar novamente?\n1 - Sim\n2 - Não");

                input_line.clear();
                io::stdin().read_line(&mut input_line).expect("Failed to read line.");
                let num : i32 = input_line.trim().parse().expect("The input is not an integer.");

                if num == 1 {
                    continue;
                } else {
                    return;
                }
            }
        };

        let agm_csv_path = "data/AGM.csv";
        match save_graph_to_csv(&agm, agm_csv_path) {
            Ok(_) => println!("Árvore Geradora Minima salva em {}", agm_csv_path),
            Err(e) => eprintln!("Erro ao salvar a Árvore Geradora Minima em CSV: {}", e),
        }

        break;
    }
}