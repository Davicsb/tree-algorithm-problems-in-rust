# Planejador de Rota com PRM e Árvore Geradora Mínima (AGM)

Este é um projeto de planejamento de rotas (pathfinding) que utiliza a técnica de **Probabilistic Roadmap (PRM)**.

O **Rust** é responsável por:
1.  Carregar um mapa de obstáculos.
2.  Gerar um grafo aleatório (PRM) sobre o mapa, conectando nós que possuem uma linha de visão livre.
3.  Calcular a **Árvore Geradora Mínima (AGM)** desse grafo para criar um "roadmap" conectado.
4.  Encontrar o caminho mais curto entre dois pontos usando esse roadmap, retornando o vértice mais próximo.

Os scripts em **Python** são usados para ler os dados gerados pelo Rust (`.csv`) e criar animações do processo sobre a imagem do mapa.

## Resultado Final

### Para visualização do vídeo é recomendado a clonagem do repositório para ver o vídeo diratamente na IDE de programação

```bash
    git clone ttps://github.com/Davicsb/tree-algorithm-problems-in-rust.git
```

### 1. Geração do Grafo (PRM) e da AGM

/output/prm_animation.mp4

### 2. Busca do Caminho na AGM

### Exemplo com vértice inicial: 183.94,499.82 e final: 292.41,202.1
/output/caminho_animation.mp4

## Como Executar

### Gerando o grafo e o caminho

Terminal
```bash
    cargo run
```
Abrirá o menu com duas escolhas, para gerar o grafo aperte 1. Se quiser as funções do vértice mais próximo e fazer um caminho a partir do grafo salvo aperte 2

### Plotando o grafo e árvore geradora minima

Terminal
```bash
    cd plot
    python plot.py
```

### Plotando o caminho a partir da árvore geradora mínima

Terminal
```bash
    cd plot
    python plot_caminho.py
```

## Documentation - explicação das funções

A partir da raíz do projeto
Terminal
```bash
    cargo doc --open
```
Se está em WSL e não tem WSL Utilities
```bash
    sudo apt update && sudo apt install wslu

```





