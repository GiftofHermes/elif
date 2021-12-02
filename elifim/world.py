import random
from enum import Enum
import cv2
import numpy as np
from life import Life

class MapFullException(Exception):
    """Can't insert the required resource to the map and stuck in an infinite loop"""
    pass

class Color(Enum):
     # BGR because why not
     EMPTY = (255,255,255) # white
     FOOD = (1, 0, 0) # red
     ALIVE = (0,0,1) #

class World:

    def __init__(self, n, num_life):
        self.map = np.ones((n,n,3))
        self.population = []
        for i in range(num_life):
            self.insert_life(n)

    def insert_life(self, n):
        counter = 0
        y, x = random.randint(2,n-1), random.randint(2,n-2)
        while (self.map[y-1:y+2,x-1:x+2] -1).sum() < -1e-5: # whites are 1,1,1 therefore we check negative
            y, x = random.randint(0,n), random.randint(0,n)
            counter += 1
            if counter > 1_000_000:
                raise MapFullException

        life = Life(y,x) # add size? # all checkes with size?

        self.map[y-1:y+2,x-1:x+2] = Color.ALIVE.value
        self.population.append(life)


    def draw(self):
        img = cv2.imshow('map', self.map * 255)
        cv2.waitKey()


class Population:
    def __init__(self):
        pass



if __name__ == '__main__':
    world = World(500, 100)
    world.draw()
