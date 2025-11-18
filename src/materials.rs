use crate::geometry::Vec3;
use crate::texture::Texture;

#[derive(Clone)]
pub struct Material {
    pub albedo: Vec3,
    pub specular: f32,
    pub transparency: f32,
    pub reflectivity: f32,
    pub refraction_index: f32,
    pub texture: Option<Texture>,
    pub emissive: Vec3,
    pub roughness: f32,
}

impl Material {
    pub fn new() -> Self {
        Material {
            albedo: Vec3::new(0.8, 0.8, 0.8),
            specular: 0.1,
            transparency: 0.0,
            reflectivity: 0.1,
            refraction_index: 1.0,
            texture: None,
            emissive: Vec3::zero(),
            roughness: 0.5,
        }
    }
    
    pub fn emissive(color: Vec3, intensity: f32) -> Self {
        Material {
            albedo: color,
            specular: 0.0,
            transparency: 0.0,
            reflectivity: 0.0,
            refraction_index: 1.0,
            texture: None,
            emissive: color * intensity,
            roughness: 1.0,
        }
    }
    
    pub fn with_texture(mut self, texture: Texture) -> Self {
        self.texture = Some(texture);
        self
    }
    
    pub fn with_emissive(mut self, emissive: Vec3) -> Self {
        self.emissive = emissive;
        self
    }
    
    pub fn with_properties(mut self, albedo: Vec3, specular: f32, transparency: f32, reflectivity: f32) -> Self {
        self.albedo = albedo;
        self.specular = specular;
        self.transparency = transparency;
        self.reflectivity = reflectivity;
        self
    }
    
    pub fn sample_texture_quality(&self, uv: (f32, f32), time: f32, quality: crate::texture::TextureQuality) -> Vec3 {
        if let Some(ref texture) = self.texture {
            texture.sample_quality(uv.0, uv.1, time, quality)
        } else {
            self.albedo
        }
    }
    
    pub fn is_emissive(&self) -> bool {
        self.emissive.length_squared() > 0.001
    }
    
    pub fn is_transparent(&self) -> bool {
        self.transparency > 0.001
    }
    
    pub fn is_reflective(&self) -> bool {
        self.reflectivity > 0.001
    }
}