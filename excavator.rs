// excavator.rs
//
// This file contains utility functions for generating room shapes and offsets
// in a two-dimensional grid-based dungeon system. The main function in this file
// is `get_room_offsets`, which determines the relative coordinates (offsets) of
// tiles within a room based on the room's shape and size.
//
// Functionality:
// - `get_room_offsets`
//   - Inputs:
//     - `size` (u32): Determines the scale of the room by multiplying the offsets
//       of each shape's pattern. A size of 1 represents the base pattern, while larger
//       sizes scale the offsets proportionally.
//     - `shape` (String): A character or string representing the room's shape. Valid
//       inputs include alphanumeric characters (`0-9`, `a-z`, `A-Z`), with each
//       character corresponding to a predefined room pattern.
//   - Outputs:
//     - A vector of 2D coordinate tuples `(i32, i32)` representing the relative positions
//       of tiles in the room.
//
// Supported Room Shapes:
// - `0-9`: Numeric patterns, including single points, lines, and crosses.
// - `a-z`: Alphabetical patterns, such as diamonds, circles, zigzags, and various room structures.
// - `A-Z`: Mapped to the lowercase equivalents of `a-z` for consistency.
//
// Scaling Behavior:
// - The `size` parameter scales each offset proportionally, allowing room patterns
//   to grow in size while maintaining their relative layout.
//
// Example Usage:
// ```rust
// let size = 3;
// let shape = "a".to_string(); // Represents a cross pattern
// let offsets = get_room_offsets(size, shape);
// println!("{:?}", offsets);
// // Output: [(0, 3), (0, -3), (3, 0), (-3, 0)]
// ```
//
// Default Behavior:
// - If the `shape` is not recognized, the function returns an empty vector.
//
// This file can be extended to include additional utility functions for dungeon
// generation or shape manipulation.

pub fn get_room_offsets(size: u32, shape: String) -> Vec<(i32, i32)> {
    let size = size as i32; // Convert size to i32 for calculations
    let shape_char = shape.to_ascii_lowercase(); // Normalize shape to lowercase for consistent matching

    // Define offsets based on shape character
    let mut offsets = match shape_char.as_str() {
        // 0-9
        "0" => vec![(0, 0)], // Single point
        "1" => vec![(0, 1), (0, -1)], // Vertical line
        "2" => vec![(1, 0), (-1, 0)], // Horizontal line
        "3" => vec![(1, 1), (-1, -1)], // Diagonal line
        "4" => vec![(0, 1), (1, 0)], // L-shape
        "5" => vec![(0, -1), (-1, 0)], // Reverse L-shape
        "6" => vec![(1, 1), (-1, -1), (1, -1), (-1, 1)], // Diagonal cross
        "7" => vec![(0, 1), (0, -1), (1, 0)], // T-shape
        "8" => vec![(0, 1), (0, -1), (1, 0), (-1, 0)], // Cross
        "9" => vec![(0, 2), (0, -2), (2, 0), (-2, 0)], // Large cross

        // a-z
        "a" => vec![(0, 1), (0, -1), (1, 0), (-1, 0)], // Cross
        "b" => vec![(1, 1), (-1, 1), (1, -1), (-1, -1)], // Diagonal cross
        "c" => vec![(1, 1), (1, -1), (-1, 1), (-1, -1)], // Diamond
        "d" => vec![(0, 2), (0, -2), (2, 0), (-2, 0)], // Large cross
        "e" => vec![(1, 1), (2, 2), (-1, -1), (-2, -2)], // Expanding diagonal
        "f" => vec![(0, 1), (0, -1), (1, 0), (-1, 0), (1, 1)], // Extended cross
        "g" => vec![(1, 0), (-1, 0), (2, 0), (-2, 0)], // Horizontal line
        "h" => vec![(0, 1), (0, -1), (0, 2), (0, -2)], // Vertical line
        "i" => vec![(0, 0)], // Single point
        "j" => vec![(0, 1), (1, 1), (1, 0)], // Corner
        "k" => vec![(0, 2), (1, 1), (-1, 1)], // Triangle
        "l" => vec![(0, -2), (1, -1), (-1, -1)], // Reverse triangle
        "m" => vec![(1, 1), (1, 0), (0, 1)], // M-shape
        "n" => vec![(0, 1), (1, 1), (0, -1), (-1, -1)], // Zigzag
        "o" => vec![(1, 1), (1, -1), (-1, 1), (-1, -1), (0, 0)], // Full diamond
        "p" => vec![(0, 1), (1, 1), (1, 0), (1, -1)], // Partial diamond
        "q" => vec![(0, 1), (-1, 1), (-1, 0), (-1, -1)], // Partial diamond reversed
        "r" => vec![(1, 0), (1, 1), (0, 1), (-1, 1)], // Semi-circle
        "s" => vec![(0, -1), (1, -1), (1, 0), (1, 1)], // Semi-circle reversed
        "t" => vec![(0, 1), (0, -1), (1, 0), (-1, 0), (0, 0)], // Cross with center
        "u" => vec![(1, 1), (1, 0), (-1, 0), (-1, -1)], // U-shape
        "v" => vec![(1, 1), (0, 1), (-1, 1)], // V-shape
        "w" => vec![(1, -1), (0, -1), (-1, -1)], // W-shape
        "x" => vec![(1, 1), (-1, -1), (1, -1), (-1, 1)], // X-shape
        "y" => vec![(0, 1), (1, 1), (-1, -1)], // Y-shape
        "z" => vec![(0, -1), (1, 0), (-1, 0)], // Z-shape

        // A-Z (mapped to their lowercase equivalents)
        _ => vec![], // Default to no offsets if shape is not recognized
    };

    // Scale offsets based on size
    if size > 1 {
        offsets = offsets
            .iter()
            .map(|&(x, y)| (x * size, y * size))
            .collect();
    }

    offsets
}
