//! # Excavator Library for Dungeon Generation
//!   by DEVCI
//! 
//! This library provides functionalities for decoding an NFT-based `nft_id` 
//! into structured dungeon attributes and generating a detailed 2D dungeon map.
//!
//! ## Features:
//! 1. **NFT Decoding**:
//!    - Decodes the `nft_id` (e.g., "nft1qgqarlcwfjj7ct7kvh0zt067am2mgewp4y7a2nzfx8d9x8mudmes4u8mnv") 
//!      to extract dungeon properties.
//!
//! 2. **Dungeon Attributes**:
//!    - **Number of Rooms**: Determined by the first character after "nft1".
//!      For example:
//!        - '1' corresponds to (2 + 1) rooms.
//!        - 'z' corresponds to (2 + 36) rooms.
//!    - **Room Center Coordinates**: Starting from the character immediately after the room count,
//!      every two characters represent an (x, y) coordinate. If the number of rooms exceeds the 
//!      available characters for encoding, the process wraps to reuse characters.
//!    - **Room Sizes**: The size of each room is calculated based on the square area formula:
//!      `size = (1 + character value)^2`.
//!    - **Room Shapes**: Shapes are derived from the character following the room coordinates.
//!      Each shape is represented as a unique pattern of offsets relative to the room center.
//!
//! 3. **Additional Properties**:
//!    - **Dungeon Type**: The most frequent character in the `nft_id` determines the environment 
//!      (e.g., "Forest", "Hell").
//!    - **Dungeon Level**: Computed based on the total area of the rooms, categorized every 1000 units.
//!      For example:
//!        - Area 0-999 → Level 1
//!        - Area 1000-1999 → Level 2
//!
//! 4. **Excavation and Connections**:
//!    - Excavates rooms based on their sizes and shapes.
//!    - Randomly adds extra excavated points within the dungeon's x and y ranges to simulate scattered elements.
//!    - Generates tunnels connecting room centers using Manhattan-style paths, ensuring connectivity.
//!
//! 5. **Generated Map**:
//!    - Outputs a 2D grid of dungeon tiles using ASCII characters or can be plotted graphically.
//!    - Symbols:
//!        - `@`: Empty space.
//!        - `O`: Excavated room or tunnel point.
//!
//! ## Functions:
//!
//! ### Core Functions:
//! - `parse_nft_id`: Decodes the `nft_id` and returns detailed dungeon attributes, including the
//!   number of rooms, coordinates, sizes, shapes, and dungeon map.
//!
//! - `get_room_offsets`: Generates offset coordinates for a room based on its shape and size.
//!
//! - `add_random_excavated_points`: Adds randomly scattered excavated points within a given range.
//!
//! - `generate_tunnels`: Creates tunnels connecting room centers to ensure the dungeon is fully connected.
//!
//! ### Helper Functions:
//! - `char_to_num`: Converts a character into a numeric value, handling both alphanumeric characters.
//!
//! ## Example Usage:
//!
//! ```rust
//! let nft_id = "nft1qgqarlcwfjj7ct7kvh0zt067am2mgewp4y7a2nzfx8d9x8mudmes4u8mnv";
//! let dungeon_data = parse_nft_id(nft_id).expect("Failed to parse NFT ID");
//!
//! println!("Dungeon Level: {}", dungeon_data.10);
//! println!("Dungeon Type: {}", dungeon_data.9);
//! println!("Dungeon Map: {:?}", dungeon_data.11);
//! ```

use std::collections::HashMap;
use rand::Rng;
use std::collections::HashSet;

fn get_dungeon_type(most_frequent_char: &str) -> String {
    match most_frequent_char {
        "a" => "Ancient Ruins".to_string(),
        "b" => "Barrens".to_string(),
        "c" => "Cave".to_string(),
        "d" => "Desert".to_string(),
        "e" => "Enchanted Forest".to_string(),
        "f" => "Forest".to_string(),
        "g" => "Grassland".to_string(),
        "h" => "Hell".to_string(),
        "i" => "Ice Cavern".to_string(),
        "j" => "Jungle".to_string(),
        "k" => "Kingdom Ruins".to_string(),
        "l" => "Lava Pits".to_string(),
        "m" => "Mountain".to_string(),
        "n" => "Necropolis".to_string(),
        "o" => "Ocean Depths".to_string(),
        "p" => "Poison Swamp".to_string(),
        "q" => "Quagmire".to_string(),
        "r" => "Rainforest".to_string(),
        "s" => "Swamp".to_string(),
        "t" => "Temple".to_string(),
        "u" => "Underground Tunnels".to_string(),
        "v" => "Volcanic Crater".to_string(),
        "w" => "Water".to_string(),
        "x" => "Xeno Hive".to_string(),
        "y" => "Yellow Wasteland".to_string(),
        "z" => "Zephyr Highlands".to_string(),
        _ => "Unknown".to_string(), // Default case for unmapped characters
    }
}

