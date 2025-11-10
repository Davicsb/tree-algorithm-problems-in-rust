//! Recebe uma coordenada x e y e retorna o indice do vértice mais próximo no mapa

use crate::structs::{Graph, Point};
use crate::readmap::OccupancyMap;


/// Recebe uma coordenada x e y e retorna o indice do vértice mais próximo no mapa
pub fn vertice_mais_prox(gr: &Graph, map: &OccupancyMap, x: f64, y: f64) -> Result<usize, String> {
    // Cria um objeto Point para a coordenada
    let point = Point {x, y};

    // Checa se as coordenadas não estão fora dos limites do mapa
    if x < map.pixel_bounds.0 as f64 || y < map.pixel_bounds.1 as f64 || x >= map.pixel_bounds.2 as f64 || y >= map.pixel_bounds.3 as f64 {
        return Err("Coordenada fora dos limites da imagem.".to_string());
    }

    // Checa se o ponto está dentro de um obstáculo
    if map.is_obstructed(&point) {
        return Err("Coordenada está em um obstáculo.".to_string());
    }

    // Define a menor distancia inicial como infinito e o vértice mais próximo como 0 de indice
    let mut menor_dist = f64::MAX;
    let mut i_mais_prox = 0;

    // Itera i (indice do vértice) e seu respectivo v (ponto/Point/coordenada do vertice) na lista de vertices
    for (i, v) in gr.vertices.iter().enumerate() {
        // Pega a distância da coordenada até o ponto do vértice
        let dist = v.dist(&point);

        // Se a distância for menor que a menor_dist atual, atualiza menor_dist e i_mais_prox
        if dist < menor_dist {
            menor_dist = dist;
            i_mais_prox = i;
        }
    }

    Ok(i_mais_prox)
}