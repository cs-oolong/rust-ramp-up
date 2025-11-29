use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::{self, disable_raw_mode, enable_raw_mode},
};

fn read_arrow() -> Option<KeyCode> {
    if let Ok(Event::Key(key)) = event::read() {

        if key.modifiers.contains(KeyModifiers::CONTROL)
            && matches!(key.code, KeyCode::Char('c'))
        {
            disable_raw_mode().unwrap();
            std::process::exit(0);
        }

        match key.code {
            KeyCode::Up    => return Some(KeyCode::Up),
            KeyCode::Down  => return Some(KeyCode::Down),
            KeyCode::Left  => return Some(KeyCode::Left),
            KeyCode::Right => return Some(KeyCode::Right),
            _ => {}
        }
    }
    None
}

fn main() {
    let mut player_w = 10;
    let mut player_h = 5;
    let map_height = 10;
    let map_width = 20;

    print_basic_map(map_width, map_height, player_w, player_h);

    enable_raw_mode().unwrap();

    loop {
        if let Some(code) = read_arrow() {
            match code {
                KeyCode::Up    => player_h -= 1,
                KeyCode::Down  => player_h += 1,
                KeyCode::Left  => player_w -= 1, 
                KeyCode::Right => player_w += 1,
                _ => unreachable!(),
            }
        }
        disable_raw_mode().unwrap();
        print_basic_map(map_width, map_height, player_w, player_h);
        enable_raw_mode().unwrap();
    }
    disable_raw_mode().unwrap();
}

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
