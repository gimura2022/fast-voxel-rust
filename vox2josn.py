import midvoxio
import midvoxio.vox
import midvoxio.voxio
import json

class Node:
    def __init__(self, size) -> None:
        self.size = size
        self.childs = None
        self.material = None\
    
FILE = "test.vox"
voxels = midvoxio.voxio.get_vox(FILE).voxels
print(len(voxels[0]))

data = []

for voxel in voxels[0]: 
    data.append({
        "pos": voxel[:3],
        "rot": [
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0]
        ],
        "size": 0.5,
        "material": {
            "emmitance": [0.0, 0.3, 0.0],
            "reflectance": [1.0, 1.0, 1.0],
            "roughness": 0.0,
            "opacity": 0.0
        },
        "childs": [
            0, 0, 0, 0, 0, 0, 0, 0
        ],
        "is_leaf": 0
    })

data.append({
        "pos": [-3.0, 0.0, 7.0],
        "rot": [
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0]
        ],
        "size": 2.0,
        "material": {
            "emmitance": [1.0, 1.0, 1.0],
            "reflectance": [1.0, 1.0, 1.0],
            "roughness": 0.0,
            "opacity": 0.0
        },
        "childs": [
            0, 0, 0, 0, 0, 0, 0, 0
        ],
        "is_leaf": 0
    })

with open("scene_vox.json", "w") as file:
    json.dump(data, file)