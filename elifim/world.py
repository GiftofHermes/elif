import numpy as np

class World:

    def __init__(self, n):
        self.map = np.zeros((n,n))
        pass

if __name__ == '__main__':
    world = World(12)
    print(world.map)
