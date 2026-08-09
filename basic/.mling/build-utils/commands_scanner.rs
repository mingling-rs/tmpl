use std::fs;
use std::path::Path;

pub(crate) fn update_commands_setup() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let command_dir = Path::new(&manifest_dir).join("src").join("command");
    let setup_path = Path::new(&manifest_dir)
        .join("src")
        .join("setup")
        .join("commands_setup.rs");

    println!("cargo:rerun-if-changed={}", command_dir.display());

    let command_names = collect_command_names(&command_dir);

    let mut dispatchers = String::new();
    for name in &command_names {
        dispatchers.push_str(&format!(
            "    p.with_dispatcher(crate::command::{name}::CMD{});\n",
            pascal_case(name)
        ));
    }

    let content = format!(
        "use mingling::{{Program, macros::program_setup}};\n\
         \n\
         #[allow(unused)]\n\
         #[program_setup]\n\
         pub fn commands_setup(p: &mut Program<crate::ThisProgram>) {{\n\
         {dispatchers}\
         }}\n"
    );

    write_if_changed(&setup_path, &content);
}

fn collect_command_names(dir: &Path) -> Vec<String> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut names = Vec::new();
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.starts_with('_') {
            continue;
        }
        if entry.path().is_dir() {
            names.push(name);
        } else if name.ends_with(".rs") && name != "mod.rs" {
            names.push(name.trim_end_matches(".rs").to_string());
        }
    }
    names.sort();
    names
}

fn pascal_case(input: &str) -> String {
    let mut result = String::new();
    for part in input.split(['_', '-']) {
        let mut chars = part.chars();
        if let Some(first) = chars.next() {
            result.push_str(&first.to_uppercase().to_string());
            result.push_str(chars.as_str());
        }
    }
    result
}

fn write_if_changed(path: &Path, content: &str) {
    if let Ok(existing) = fs::read_to_string(path) {
        if existing == content {
            return;
        }
    }
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(path, content);
}
