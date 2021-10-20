mod shader_type;
pub use anyhow;
use anyhow::{Context, Result};
use naga::front::wgsl;
use serde::Deserialize;
pub use shader_type::{scalar_from_naga, type_from_naga};
use std::{fs::File, io::Read, path::Path};
pub const VERTEX_SHADER_MAIN: &str = "vs_main";
pub const FRAGMENT_SHADER_MAIN: &str = "fs_main";

#[derive(Deserialize)]
pub struct Project {
    /// Path relative to project root specififying location of vertex shader
    pub shader_path: String,
}
#[derive(Debug)]
pub struct Options {
    /// print debug output
    pub verbose: bool,
}
/// Intermediate represention of shader. Used to convert to production
/// shading langages such as SPIR-V for Vulkan or glsl for webgl2.
pub struct ShaderIR {
    pub module: naga::Module,
    pub info: naga::valid::ModuleInfo,
}
impl ShaderIR {
    pub fn compile_from_disk<P: AsRef<Path>>(path: P, options: Options) -> Result<Self> {
        let path: &Path = path.as_ref();
        let project_index_path = path.join("project.json");
        if options.verbose {
            if let Some(path_str) = project_index_path.to_str() {
                println!("loading project file at: {}", path_str);
            }
        }
        let file = File::open(project_index_path.clone()).with_context(|| {
            format!(
                "failed to open project file: {}",
                project_index_path.to_str().unwrap()
            )
        })?;
        if options.verbose {
            if let Some(path_str) = project_index_path.to_str() {
                println!("parsing project file at: {}", path_str);
            }
        }
        let project_data: Project =
            serde_json::from_reader(file).with_context(|| "Failed to parse project file")?;

        let mut shader_string = String::new();

        let shader_path = path.join(project_data.shader_path);
        if options.verbose {
            if let Some(path_str) = shader_path.to_str() {
                println!("loading shader at: {}", path_str);
            }
        }

        let mut shader_file = File::open(shader_path.clone()).with_context(|| {
            format!(
                "failed to open shader file {}",
                shader_path.to_str().unwrap()
            )
        })?;
        shader_file
            .read_to_string(&mut shader_string)
            .with_context(|| "failed to read from shader file")?;
        if options.verbose {
            println!("parsing shader string:\n\"\"\"\n{}\n\"\"\"", shader_string);
        }

        let module = wgsl::parse_str(&shader_string).with_context(|| "failed to parse shader")?;
        let mut validator = naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            naga::valid::Capabilities::PUSH_CONSTANT,
        );

        let info = validator.validate(&module)?;
        Ok(Self { module, info })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
