import pandas as pd
import matplotlib.pyplot as plt
import matplotlib.animation as animation
import matplotlib.image as mpimg
import os

# ENTRADA: O CSV gerado pelo Rust
graph_csv = "../data/graph.csv"
# ENTRADA: O mapa usado pelo Rust
map_path = "../data/map.jpg" 
# SAÍDA: O nome do vídeo que será criado
output_video = "../output/prm_animation.mp4"

try:
    edges = pd.read_csv(graph_csv)
except FileNotFoundError:
    print(f"Erro: Arquivo não encontrado em {graph_csv}")
    print("Verifique se você executou o código Rust primeiro.")
    exit()
    
try:
    map_img = mpimg.imread(map_path)
except FileNotFoundError:
    print(f"Erro: Imagem do mapa não encontrada em {map_path}")
    exit()

# Extrai todos os nós (vértices) únicos da lista de arestas
nodes1 = edges[['x1', 'y1']].rename(columns={'x1': 'x', 'y1': 'y'})
nodes2 = edges[['x2', 'y2']].rename(columns={'x2': 'x', 'y2': 'y'})
all_nodes = pd.concat([nodes1, nodes2]).drop_duplicates().reset_index(drop=True)

print(f"Carregados {len(all_nodes)} nós e {len(edges)} arestas.")

# --- 2. Configurar a Plotagem ---

fig, ax = plt.subplots(figsize=(10, 8))
ax.imshow(map_img)
ax.set_title("Geração do Grafo Aleatório (PRM)")
ax.set_xlabel("Coordenada X (pixels)")
ax.set_ylabel("Coordenada Y (pixels)")
# ax.axis("equal") # Removido caso a imagem tenha proporções diferentes

lines = []   # Lista para guardar as arestas (linhas)
points = []  # Lista para guardar os nós (pontos)

total_nodes = len(all_nodes)
total_edges = len(edges)

# --- 3. Função de Animação (Update) ---

def update(frame):
    i = frame
    
    # Fase 1: Desenha os Nós (um por frame)
    if i < total_nodes:
        if i == 0:
            print("Fase 1/2: Desenhando nós...")
        node = all_nodes.iloc[i]
        # Desenha o nó
        point, = ax.plot(node["x"], node["y"], "b.", markersize=4)
        points.append(point)
        
    # Fase 2: Desenha as Arestas (uma por frame)
    elif i < total_nodes + total_edges:
        if i == total_nodes:
            print("Fase 2/2: Desenhando arestas...")
        edge_idx = i - total_nodes
        edge = edges.iloc[edge_idx]
        
        # Desenha a aresta
        line, = ax.plot([edge["x1"], edge["x2"]],
                          [edge["y1"], edge["y2"]],
                          "g-", linewidth=0.5, alpha=0.6)
        lines.append(line)
        
    # Fase 3: Pausa (mostra o resultado final)
    else:
        if i == total_nodes + total_edges:
            print("Animação completa.")
        pass
        
    # Retorna todos os artistas desenhados
    return lines + points

# --- 4. Criar e Salvar a Animação ---

# Total de frames: nós + arestas + 30 frames de pausa
total_frames = total_nodes + total_edges + 30

ani = animation.FuncAnimation(
    fig,
    update,
    frames=total_frames,
    interval=20,  # Intervalo de 20ms entre frames
    blit=False,   # blit=False é mais simples e funciona bem
    repeat=False
)

# Cria o diretório de saída se ele não existir
os.makedirs(os.path.dirname(output_video), exist_ok=True)

try:
    # Tenta salvar como MP4 (precisa de FFmpeg)
    FFMpegWriter = animation.writers["ffmpeg"]
    writer = FFMpegWriter(fps=30, codec="libx264")
    ani.save(output_video, writer=writer)
    print(f"✅ Vídeo MP4 salvo em: {output_video}")
except Exception as e:
    print(f"⚠️ FFmpeg não disponível (Erro: {e}), salvando como GIF...")
    # Se FFmpeg falhar, salva como GIF
    gif_output = output_video.replace(".mp4", ".gif")
    ani.save(gif_output, writer=animation.PillowWriter(fps=30))
    print(f"✅ Animação GIF salva em: {gif_output}")

plt.close(fig)