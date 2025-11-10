
//! #  Calcula o caminho entre dois vértices.

#![allow(warnings)]
use crate::structs::{Graph};
use std::fs::File;
use std::io::Write;
use csv::Writer;

/// Encontra o caminho entre dois vértices de uma árvore usando busca em profundidade (Depth First Search)
/// O caminho é uma lista de indices do vértice
pub fn dfs_path(gr: &Graph, start: usize, end: usize) -> Option<Vec<usize>> {

    // Marca os vértices ja visitados
    let mut visited = vec![false; gr.vertices.len()];
    // Guarda o caminho atual
    let mut path = Vec::new();

    /// Função interna para entrar em loop
    fn dfs(gr: &Graph, head: usize, end: usize, visited: &mut Vec<bool>, path: &mut Vec<usize>) -> bool {

        // Marca o vértice atual como visitado
        visited[head] = true;
        // Em primeiro momento, coloca ele no caminho
        path.push(head);

        // Se chegou no destino retorna true
        if head == end {
            return true
        }

        // Se não chegou no destino ele itera os vizinhos de head
        for edge in &gr.adj[head] {
            // Se o vizinho não foi visitado ele entra em recursão com ele e se na recursão ele chegar no destino, retorna true
            if !visited[edge.to_idx] {
                if dfs(gr, edge.to_idx, end, visited, path){
                    return true
                }
            }
        }

        // Se o caminho não chegar no destino, ele volta dando pop no path
        path.pop();
        false
    }

    // Entra na recursao para preencher o path
    if dfs(gr, start, end, &mut visited, &mut path) {
        Some(path)
    } else {
        None
    }
}

/// Exporta o caminho encontrado pelo dfs para um .csv
pub fn path_export(gr: &Graph, path: &[usize], filename: &str) -> Result<(), Box<dyn std::error::Error>> {

    // Cria um arquivo com filename de nome
    let mut wtr = Writer::from_path(filename)?;

    // Escreve o cabeçalho x y
    wtr.write_record(&["x", "y"])?;

    // Itera pelos indices em path
    for &idx in path {

        // Pega o respectivo ponto do indice e escreve
        let point = gr.vertices[idx];
        wtr.write_record(&[point.x.to_string(), point.y.to_string()])?;
    }

    Ok(())
}
