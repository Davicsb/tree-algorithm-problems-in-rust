#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn dist(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

// --- Estruturas de Grafo Ponderado ---

/// Representa uma aresta direcionada e ponderada no grafo.
#[derive(Clone, Copy, Debug)]
pub struct Edge {
    /// O índice do vértice de destino na lista 'vertices' do grafo.
    pub to_idx: usize,
    /// O peso associado a esta aresta.
    pub weight: f64,
}

/// Representa um grafo espacial ponderado.
#[derive(Debug)]
pub struct Graph {
    /// Armazena os dados de cada vértice (sua coordenada)
    pub vertices: Vec<Point>,
    
    /// Lista de adjacência: armazena as arestas ponderadas.
    pub adj: Vec<Vec<Edge>>,
}

impl Graph {
    /// Cria um novo grafo vazio.
    pub fn new() -> Self {
        Graph { vertices: Vec::new(), adj: Vec::new() }
    }

    /// Adiciona um novo vértice ao grafo com a coordenada 'point' especificada.
    /// Retorna o índice do novo vértice.
    pub fn add_vertex(&mut self, point: Point) -> usize {
        let new_idx = self.vertices.len();
        self.vertices.push(point);
        self.adj.push(Vec::new()); 
        new_idx
    }

    /// Adiciona uma aresta DIRECIONADA de 'from_idx' para 'to_idx'
    /// com um 'weight' (peso) especificado.
    pub fn add_edge(&mut self, from_idx: usize, to_idx: usize, weight: f64) {
        if from_idx < self.vertices.len() && to_idx < self.vertices.len() {
            self.adj[from_idx].push(Edge { to_idx, weight });
        }
    }
    
    /// Retorna a coordenada de um vértice pelo seu índice.
    pub fn get_vertex(&self, idx: usize) -> Option<&Point> {
        self.vertices.get(idx)
    }

    /// Retorna a lista de arestas (vizinhos e pesos) de um vértice.
    pub fn get_neighbors(&self, idx: usize) -> Option<&Vec<Edge>> {
        self.adj.get(idx)
    }
}