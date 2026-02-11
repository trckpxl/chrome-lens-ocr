use std::{fs, path::PathBuf};

use arboard::Clipboard;
use clap::Parser;
use chrome_lens_ocr::LensClient;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the image file
    image_path: String,

    /// Output the text to a file with the same name as the input image
    #[arg(long)]
    text: bool,

    /// Copy the text to the clipboard
    #[arg(long)]
    clip: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let args = Args::parse();
    let image_path = &args.image_path;

    let client = LensClient::new(None);

    match client.process_image_path(image_path, Some("en")).await {
        Ok(result) => {
            if !args.text && !args.clip {
                println!("{}", result.full_text);
            }

            if args.text {
                let mut path = PathBuf::from(image_path);
                path.set_extension("txt");
                fs::write(&path, &result.full_text)?;
                // println!("Text saved to: {:?}", path);
            }

            if args.clip {
                match Clipboard::new() {
                    Ok(mut clipboard) => {
                        if let Err(e) = clipboard.set_text(&result.full_text) {
                            eprintln!("Failed to copy to clipboard: {}", e);
                        }
                    }
                    Err(e) => eprintln!("Failed to initialize clipboard: {}", e),
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
        }
    }

    Ok(())
}
