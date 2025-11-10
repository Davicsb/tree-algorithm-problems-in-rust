
//! #  Algoritmo que faz a árvore geradora mínima

#![allow(warnings)]

use crate::structs::{Graph};


/// Disjoint Set Union, usada para detectar ciclos no grafo/árvore
#[derive(Debug)]
pub struct DSU {
    /// Aponta quem é o "pai" do elemento i
    parent: Vec<usize>,
}

impl DSU {

    /// Retorna um DSU novo
    pub fn new(size: usize) -> Self {
        /// Cria uma lista com [0, 1, ..., size-1], ou seja, cada vértice é seu proprio pai em conjuntos separados
        DSU {
            parent: (0..size).collect(),
        }
    }

    /// Retorna a raíz do conjunto de `x`
    pub fn find(&mut self, x: usize) -> usize {
        /// Se x não for seu próprio pai, ele entra em recursão até achar a raíz
        /// Atualiza o pai já para a raíz para não ter buscas desnecessárias no futuro
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    /// Une dois conjuntos `a` e `b` se não estiverem conectados
    pub fn union(&mut self, a: usize, b: usize) -> bool {
        /// Pega as raízes de cada conjunto
        let rootA = self.find(a);
        let rootB = self.find(b);

        /// Se as raízes forem iguais, `a` e `b` já estão no mesmo conjunto e portanto a união formaria um ciclo
        /// Se as raízes não forem iguais, une os dois conjuntos e retorna true
        if rootA == rootB {
            false
        } else {
            self.parent[rootB] = rootA; // une os conjuntos colocando a raíz de `a` como a raíz de `b` também
            true
        }
    }
}

/// Retorna uma Árvore Geradora Minima
pub fn kruskal(gr: &Graph) -> Result<Graph, String> {
    /// Pega todas as arestas do grafo original
    let mut edges = gr.get_undirected_edges();
    /// Ordena as arestas por peso crescente
    edges.sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap()); // partial_cmp pq o peso ta com f64

    /// Inicializa a Disjoint Set Union para a detecção de ciclos
    let mut dsu = DSU::new(gr.vertices.len());
    /// Inicializa a Árvore Geradora Minima como um grafo (que vai ser aciclico)
    let mut agm = Graph::new();

    /// A árvore recebe TODOS os vértices do grafo original
    for &v in &gr.vertices {
        agm.add_vertex(v);
    }

    /// Itera na lista de arestas ordenadas
    for edge in edges {
        /// Se os vértices não estiverem conectados (não há possibilidade de ciclo), a aresta é adicionada na AGM
        if dsu.union(edge.from, edge.to) {
            agm.add_edge(edge.from, edge.to, edge.weight);
        }
    }

    // Checagem se o grafo é conexo
    let mut roots = std::collections::HashSet::new();
    // Itera por todos os vértices, achando a raíz de cada um e salvando-as
    for i in 0..gr.vertices.len() {
        roots.insert(dsu.find(i));
    }
    // Se houver mais de um elemento em roots, signifca que há mais de uma raíz e que, portanto, o grafo não é conexo
    if roots.len() > 1 {
        return Err(format!("O grafo não é conexo. Existem {} componentes.", roots.len()));
    }

    Ok(agm)
}
