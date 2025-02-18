use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

pub fn main() -> io::Result<()> {
    println!("cargo:rustc-env=TARGET={}", std::env::var("TARGET").unwrap());
    // Determine the stubs directory and output path
    let stubs_dir = Path::new("stubs");

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR environment variable not set");
    let output_file = Path::new(&out_dir).join("stubs_map.rs");

    // Ensure the stubs directory exists
    if !stubs_dir.exists() {
        panic!("Stubs directory does not exist: {:?}", stubs_dir);
    }

    // Collect all PHP stub files
    let mut stubs_map = Vec::new();
    collect_files(stubs_dir, stubs_dir, &mut stubs_map)?;

    // Prepare the map content
    let map_content = stubs_map
        .into_iter()
        .map(|(simplified_path, include_path)| {
            format!(r##"    (r#"@{simplified_path}"#, include_str!("{include_path}"))"##)
        })
        .collect::<Vec<_>>();
    let count = map_content.len();

    // Write to the map.inc file
    let mut file = File::create(output_file)?;

    writeln!(file, "// This file is generated by the build script")?;
    writeln!(file, "// Do not modify this file manually")?;
    writeln!(file)?;
    writeln!(file, "pub static PHP_STUBS: [(&str, &str); {}] = [", count)?;
    writeln!(file, "{}", map_content.join(",\n"))?;
    writeln!(file, "];")?;

    Ok(())
}

fn collect_files(root: &Path, dir: &Path, stubs_map: &mut Vec<(String, String)>) -> io::Result<()> {
    let file_separator = if cfg!(target_os = "windows") { "\\" } else { "/" };

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Recursively collect files from subdirectories
            collect_files(root, &path, stubs_map)?;
        } else if let Some(ext) = path.extension() {
            if ext == "php" {
                // Simplify the path
                let relative_path = path.strip_prefix(root).unwrap();
                let simplified_path = relative_path
                    .components()
                    .map(|component| {
                        let part = component.as_os_str().to_string_lossy().to_lowercase();
                        part.replace(" ", "-")
                    })
                    .collect::<Vec<_>>()
                    .join(file_separator);

                // Prepare absolute path for include_str
                let include_path = fs::canonicalize(&path)?.to_string_lossy().replace("\\", "/");

                // Add to the map
                stubs_map.push((format!("stubs{file_separator}{simplified_path}"), include_path));
            }
        }
    }
    Ok(())
}
