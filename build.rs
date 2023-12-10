use std::path::*;
use std::process::Command;

const SHADERS: &[&str] = &[
    "shaders/rect.vert",
    "shaders/rect.frag",
    "shaders/image.vert",
    "shaders/image.frag",
    "shaders/glyph_sdf.vert",
    "shaders/glyph_sdf.frag",
];

fn main() {
    let build_dir = std::env::current_dir().unwrap();
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    let build_file_metadata = build_dir.join("build.rs").metadata().unwrap();

    SHADERS.iter().for_each(|&shader| {
        let source_path = build_dir.join(shader);
        assert!(
            source_path.exists(),
            "Shader file not found: {}",
            source_path.display()
        );
        println!("cargo:rerun-if-changed={}", source_path.display());

        let output_path = out_dir.join(format!("{}.spv", shader));
        if !output_path.parent().unwrap().exists() {
            std::fs::create_dir_all(output_path.parent().unwrap()).unwrap();
        }

        let dep_path = out_dir.join(format!("{}.d", shader));

        if !output_path.exists()
            || output_path.metadata().unwrap().modified().unwrap()
                < source_path.metadata().unwrap().modified().unwrap()
            || output_path.metadata().unwrap().modified().unwrap()
                < build_file_metadata.modified().unwrap()
        {
            compile_shader(&source_path, &output_path, &dep_path);
        }

        // let dependencies = get_depenencies(&dep_path);
        // println!("{:#?}", dependencies);
    });
}

// fn get_depenencies(dep_path: &Path) -> Vec<PathBuf> {
//     let mut file = std::fs::File::open(dep_path).unwrap();
//     let mut string = String::new();
//     file.read_to_string(&mut string).unwrap();
//
//     // string
//     //     .split(":")
//     //     .skip(1)
//     //     .map(|s| s.split(' '))
//     //     .map(|s| s.collect())
//     //     .collect()
//
//     string.
// }

fn compile_shader(source_path: &Path, output_path: &Path, dep_path: &Path) {
    let mut command = Command::new("glslc");
    // #[cfg(not(debug_assertions))]
    // command.arg("-O");
    command
        .arg("-MD")
        .arg("-MF")
        .arg(dep_path)
        .arg("--target-env=vulkan1.3")
        .arg(source_path)
        .arg("-o")
        .arg(output_path);

    command.status().unwrap();
}