fn get_dungeon_level(area_size: u64) -> u64 {
    (area_size / 1000) + 1
}

fn get_room_offsets(size: u32, shape: String) -> Vec<(i32, i32)> {
    let size = size as i32; // Convert size to i32 for calculations
    let shape_char = shape.to_ascii_lowercase(); // Normalize shape to lowercase for consistent matching

    // Define base offsets based on shape character
    let base_offsets = match shape_char.as_str() {
        // 0-9 (unique patterns)
        "0" => vec![(0, 0)], // Single point
        "1" => vec![(0, 1), (0, -1)], // Vertical line
        "2" => vec![(1, 0), (-1, 0)], // Horizontal line
        "3" => vec![(1, 1), (-1, -1)], // Diagonal line
        "4" => vec![(-1, 0), (1, 0), (0, 1)], // L-shape
        "5" => vec![(0, -1), (1, 0), (-1, 1)], // Reverse L-shape
        "6" => vec![(-1, -1), (1, 1), (1, -1), (-1, 1)], // Diagonal cross
        "7" => vec![(0, 1), (1, 0), (0, -1), (-1, 0)], // Full cross
        "8" => vec![(-2, 0), (2, 0), (0, -2), (0, 2)], // Large cross
        "9" => vec![(-3, 0), (3, 0), (0, -3), (0, 3)], // Very large cross

        // a-z (unique patterns with distinct offsets)
        "a" => vec![(0, 1), (-1, 0), (1, 0), (0, -1)], // Cross
        "b" => vec![(-1, 1), (1, -1)], // Diagonal corners
        "c" => vec![(-1, 1), (1, 1), (1, -1), (-1, -1)], // Full diamond
        "d" => vec![(-2, 2), (2, 2), (-2, -2), (2, -2)], // Large diamond
        "e" => vec![(-2, 0), (2, 0), (0, -2), (0, 2)], // Expanded cross
        "f" => vec![(1, 1), (2, 2)], // Expanding diagonal
        "g" => vec![(-1, 0), (-2, 0), (-3, 0)], // Horizontal line left
        "h" => vec![(0, 1), (0, 2), (0, 3)], // Vertical line up
        "i" => vec![(0, 0)], // Single point
        "j" => vec![(-1, 1), (0, 1), (1, 0)], // Corner
        "k" => vec![(0, 2), (-1, 1), (1, -1)], // Triangle
        "l" => vec![(-2, 0), (1, -1), (2, -2)], // Reverse diagonal
        "m" => vec![(-1, -1), (0, 1), (1, 0), (-1, 1)], // M-shape
        "n" => vec![(-1, 1), (1, -1), (0, 0)], // Zigzag
        "o" => vec![(-2, 2), (2, -2), (0, 0)], // Circle-like
        "p" => vec![(-1, 1), (1, 1), (1, -1)], // Partial diamond
        "q" => vec![(-1, 1), (-1, -1)], // Partial diamond reversed
        "r" => vec![(-2, 2), (0, 2), (2, 2)], // Semi-circle
        "s" => vec![(-2, -2), (0, -2), (2, -2)], // Semi-circle reversed
        "t" => vec![(-1, 0), (0, 0), (1, 0)], // T-shape
        "u" => vec![(-1, -1), (1, -1)], // U-shape
        "v" => vec![(0, 2), (-1, 1), (1, 1)], // V-shape
        "w" => vec![(-1, 1), (0, 0), (1, -1)], // W-shape
        "x" => vec![(-2, 2), (2, -2), (-2, -2), (2, 2)], // X-shape
        "y" => vec![(0, 2), (-1, 1), (1, -1)], // Y-shape
        "z" => vec![(-1, 0), (0, 1), (1, 0)], // Z-shape
        _ => vec![], // Default to no offsets if shape is not recognized
    };


    // Generate all points within the extended range based on size
    let mut offsets = Vec::new();

    for &(base_x, base_y) in &base_offsets {
        for x in (base_x - (size - 1))..=(base_x + (size - 1)) {
            for y in (base_y - (size - 1))..=(base_y + (size - 1)) {
                if !offsets.contains(&(x, y)) { // Avoid duplicates
                    offsets.push((x, y));
                }
            }
        }
    }

    offsets
}

