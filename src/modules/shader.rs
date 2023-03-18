use std::{
    fs::read_to_string,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};


pub struct Shader<'a> {
    path: &'a str,
    last_hash: u64,
    shader_source: wgpu::ShaderSource<'a>,
}

fn hash_file(file: &String) -> u64 {
    let mut s = DefaultHasher::new();
    file.hash(&mut s);
    s.finish()
}

impl<'a> Shader<'a> {
    pub fn new(path: &'a str) -> Self {
        let shader = read_to_string(path).unwrap();
        let last_hash = hash_file(&shader);
        let shader_source = wgpu::ShaderSource::Wgsl(shader.into());

        Shader { 
            path, 
            last_hash, 
            shader_source
        }
    }

    /// possibly computation heavy
    pub fn get_source(&mut self) -> wgpu::ShaderSource {
        let shader = read_to_string(self.path).unwrap();
        let hash = hash_file(&shader);

        if self.last_hash != hash {
            self.shader_source = wgpu::ShaderSource::Wgsl(shader.into());
        }

        self.shader_source.clone()
    }
}
