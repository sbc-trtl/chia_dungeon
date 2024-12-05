mod utils;
use rand::Rng;
use std::collections::HashSet;
use plotters::prelude::*;


fn generate_nft_id() -> String {
    let mut rng = rand::thread_rng();
    let mut nft_id = String::from("nft1");

    // Generate the remaining 58 random characters
    for _ in 0..58 {
        let char_index = rng.gen_range(0..62); // 0-61 for base62
        let random_char = match char_index {
            0..=9 => (b'0' + char_index as u8) as char,      // Numbers '0'-'9'
            10..=35 => (b'a' + (char_index - 10) as u8) as char, // Lowercase letters 'a'-'z'
            36..=61 => (b'A' + (char_index - 36) as u8) as char, // Uppercase letters 'A'-'Z'
            _ => unreachable!(), // Should never reach here
        };
        nft_id.push(random_char);
    }
    nft_id
}

/// Generate and plot the dungeon map
fn plot_dungeon_map(
    excavated_coordinates: Vec<(i32, i32)>,
    x_range: (i32, i32),
    y_range: (i32, i32),
) -> Result<(), Box<dyn std::error::Error>> {
    let room_coords: HashSet<(i32, i32)> = excavated_coordinates.into_iter().collect();

    // Create the plot using plotters
    let root = BitMapBackend::new("dungeon_map.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Dungeon Map", ("sans-serif", 30))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(x_range.0..x_range.1, y_range.0..y_range.1)?;

    chart.configure_mesh().draw()?;

    // Draw the dungeon map
    chart.draw_series(room_coords.iter().map(|&(x, y)| {
        Circle::new((x, y), 3, &RED) // Room excavated
    }))?;

    // Save the plot
    root.present()?;
    println!("Dungeon map saved to 'dungeon_map.png'");

    Ok(())
}

fn main() {
    //! This program generates a dungeon and simulates player movement based on an NFT code.
    //! Description of the original implementation:
    //! 1. Decodes the `nft_id` to determine:
    //!     - The number of rooms.
    //!     - Room center coordinates.
    //!     - Room sizes.
    //!     - Room shapes.
    //! 2. Creates a dungeon with interconnected rooms based on the decoded parameters.
    //! 3. Simulates a simple game loop where the player can explore the dungeon.
    //! 4. Includes random room type generation and validation for room existence.

    // Generate a random NFT ID
    let nft_code = generate_nft_id();
    println!("Generated NFT ID: {}", nft_code);

    // Parse the NFT ID
    match utils::excavator::parse_nft_id(&nft_code) {
        Ok((_num_rooms, _coordinates, _sizes, _shapes, x_range, y_range, _area_size, _char_frequency, _most_frequent_char, dungeon_type, dungeon_level, excavated_coordinates)) => {
            println!("Parsed NFT ID:");
            println!("Type: {:?}", dungeon_type);
            println!("Level: {:?}", dungeon_level);
            println!("Excavated rooms: {:?}", excavated_coordinates);

            // Print the dungeon map
            println!("Dungeon Map:");
            let _ = plot_dungeon_map(excavated_coordinates, x_range, y_range);
        }
        Err(err) => println!("Error parsing NFT ID: {}", err),
    }
}

