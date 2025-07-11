mod ocr;
mod screenshot;
use std::env;

use crate::ocr::extract_text_from_image;
use crate::screenshot::capture_screenshot;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"--capture".to_string()) {
        println!("Capture triggered");
        let path = capture_screenshot()?;

        let path_str = path.to_str().ok_or("Invalid UTF-8 in image path")?;
        println!("Screenshot saved to {:?}", path_str);

        let text = extract_text_from_image(path_str)?;
        println!("Extracted: {:?}", text);
    }

    Ok(())
}
