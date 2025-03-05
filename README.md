# Napari bermuda

*📌 Experimental: under active development. Please do not use it in production.*

Rust backend for napari contains code to speed up triangulation.

## Usage

Currently, this package exports three functions:

* `triangulate_path_edge` – path triangulation
* `triangulate_polygons_face` – polygon face triangulation  
* `triangulate_polygons_with_edge` – polygon face and border path triangulation

All functions accept only array data as array of `float32` are supported.     

Below are signatures with docstrings for document API. 

```python
from typing import Literal
import numpy as np
import numpy.typing as npt

def triangulate_path_edge(
    path: npt.NDArray[tuple[int, Literal[2]], np.float32],
    closed: bool = False,
    limit: float = 3.0,
    bevel: bool = False,
) -> tuple[
    npt.NDArray[tuple[int, Literal[2]], np.float32],
    npt.NDArray[tuple[int, Literal[2]], np.float32],
    npt.NDArray[tuple[int, Literal[3]], np.uint32],
]:
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
        Bool flag to enforce bevel join. If False
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


def triangulate_polygons_face(
    polygons: list[npt.NDArray[tuple[int, int, Literal[2]], np.float32]],
) -> tuple[
    npt.NDArray[tuple[int, Literal[3]], np.uint32],
    npt.NDArray[tuple[int, Literal[2]], np.float32],
]:
   """Triangulate multiple polygons using a sweeping line algorithm.

   This function performs triangulation of one or more 2D polygons, handling
   self-intersecting edges and repeated vertices. It returns both the triangulation
   indices and the vertex coordinates used in the triangulation.

   Parameters
   ----------
   polygons : list[numpy.ndarray]
       List of polygon vertex arrays. Each array should be Nx2 float32 array
       containing (x, y) coordinates of polygon vertices in counter-clockwise order.
       The polygons can be non-convex and can contain holes.

   Returns
   -------
   triangles : numpy.ndarray
       Kx3 uint32 array where each row contains three indices defining a triangle
       in the triangulation. The indices reference points in the returned points array.
   points : numpy.ndarray
       Lx2 float32 array containing (x, y) coordinates of vertices used in the
       triangulation. This includes original vertices and may contain additional points
       created at polygon intersections.

   Notes
   -----
   - The function automatically handles self-intersecting edges by splitting them
     at intersection points
   - Duplicate vertices are removed from the output
   - Input polygons must be oriented counter-clockwise
   - All coordinates must be finite (no NaN or infinite values)
   - Zero-area segments should be avoided

   Examples
   --------
   >>> # Simple square
   >>> polygon = np.array([[0, 0], [1, 0], [1, 1], [0, 1]], dtype=np.float32)
   >>> triangles, points = triangulate_polygons_face([polygon])

   >>> # Multiple polygons
   >>> poly1 = np.array([[0, 0], [2, 0], [2, 2], [0, 2]], dtype=np.float32)
   >>> poly2 = np.array([[0.5, 0.5], [1.5, 0.5], [1, 1.5]], dtype=np.float32)
   >>> triangles, points = triangulate_polygons_face([poly1, poly2])
   """
   ...


def triangulate_polygons_with_edge(
   polygons: list[npt.NDArray[tuple[int, int, Literal[2]], np.float32]],
) -> tuple[
   tuple[
       npt.NDArray[tuple[int, Literal[2]], np.float32],
       npt.NDArray[tuple[int, Literal[2]], np.float32],
       npt.NDArray[tuple[int, Literal[3]], np.uint32],
   ],
   tuple[
       npt.NDArray[tuple[int, Literal[3]], np.uint32],
       npt.NDArray[tuple[int, Literal[2]], np.float32],
   ],
]:
   """Triangulate multiple polygons generating both face and edge triangulations.
   
   This function performs two types of triangulation:
   1. Face triangulation of the polygon interiors using a sweeping line algorithm
   2. Edge triangulation of the polygon boundaries with miter joins
   
   Parameters
   ----------
   polygons : list[numpy.ndarray]
      List of polygon vertex arrays. Each array should be Nx2 float32 array
      containing (x, y) coordinates in counter-clockwise order. The polygons
      can be non-convex and can contain holes.
   
   Returns
   -------
   face_triangulation : tuple
      A tuple containing face triangulation data:
      - triangles : numpy.ndarray
          Mx3 uint32 array of vertex indices forming triangles for polygon faces
      - points : numpy.ndarray
          Px2 float32 array of vertex coordinates used in face triangulation
   edge_triangulation : tuple
      A tuple containing edge triangulation data:
      - centers : numpy.ndarray
          Qx2 float32 array of central coordinates for edge triangles
      - offsets : numpy.ndarray
          Qx2 float32 array of offset vectors for edge vertices
      - triangles : numpy.ndarray
          Rx3 uint32 array of vertex indices for edge triangles
   
   Notes
   -----
   - Handles self-intersecting edges by splitting them at intersection points
   - Removes duplicate vertices from the output
   - Uses fixed parameters for edge triangulation:
      * Treats polygons as closed
      * Miter limit = 3.0
      * Uses miter joins (no beveling)
   - Input polygons must be oriented counter-clockwise
   - All coordinates must be finite (no NaN or infinite values)
   
   Examples
   --------
   >>> # Single square polygon
   >>> square = np.array([[0, 0], [1, 0], [1, 1], [0, 1]], dtype=np.float32)
   >>> face_tris, edge_tris = triangulate_polygons_with_edge([square])
   
   >>> # Polygon with a hole
   >>> outer = np.array([[0, 0], [2, 0], [2, 2], [0, 2]], dtype=np.float32)
   >>> inner = np.array([[0.5, 0.5], [1.5, 0.5], [1, 1.5]], dtype=np.float32)
   >>> face_tris, edge_tris = triangulate_polygons_with_edge([outer, inner])
   """
   ...

```


## Development setup

1. [Install rust](https://www.rust-lang.org/tools/install).
   This includes `cargo` packaging and build tool and the `rustc` compiler.
2. `cargo build` compiles the source code and builds an executable.
3. `cargo test` runs tests.
4. `cargo doc --open` builds and serves docs (auto-generated from code).
