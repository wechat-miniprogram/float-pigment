//! Dynamic Grid Data Structure
//!
//! A dynamically expandable 2D grid that supports growing in both dimensions.
//! Unlike the `grid` crate which requires fixed dimensions at creation time,
//! this structure can expand on-demand during item placement.

use std::fmt::Debug;
use std::ops::{Index, IndexMut};

/// A dynamically expandable 2D grid stored in row-major order.
///
/// The grid uses a single `Vec<T>` for storage, with elements arranged
/// row by row. This provides good cache locality for row-wise iteration.
///
/// ## Memory Layout
/// For a 3x4 grid (3 rows, 4 columns):
/// ```text
/// Index:  0  1  2  3  4  5  6  7  8  9 10 11
/// Row:    [  row 0  ] [  row 1  ] [  row 2  ]
/// ```
pub struct DynamicGrid<T> {
    /// Storage for grid cells, in row-major order
    data: Vec<T>,
    /// Number of rows
    rows: usize,
    /// Number of columns
    cols: usize,
}

impl<T: Default> DynamicGrid<T> {
    /// Create an empty grid with 0 rows and 0 columns.
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            rows: 0,
            cols: 0,
        }
    }

    /// Create a grid with specified initial dimensions.
    ///
    /// All cells are initialized to `T::default()`.
    pub fn with_size(rows: usize, cols: usize) -> Self {
        let mut data = Vec::with_capacity(rows * cols);
        for _ in 0..(rows * cols) {
            data.push(T::default());
        }
        Self { data, rows, cols }
    }

    /// Ensure the grid has at least `(min_rows, min_cols)` capacity.
    ///
    /// If expansion is needed:
    /// - When only rows increase: simply append new default elements
    /// - When columns increase: data must be rearranged (more expensive)
    ///
    /// This is called automatically when setting values at new positions.
    pub fn ensure_size(&mut self, min_rows: usize, min_cols: usize) {
        if min_rows <= self.rows && min_cols <= self.cols {
            return; // No expansion needed
        }

        let new_rows = min_rows.max(self.rows);
        let new_cols = min_cols.max(self.cols);

        if new_cols != self.cols {
            // Columns changed - need to rearrange data
            let mut new_data = Vec::with_capacity(new_rows * new_cols);
            for _ in 0..(new_rows * new_cols) {
                new_data.push(T::default());
            }

            // Copy existing data to new positions using swap
            for r in 0..self.rows {
                for c in 0..self.cols {
                    std::mem::swap(
                        &mut new_data[r * new_cols + c],
                        &mut self.data[r * self.cols + c],
                    );
                }
            }

            self.data = new_data;
        } else if new_rows > self.rows {
            // Only rows increased - just append default values
            for _ in 0..((new_rows - self.rows) * new_cols) {
                self.data.push(T::default());
            }
        }

        self.rows = new_rows;
        self.cols = new_cols;
    }

    /// Set the element at (row, col), automatically expanding if needed.
    ///
    /// This is the key method for dynamic grid expansion - it ensures
    /// the grid is large enough before setting the value.
    pub fn set(&mut self, row: usize, col: usize, value: T) {
        self.ensure_size(row + 1, col + 1);
        self.data[row * self.cols + col] = value;
    }
}

impl<T> DynamicGrid<T> {
    /// Returns the number of rows.
    #[inline(always)]
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the number of columns.
    #[inline(always)]
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Get a reference to the element at (row, col).
    ///
    /// Returns `None` if the position is out of bounds.
    #[inline]
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.rows && col < self.cols {
            Some(&self.data[row * self.cols + col])
        } else {
            None
        }
    }

    /// Get a mutable reference to the element at (row, col).
    ///
    /// Returns `None` if the position is out of bounds.
    #[inline]
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if row < self.rows && col < self.cols {
            Some(&mut self.data[row * self.cols + col])
        } else {
            None
        }
    }

    /// Returns an iterator over all elements in row-major order.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    /// Returns a mutable iterator over all elements in row-major order.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.data.iter_mut()
    }

    /// Returns an iterator over elements in a specific row.
    ///
    /// # Panics
    /// Panics if `row >= self.rows`.
    pub fn iter_row(&self, row: usize) -> impl Iterator<Item = &T> {
        assert!(row < self.rows, "Row index out of bounds");
        let start = row * self.cols;
        let end = start + self.cols;
        self.data[start..end].iter()
    }

    /// Returns a mutable iterator over elements in a specific row.
    ///
    /// # Panics
    /// Panics if `row >= self.rows`.
    pub fn iter_row_mut(&mut self, row: usize) -> impl Iterator<Item = &mut T> {
        assert!(row < self.rows, "Row index out of bounds");
        let start = row * self.cols;
        let end = start + self.cols;
        self.data[start..end].iter_mut()
    }

    /// Returns an iterator over elements in a specific column.
    ///
    /// Note: This is less efficient than row iteration due to non-contiguous access.
    ///
    /// # Panics
    /// Panics if `col >= self.cols`.
    pub fn iter_col(&self, col: usize) -> impl Iterator<Item = &T> + '_ {
        assert!(col < self.cols, "Column index out of bounds");
        (0..self.rows).map(move |r| &self.data[r * self.cols + col])
    }

    /// Returns a mutable iterator over elements in a specific column.
    pub fn iter_col_mut(&mut self, col: usize) -> ColumnIterMut<'_, T> {
        assert!(col < self.cols, "Column index out of bounds");
        ColumnIterMut {
            grid: self,
            col,
            current_row: 0,
        }
    }
}

