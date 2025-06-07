use clap::Parser;
use rand::seq::SliceRandom;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "image-finder")]
#[command(about = "Find images in specified subdirectories")]
struct Args {
    #[arg(help = "Directory to search in")]
    directory: PathBuf,
    
    #[arg(help = "Subdirectory names to search in")]
    subdirectories: Vec<String>,
    
    #[arg(short = 'l', long = "limit", help = "Maximum number of images to display (random selection)")]
    limit: Option<usize>,
    
    #[arg(short = 'n', long = "names-only", help = "Print only image names separated by spaces")]
    names_only: bool,
}

fn main() {
    let args = Args::parse();
    
    if !args.directory.exists() {
        eprintln!("Error: Directory '{}' does not exist", args.directory.display());
        std::process::exit(1);
    }
    
    if args.subdirectories.is_empty() {
        eprintln!("Error: Please provide at least one subdirectory to search in");
        std::process::exit(1);
    }
    
    let image_extensions = ["jpg", "jpeg", "png", "gif", "bmp", "tiff", "webp", "svg"];
    let mut found_images = Vec::new();
    
    for subdir_name in &args.subdirectories {
        let subdir_path = args.directory.join(subdir_name);
        
        if !subdir_path.exists() {
            eprintln!("Warning: Subdirectory '{}' does not exist", subdir_path.display());
            continue;
        }
        
        if !subdir_path.is_dir() {
            eprintln!("Warning: '{}' is not a directory", subdir_path.display());
            continue;
        }
        
        for entry in WalkDir::new(&subdir_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            
            if let Some(extension) = path.extension() {
                if let Some(ext_str) = extension.to_str() {
                    if image_extensions.iter().any(|&ext| ext.eq_ignore_ascii_case(ext_str)) {
                        found_images.push(path.to_path_buf());
                    }
                }
            }
        }
    }
    
    if found_images.is_empty() {
        if !args.names_only {
            println!("No images found in the specified subdirectories.");
        }
    } else {
        let total_found = found_images.len();
        
        // Randomize and limit the images if requested
        if let Some(limit) = args.limit {
            if limit < total_found {
                let mut rng = rand::thread_rng();
                found_images.shuffle(&mut rng);
                found_images.truncate(limit);
                if !args.names_only {
                    println!("Found {} image(s), displaying {} random selection(s):", total_found, limit);
                }
            } else {
                if !args.names_only {
                    println!("Found {} image(s) (limit {} not applied - showing all):", total_found, limit);
                }
            }
        } else {
            if !args.names_only {
                println!("Found {} image(s):", total_found);
            }
        }
        
        if args.names_only {
            let paths: Vec<String> = found_images
                .iter()
                .map(|path| path.display().to_string())
                .collect();
            println!("{}", paths.join(" "));
        } else {
            for image in found_images {
                println!("  {}", image.display());
            }
        }
    }
}
