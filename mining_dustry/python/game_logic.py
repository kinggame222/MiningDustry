import numpy as np
from noise import snoise2

class WorldGenerator:
    def __init__(self, width, height):
        self.width = width
        self.height = height

    def generate(self):
        # Utiliser du bruit de Perlin pour la génération
        scale = 50.0
        octaves = 6
        persistence = 0.5
        lacunarity = 2.0

        world = np.zeros((self.width, self.height))

        for i in range(self.width):
            for j in range(self.height):
                world[i][j] = snoise2(i/scale,
                                      j/scale,
                                      octaves=octaves,
                                      persistence=persistence,
                                      lacunarity=lacunarity)

        # Normaliser entre 0 et 5 pour différents types de blocs
        world = (world + 1) * 2.5
        return world.astype(int).tolist()