impl<T: Default> Default for DynamicGrid<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> Clone for DynamicGrid<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            rows: self.rows,
            cols: self.cols,
        }
    }
}

impl<T: PartialEq> PartialEq for DynamicGrid<T> {
    fn eq(&self, other: &Self) -> bool {
        self.rows == other.rows && self.cols == other.cols && self.data == other.data
    }
}

impl<T> Index<(usize, usize)> for DynamicGrid<T> {
    type Output = T;

    #[inline]
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        assert!(
            row < self.rows && col < self.cols,
            "Index ({}, {}) out of bounds for grid of size ({}, {})",
            row,
            col,
            self.rows,
            self.cols
        );
        &self.data[row * self.cols + col]
    }
}

impl<T> IndexMut<(usize, usize)> for DynamicGrid<T> {
    #[inline]
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        assert!(
            row < self.rows && col < self.cols,
            "Index ({}, {}) out of bounds for grid of size ({}, {})",
            row,
            col,
            self.rows,
            self.cols
        );
        &mut self.data[row * self.cols + col]
    }
}

impl<T: Debug> Debug for DynamicGrid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "DynamicGrid {}x{} {{", self.rows, self.cols)?;
        for row in 0..self.rows {
            write!(f, "  [")?;
            for col in 0..self.cols {
                if col > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:?}", self.data[row * self.cols + col])?;
            }
            writeln!(f, "]")?;
        }
        write!(f, "}}")
    }
}

/// Mutable iterator over a column.
///
/// This uses an index-based approach due to Rust's borrowing rules
/// preventing multiple mutable references.
pub struct ColumnIterMut<'a, T> {
    grid: &'a mut DynamicGrid<T>,
    col: usize,
    current_row: usize,
}

impl<'a, T> Iterator for ColumnIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_row >= self.grid.rows {
            return None;
        }

        let row = self.current_row;
        self.current_row += 1;

        // SAFETY: We're iterating through unique indices, so no aliasing occurs
        let ptr = self.grid.data.as_mut_ptr();
        let index = row * self.grid.cols + self.col;
        unsafe { Some(&mut *ptr.add(index)) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_grid() {
        let grid: DynamicGrid<i32> = DynamicGrid::new();
        assert_eq!(grid.rows(), 0);
        assert_eq!(grid.cols(), 0);
    }

    #[test]
    fn test_with_size() {
        let grid: DynamicGrid<i32> = DynamicGrid::with_size(3, 4);
        assert_eq!(grid.rows(), 3);
        assert_eq!(grid.cols(), 4);
    }

    #[test]
    fn test_dynamic_expansion() {
        let mut grid: DynamicGrid<i32> = DynamicGrid::new();

        // Grid starts empty
        assert_eq!(grid.rows(), 0);
        assert_eq!(grid.cols(), 0);

        // Set expands to fit
        grid.set(0, 0, 1);
        assert_eq!(grid.rows(), 1);
        assert_eq!(grid.cols(), 1);
        assert_eq!(grid[(0, 0)], 1);

        // Expand rows only
        grid.set(2, 0, 3);
        assert_eq!(grid.rows(), 3);
        assert_eq!(grid.cols(), 1);
        assert_eq!(grid[(2, 0)], 3);

        // Expand columns (requires rearrangement)
        grid.set(1, 2, 5);
        assert_eq!(grid.rows(), 3);
        assert_eq!(grid.cols(), 3);
        assert_eq!(grid[(1, 2)], 5);

        // Verify previous values preserved
        assert_eq!(grid[(0, 0)], 1);
        assert_eq!(grid[(2, 0)], 3);
    }

    #[test]
    fn test_row_iteration() {
        let mut grid: DynamicGrid<i32> = DynamicGrid::with_size(2, 3);
        grid[(0, 0)] = 1;
        grid[(0, 1)] = 2;
        grid[(0, 2)] = 3;

        let row0: Vec<_> = grid.iter_row(0).cloned().collect();
        assert_eq!(row0, vec![1, 2, 3]);
    }

    #[test]
    fn test_col_iteration() {
        let mut grid: DynamicGrid<i32> = DynamicGrid::with_size(3, 2);
        grid[(0, 0)] = 1;
        grid[(1, 0)] = 2;
        grid[(2, 0)] = 3;

        let col0: Vec<_> = grid.iter_col(0).cloned().collect();
        assert_eq!(col0, vec![1, 2, 3]);
    }
}
