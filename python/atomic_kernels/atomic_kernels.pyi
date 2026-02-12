import numpy as np
from numpy.typing import NDArray

def distance_matrix(
    positions: NDArray[np.float64],
    numbers: NDArray[np.int32],
    cell: NDArray[np.float64],
    pbc: NDArray[np.bool_]
) -> NDArray[np.float64]: ...