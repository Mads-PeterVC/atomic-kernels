import numpy as np
from ase import Atoms

from ak_neighborlist import NeighborList, neighbor_list


def test_neighbor_list_non_periodic():
    atoms = Atoms(
        numbers=[1, 8, 1],
        positions=[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
        cell=np.eye(3) * 4.0,
        pbc=[False, False, False],
    )

    neighbors = neighbor_list(atoms, cutoff=1.1, symmetrize=False)

    assert isinstance(neighbors, NeighborList)
    np.testing.assert_array_equal(neighbors.i, [0, 0])
    np.testing.assert_array_equal(neighbors.j, [1, 2])
    np.testing.assert_array_equal(neighbors.S, [[0, 0, 0], [0, 0, 0]])


def test_neighbor_list_periodic_shifts():
    atoms = Atoms(
        numbers=[1, 1],
        positions=[[0.0, 0.0, 0.0], [2.7, 0.0, 0.0]],
        cell=np.eye(3) * 3.0,
        pbc=[True, False, False],
    )

    neighbors = neighbor_list(atoms, cutoff=0.5, symmetrize=False, method="naive")

    np.testing.assert_array_equal(neighbors.i, [0, 1])
    np.testing.assert_array_equal(neighbors.j, [1, 0])
    np.testing.assert_array_equal(neighbors.S, [[-1, 0, 0], [1, 0, 0]])


def test_neighbor_list_symmetrizes_non_periodic_edges():
    atoms = Atoms(
        numbers=[1, 1],
        positions=[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
        cell=np.eye(3) * 4.0,
        pbc=[False, False, False],
    )

    neighbors = neighbor_list(atoms, cutoff=1.1, symmetrize=True)

    np.testing.assert_array_equal(neighbors.i, [0, 1])
    np.testing.assert_array_equal(neighbors.j, [1, 0])
    np.testing.assert_array_equal(neighbors.S, [[0, 0, 0], [0, 0, 0]])


def test_neighbor_list_sort_orders_by_sender_then_receiver():
    atoms = Atoms(
        numbers=[1, 1, 1],
        positions=[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [-1.0, 0.0, 0.0]],
        cell=np.eye(3) * 4.0,
        pbc=[False, False, False],
    )

    neighbors = neighbor_list(atoms, cutoff=1.1, symmetrize=True, sort=True)

    pairs = list(zip(neighbors.i.tolist(), neighbors.j.tolist(), strict=True))
    assert pairs == [(0, 1), (0, 2), (1, 0), (2, 0)]
