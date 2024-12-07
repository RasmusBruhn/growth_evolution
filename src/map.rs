use crate::{
    constants::{CHUNK_SIZE, INV_SQRT_3, SQRT_3},
    types,
};
use once_cell::sync::Lazy;
use std::{f64::consts::PI, fmt::Debug, iter};
use thiserror::Error;

/// Calculates what tile the given cartesian coordinate is within and returns its tile index,
/// the direction for positive x tiles is up-left and the direction for positive y tiles is up
///
/// # Parameters
///
/// point: The cartesian coordinates to use
pub fn coordinate_to_tile(point: &types::Point) -> types::Index {
    // Calculate the skewed coordinates and index
    let skew_point = types::Point::new(0.5 * (SQRT_3 * point.x - point.y), point.y);
    let skew_index = types::Index::new(
        (skew_point.x + 0.5).floor() as i64,
        (skew_point.y + 0.5).floor() as i64,
    );

    // Figure out what strip of locations this index is part of
    let strip = (skew_index.y - skew_index.x) % 3;
    let strip = if strip < 0 { strip + 3 } else { strip };

    // Get the relative coordinate
    let rel_point = skew_point - types::Point::new(skew_index.x as f64, skew_index.y as f64);

    // Find the correct tile
    match strip {
        0 => {
            let main_index = types::Index::new(
                -(4 * skew_index.x + 2 * skew_index.y) / 3,
                (2 * skew_index.x + 4 * skew_index.y) / 3,
            );

            main_index
                + if rel_point.x + rel_point.y < -0.5 {
                    types::Index::new(1, -1)
                } else if rel_point.x + rel_point.y > 0.5 {
                    types::Index::new(-1, 1)
                } else {
                    types::Index::new(0, 0)
                }
        }
        1 => {
            let main_index = types::Index::new(
                -(4 * skew_index.x + 2 * skew_index.y + 1) / 3,
                (2 * skew_index.x + 4 * skew_index.y - 1) / 3,
            );

            main_index
                + if rel_point.x > 0.0 && rel_point.y < 0.0 {
                    types::Index::new(0, 0)
                } else if rel_point.x + rel_point.y < 0.0 {
                    types::Index::new(1, 0)
                } else {
                    types::Index::new(0, 1)
                }
        }
        _ => {
            let main_index = types::Index::new(
                -(4 * skew_index.x + 2 * skew_index.y - 1) / 3,
                (2 * skew_index.x + 4 * skew_index.y + 1) / 3,
            );

            main_index
                + if rel_point.x < 0.0 && rel_point.y > 0.0 {
                    types::Index::new(0, 0)
                } else if rel_point.x + rel_point.y < 0.0 {
                    types::Index::new(0, -1)
                } else {
                    types::Index::new(-1, 0)
                }
        }
    }
}

/// Calculates the center cartesian coordinate of the given tile,
/// the direction for positive x tiles is up-left and the direction for positive y tiles is up
///
/// # Parameters
///
/// index: The index of the til to use
pub fn tile_to_coordinate(index: &types::Index) -> types::Point {
    return types::Point::new(-1.5 * INV_SQRT_3, 0.5) * (index.x as f64)
        + types::Point::new(0.0, 1.0) * (index.y as f64);
}

/// Calculates what chunk the given cartesian coordinate is within and returns its chunk index,
/// the direction for positive x chunks is right and the direction for positive y tiles is up-right
///
/// # Parameters
///
/// point: The cartesian coordinates to use
pub fn coordinate_to_chunk(point: &types::Point) -> types::Index {
    // Convert to rotated and scaled coordinates
    let factor = INV_SQRT_3 / (CHUNK_SIZE as f64);
    let new_point = types::Point::new(-point.y, point.x) * factor;

    // Get the index
    let index = coordinate_to_tile(&new_point);

    // Switch x and y for the correct index
    return types::Index::new(index.y, index.x);
}

/// Calculates the center cartesian coordinate of the given chunk,
/// the direction for positive x chunks is right and the direction for positive y tiles is up-right
///
/// # Parameters
///
/// index: The index of the til to use
pub fn chunk_to_coordinate(index: &types::Index) -> types::Point {
    return types::Point::new(3.0 * INV_SQRT_3, 0.0) * (((CHUNK_SIZE as i64) * index.x) as f64)
        + types::Point::new(1.5 * INV_SQRT_3, 1.5) * (((CHUNK_SIZE as i64) * index.y) as f64);
}

