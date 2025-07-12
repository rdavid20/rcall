use rake::KeywordScore;
use rusqlite::{Connection, Result, params};

pub fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CrEaTe ViRtUaL tAbLe If NoT eXiStS images uSiNg fts5(
      path,       -- image path
      keywords    -- keywords
        );",
        [],
    )?;
    Ok(())
}

pub fn insert_image(conn: &Connection, path: &str, keywords: Vec<KeywordScore>) -> Result<()> {
    let keyword_string = keywords
        .iter()
        .map(|ks| ks.keyword.as_str())
        .collect::<Vec<&str>>()
        .join(" ");
    conn.execute(
        "INSERT INTO images (path, keywords) VALUES (?1, ?2);",
        params![path, keyword_string],
    )?;
    Ok(())
}

pub fn search_images(conn: &Connection, query: &str) -> Result<Vec<String>> {
    let mut statement = conn.prepare("SELECT path FROM images WHERE keywords MATCH ?1;")?;
    let rows = statement.query_map([query], |row| row.get(0))?;

    let results = rows.filter_map(Result::ok).collect();
    Ok(results)
}