/// Add random excavated points to the map
fn add_random_excavated_points(
    existing_points: Vec<(i32, i32)>,
    x_range: (i32, i32),
    y_range: (i32, i32),
    num_points: usize,
) -> Vec<(i32, i32)> {
    let mut rng = rand::thread_rng();
    let mut point_set: HashSet<(i32, i32)> = existing_points.iter().copied().collect();

    while point_set.len() < existing_points.len() + num_points {
        let random_x = rng.gen_range(x_range.0..=x_range.1);
        let random_y = rng.gen_range(y_range.0..=y_range.1);
        point_set.insert((random_x, random_y));
    }

    point_set.into_iter().collect()
}

/// Generates tunnels connecting room centers
/// Connects the first room to the second, the third to the fourth, and so on.
fn generate_tunnels(room_centers: &Vec<(i32, i32)>) -> Vec<Vec<(i32, i32)>> {
    let mut tunnels = Vec::new();

    // Iterate through pairs of room centers
    for i in (0..room_centers.len()).step_by(2) {
        if i + 1 < room_centers.len() {
            let start = room_centers[i];
            let end = room_centers[i + 1];

            // Generate a tunnel path
            let tunnel = create_tunnel(start, end);
            tunnels.push(tunnel);
        }
    }

    tunnels
}

/// Creates a tunnel (a series of points) connecting two room centers
fn create_tunnel(start: (i32, i32), end: (i32, i32)) -> Vec<(i32, i32)> {
    let mut tunnel = Vec::new();

    // Use a simple Manhattan-style path creation
    let (mut x, mut y) = start;

    // Move horizontally towards the target x-coordinate
    while x != end.0 {
        tunnel.push((x, y));
        if x < end.0 {
            x += 1;
        } else {
            x -= 1;
        }
    }

    // Move vertically towards the target y-coordinate
    while y != end.1 {
        tunnel.push((x, y));
        if y < end.1 {
            y += 1;
        } else {
            y -= 1;
        }
    }

    tunnel
}

