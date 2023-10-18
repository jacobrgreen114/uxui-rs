use std::path::*;
use std::process::Command;

const SHADERS: &[&str] = &[
    "shaders/rect.vert",
    "shaders/rect.frag",
    "shaders/glyph_sdf.vert",
    "shaders/glyph_sdf.frag",
];

fn main() {
    let build_dir = std::env::current_dir().unwrap();
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    let build_file_metadata = build_dir.join("build.rs").metadata().unwrap();

    println!("{:?}", build_file_metadata);

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

        if !output_path.exists()
            || output_path.metadata().unwrap().modified().unwrap()
                < source_path.metadata().unwrap().modified().unwrap()
            || output_path.metadata().unwrap().modified().unwrap()
                < build_file_metadata.modified().unwrap()
        {
            compile_shader(&source_path, &output_path);
        }
    });
}

fn compile_shader(source_path: &Path, output_path: &Path) {
    let mut command = Command::new("glslc");
    // #[cfg(not(debug_assertions))]
    // command.arg("-O");
    command
        .arg("--target-env=vulkan1.3")
        .arg(source_path)
        .arg("-o")
        .arg(output_path);

    command.status().unwrap();
}