/// A map consiting of a single chunk with cyclic boundaries
#[derive(Clone, Debug)]
pub struct MapCyclic {
    /// The bulk of the chunk
    chunks_bulk: Chunk,
    /// All the different edges
    chunks_edge: [Chunk; 3],
    /// All the different vertices
    chunks_vertex: [Chunk; 2],
}

impl MapData for MapCyclic {
    fn get_index(&self, _chunk_type: &ChunkType, _coordinates: types::Index) -> Option<usize> {
        return Some(0);
    }

    fn get_chunk(&self, chunk_type: &ChunkType, _index: usize) -> &Chunk {
        return match chunk_type {
            ChunkType::Bulk => &self.chunks_bulk,
            ChunkType::Edge(edge_type) => &self.chunks_edge[edge_type.id()],
            ChunkType::Vertex(vertex_type) => &self.chunks_vertex[vertex_type.id()],
        };
    }

    fn get_chunk_mut(&mut self, chunk_type: &ChunkType, _index: usize) -> &mut Chunk {
        return match chunk_type {
            ChunkType::Bulk => &mut self.chunks_bulk,
            ChunkType::Edge(edge_type) => &mut self.chunks_edge[edge_type.id()],
            ChunkType::Vertex(vertex_type) => &mut self.chunks_vertex[vertex_type.id()],
        };
    }

    fn get_chunks(&self) -> Vec<&Chunk> {
        return iter::once(&self.chunks_bulk)
            .chain(self.chunks_edge.iter())
            .chain(self.chunks_vertex.iter())
            .collect();
    }

    fn get_chunks_mut(&mut self) -> Vec<&mut Chunk> {
        return iter::once(&mut self.chunks_bulk)
            .chain(self.chunks_edge.iter_mut())
            .chain(self.chunks_vertex.iter_mut())
            .collect();
    }
}

/// Holds all data for an entire map
#[derive(Debug)]
pub struct Map {
    /// All chunk data, use different implementations of MapData for different chunk layouts
    data: Box<dyn MapData>,
    /// All sources of resources
    sources: SourceMap,
}

impl Map {
    pub fn new(data: Box<dyn MapData>, sources: SourceMap) -> Self {
        // Create the map
        let mut map = Self { data, sources };

        // Populate
        map.populate_resources();

        return map;
    }

    /// Retrieves a reference to the chunk data
    pub fn get_data(&self) -> &dyn MapData {
        return self.data.as_ref();
    }

    /// Retrieves a mutable reference to the chunk data
    pub fn get_data_mut(&mut self) -> &mut dyn MapData {
        return self.data.as_mut();
    }

    /// Retrieves a reference to the source map
    pub fn get_sources(&self) -> &SourceMap {
        return &self.sources;
    }

    /// Retrieves a mutable reference to the source map, once the mutator is
    /// destroyed it will reload the base resources of the map
    pub fn get_sources_mut(&mut self) -> SourceMapMut {
        return SourceMapMut { map: self };
    }

    /// Populates all tiles with the correct base resources as given by the sources
    fn populate_resources(&mut self) {
        self.populate_resource(
            |source_map| return &source_map.nutrients,
            |resources| return &mut resources.nutrients,
        );
        self.populate_resource(
            |source_map| return &source_map.energy,
            |resources| return &mut resources.energy,
        );
        self.populate_resource(
            |source_map| return &source_map.water,
            |resources| return &mut resources.water,
        );
    }

