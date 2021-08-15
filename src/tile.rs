//! Utilities related to creating a tilemap.
use crate::{
    camera::Camera,
    error::{CtraitError, CtraitResult},
    graphics::RenderContext,
    math::Vector2,
    rect::Rect,
    sprite::Sprite,
    traits::Renderable,
    Color,
};
use std::{
    ops::{Index, IndexMut},
    path::PathBuf,
};

/// 2D layout for a [`Tilemap`].
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TilemapLayout<const ROWS: usize, const COLUMNS: usize>(Vec<Option<usize>>);

impl<const ROWS: usize, const COLUMNS: usize> Default for TilemapLayout<ROWS, COLUMNS> {
    fn default() -> Self {
        Self(vec![None; ROWS * COLUMNS])
    }
}

impl<const ROWS: usize, const COLUMNS: usize> TilemapLayout<ROWS, COLUMNS> {
    /// Create a new layout from the given slice.
    ///
    /// # Errors
    ///
    /// This will return an error if the slice is not of appropriate size.
    /// For a tile layout of [`TilemapLayout<ROWS, COLUMNS>`], the slice should have a length equal to `ROWS` * `COLUMNS`.
    ///
    /// # Examples
    ///
    /// The following example creates a `3x3` tile layout:
    ///
    /// ```
    /// use ctrait::tile::TilemapLayout;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let layout = [
    ///    None,
    ///    Some(1),
    ///    None,
    ///    Some(2),
    ///    Some(1),
    ///    Some(1),
    ///    None,
    ///    Some(1),
    ///    None,
    /// ]; // Slice has length of 9 = 3 * 3.
    ///
    /// let tile_layout = TilemapLayout::<3, 3>::new(&layout)?;
    /// assert_eq!(tile_layout[1][0], Some(2));
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// The following example should panic as the slice's length does not match the specified tile
    /// layout dimensions:
    ///
    /// ```should_panic
    /// use ctrait::tile::TilemapLayout;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let layout = [Some(1), Some(1), None]; // Slice has length of 3.
    /// let tile_layout = TilemapLayout::<2, 3>::new(&layout)?; // Expects slice of length 6.
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(layout: &[Option<usize>]) -> CtraitResult<Self> {
        if layout.len() == ROWS * COLUMNS {
            Ok(Self(layout.to_vec()))
        } else {
            Err(CtraitError::Other(format!(
                "number of elements in layout must be equal to {}",
                ROWS * COLUMNS
            )))
        }
    }
}

impl<const ROWS: usize, const COLUMNS: usize> Index<usize> for TilemapLayout<ROWS, COLUMNS> {
    type Output = [Option<usize>];
    fn index(&self, row: usize) -> &Self::Output {
        let start = row * COLUMNS;
        &self.0[start..start + COLUMNS]
    }
}

impl<const ROWS: usize, const COLUMNS: usize> IndexMut<usize> for TilemapLayout<ROWS, COLUMNS> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        let start = row * COLUMNS;
        &mut self.0[start..start + COLUMNS]
    }
}

/// Enum representing possible tile types.
///
/// Each tile in a [`Tilemap`] can either be a sprite ([`Sprite`](Self::Sprite)) or colored square
/// ([`Color`](Self::Color)).
#[derive(Debug, Clone, PartialEq)]
pub enum Tile {
    /// Represents a sprite tile, holding a [`PathBuf`] to the sprite texture.
    Sprite(PathBuf),
    /// Represents a colored square tile, holding a [`Color`].
    Color(Color),
}

/// 2D tilemap.
#[derive(Debug)]
pub struct Tilemap<const ROWS: usize, const COLUMNS: usize> {
    /// Center world position of the tilemap.
    pub position: Vector2<f32>,
    /// Layout of the tilemap.
    ///
    /// Each element represents a tile with an index corresponding to the index of the tile type in the
    /// tile set.
    pub layout: TilemapLayout<ROWS, COLUMNS>,
    tile_set: Vec<Tile>,
    tile_size: f32,
}

impl<const ROWS: usize, const COLUMNS: usize> Tilemap<ROWS, COLUMNS> {
    /// Creates a new tilemap with a tile set and the size of each tile in pixels.
    ///
    /// # Examples
    ///
    /// ```
    /// use ctrait::{Color, tile::{Tilemap, Tile}};
    /// use std::path::PathBuf;
    ///
    /// // Create a tilemap with a set consisting of a red square and sprite.
    /// // Each tile will be rendered with a width and height of 64.
    /// let tilemap = Tilemap::<10, 5>::new(
    ///     &[Tile::Color(Color::RED), Tile::Sprite(PathBuf::from("path/to/texture.png"))],
    ///     64.0,
    /// );
    /// ```
    #[must_use]
    pub fn new(set: &[Tile], tile_size: f32) -> Self {
        Self {
            position: Vector2::zeros(),
            layout: TilemapLayout::default(),
            tile_set: set.to_vec(),
            tile_size,
        }
    }

