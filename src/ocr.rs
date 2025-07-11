use tesseract::Tesseract;

pub fn extract_text_from_image(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut tess = Tesseract::new(None, Some("eng"))?;
    tess = tess.set_image(path)?;
    let text = tess.get_text()?;
    Ok(text)
}