    /// Populates all tiles with the correct base nutrients as given by the sources
    fn populate_resource<SourcesAccess, ResourcesAccess>(
        &mut self,
        sources_access: SourcesAccess,
        resources_access: ResourcesAccess,
    ) where
        SourcesAccess: Fn(&SourceMap) -> &[Source],
        ResourcesAccess: Fn(&mut Resources) -> &mut f64,
    {
        // Reset the nutrients and set modified to true
        self.data.get_chunks_mut().into_iter().for_each(|chunk| {
            chunk.modified = true;
            chunk.tiles.iter_mut().for_each(|tile| {
                *resources_access(&mut tile.base_resources) = 0.0;
            });
        });

        // Populate each source
        sources_access(&self.sources).iter().for_each(|source| {
            // Get the range for the source in units of chunk widths
            let range = (source.range() / (1.5 * CHUNK_SIZE as f64)).ceil() as i64;

            // Get the current chunk
            let center = coordinate_to_chunk(&source.center());

            // Loop over relative y values to the current chunk
            (-range..range + 1).for_each(|y| {
                let (min_x, max_x) = if y < 0 {
                    (-range - y, range)
                } else {
                    (-range, range - y)
                };
                (min_x..max_x + 1).for_each(|x| {
                    // Get the coordinates for the center of the chunk
                    let chunk_index = center + types::Index::new(x, y);
                    let chunk_coords = chunk_to_coordinate(&chunk_index);

                    // Loop over all chunk types
                    [
                        ChunkType::Bulk,
                        ChunkType::Edge(ChunkEdgeType::Top),
                        ChunkType::Edge(ChunkEdgeType::Middle),
                        ChunkType::Edge(ChunkEdgeType::Bottom),
                        ChunkType::Vertex(ChunkVertexType::Top),
                        ChunkType::Vertex(ChunkVertexType::Bottom),
                    ]
                    .iter()
                    .for_each(|chunk_type| {
                        // Get the chunk
                        let chunk = self.data.get_chunk_mut(
                            chunk_type,
                            match self.data.get_index(chunk_type, chunk_index) {
                                Some(value) => value,
                                None => return,
                            },
                        );

                        // Calculate population for each tile
                        let pop = source.evaluate(&chunk_coords, chunk_type.get_tile_centers());

                        // Add the population
                        pop.iter().zip(chunk.get_tiles_mut().iter_mut()).for_each(
                            |(value, tile)| {
                                *resources_access(&mut tile.base_resources) += value;
                            },
                        );
                    });
                });
            });
        });

        // Clamp all values
        self.data.get_chunks_mut().into_iter().for_each(|chunk| {
            chunk.tiles.iter_mut().for_each(|tile| {
                *resources_access(&mut tile.base_resources) =
                    resources_access(&mut tile.base_resources).clamp(0.0, 1.0);
            });
        });
    }
}

/// A mutator for the source map
#[derive(Debug)]
pub struct SourceMapMut<'map> {
    map: &'map mut Map,
}

impl<'map> SourceMapMut<'map> {
    /// Retrieves a reference to the sources
    pub fn get(&self) -> &SourceMap {
        return &self.map.sources;
    }

    /// Retrieves a mutable reference to the sources
    pub fn get_mut(&mut self) -> &mut SourceMap {
        return &mut self.map.sources;
    }
}

impl<'map> Drop for SourceMapMut<'map> {
    fn drop(&mut self) {
        // Force update the base resources for the map
        self.map.populate_resources();
    }
}

/// Holds all resource sources for an entire map
#[derive(Clone, Debug)]
pub struct SourceMap {
    /// The sources for nutrients
    pub nutrients: Vec<Source>,
    /// The sources for energy
    pub energy: Vec<Source>,
    /// The sources for water
    pub water: Vec<Source>,
}

/// The trait for any map of chunks, different layouts can be encoded in
/// different types, all logic must go through this interface
pub trait MapData: Debug {
    /// Retrieves the index of the chunk at the given index coordinate, None if it is out of bounds
    ///
    /// # Parameters
    ///
    /// coordinates: The index coordinates to get the chunk for
    fn get_index(&self, chunk_type: &ChunkType, coordinates: types::Index) -> Option<usize>;

    /// Retrieves the chunk at the given index
    ///
    /// # Parameters
    ///
    /// chunk_type: The type of chunk to retrieve
    ///
    /// index: The index of the chunk to retrieve
    fn get_chunk(&self, chunk_type: &ChunkType, index: usize) -> &Chunk;

    /// Retrieves the chunk at the given index as mutable
    ///
    /// # Parameters
    ///
    /// chunk_type: The type of chunk to retrieve
    ///
    /// index: The index of the chunk to retrieve
    fn get_chunk_mut(&mut self, chunk_type: &ChunkType, index: usize) -> &mut Chunk;

