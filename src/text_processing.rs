use rake::*;
use tesseract::Tesseract;

pub fn extract_text_from_image(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut tess = Tesseract::new(None, Some("eng"))?;
    tess = tess.set_image(path)?;
    let mut text = tess.get_text()?;
    text = clean_text(&text);
    Ok(text)
}

pub fn get_keywords_from_text(text: &String) -> Vec<KeywordScore> {
    let stop_words_list_path = "SmartStoplist.txt";
    let sw = StopWords::from_file(stop_words_list_path).unwrap();
    let r = Rake::new(sw);
    let keywords = r.run(&text);
    keywords
}

fn clean_text(text: &String) -> String {
    let replaced = text
        .replace('\n', " ")
        .replace('\r', " ")
        .replace('\t', " ");

    let cleaned = replaced.split_whitespace().collect::<Vec<&str>>().join(" ");
    cleaned
}
