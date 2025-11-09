use std::io;

/// Lê dois números e retorna-os como f64
pub fn read_coord() -> (f64, f64) {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");

    let valores: Vec<f64> = line
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    
    if valores.len() != 2 {
        panic!("Digite dois números separados por espaço.");
    }

    (valores[0], valores[1])
}