    /// Retrieves an iterator over all chunks
    fn get_chunks(&self) -> Vec<&Chunk>;

    /// Retrieves an iterator over all mutable chunks
    fn get_chunks_mut(&mut self) -> Vec<&mut Chunk>;
}

/// A chunk of tiles clustered together can be used as the bulk, an edge or a vertex
#[derive(Clone, Debug)]
pub struct Chunk {
    /// All tiles for this chunk
    tiles: Vec<Tile>,
    /// The type of chunk stored
    chunk_type: ChunkType,
    /// The index of this chunk in the list of chunks
    index: usize,
    /// True if one of the tiles has been modified, false once the graphics has updated
    modified: bool,
}

impl Chunk {
    /// Creates a new chunk from the given tiles
    ///
    /// # Parameters
    ///
    /// chunk_type: The type of chunk which is being constructed
    ///
    /// index: The index of this chunk in the list of chunks
    ///
    /// tiles: The tiles to set for the new chunk
    pub fn new(
        chunk_type: ChunkType,
        index: usize,
        tiles: Vec<Tile>,
    ) -> Result<Self, NewChunkError> {
        if tiles.len() != chunk_type.get_tile_count() {
            return Err(NewChunkError::InvalidSize(
                tiles.len(),
                chunk_type.get_tile_count(),
            ));
        }

        return Ok(Self {
            tiles,
            chunk_type,
            index,
            modified: true,
        });
    }

    /// Constructs a new chunk with all base resources set to 0
    ///
    /// # Parameters
    ///
    /// chunk_type: The type of chunk which is being constructed
    ///
    /// index: The index of this chunk in the list of chunks
    pub fn new_empty(chunk_type: ChunkType, index: usize) -> Self {
        // Get the number of tiles
        let tile_count = chunk_type.get_tile_count();

        // Create the tiles
        let tiles = (0..tile_count)
            .map(|_| {
                return Tile::new(Resources {
                    nutrients: 0.0,
                    energy: 0.0,
                    water: 0.0,
                });
            })
            .collect::<Vec<Tile>>();

        return Self::new(chunk_type, index, tiles).expect("Should not happen");
    }

    /// Checks if the chunk has been modified
    pub fn is_modified(&self) -> bool {
        return self.modified;
    }

    /// Sets the modified tag to false
    pub fn resolved(&mut self) {
        self.modified = false;
    }

    /// Retrieves the index of this chunk in the list of chunks
    pub fn get_index(&self) -> usize {
        return self.index;
    }

    /// Retrieves the type of this chunk
    pub fn get_chunk_type(&self) -> &ChunkType {
        return &self.chunk_type;
    }

    /// Retrieves a reference to the tiles
    pub fn get_tiles(&self) -> &[Tile] {
        return &self.tiles;
    }

    /// Retrieves a mutable reference to the tiles
    pub fn get_tiles_mut(&mut self) -> &mut Vec<Tile> {
        return &mut self.tiles;
    }
}

/// The type of chunk
#[derive(Clone, Copy, Debug)]
pub enum ChunkType {
    /// The largest type of chunk holding the bulk of the tiles
    Bulk,
    /// The edge chunk which surrounds the bulk
    Edge(ChunkEdgeType),
    /// The vertex chunk consisting only of a single tile
    Vertex(ChunkVertexType),
}

impl ChunkType {
    /// Retrieves the number of tiles in the chunk
    pub fn get_tile_count(&self) -> usize {
        return match self {
            ChunkType::Bulk => CHUNK_SIZE * (CHUNK_SIZE - 1) / 2 * 6 + 1,
            ChunkType::Edge(_) => CHUNK_SIZE - 1,
            ChunkType::Vertex(_) => 1,
        };
    }

    /// Retrieves centers relative to the center of the chunk for all tiles in
    /// this chunk type, they are sorted in the same way that they are stored in
    /// the chunk
    pub fn get_tile_centers(&self) -> &[types::Point] {
        return match self {
            ChunkType::Bulk => CHUNK_CENTERS_BULK.as_slice(),
            ChunkType::Edge(edge) => edge.get_tile_centers(),
            ChunkType::Vertex(vertex) => vertex.get_tile_centers(),
        };
    }
}