    /// Constructs tilemap with a specified center world position.
    ///
    /// # Examples
    ///
    /// ```
    /// use ctrait::{math::Vector2, tile::Tilemap};
    ///
    /// let tilemap = Tilemap::<4, 4>::new(&[], 8.0)
    ///     .with_position(&Vector2::new(5.0, 10.0));
    /// ```
    #[must_use]
    pub fn with_position(mut self, position: &Vector2<f32>) -> Self {
        self.position = *position;
        self
    }

    /// Constructs tilemap with a specified layout.
    ///
    /// # Examples
    ///
    /// ```
    /// use ctrait::{
    ///     tile::{TilemapLayout, Tile, Tilemap},
    ///     Color,
    /// };
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let tilemap = Tilemap::<2, 3>::new(
    ///     &[Tile::Color(Color::RED), Tile::Color(Color::WHITE)],
    ///     64.0,
    /// )
    /// .with_layout(TilemapLayout::new(&[
    ///     Some(0), // Red tile will be rendered at the top-left.
    ///     None,    // No tile will be rendered.
    ///     Some(1), // White tile will be rendered.
    ///     Some(1),
    ///     Some(0),
    ///     None,
    /// ])?);
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn with_layout(mut self, layout: TilemapLayout<ROWS, COLUMNS>) -> Self {
        self.layout = layout;
        self
    }
}

impl<const ROWS: usize, const COLUMNS: usize> Renderable for Tilemap<ROWS, COLUMNS> {
    fn render(&self, camera: &Camera, context: &mut RenderContext) {
        let half_tilemap_dimensions = (Vector2::new(COLUMNS, ROWS).cast() * self.tile_size) / 2.;
        for row in 0..ROWS {
            for column in 0..COLUMNS {
                let index = self.layout[row][column];
                if let Some(index) = index {
                    if let Some(tile) = self.tile_set.get(index) {
                        let mut rect = Rect::new(
                            column as f32 * self.tile_size,
                            row as f32 * self.tile_size,
                            self.tile_size,
                            self.tile_size,
                        );
                        // Adjust for offset relative to world position and tilemap position.
                        rect.position -= half_tilemap_dimensions - self.position;
                        match tile {
                            Tile::Sprite(path) => {
                                // Render sprite.
                                let sprite = Sprite::new(path, &rect);
                                sprite.render(camera, context);
                            }
                            Tile::Color(color) => {
                                // Render rect with specified color.
                                let rect = rect.with_color(color);
                                rect.render(camera, context);
                            }
                        }
                    } else {
                        panic!("no tile in tile set corresponds with index {}", index);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Color, PathBuf, Tile, Tilemap, TilemapLayout, Vector2};

    #[test]
    fn tile_layout_default() {
        // Default constructor of TilemapLayout should result in passing a layout of length equal to
        // product of const generic ROWS and COLUMNS.
        let tile_layout = TilemapLayout::<2, 3>::default();
        assert_eq!(tile_layout.0.len(), 6);
        // By default, all tiles in the layout should be None.
        assert!(tile_layout.0.iter().all(|&tile| tile.is_none()));
    }

    #[test]
    fn tile_layout_new() {
        let tile_layout = TilemapLayout::<3, 2>::new(&[None; 6]).unwrap();
        assert_eq!(tile_layout.0.len(), 6);
    }

    #[test]
    fn tile_layout_new_error() {
        let result = TilemapLayout::<3, 2>::new(&[None]);
        assert!(result.is_err());
    }

    #[test]
    fn tile_layout_index() {
        let tile_layout = TilemapLayout::<2, 2>::new(&[None, None, Some(0), None]).unwrap();
        assert_eq!(tile_layout[1][0], Some(0));
    }

    #[test]
    fn tile_layout_index_mut() {
        let mut tile_layout = TilemapLayout::<2, 2>::new(&[None, None, None, None]).unwrap();
        tile_layout[1][0] = Some(0);
        assert_eq!(tile_layout[1][0], Some(0));
    }

    #[test]
    fn tilemap_new() {
        let tilemap = Tilemap::<10, 5>::new(
            &[
                Tile::Color(Color::RED),
                Tile::Sprite(PathBuf::from("texture.png")),
            ],
            64.0,
        );
        assert_eq!(tilemap.position, Vector2::zeros());
        assert_eq!(tilemap.layout, TilemapLayout::default());
        assert_eq!(
            tilemap.tile_set,
            vec![
                Tile::Color(Color::RED),
                Tile::Sprite(PathBuf::from("texture.png"))
            ]
        );
        assert!((tilemap.tile_size - 64.0).abs() < f32::EPSILON);
    }

    #[test]
    fn tilemap_with_position() {
        let tilemap = Tilemap::<1, 1>::new(&[], 0.0).with_position(&Vector2::new(5.0, 3.0));
        assert_eq!(tilemap.position, Vector2::new(5.0, 3.0));
    }

    #[test]
    fn tilemap_with_layout() {
        let tile_layout = TilemapLayout::<1, 2>::new(&[None, None]).unwrap();
        let tilemap = Tilemap::new(&[], 0.0).with_layout(tile_layout.clone());
        assert_eq!(tilemap.layout, tile_layout);
    }
}
