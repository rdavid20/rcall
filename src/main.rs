mod database;
mod gui;
mod screenshot;
mod text_processing;

use crate::screenshot::capture_screenshot;
use clap::Parser;
use rusqlite::{Connection, Result};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(
        short = 'c',
        long,
        help = "Capture screenshot that can be saved to the DB"
    )]
    capture: bool,

    #[arg(short = 's',
      long,
      value_name = "KEYWORD",
      num_args = 1..,
      help = "Space separated list of keywords to searh for")]
    search: Option<Vec<String>>,

    #[arg(short = 'g', long, help = "Launch GUI application")]
    gui: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let conn = Connection::open("images.db")?;
    database::create_table(&conn)?;

    if args.capture {
        let path = capture_screenshot()?;
        let path_str = path.to_str().ok_or("Invalid UTF-8 in image path")?;
        let text = text_processing::extract_text_from_image(path_str)?;
        let keywords = text_processing::get_keywords_from_text(&text);

        database::insert_image(&conn, path_str, keywords)?;
    }

    if let Some(keywords) = args.search {
        for kw in keywords {
            println!("\nSearching for keyword: {}", kw);

            let results = database::search_images(&conn, &kw)?;
            for path in results {
                println!("{}", path);
            }
        }
    }

    if args.gui {
        let options = eframe::NativeOptions::default();
        eframe::run_native(
            "RCall GUI",
            options,
            Box::new(|_cc| Box::new(gui::GUI::new(conn))),
        )?;
    }

    Ok(())
}