/// The location of the edge chunk compared to the bulk
#[derive(Clone, Copy, Debug)]
pub enum ChunkEdgeType {
    /// This is the top left edge
    Top,
    /// This is the left edge
    Middle,
    /// This is the bottom left edge
    Bottom,
}

impl ChunkEdgeType {
    /// Retrieves the id of this type of edge piece
    pub fn id(&self) -> usize {
        return match self {
            ChunkEdgeType::Top => 0,
            ChunkEdgeType::Middle => 1,
            ChunkEdgeType::Bottom => 2,
        };
    }

    /// Retrieves centers relative to the center of the chunk for all tiles in
    /// this chunk type, they are sorted in the same way that they are stored in
    /// the chunk
    pub fn get_tile_centers(&self) -> &[types::Point] {
        return match self {
            ChunkEdgeType::Top => CHUNK_CENTERS_EDGE_TOP.as_slice(),
            ChunkEdgeType::Middle => CHUNK_CENTERS_EDGE_MIDDLE.as_slice(),
            ChunkEdgeType::Bottom => CHUNK_CENTERS_EDGE_BOTTOM.as_slice(),
        };
    }
}

/// The location of the vertec chunk compared to the bulk
#[derive(Clone, Copy, Debug)]
pub enum ChunkVertexType {
    /// The top left vertex
    Top,
    /// The bottom left vertex
    Bottom,
}

impl ChunkVertexType {
    /// Retrieves the id of this type of vertex
    pub fn id(&self) -> usize {
        return match self {
            ChunkVertexType::Top => 0,
            ChunkVertexType::Bottom => 0,
        };
    }

    /// Retrieves centers relative to the center of the chunk for all tiles in
    /// this chunk type, they are sorted in the same way that they are stored in
    /// the chunk
    pub fn get_tile_centers(&self) -> &[types::Point] {
        return match self {
            ChunkVertexType::Top => CHUNK_CENTERS_VERTEX_TOP.as_slice(),
            ChunkVertexType::Bottom => CHUNK_CENTERS_VERTEX_BOTTOM.as_slice(),
        };
    }
}

static CHUNK_CENTERS_BULK: Lazy<[types::Point; 3 * CHUNK_SIZE * (CHUNK_SIZE - 1) + 1]> =
    Lazy::new(|| {
        std::array::from_fn(|id| {
            // Do id == 0 as a special case
            if id == 0 {
                return types::Point::new(0.0, 0.0);
            }

            // Get the layer and location
            let layer =
                (0.5 * (1.0 + (1.0 + 4.0 / 3.0 * ((id as f64) - 1.0)).sqrt())).floor() as usize;
            let rel_id = id - (3 * layer * (layer - 1) + 1);
            let slice_id = rel_id / layer;
            let location_id = rel_id - slice_id * layer;

            // Get the coordinates
            let (start, dir) = match slice_id {
                0 => (
                    types::Point::new(0.0, 1.0),
                    types::Point::new(-1.5 * INV_SQRT_3, -0.5),
                ),
                1 => (
                    types::Point::new(-1.5 * INV_SQRT_3, 0.5),
                    types::Point::new(0.0, -1.0),
                ),
                2 => (
                    types::Point::new(-1.5 * INV_SQRT_3, -0.5),
                    types::Point::new(1.5 * INV_SQRT_3, -0.5),
                ),
                3 => (
                    types::Point::new(0.0, -1.0),
                    types::Point::new(1.5 * INV_SQRT_3, 0.5),
                ),
                4 => (
                    types::Point::new(1.5 * INV_SQRT_3, -0.5),
                    types::Point::new(0.0, 1.0),
                ),
                _ => (
                    types::Point::new(1.5 * INV_SQRT_3, 0.5),
                    types::Point::new(-1.5 * INV_SQRT_3, 0.5),
                ),
            };

            return start * (layer as f64) + dir * (location_id as f64);
        })
    });
