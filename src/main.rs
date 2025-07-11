mod screenshot;
mod text_processing;
use rake::*;
use std::env;

use crate::screenshot::capture_screenshot;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.contains(&"--capture".to_string()) {
        println!("Capture triggered");
        let path = capture_screenshot()?;

        let path_str = path.to_str().ok_or("Invalid UTF-8 in image path")?;
        println!("Screenshot saved to {:?}", path_str);

        let text = text_processing::extract_text_from_image(path_str)?;

        let keywords = text_processing::get_keywords_from_text(&text);

        keywords.iter().for_each(
            |&KeywordScore {
                 ref keyword,
                 ref score,
             }| println!("{}: {}", keyword, score),
        );
    }

    Ok(())
}
