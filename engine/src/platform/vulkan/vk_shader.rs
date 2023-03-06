use std::{fs::read_to_string, sync::Arc};
use anyhow::Result;
use shaderc::{CompileOptions, Compiler, ShaderKind};
use spirv_reflect::types::ReflectFormat;
use vulkano::{
    shader::{
        spirv::{Capability, ExecutionModel},
        EntryPointInfo, ShaderExecution, ShaderInterface, ShaderInterfaceEntry,
        ShaderInterfaceEntryType, ShaderModule, ShaderScalarType,
    },
    Version,
};
use super::vk_core::VkCore;

pub struct VkShader {
    module: Arc<ShaderModule>,
}

impl VkShader {
    pub fn new(source: &str, core: &VkCore) -> Result<VkShader> {
        let (kind, spv) = Self::compile_shader(source)?;

        let entry_point = Self::query_shader_reflection(spv.as_binary_u8(), &kind);

        let module = unsafe {
            ShaderModule::from_bytes_with_data(
                core.device().clone(),
                &spv.as_binary_u8(),
                Version::V1_1,
                [&Capability::Shader],
                [],
                [(
                    "main".to_owned(),
                    match entry_point.execution {
                        ShaderExecution::Vertex => ExecutionModel::Vertex,
                        ShaderExecution::TessellationControl => ExecutionModel::TessellationControl,
                        ShaderExecution::TessellationEvaluation => ExecutionModel::TessellationEvaluation,
                        ShaderExecution::Geometry(_) => ExecutionModel::Geometry,
                        ShaderExecution::Fragment => ExecutionModel::Fragment,
                        ShaderExecution::Compute => panic!("Compute shaders are not supported"),
                        ShaderExecution::RayGeneration => ExecutionModel::RayGenerationKHR,
                        ShaderExecution::AnyHit => ExecutionModel::AnyHitKHR,
                        ShaderExecution::ClosestHit => ExecutionModel::ClosestHitKHR,
                        ShaderExecution::Miss => ExecutionModel::MissKHR,
                        ShaderExecution::Intersection => ExecutionModel::IntersectionKHR,
                        ShaderExecution::Callable => ExecutionModel::CallableKHR,
                    },
                    entry_point,
                )],
            )
        }?;

        Ok(VkShader { module })
    }

    fn query_shader_reflection(spv: &[u8], kind: &ShaderKind) -> EntryPointInfo {
        let reflection_module = spirv_reflect::ShaderModule::load_u8_data(spv)
            .expect("Failed to get shader reflection");

        let shader_input = unsafe {
            ShaderInterface::new_unchecked(
                reflection_module
                    .enumerate_input_variables(None)
                    .expect("Failed to get input variables")
                    .iter()
                    .map(|inuput| ShaderInterfaceEntry {
                        location: inuput.location,
                        component: 0,
                        name: Some(inuput.name.clone().into()),
                        ty: match inuput.format {
                            ReflectFormat::R32G32_SFLOAT => ShaderInterfaceEntryType {
                                base_type: ShaderScalarType::Float,
                                num_components: 2,
                                num_elements: 1,
                                is_64bit: false,
                            },
                            _ => panic!("Unsupported shader input format"),
                        },
                    })
                    .collect(),
            )
        };

        EntryPointInfo {
            execution: match kind {
                ShaderKind::Vertex => ShaderExecution::Vertex,
                ShaderKind::Fragment => ShaderExecution::Fragment,
                _ => panic!("Unsupported shader type"),
            },
            descriptor_requirements: [].into_iter().collect(),
            push_constant_requirements: None,
            specialization_constant_requirements: [].into_iter().collect(),
            input_interface: shader_input,
            output_interface: unsafe { ShaderInterface::new_unchecked(Vec::new()) },
        }
    }

    fn compile_shader(source: &str) -> Result<(ShaderKind, shaderc::CompilationArtifact)> {
        let kind = match source.split('.').last().unwrap() {
            "vert" => ShaderKind::Vertex,
            "frag" => ShaderKind::Fragment,
            _ => panic!("Unsupported shader type"),
        };

        let code = read_to_string(source)?;

        let compiler = Compiler::new().unwrap();
        
        let mut options = CompileOptions::new().unwrap();
        options.add_macro_definition("EP", Some("main"));
        
        let spv = compiler.compile_into_spirv(
            code.as_str(),
            kind,
            source,
            "main",
            Some(&options),
        )?;

        Ok((kind, spv))
    }
}