static CHUNK_CENTERS_EDGE_TOP: Lazy<[types::Point; CHUNK_SIZE - 1]> = Lazy::new(|| {
    std::array::from_fn(|id| {
        return types::Point::new(
            -1.5 * INV_SQRT_3 * ((id + 1) as f64),
            (CHUNK_SIZE as f64) - 0.5 * ((id + 1) as f64),
        );
    })
});
static CHUNK_CENTERS_EDGE_MIDDLE: Lazy<[types::Point; CHUNK_SIZE - 1]> = Lazy::new(|| {
    std::array::from_fn(|id| {
        return types::Point::new(
            -1.5 * (CHUNK_SIZE as f64) * INV_SQRT_3,
            0.5 * (CHUNK_SIZE as f64) - ((id + 1) as f64),
        );
    })
});
static CHUNK_CENTERS_EDGE_BOTTOM: Lazy<[types::Point; CHUNK_SIZE - 1]> = Lazy::new(|| {
    std::array::from_fn(|id| {
        return types::Point::new(
            -1.5 * ((CHUNK_SIZE - (id + 1)) as f64) * INV_SQRT_3,
            -0.5 * ((CHUNK_SIZE + (id + 1)) as f64),
        );
    })
});
static CHUNK_CENTERS_VERTEX_TOP: Lazy<[types::Point; 1]> = Lazy::new(|| {
    [types::Point {
        x: -1.5 * (CHUNK_SIZE as f64) * INV_SQRT_3,
        y: 0.5 * (CHUNK_SIZE as f64),
    }]
});
static CHUNK_CENTERS_VERTEX_BOTTOM: Lazy<[types::Point; 1]> = Lazy::new(|| {
    [types::Point {
        x: -1.5 * (CHUNK_SIZE as f64) * INV_SQRT_3,
        y: -0.5 * (CHUNK_SIZE as f64),
    }]
});

/// All data for a single tile including the base resources and current resources
#[derive(Clone, Copy, Debug)]
pub struct Tile {
    /// The base values for the resources which the actual values will attempt to gravitate towards
    pub base_resources: Resources,
}

impl Tile {
    /// Constructs a new til with the given base resources
    ///
    /// # Parameters
    ///
    /// base_resources: The base resources of this tile
    pub fn new(base_resources: Resources) -> Self {
        return Self { base_resources };
    }
}

/// All the main resource types
#[derive(Clone, Copy, Debug)]
pub struct Resources {
    /// Nutrients used to grow
    pub nutrients: f64,
    /// Energy used to survive
    pub energy: f64,
    /// Water used to allow processes to work
    pub water: f64,
}

/// A source of some resource
#[derive(Clone, Debug)]
pub enum Source {
    /// A source with a Gaussian distribution
    Gaussian(types::Gaussian),
}

impl Source {
    /// Calculates the range for this source to be relevant
    pub fn range(&self) -> f64 {
        return match self {
            Source::Gaussian(gaussian) => {
                // Get the longest variance
                let variances = gaussian.get_covariance().eigenvalues();

                // Make sure it is not invalid
                if cfg!(debug_assertions) && variances[1] < 0.0 {
                    panic!("The variance is negative: {:?}", gaussian.get_covariance());
                }

                // Calculate the range at which value of the Gaussian is 1/256
                (variances[0]
                    * (gaussian.norm * gaussian.norm * 256.0 * 256.0
                        / (4.0 * PI * PI * variances[0] * variances[1])))
                    .sqrt()
            }
        };
    }

    /// Calculates the center of the source
    pub fn center(&self) -> types::Point {
        return match self {
            Source::Gaussian(gaussian) => gaussian.mean,
        };
    }

    /// Evaluates the contribution from this source on the resources at the given positions
    ///
    /// # Parameters
    ///
    /// points: The positions to evaluate the source at
    pub fn evaluate(&self, offset: &types::Point, points: &[types::Point]) -> Vec<f64> {
        return match self {
            Source::Gaussian(gaussian) => gaussian.evaluate(offset, points),
        };
    }
}

/// The error types for when creating a new chunk
#[derive(Error, Debug, Clone)]
pub enum NewChunkError {
    /// The number of tiles was incorrect
    #[error("The number of tiles was incorrect, received {:?} but expected {:?}", .0, .1)]
    InvalidSize(usize, usize),
}
