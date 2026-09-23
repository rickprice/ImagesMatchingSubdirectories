use clap::Parser;
use rand::seq::SliceRandom;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "images-matching-subdirectories")]
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

const IMAGE_EXTENSIONS: &[&str] = &["jpg", "jpeg", "png", "gif", "bmp", "tiff", "webp"];

fn is_image_extension(ext: &str) -> bool {
    IMAGE_EXTENSIONS.iter().any(|&e| e.eq_ignore_ascii_case(ext))
}

fn collect_images(directory: &Path, subdirectories: &[String]) -> Vec<PathBuf> {
    let mut found_images = Vec::new();

    for subdir_name in subdirectories {
        let subdir_path = directory.join(subdir_name);

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
                    if is_image_extension(ext_str) {
                        found_images.push(path.to_path_buf());
                    }
                }
            }
        }
    }

    found_images
}

fn apply_limit(images: &mut Vec<PathBuf>, limit: usize) {
    if limit < images.len() {
        let mut rng = rand::rng();
        images.shuffle(&mut rng);
        images.truncate(limit);
    }
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

    let mut found_images = collect_images(&args.directory, &args.subdirectories);

    if found_images.is_empty() {
        if !args.names_only {
            println!("No images found in the specified subdirectories.");
        }
    } else {
        let total_found = found_images.len();

        if let Some(limit) = args.limit {
            if limit < total_found {
                apply_limit(&mut found_images, limit);
                if !args.names_only {
                    println!("Found {} image(s), displaying {} random selection(s):", total_found, limit);
                }
            } else if !args.names_only {
                println!("Found {} image(s) (limit {} not applied - showing all):", total_found, limit);
            }
        } else if !args.names_only {
            println!("Found {} image(s):", total_found);
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn make_files(dir: &Path, names: &[&str]) {
        for name in names {
            let path = dir.join(name);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).unwrap();
            }
            fs::write(&path, b"").unwrap();
        }
    }

    fn subdirs(names: &[&str]) -> Vec<String> {
        names.iter().map(|s| s.to_string()).collect()
    }

    // --- is_image_extension ---

    #[test]
    fn image_extensions_lowercase_accepted() {
        for ext in &["jpg", "jpeg", "png", "gif", "bmp", "tiff", "webp"] {
            assert!(is_image_extension(ext), "{ext} should be accepted");
        }
    }

    #[test]
    fn image_extensions_uppercase_accepted() {
        for ext in &["JPG", "JPEG", "PNG", "GIF", "BMP", "TIFF", "WEBP"] {
            assert!(is_image_extension(ext), "{ext} should be accepted");
        }
    }

    #[test]
    fn image_extensions_mixed_case_accepted() {
        assert!(is_image_extension("Jpg"));
        assert!(is_image_extension("pNg"));
    }

    #[test]
    fn non_image_extensions_rejected() {
        for ext in &["txt", "rs", "pdf", "exe", "mp4", "doc", ""] {
            assert!(!is_image_extension(ext), "{ext} should be rejected");
        }
    }

    // --- collect_images ---

    #[test]
    fn finds_images_in_subdirectory() {
        let tmp = TempDir::new().unwrap();
        make_files(tmp.path(), &["photos/a.jpg", "photos/b.png"]);

        let images = collect_images(tmp.path(), &subdirs(&["photos"]));

        assert_eq!(images.len(), 2);
    }

    #[test]
    fn ignores_non_image_files() {
        let tmp = TempDir::new().unwrap();
        make_files(tmp.path(), &["docs/readme.txt", "docs/photo.jpg", "docs/data.csv"]);

        let images = collect_images(tmp.path(), &subdirs(&["docs"]));

        assert_eq!(images.len(), 1);
        assert!(images[0].to_str().unwrap().ends_with("photo.jpg"));
    }

    #[test]
    fn skips_missing_subdirectory_without_panic() {
        let tmp = TempDir::new().unwrap();

        let images = collect_images(tmp.path(), &subdirs(&["nonexistent"]));

        assert!(images.is_empty());
    }

    #[test]
    fn searches_multiple_subdirectories() {
        let tmp = TempDir::new().unwrap();
        make_files(tmp.path(), &["cats/cat.jpg", "dogs/dog.png", "docs/readme.txt"]);

        let images = collect_images(tmp.path(), &subdirs(&["cats", "dogs"]));

        assert_eq!(images.len(), 2);
    }

    #[test]
    fn finds_images_recursively() {
        let tmp = TempDir::new().unwrap();
        make_files(tmp.path(), &["gallery/2023/jan.jpg", "gallery/2024/feb.png"]);

        let images = collect_images(tmp.path(), &subdirs(&["gallery"]));

        assert_eq!(images.len(), 2);
    }

    #[test]
    fn returns_empty_when_subdirectory_has_no_images() {
        let tmp = TempDir::new().unwrap();
        make_files(tmp.path(), &["docs/readme.txt", "docs/notes.md"]);

        let images = collect_images(tmp.path(), &subdirs(&["docs"]));

        assert!(images.is_empty());
    }

    #[test]
    fn accepts_all_supported_extensions() {
        let tmp = TempDir::new().unwrap();
        make_files(tmp.path(), &[
            "img/a.jpg", "img/b.jpeg", "img/c.png", "img/d.gif",
            "img/e.bmp", "img/f.tiff", "img/g.webp",
        ]);

        let images = collect_images(tmp.path(), &subdirs(&["img"]));

        assert_eq!(images.len(), 7);
    }

    #[test]
    fn extension_matching_is_case_insensitive() {
        let tmp = TempDir::new().unwrap();
        make_files(tmp.path(), &["img/A.JPG", "img/B.Png", "img/C.TIFF"]);

        let images = collect_images(tmp.path(), &subdirs(&["img"]));

        assert_eq!(images.len(), 3);
    }

    // --- apply_limit ---

    #[test]
    fn limit_below_count_truncates_to_limit() {
        let mut images: Vec<PathBuf> = (0..10).map(|i| PathBuf::from(format!("{i}.jpg"))).collect();
        apply_limit(&mut images, 3);
        assert_eq!(images.len(), 3);
    }

    #[test]
    fn limit_equal_to_count_keeps_all() {
        let mut images: Vec<PathBuf> = (0..5).map(|i| PathBuf::from(format!("{i}.jpg"))).collect();
        apply_limit(&mut images, 5);
        assert_eq!(images.len(), 5);
    }

    #[test]
    fn limit_above_count_keeps_all() {
        let mut images: Vec<PathBuf> = (0..3).map(|i| PathBuf::from(format!("{i}.jpg"))).collect();
        apply_limit(&mut images, 100);
        assert_eq!(images.len(), 3);
    }

    #[test]
    fn limit_zero_produces_empty_vec() {
        let mut images: Vec<PathBuf> = (0..5).map(|i| PathBuf::from(format!("{i}.jpg"))).collect();
        apply_limit(&mut images, 0);
        assert!(images.is_empty());
    }
}
