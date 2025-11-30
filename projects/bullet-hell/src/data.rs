use crate::game::ProtoProjectile;

pub fn load_blueprints(path: &str) -> Vec<ProtoProjectile> {
    let txt = std::fs::read_to_string(path)
        .unwrap_or_else(|e| panic!("cannot read {}: {}", path, e));
    ron::from_str(&txt).expect("bad RON")
}