// src/prm_generator.rs
#![allow(warnings)]

use crate::readmap::OccupancyMap;
use crate::structs::{Graph, Point};
use rand::Rng;
use std::error::Error;
use csv::Writer;

// --- Parte 1: Geração do Grafo Aleatório (PRM) ---

/// Gera um ponto aleatório VÁLIDO (não obstruído) no mapa.
fn sample_valid_point(map: &OccupancyMap, mut rng: &mut impl Rng) -> Point {
    let (x_min, y_min, x_max, y_max) = map.pixel_bounds;
    loop {
        let x = rng.gen_range(x_min as f64..=x_max as f64);
        let y = rng.gen_range(y_min as f64..=y_max as f64);
        let p = Point { x, y };
        
        // Se não estiver obstruído, retorna o ponto
        if !map.is_obstructed(&p) {
            return p;
        }
        // Se estiver obstruído, o loop continua e tenta novamente
    }
}

/// Gera um grafo aleatório (PRM) no mapa.
pub fn generate_random_graph(map: &OccupancyMap, num_vertices: usize, connection_radius: f64) -> Graph {
    let mut graph = Graph::new();
    let mut rng = rand::thread_rng();

    // 1. Fase de Amostragem: Adiciona N vértices válidos
    for _ in 0..num_vertices {
        let point = sample_valid_point(map, &mut rng);
        graph.add_vertex(point);
    }
    
    // Define a quantidade de verificações de colisão por segmento
    let collision_check_steps = (connection_radius / 2.0).ceil().max(1.0) as i32;

    // 2. Fase de Conexão: Tenta conectar vértices próximos
    let vertices = graph.vertices.clone(); // Clona para evitar problemas de empréstimo
    for i in 0..vertices.len() {
        for j in (i + 1)..vertices.len() {
            let p1 = vertices[i];
            let p2 = vertices[j];
            
            let dist = p1.dist(&p2);

            // Se estiverem dentro do raio de conexão
            if dist <= connection_radius {
                // E se o caminho entre eles for livre
                if !map.is_path_colliding(&p1, &p2, collision_check_steps) {
                    // Adiciona a aresta não-direcionada (em ambas as direções)
                    graph.add_edge(i, j, dist);
                    //graph.add_edge(j, i, dist); comentado pq add_edge ja faz isso
                }
            }
        }
    }
    graph
}


// --- Parte 2: Salvar o Grafo ---

/// Salva o grafo aleatório completo (PRM) em um CSV para plotagem.
/// O formato é 'x1,y1,x2,y2,weight', representando cada aresta.
pub fn save_graph_to_csv(graph: &Graph, file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(file_path)?;

    // Escreve o cabeçalho
    wtr.write_record(&["x1", "y1", "x2", "y2", "weight"])?;

    let num_vertices = graph.vertices.len();

    // Itera por todos os vértices
    for u_idx in 0..num_vertices {
        let p1 = graph.vertices[u_idx];
        
        // Pega os vizinhos de u_idx
        if let Some(neighbors) = graph.get_neighbors(u_idx) {
            for edge in neighbors {
                let v_idx = edge.to_idx;
                
                // Para evitar duplicatas em um grafo não-direcionado,
                // só salvamos a aresta se o índice do nó inicial for menor.
                if u_idx < v_idx {
                    let p2 = graph.vertices[v_idx];
                    
                    let x1 = format!("{:.2}", p1.x);
                    let y1 = format!("{:.2}", p1.y);
                    let x2 = format!("{:.2}", p2.x);
                    let y2 = format!("{:.2}", p2.y);
                    let weight = format!("{:.4}", edge.weight);
                    
                    wtr.write_record(&[x1, y1, x2, y2, weight])?;
                }
            }
        }
    }

    wtr.flush()?;
    Ok(())
}
