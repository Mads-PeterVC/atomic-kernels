from atomic_kernels import distance_matrix
import numpy as np
from scipy.spatial.distance import cdist

P = np.ascontiguousarray(np.array([[0, 0, 0], [1, 0, 0], [0, 1, 0]], dtype=float))
numbers = np.ascontiguousarray(np.array([1, 2, 3]), dtype=np.int32)
cell = np.ascontiguousarray(np.eye(3) * 10)
pbc = np.array([True, True, True])


D = distance_matrix(P, numbers, cell, pbc)
D_ref = cdist(P, P)

print(D)
print(D_ref)