pub fn parse_nft_id(
    nft_id: &str,
) -> Result<(usize, Vec<(i32, i32)>, Vec<u32>, Vec<String>, (i32, i32), (i32, i32), u64, HashMap<char, usize>, String, String, u64, Vec<(i32, i32)>), String> {
    // Ensure the NFT ID starts with "nft1" and has sufficient length
    if !nft_id.starts_with("nft1") || nft_id.len() < 4 {
        return Err("Invalid NFT ID format. It must start with 'nft1' and be long enough.".to_string());
    }

    // Extract the number of rooms
    let room_char = nft_id.chars().nth(4).unwrap(); // First character after "nft1"
    let num_rooms = match room_char.to_digit(36) {
        Some(val) => 2 + val as usize,
        None => return Err("Invalid character for room count.".to_string()),
    };

    // Extract coordinates
    let mut coordinates = Vec::new();
    let coord_start = 5; // Start reading coordinates after "nft1" + room count character
    let mut coord_index = coord_start;

    for _ in 0..num_rooms {
        let x_char = nft_id.chars().nth(coord_index).unwrap_or_else(|| {
            nft_id.chars().nth((coord_index - coord_start) % (nft_id.len() - coord_start)).unwrap()
        });
        let y_char = nft_id.chars().nth(coord_index + 1).unwrap_or_else(|| {
            nft_id.chars().nth((coord_index - coord_start + 1) % (nft_id.len() - coord_start)).unwrap()
        });

        let x = (char_to_num(x_char) as f64 * (num_rooms as f64).sqrt()).round() as i32;
        let y = (char_to_num(y_char) as f64 * (num_rooms as f64).sqrt()).round() as i32;
        coordinates.push((x, y));

        coord_index += 2;
    }

    // Extract room sizes
    let mut sizes = Vec::new();
    let mut area_size = 0;
    let size_start = nft_id.len() - num_rooms;
    for i in 0..num_rooms {
        let size_char = nft_id.chars().nth(size_start + i).unwrap();
        let size = (2 + ((char_to_num(size_char) as f64).sqrt() * 1.5).round() as i32
        - ((num_rooms as f64).sqrt() / 4.0).round() as i32) as u32;
        sizes.push(size);
        area_size += ((size * 2 + 1).pow(2)) as u64; // Calculate area and add it to `area_size`
    }

    // Determine dungeon level based on area size
    let dungeon_level = get_dungeon_level(area_size);

    // Extract room shapes
    let mut shapes = Vec::new();
    let shape_start = coord_start + (2 * num_rooms);
    let mut shape_index = shape_start;

    for _ in 0..num_rooms {
        let shape_char = nft_id.chars().nth(shape_index).unwrap_or_else(|| {
            nft_id.chars().nth((shape_index - coord_start) % (nft_id.len() - coord_start)).unwrap()
        });
        shapes.push(shape_char.to_string());
        shape_index += 1;
    }

    // Determine dungeon width and height
    let min_x = coordinates.iter().map(|&(x, _)| x).min().unwrap_or(0) - 1;
    let max_x = coordinates.iter().map(|&(x, _)| x).max().unwrap_or(0) + 1;
    let min_y = coordinates.iter().map(|&(_, y)| y).min().unwrap_or(0) - 1;
    let max_y = coordinates.iter().map(|&(_, y)| y).max().unwrap_or(0) + 1;

    // Calculate frequency of each character a-z
    let mut char_frequency: HashMap<char, usize> = HashMap::new();
    for c in nft_id.chars() {
        if c.is_ascii_lowercase() {
            *char_frequency.entry(c).or_insert(0) += 1;
        }
    }

    // Find the first character with the highest frequency
    let most_frequent_char = char_frequency
        .iter()
        .max_by_key(|&(_, &count)| count)
        .map(|(&c, _)| c.to_string())
        .unwrap_or("None".to_string());
    
    // Determine dungeon type
    let dungeon_type = get_dungeon_type(&most_frequent_char);

    // Generate excavated room coordinates
    let mut excavated_coordinates = Vec::new();
    for i in 0..num_rooms {
        let room_center = coordinates[i];
        let room_offsets = get_room_offsets(sizes[i], shapes[i].clone());
        let room_coords: Vec<(i32, i32)> = room_offsets
            .iter()
            .map(|&(ox, oy)| (room_center.0 + ox, room_center.1 + oy))
            .collect();

        // Skip adding if the room coordinates are empty
        if !room_coords.is_empty() {
            excavated_coordinates.push(room_coords);
        }
    }

    let mut all_excavated_coords: Vec<(i32, i32)> = excavated_coordinates.iter().flatten().copied().collect();
    let _min_x = all_excavated_coords.iter().map(|&(x, _)| x).min().unwrap_or(0);
    let _max_x = all_excavated_coords.iter().map(|&(x, _)| x).max().unwrap_or(0);
    let _min_y = all_excavated_coords.iter().map(|&(_, y)| y).min().unwrap_or(0);
    let _max_y = all_excavated_coords.iter().map(|&(_, y)| y).max().unwrap_or(0);

    // Generate tunnels between room centers
    let tunnels = generate_tunnels(&coordinates);

    // Flatten and append tunnels to excavated_coordinates
    for tunnel in tunnels {
        all_excavated_coords.extend(tunnel);
    }

    // Add random points to the dungeon
    let final_excavated_coords = add_random_excavated_points(all_excavated_coords, (min_x, max_x), (min_y, max_y), area_size as usize / 50);

    Ok((num_rooms, coordinates, sizes, shapes, (min_x, max_x), (min_y, max_y), area_size, char_frequency, most_frequent_char, dungeon_type, dungeon_level, final_excavated_coords,))
}

// Helper function to map a character to a number
fn char_to_num(c: char) -> i32 {
    if c.is_digit(10) {
        c.to_digit(10).unwrap() as i32
    } else {
        c.to_ascii_lowercase() as i32 - 'a' as i32 + 10
    }
}
