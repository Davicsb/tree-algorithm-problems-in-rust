#![allow(warnings)]
pub mod case_first;
pub mod case_second;
pub mod structs;
pub mod readmap;
pub mod prm_generator;
pub mod kruskal;
pub mod dfs;
pub mod read_graph;
pub mod vertice_mais_proximo;
pub mod read_coord;


use std::io;
use case_first::first_case;
use case_second::second_case;

fn main() {
    println!("Qual operação você quer realizar?\n1 - Gerar um grafo e AGM para o mapa (primeira escolha)\n2 - Achar o caminho entre dois pontos utilizando a AGM criada.");

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Failed to read line.");
    let num : i32 = input_line.trim().parse().expect("The input is not an integer.");

    if num == 1 {
        first_case();
    } else {
        second_case();
    }
}
