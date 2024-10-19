// write a function convert float to usd format
// add $ in the beginning
pub fn to_usd_format(f: f64) -> String {
    format!("${:.2}", f)
}
