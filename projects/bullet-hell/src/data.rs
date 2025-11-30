use crate::game::ProtoProjectile;

pub fn load_blueprints(path: &str) -> Vec<ProtoProjectile> {
    let txt =
        std::fs::read_to_string(path).unwrap_or_else(|e| panic!("cannot read {}: {}", path, e));
    ron::from_str(&txt).expect("bad RON")
}

#[cfg(test)]
mod tests {
    use super::*;

    //! Excluded from coverage because it's just a helper function for the tests below.
    fn create_temp_ron_file(content: &str) -> tempfile::NamedTempFile {
        use std::io::Write;
        let mut file = tempfile::NamedTempFile::new().expect("Failed to create temp file");
        file.write_all(content.as_bytes()).expect("Failed to write to temp file");
        file.flush().expect("Failed to flush temp file");
        file
    }

    #[test]
    fn successfully_loads_from_assets_file() {
        let projectiles = load_blueprints("assets/projectiles.ron");
        let expected = vec![
            ProtoProjectile {
                x: 1,
                y: 10,
                pattern: vec![(1, 0)],
            },
            ProtoProjectile {
                x: 38,
                y: 8,
                pattern: vec![(-1, 0)],
            },
            ProtoProjectile {
                x: 5,
                y: 2,
                pattern: vec![(1, 1)],
            },
            ProtoProjectile {
                x: 20,
                y: 5,
                pattern: vec![
                    (0, -1),
                    (0, -1),
                    (0, 1),
                    (0, 1),
                    (-1, 0),
                    (-1, 0),
                    (1, 0),
                    (1, 0),
                    (1, 1),
                ],
            },
        ];
        assert_eq!(projectiles, expected);
    }

    #[test]
    fn successfully_loads_from_any_existing_valid_file() {
        let ron_content = r#"[
        (x: 1, y: 2, pattern: [(1, 0)]),
        (x: 10, y: 20, pattern: [(0, 1)])
    ]"#;
        let temp_file = create_temp_ron_file(ron_content).unwrap();
        let path = temp_file.path().to_str().unwrap();

        let projectiles = load_blueprints(path);
        let expected = vec![
            ProtoProjectile {
                x: 1,
                y: 2,
                pattern: vec![(1, 0)],
            },
            ProtoProjectile {
                x: 10,
                y: 20,
                pattern: vec![(0, 1)],
            },
        ];

        assert_eq!(projectiles, expected);
    }

    #[test]
    #[should_panic(expected = "cannot read")]
    fn panics_when_file_does_not_exist() {
        load_blueprints("/nonexistent/path/projectiles.ron");
    }

    #[test]
    #[should_panic(expected = "bad RON")]
    fn panics_when_ron_syntax_is_invalid() {
        let ron_content = r#"[(x: 5, y: 10, pattern: [(1, 0)"#; // Missing closing brackets
        let temp_file = create_temp_ron_file(ron_content).unwrap();
        let path = temp_file.path().to_str().unwrap();
        load_blueprints(path);
    }
}
