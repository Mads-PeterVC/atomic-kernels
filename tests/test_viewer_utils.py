from __future__ import annotations

import numpy as np
import pytest
from ase import Atoms

from atomic_kernels.viewer._utils import (
    bonds_from_adjacency,
    normalize_bonds,
    normalize_colormap,
    normalize_face_colors,
    normalize_faces,
    normalize_rgba,
    selection_mask,
)


def test_normalize_colormap_adds_alpha_channel():
    colors = normalize_colormap([(0.1, 0.2, 0.3), (0.4, 0.5, 0.6)])

    assert colors == [
        (0.10000000149011612, 0.20000000298023224, 0.30000001192092896, 1.0),
        (0.4000000059604645, 0.5, 0.6000000238418579, 1.0),
    ]


def test_normalize_colormap_rejects_single_sample():
    with pytest.raises(ValueError, match="at least two samples"):
        normalize_colormap([(0.1, 0.2, 0.3)])


def test_normalize_rgba_accepts_rgb_and_adds_alpha():
    assert normalize_rgba((0.1, 0.2, 0.3)) == pytest.approx((0.1, 0.2, 0.3, 1.0))


def test_normalize_bonds_canonicalizes_and_deduplicates_pairs():
    bonds = normalize_bonds([(2, 1), (1, 2), (0, 0), (3, 4)])

    assert bonds == [(1, 2), (3, 4)]


def test_bonds_from_adjacency_uses_upper_triangle():
    adjacency = np.array(
        [
            [False, True, False],
            [False, False, True],
            [False, False, False],
        ]
    )

    assert bonds_from_adjacency(adjacency) == [(0, 1), (1, 2)]


def test_normalize_faces_deduplicates_rotations_and_reversals():
    faces = normalize_faces([(0, 1, 2, 3), (2, 3, 0, 1), (3, 2, 1, 0)])

    assert faces == [[0, 1, 2, 3]]


def test_normalize_face_colors_broadcasts_shared_color():
    colors = normalize_face_colors(2, (0.1, 0.2, 0.3))

    assert colors == [
        pytest.approx((0.1, 0.2, 0.3, 1.0)),
        pytest.approx((0.1, 0.2, 0.3, 1.0)),
    ]


def test_selection_mask_accepts_indices_and_callables():
    atoms = Atoms("H2O")

    by_index = selection_mask(atoms, [0, 2])
    by_callable = selection_mask(atoms, lambda frame: [atom.symbol == "H" for atom in frame])

    assert by_index.tolist() == [True, False, True]
    assert by_callable.tolist() == [True, True, False]


def test_selection_mask_validates_boolean_shape():
    atoms = Atoms("H2O")

    with pytest.raises(ValueError, match="boolean selection must have shape"):
        selection_mask(atoms, [True, False])
