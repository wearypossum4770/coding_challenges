from typing import List, Dict
from functools import reduce
from itertools import permutations
def cuboid(x:int,y:int,z:int,n:int)->List[List[int]]:
    """
    """
    inner = []
    for _x in range(0,x+1):
        for _y in range(0,y+1):
            for _z in range(0,z+1):
                inner.append([_x,_y,_z])
    return [value for value in inner if reduce(lambda x,y: x+y,value) !=n]
print(cuboid(1,1,1,2))
