use yaml_rust::{YamlLoader, Yaml};
use glob::glob;
use std::fs;
use std::path::{PathBuf, Path};
use std::env;
use std::process::Command;

fn yaml_to_markdown(yaml: &Yaml, indent: usize) -> String {
    let mut result = String::new();
    let indent_str = "    ".repeat(indent);

    match yaml {
        Yaml::Null => result.push_str("null\n"),
        Yaml::Boolean(b) => result.push_str(&format!("{}\n", b)),
        Yaml::Integer(i) => result.push_str(&format!("{}\n", i)),
        Yaml::Real(r) => result.push_str(&format!("{}\n", r)),
        Yaml::String(s) => {
            let lines: Vec<&str> = s.lines().collect();
            for line in lines {
                result.push_str(&format!("{}{}\n", indent_str, line));
            }
        }
        Yaml::Array(arr) => {
            for item in arr {
                result.push_str(&yaml_to_markdown(item, indent));
            }
        }
        Yaml::Hash(h) => {
            for (key, val) in h {
                let key_str = match key {
                    Yaml::String(s) => s.clone(),
                    _ => format!("{:?}", key),
                };
                let heading_level = if indent < 6 { indent + 1 } else { 6 };
                let heading_prefix = "#".repeat(heading_level);
                result.push_str(&format!("\n{} {}{}\n\n", heading_prefix, key_str, "  "));
                result.push_str(&yaml_to_markdown(val, indent + 1));
            }
        }
        Yaml::Alias(_) => result.push_str("Alias\n"),
        Yaml::BadValue => result.push_str("Bad Value\n"),
    }

    result
}

// Function to attempt recovery from parsing errors
fn recover_yaml_content(yaml_content: &str) -> Option<Yaml> {
    let mut lines = yaml_content.lines();
    let mut content = String::new();

    while let Some(line) = lines.next() {
        content.push_str(line);
        content.push('\n');

        // Attempt to parse the current content
        if let Ok(docs) = YamlLoader::load_from_str(&content) {
            if !docs.is_empty() {
                // Return the first document
                return Some(docs[0].clone());
            }
        }
    }

    None
}

fn main() {
    // Get the current directory (should be /preprocess)
    let current_dir: PathBuf = env::current_dir().expect("Failed to get current directory");
    // Get the repository root directory (parent of /preprocess)
    let repo_root: &Path = current_dir.parent().expect("Failed to get parent directory");

    // Construct the path to the contracts directory
    let contracts_dir: PathBuf = repo_root.join("contracts");
    // Construct the glob pattern for YAML files
    let pattern: String = format!("{}/**/*.yaml", contracts_dir.display());
    println!("Using glob pattern: {}", pattern);

    let entries: glob::Paths = glob(&pattern).expect("Failed to read glob pattern");
    let mut file_count = 0;

    for entry in entries {
        match entry {
            Ok(path) => {
                println!("Processing file: {}", path.display());
                file_count += 1;

                match fs::read_to_string(&path) {
                    Ok(yaml_content) => {
                        // Attempt to parse the YAML content
                        match YamlLoader::load_from_str(&yaml_content) {
                            Ok(docs) => {
                                for doc in docs {
                                    process_yaml_doc(&doc, &path, &contracts_dir, &repo_root);
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to parse YAML in {}: {}", path.display(), e);

                                // Attempt to recover the content
                                if let Some(doc) = recover_yaml_content(&yaml_content) {
                                    println!("Recovered content from {}", path.display());
                                    process_yaml_doc(&doc, &path, &contracts_dir, &repo_root);
                                } else {
                                    eprintln!("Could not recover content from {}", path.display());
                                }
                            }
                        }
                    }
                    Err(e) => eprintln!("Failed to read {}: {}", path.display(), e),
                }
            }
            Err(e) => eprintln!("Error reading file: {}", e),
        }
    }

    if file_count == 0 {
        println!("No files were processed. Please check the glob pattern and file locations.");
    } else {
        println!("Processed {} file(s).", file_count);
    }
}

fn process_yaml_doc(doc: &Yaml, path: &Path, contracts_dir: &Path, repo_root: &Path) {
    // Extract title from info.title
    let title = extract_title(doc);
    // Extract tags from tags
    let tags = extract_tags(doc);
    // Get last commit date of the input YAML file
    let date = get_last_commit_date(repo_root, path);

    // Build the front matter
    let mut front_matter = String::from("---\n");
    if let Some(title) = title {
        front_matter.push_str(&format!("title: {}\n", title));
    } else {
        front_matter.push_str("title: Untitled\n");
    }
    if let Some(tags) = tags {
        front_matter.push_str("tags:\n");
        for tag in tags {
            front_matter.push_str(&format!("  - {}\n", tag));
        }
    }
    if let Some(date) = date {
        front_matter.push_str(&format!("date: {}\n", date));
    } else {
        front_matter.push_str("date: Unknown\n");
    }
    front_matter.push_str("---\n\n");

    // Generate the markdown content
    let markdown_content = yaml_to_markdown(doc, 0);

    // Combine front matter and markdown content
    let full_content = format!("{}{}", front_matter, markdown_content);

    // Get relative path from contracts_dir
    let relative_path = path.strip_prefix(contracts_dir).unwrap();

    // Strip the extension from the relative path
    let relative_path_no_ext = relative_path.with_extension("");

    // Create a unique filename by replacing directory separators
    let file_stem = relative_path_no_ext.to_string_lossy()
        .replace("/", "_")
        .replace("\\", "_")
        .replace(".", "_"); // Replace dots to avoid issues

    let markdown_filename = format!("{}.md", file_stem);

    // Build the output path under /content/posts
    let output_dir = repo_root.join("content/posts");
    let markdown_path = output_dir.join(markdown_filename);

    // Ensure the output directory exists
    fs::create_dir_all(&output_dir).expect("Failed to create output directory");

    // Write the full content to the output file
    fs::write(&markdown_path, full_content).expect("Failed to write markdown file");

    println!("Converted {} to {}", path.display(), markdown_path.display());
}


fn extract_title(doc: &Yaml) -> Option<String> {
    if let Yaml::Hash(ref h) = doc {
        if let Some(info) = h.get(&Yaml::String("info".to_string())) {
            if let Yaml::Hash(ref info_h) = info {
                if let Some(title) = info_h.get(&Yaml::String("title".to_string())) {
                    if let Yaml::String(ref title_str) = title {
                        return Some(title_str.clone());
                    }
                }
            }
        }
    }
    None
}

fn extract_tags(doc: &Yaml) -> Option<Vec<String>> {
    if let Yaml::Hash(ref h) = doc {
        if let Some(tags) = h.get(&Yaml::String("tags".to_string())) {
            if let Yaml::Array(ref tag_array) = tags {
                let mut tags_vec = Vec::new();
                for tag in tag_array {
                    if let Yaml::String(ref tag_str) = tag {
                        tags_vec.push(tag_str.clone());
                    }
                }
                return Some(tags_vec);
            }
        }
    }
    None
}

fn get_last_commit_date(repo_root: &Path, file_path: &Path) -> Option<String> {
    let relative_path = file_path.strip_prefix(repo_root).ok()?;
    let output = Command::new("git")
        .args(&["log", "-1", "--format=%cs", "--", &relative_path.to_string_lossy()])
        .current_dir(repo_root)
        .output()
        .ok()?;

    if output.status.success() {
        let date = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Some(date)
    } else {
        None
    }
}