//! #  Lê o mapa, a AGM e duas coordenadas de inicio e fim, calcula o vértice mais proximo de cada coordenada e o caminho entre elas

#![allow(warnings)]

pub use crate::structs;
pub use crate::readmap::OccupancyMap;
pub use crate::read_graph::read_graph;
pub use crate::read_coord::read_coord;
pub use crate::vertice_mais_proximo::vertice_mais_prox;
pub use crate::dfs::{dfs_path, path_export};
use std::io;

/// Lê o mapa, a AGM e duas coordenadas de inicio e fim, calcula o vértice mais proximo de cada coordenada e o caminho entre elas
pub fn second_case() {
    let mut input_line = String::new();

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
                let vertice = &agm.vertices[idx];
                println!("Vértice mais próximo: (x: {}, y: {})", vertice.x, vertice.y);
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
                let vertice = &agm.vertices[idx];
                println!("Vértice mais próximo: (x: {}, y: {})", vertice.x, vertice.y);
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