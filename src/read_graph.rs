//! Lê um arquivo csv e retorna um grafo completo

#![allow(warnings)]
use crate::structs::{Graph, Point, Edge};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use csv::Reader;

/// Lê um arquivo csv e retorna um grafo completo
pub fn read_graph(file_path: &str) -> Result<Graph, Box<dyn Error>> {

    // Abre o arquivo csv
    let mut reader = csv::Reader::from_path(file_path)?;

    // Cria um Graph novo e um hashmap de ponto para indice de vertice
    let mut graph = Graph::new();
    let mut point_to_idx: HashMap<Point, usize> = HashMap::new();

    // Lê cada linha do arquivo
    for result in reader.records() {
        let record = result?;

        // Converte os valores de string para f64
        let x1: f64 = record[0].parse()?;
        let y1: f64 = record[1].parse()?;
        let x2: f64 = record[2].parse()?;
        let y2: f64 = record[3].parse()?;
        let weight: f64 = record[4].parse()?;

        // Cria os pontos
        let p1 = Point { x: x1, y: y1 };
        let p2 = Point { x: x2, y: y2 };

        // Adiciona os vértices no grafo
        let u_idx = *point_to_idx.entry(p1).or_insert_with(|| graph.add_vertex(p1));
        let v_idx = *point_to_idx.entry(p2).or_insert_with(|| graph.add_vertex(p2));

        // Adiciona a aresta (bidirecional)
        graph.add_edge(u_idx, v_idx, weight);
    }

    // Retorna o grafo
    Ok(graph)
}
