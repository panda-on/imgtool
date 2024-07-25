use std::env;
use std::path::Path;
use chrono::Local;

fn main() {
    let image_path = env::args().skip(1).next().unwrap();
    let path = Path::new(&image_path);
    let img = image::open(path).unwrap();
    let rotated = img.rotate90();
    let ext_idx = image_path.rfind(".").unwrap();
    let ts = Local::now();
    let new_path_str = format!(
        "{}_{}{}",
        &image_path[0..ext_idx],
        ts.format("%Y%m%d_%H%M%S%3f"),
        &image_path[ext_idx..image_path.len()]
    );
    let new_path = Path::new(&new_path_str);
    println!("new path: {}", new_path_str);
    rotated.save(new_path).unwrap();
}
