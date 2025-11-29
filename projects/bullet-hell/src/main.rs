fn print_basic_map(map_width: u32, map_height: u32, player_x: u32, player_y: u32) {
    let mut map = String::from("");

    for column in 0..map_width {
        map.push_str(". "); // (x, 10)
    }
    map.push_str("\n");

    for row in 0..map_height {
        map.push_str("."); // leftmost wall piece
        for column in 1..=map_width-2 {
            if row == player_y && column == player_x {
                map.push_str("@ ");
                continue;
            }
            map.push_str("  "); // blank parts
        }
        map.push_str(" ."); // rightmost wall piece
        map.push_str("\n");
    }

    for column in 0..map_width {
        map.push_str(". "); // (x, 0)
    }
    map.push_str("\n");

    println!("{}", map);
}

fn main() {
    print_basic_map(20, 10, 10, 5);
}
