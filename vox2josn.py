import midvoxio
import midvoxio.vox
import midvoxio.voxio
import json
import sys
import random

sys.setrecursionlimit(1000000000)

class Node:
    def __init__(self, size, is_leaf, is_none) -> None:
        self.size = size
        self.childs = None
        self.material = None
        self.is_leaf = is_leaf
        self.is_none = is_none

    def add_voxel(self, pos):
        if self.size == 1:
            self.is_leaf = True
            self.is_none = False

            return
        else:
            self.is_leaf = False
            self.is_none = False

        if self.childs == None:
            self.childs = [
                Node(self.size / 2, True, True), # 1, 1, 1
                Node(self.size / 2, True, True), # -1, -1, -1
                Node(self.size / 2, True, True), # 1, -1, -1
                Node(self.size / 2, True, True), # 1, 1, -1
                Node(self.size / 2, True, True), # -1, 1, -1
                Node(self.size / 2, True, True), # 1, -1, 1
                Node(self.size / 2, True, True), # -1, -1, 1
                Node(self.size / 2, True, True), # -1, 1, 1
            ]

        if (
            pos[0] > self.size / 2 and
            pos[1] > self.size / 2 and
            pos[2] > self.size / 2
        ): self.childs[0].add_voxel(pos)
        if (
            pos[0] < self.size / 2 and
            pos[1] < self.size / 2 and
            pos[2] < self.size / 2
        ): self.childs[1].add_voxel(pos)
        if (
            pos[0] > self.size / 2 and
            pos[1] < self.size / 2 and
            pos[2] < self.size / 2
        ): self.childs[2].add_voxel(pos)
        if (
            pos[0] > self.size / 2 and
            pos[1] > self.size / 2 and
            pos[2] < self.size / 2
        ): self.childs[3].add_voxel(pos)
        if (
            pos[0] < self.size / 2 and
            pos[1] > self.size / 2 and
            pos[2] < self.size / 2
        ): self.childs[4].add_voxel(pos)
        if (
            pos[0] > self.size / 2 and
            pos[1] < self.size / 2 and
            pos[2] > self.size / 2
        ): self.childs[5].add_voxel(pos)
        if (
            pos[0] < self.size / 2 and
            pos[1] < self.size / 2 and
            pos[2] > self.size / 2
        ): self.childs[6].add_voxel(pos)
        if (
            pos[0] < self.size / 2 and
            pos[1] > self.size / 2 and
            pos[2] > self.size / 2
        ): self.childs[7].add_voxel(pos)

    def get_data(self, pos):
        return ()

    def compile(self, pos):
        if self.is_leaf:
            data.append({
                "pos": pos,
                "rot": [
                    [1.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0],
                    [0.0, 0.0, 1.0]
                ],
                "size": self.size,
                "material": {
                    "emmitance": [random.random(), random.random(), random.random()],
                    "reflectance": [random.random(), random.random(), random.random()],
                    "roughness": 0.0,
                    "opacity": 0.0
                },
                "childs": [
                    0, 0, 0, 0, 0, 0, 0, 0
                ],
                "is_leaf": 1,
                "is_none": random.randint(0, 1)
            })

            return len(data) - 1

        else:
            data.append({
                "pos": pos,
                "rot": [
                    [1.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0],
                    [0.0, 0.0, 1.0]
                ],
                "size": self.size,
                "material": {
                    "emmitance": [1.0, 1.0, 1.0],
                    "reflectance": [1.0, 1.0, 1.0],
                    "roughness": 0.0,
                    "opacity": 0.0
                },
                "childs": [
                    self.childs[0].compile([ self.childs[0].size + pos[0],  self.childs[0].size + pos[1],  self.childs[0].size + pos[2]]),
                    self.childs[1].compile([-self.childs[1].size + pos[0], -self.childs[1].size + pos[1], -self.childs[1].size + pos[2]]),
                    self.childs[2].compile([ self.childs[2].size + pos[0], -self.childs[2].size + pos[1], -self.childs[2].size + pos[2]]),
                    self.childs[3].compile([ self.childs[3].size + pos[0],  self.childs[3].size + pos[1], -self.childs[3].size + pos[2]]),
                    self.childs[4].compile([-self.childs[4].size + pos[0],  self.childs[4].size + pos[1], -self.childs[4].size + pos[2]]),
                    self.childs[5].compile([ self.childs[5].size + pos[0], -self.childs[5].size + pos[1],  self.childs[5].size + pos[2]]),
                    self.childs[6].compile([-self.childs[6].size + pos[0], -self.childs[6].size + pos[1],  self.childs[6].size + pos[2]]),
                    self.childs[7].compile([-self.childs[7].size + pos[0],  self.childs[7].size + pos[1],  self.childs[7].size + pos[2]]),
                ],
                "is_leaf": 0,
                "is_none": 0
            })

            return len(data) - 1
    
FILE = "test.vox"
vox = midvoxio.voxio.get_vox(FILE)

vox_list = vox.voxels

root_node = Node(max(vox.sizes[0]), True, True)

data = []

for voxel in vox_list[0]:
    root_node.add_voxel(voxel[:3])

root_node.compile([0.0, 0.0, 0.0])

print(len(data))

with open("scene_vox.json", "w") as file:
    json.dump(data, file)