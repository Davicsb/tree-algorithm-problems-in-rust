import pandas as pd
import matplotlib.pyplot as plt
import matplotlib.animation as animation
import matplotlib.image as mpimg
import os

# ENTRADA: O CSV da arvore geradora minima gerado pelo Rust
agm_csv = "../data/AGM.csv"
# ENTRADA: O mapa usado pelo Rust
map_path = "../data/map.jpg"
# ENTRADA: O caminho dos dois vértices gerado pelo Rust
path_csv = "../data/caminho.csv"
# SAÍDA: O nome do vídeo que será criado
output_video = "../output/caminho_animation.mp4"


try:
    agm_edges = pd.read_csv(agm_csv)
except FileNotFoundError:
    print(f"Erro: AGM não encontrada em {agm_csv}")
    print("Verifique se você executou o código Rust primeiro.")
    exit()
    
try:
    map_img = mpimg.imread(map_path)
except FileNotFoundError:
    print(f"Erro: Imagem do mapa não encontrada em {map_path}")
    exit()

try:
    path = pd.read_csv(path_csv)
except FileNotFoundError:
    print(f"Erro: Caminho não encontrado em {path_csv}")
    print("Verifique se você executou o código Rust primeiro.")
    exit()

# Extrai todos os nós (vértices) únicos da lista de arestas
nodes1 = agm_edges[['x1', 'y1']].rename(columns={'x1': 'x', 'y1': 'y'})
nodes2 = agm_edges[['x2', 'y2']].rename(columns={'x2': 'x', 'y2': 'y'})
all_nodes = pd.concat([nodes1, nodes2]).drop_duplicates().reset_index(drop=True)

# Converte o caminho em arestas entre pontos consecutivos
path_edges = []
for i in range(len(path) - 1):
    x1, y1 = path.iloc[i]["x"], path.iloc[i]["y"]
    x2, y2 = path.iloc[i + 1]["x"], path.iloc[i + 1]["y"]
    path_edges.append({"x1": x1, "y1": y1, "x2": x2, "y2": y2})

path_edges = pd.DataFrame(path_edges)
total_path_edges = len(path_edges)


print(f"Carregados {len(all_nodes)} nós e {len(agm_edges)} arestas.")


# --- 2. Configurar a Plotagem ---

fig, ax = plt.subplots(figsize=(10, 8))
ax.imshow(map_img)
ax.set_title("Geração do Caminho sobre a Árvore Geradora Minima (AGM)")
ax.set_xlabel("Coordenada X (pixels)")
ax.set_ylabel("Coordenada Y (pixels)")
# ax.axis("equal") # Removido caso a imagem tenha proporções diferentes

lines = []   # Lista para guardar as arestas (linhas)
points = []  # Lista para guardar os nós (pontos)

total_nodes = len(all_nodes)
total_agm_edges = len(agm_edges)

# --- 3. Função de Animação (Update) ---

def update(frame):
    i = frame

    # Fase 1: Desenha os Nós (um por frame)
    if i < total_nodes:
        if i == 0:
            print("Fase 1/3: Desenhando nós...")
        node = all_nodes.iloc[i]
        # Desenha o nó
        point, = ax.plot(node["x"], node["y"], "b.", markersize=4)
        points.append(point)
    
    # Fase 2: Desenha as Arestas (uma por frame)
    elif i < total_nodes + total_agm_edges:
        if i == total_nodes:
            print("Fase 2/3: Desenhando arestas...")
        edge_idx = i - total_nodes
        edge = agm_edges.iloc[edge_idx]
        
        # Desenha a aresta
        line, = ax.plot([edge["x1"], edge["x2"]],
                          [edge["y1"], edge["y2"]],
                          "g-", linewidth=0.5, alpha=0.6)
        lines.append(line)
    
    # Fase 3: Desenha o caminho
    elif i < total_nodes + total_agm_edges + total_path_edges:
        if i == total_nodes + total_agm_edges:
            print("Fase 3/3: Desenhando arestas do caminho...")
        dfs_idx = i - (total_nodes + total_agm_edges)
        edge = path_edges.iloc[dfs_idx]

        # Desenha a aresta
        line, = ax.plot([edge["x1"], edge["x2"]],
                          [edge["y1"], edge["y2"]],
                          color="red", linewidth=1.5, alpha=0.9)
        lines.append(line)
    
    # Fase 3: Pausa (mostra o resultado final)
    else:
        if i == total_nodes + total_agm_edges:
            print("Animação completa.")
        pass
        
    # Retorna todos os artistas desenhados
    return lines + points


# --- 4. Criar e Salvar a Animação ---

# Total de frames: nós + arestas da AGM + arestas do caminho + 200 frames de pausa
total_frames = total_nodes + total_agm_edges + total_path_edges + 200

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