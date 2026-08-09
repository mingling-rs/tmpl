use std::fs;
use std::path::Path;

pub(crate) fn update_all_mod_files() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let lines = collect_mod_files(&manifest_dir);
    for line in lines {
        println!("cargo:rerun-if-changed={}", line);
    }
}

fn collect_mod_files<P: AsRef<Path>>(project_root: P) -> Vec<String> {
    let mut result = Vec::new();
    collect_mod_files_recursive(project_root.as_ref(), &mut result);
    result
}

fn collect_mod_files_recursive(dir: &Path, result: &mut Vec<String>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    let mut has_mod_rs = false;

    for entry in entries.flatten() {
        let path = entry.path();
        let file_name = entry.file_name();

        if path.is_dir() {
            collect_mod_files_recursive(&path, result);
        } else if file_name == "mod.rs" {
            has_mod_rs = true;
        }
    }

    if !has_mod_rs {
        return;
    }

    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    let mut mod_names: Vec<String> = Vec::new();

    for entry in entries.flatten() {
        let path = entry.path();
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy().to_string();

        if path.is_dir() {
            continue;
        }

        if file_name_str == "lib.rs" || file_name_str == "main.rs" || file_name_str == "mod.rs" {
            continue;
        }

        if !file_name_str.ends_with(".rs") {
            continue;
        }

        let mod_name = file_name_str.trim_end_matches(".rs").to_string();

        if mod_name.starts_with('_') {
            continue;
        }

        mod_names.push(mod_name);
    }

    mod_names.sort();

    let mut mod_declarations = String::new();
    for name in &mod_names {
        mod_declarations.push_str(&format!("pub(crate) mod {};\n", name));
        result.push(format!("pub(crate) mod {};", name));
    }

    let mod_rs_path = dir.join("mod.rs");
    if let Ok(content) = fs::read_to_string(&mod_rs_path) {
        if content != mod_declarations {
            let _ = fs::write(&mod_rs_path, mod_declarations);
        }
    } else {
        let _ = fs::write(&mod_rs_path, mod_declarations);
    }
}
