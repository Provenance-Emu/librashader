#![feature(type_alias_impl_trait)]
#![feature(let_chains)]

mod filter_chain;

use librashader_preprocess::ShaderSource;
use librashader_presets::ShaderPassConfig;
use librashader_reflect::back::targets::HLSL;
use librashader_reflect::back::{CompileShader, FromCompilation};
use librashader_reflect::front::shaderc::GlslangCompilation;
use rustc_hash::FxHashMap;
use std::error::Error;
use std::path::Path;

use librashader_reflect::reflect::semantics::{
    ReflectSemantics, SemanticMap, TextureSemantics, UniformSemantic, VariableSemantics,
};
use librashader_reflect::reflect::ReflectShader;

mod filter_pass;
#[cfg(test)]
mod hello_triangle;
mod texture;
mod util;
mod samplers;
mod render_target;
mod framebuffer;
mod quad_render;

#[cfg(test)]
mod tests {
    use crate::hello_triangle::DXSample;
    use super::*;

    #[test]
    fn triangle_d3d11() {
        let sample = hello_triangle::d3d11_hello_triangle::Sample::new("../test/slang-shaders/crt/crt-royale.slangp").unwrap();
        // let sample = hello_triangle::d3d11_hello_triangle::Sample::new("../test/basic.slangp").unwrap();

        hello_triangle::main(sample).unwrap();

    }
}
