#![allow(warnings)]
mod structs;
mod readmap;
mod prm_generator;
mod kruskal;
mod dfs;
mod read_graph;
mod vertice_mais_proximo;
mod read_coord;

use readmap::OccupancyMap;
use prm_generator::{generate_random_graph, save_graph_to_csv};
use kruskal::kruskal;
use dfs::{dfs_path, path_export};
use read_graph::read_graph;
use read_coord::read_coord;
use vertice_mais_proximo::vertice_mais_prox;
use std::io;

fn main() {
    println!("Qual operação você quer realizar?\n1 - Gerar um grafo e AGM para o mapa (primeira escolha)\n2 - Achar o caminho entre dois pontos utilizando a AGM criada.");

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Failed to read line.");
    let num : i32 = input_line.trim().parse().expect("The input is not an integer.");

    if num == 1 {
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

    } else {
        let agm_csv_path = "data/AGM.csv";
        let map_file_path = "data/map.jpg";
        let map = OccupancyMap::new(map_file_path);

        // Leitura da AGM
        let agm = match read_graph(agm_csv_path) {
            Ok(g) => g,
            Err(e) => {
                eprintln!("Erro ao ler o grafo: {}", e);
                return;
            }
        };

        loop {
            println!("Digite o ponto de partida (x y):");
            let (x1, y1) = read_coord();

            let i1 = match vertice_mais_prox(&agm, &map, x1, y1) {
                Ok(idx) => {
                    println!("Vértice mais próximo: {}", idx);
                    idx
                },
                Err(e) => {
                    eprintln!("{}", e);
                    println!("Você gostaria de digitar outra coordenada e tentar novamente?\n1 - Sim\n2 - Não");

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

            println!("Digite o ponto de destino (x y):");
            let (x2, y2) = read_coord();

            let i2 = match vertice_mais_prox(&agm, &map, x2, y2) {
                Ok(idx) => {
                    println!("Vértice mais próximo: {}", idx);
                    idx
                },
                Err(e) => {
                    eprintln!("{}", e);
                    println!("Você gostaria de digitar outra coordenada e tentar novamente?\n1 - Sim\n2 - Não");

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


            // Busca e consturção do caminho com o DFS
            if let Some(ref path_vec) = dfs_path(&agm, i1, i2) {
                println!("Caminho encontrado: {:?}", path_vec);

                match path_export(&agm, path_vec, "data/caminho.csv") {
                    Ok(_) => println!("Caminho salvo em data/caminho.csv"),
                    Err(e) => eprintln!("Erro ao salvar o caminho completo em CSV: {}", e),
                }

            } else {
                println!("Nenhum caminho encontrado.");
            }

            break;
            
        }
     
    }
}
