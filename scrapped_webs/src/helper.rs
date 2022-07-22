use strsim::jaro;

pub fn compare_similarity(name1: &str, name2: &str) -> f32 {
    jaro(name1, name2) as f32
}
