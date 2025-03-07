use image::codecs::png::{PngEncoder, CompressionType, FilterType};
use image::{ImageEncoder, ImageFormat};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use tauri::{command, App, Runtime};

#[command]
fn lossless_compress(input_path: &str, output_path: &str) -> Result<(), String> {
  // Read image
  let img = match image::open(input_path) {
      Ok(img) => img,
      Err(e) => return Err(e.to_string()),
  };

  // Create output file
  let output_file = match File::create(output_path) {
    Ok(file) => file,
    Err(e) => return Err(e.to_string()),
  };
  let writer = BufWriter::new(output_file);

    // Encode as PNG with lossless compression
    let encoder = PngEncoder::new_with_quality(writer, CompressionType::Best, FilterType::NoFilter);
    if let Err(e) = encoder.write_image(
        &img.to_rgba8(),
        img.width(),
        img.height(),
        image::ColorType::Rgba8,
    ) {
        return Err(e.to_string());
    }

  Ok(())
}

pub fn run<R: Runtime>(app: &mut App<R>) {
    app.manage(State::default());
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![lossless_compress])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[derive(Default)]
struct State {}
