# Napari bermuda

*📌 Experimental: under active development. Please do not use it in production.*

Rust backend for napari contains code to speed up triangulation.

## Usage

Currently, this package exports only one function, `triangulate_path_edge`, which 
takes a list of points representing a path and returns a list of triangles
to draw the path with a given width.

Currently, only float32 points are supported.

```python
from typing import Literal
import numpy as np
import numpy.typing as npt

def triangulate_path_edge(
    path: npt.NDArray[tuple[int, Literal[2]], np.float32],
    closed: bool = False,
    limit: float = 3.0,
    bevel: bool = False,
) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
    """Determines the triangulation of a path in 2D.

    The resulting `offsets`
    can be multiplied by a `width` scalar and be added to the resulting
    `centers` to generate the vertices of the triangles for the triangulation,
    i.e. `vertices = centers + width*offsets`. By using the `centers` and
    `offsets` representation, the computed triangulation can be
    independent of the line width.

    Parameters
    ----------
    path : np.ndarray
        Nx2 array of central coordinates of path to be triangulated
    closed : bool
        Bool which determines if the path is closed or not
    limit : float
        Miter limit which determines when to switch from a miter join to a
        bevel join
    bevel : bool
        Bool which if True causes a bevel join to always be used. If False
        a bevel join will only be used when the miter limit is exceeded

    Returns
    -------
    centers : np.ndarray
        Mx2 array central coordinates of path triangles.
    offsets : np.ndarray
        Mx2 array of the offsets to the central coordinates that need to
        be scaled by the line width and then added to the centers to
        generate the actual vertices of the triangulation
    triangles : np.ndarray
        (M-2)x3 array of the indices of the vertices that will form the
        triangles of the triangulation
    """
    ...
```


## Development setup

1. [Install rust](https://www.rust-lang.org/tools/install).
   This includes `cargo` packaging and build tool and the `rustc` compiler.
2. `cargo build` compiles the source code and builds an executable.
3. `cargo test` runs tests.
4. `cargo doc --open` builds and serves docs (auto-generated from code).
