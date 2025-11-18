use crate::geometry::{Vec3, noise};

#[derive(Clone)]
pub enum TextureType {
    AnimatedFire,
    NetherPortal,
    MinecraftStone,
    MinecraftGlowstone,
    MinecraftObsidian,
}

#[derive(Clone, Copy)]
pub enum TextureQuality { High, Medium, Low }

#[derive(Clone)]
pub struct Texture {
    pub texture_type: TextureType,
}

impl Texture {
    pub fn animated_fire() -> Self {
        Texture {
            texture_type: TextureType::AnimatedFire,
        }
    }
    
    pub fn nether_portal() -> Self {
        Texture {
            texture_type: TextureType::NetherPortal,
        }
    }
    
    pub fn minecraft_stone() -> Self {
        Texture {
            texture_type: TextureType::MinecraftStone,
        }
    }
    
    pub fn minecraft_glowstone() -> Self {
        Texture {
            texture_type: TextureType::MinecraftGlowstone,
        }
    }
    
    pub fn minecraft_obsidian() -> Self {
        Texture {
            texture_type: TextureType::MinecraftObsidian,
        }
    }
    
    pub fn sample_quality(&self, u: f32, v: f32, time: f32, quality: TextureQuality) -> Vec3 {
        match &self.texture_type {
            TextureType::AnimatedFire => {
                let intensity = match quality {
                    TextureQuality::High => {
                        let f1 = (u * 3.0 + time * 8.0).sin() * 0.5 + 0.5;
                        let f2 = (v * 4.0 - time * 12.0).sin() * 0.5 + 0.5;
                        let f3 = ((u + v) * 6.0 + time * 10.0).cos() * 0.5 + 0.5;
                        (f1 * f2 * f3).powf(0.5)
                    },
                    TextureQuality::Medium => {
                        let f1 = (u * 2.5 + time * 6.0).sin() * 0.5 + 0.5;
                        let f2 = (v * 3.0 - time * 9.0).sin() * 0.5 + 0.5;
                        (f1 * f2).powf(0.5)
                    },
                    TextureQuality::Low => {
                        ((u * 3.0 + v * 3.0 + time * 6.0).sin() * 0.5 + 0.5).powf(0.5)
                    },
                };
                let heat = (intensity * 2.0 - v).clamp(0.0, 1.0);
                
                let ember = Vec3::new(0.1, 0.05, 0.0);
                let orange = Vec3::new(1.0, 0.3, 0.05);
                let yellow = Vec3::new(1.0, 0.8, 0.1);
                let white = Vec3::new(1.0, 0.95, 0.8);
                
                if heat < 0.3 {
                    ember.interpolate(orange, heat / 0.3)
                } else if heat < 0.7 {
                    orange.interpolate(yellow, (heat - 0.3) / 0.4)
                } else {
                    yellow.interpolate(white, (heat - 0.7) / 0.3)
                }
            },
            
            TextureType::NetherPortal => {
                let (energy, swirl) = match quality {
                    TextureQuality::High => {
                        let p1 = (u * 2.0 + time * 3.0).sin() * 0.5 + 0.5;
                        let p2 = (v * 3.0 - time * 4.0).cos() * 0.5 + 0.5;
                        let p3 = ((u + v) * 1.5 + time * 5.0).sin() * 0.5 + 0.5;
                        let energy = (p1 * p2 + p3) * 0.5;
                        let swirl = ((u - 0.5).atan2(v - 0.5) + time * 2.0).sin() * 0.5 + 0.5;
                        (energy, swirl)
                    },
                    TextureQuality::Medium => {
                        let p1 = (u * 1.5 + time * 2.0).sin() * 0.5 + 0.5;
                        let p2 = (v * 2.0 - time * 2.5).cos() * 0.5 + 0.5;
                        ((p1 + p2) * 0.5, 0.5)
                    },
                    TextureQuality::Low => {
                        (((u + v) * 1.2 + time * 1.5).sin() * 0.5 + 0.5, 0.5)
                    },
                };
                
                let purple = Vec3::new(0.4, 0.1, 0.8);
                let magenta = Vec3::new(0.8, 0.2, 0.6);
                let white = Vec3::new(0.9, 0.8, 1.0);
                
                let base = purple.interpolate(magenta, energy);
                base.interpolate(white, swirl * 0.4)
            },
            
            TextureType::MinecraftStone => {
                let combined = match quality {
                    TextureQuality::High => {
                        let n1 = noise(Vec3::new(u * 8.0, v * 8.0, 0.0));
                        let n2 = noise(Vec3::new(u * 16.0, v * 16.0, 0.0));
                        (n1 + n2 * 0.5) / 1.5
                    },
                    TextureQuality::Medium => noise(Vec3::new(u * 8.0, v * 8.0, 0.0)),
                    TextureQuality::Low => 0.4,
                };
                
                let stone_dark = Vec3::new(0.4, 0.4, 0.4);
                let stone_light = Vec3::new(0.7, 0.7, 0.7);
                
                stone_dark.interpolate(stone_light, combined)
            },
            
            TextureType::MinecraftGlowstone => {
                let intensity = match quality {
                    TextureQuality::High => {
                        let g1 = (u * 8.0 + time * 2.0).sin() * 0.5 + 0.5;
                        let g2 = (v * 8.0 + time * 1.5).cos() * 0.5 + 0.5;
                        let pulse = (time * 4.0).sin() * 0.1 + 0.9;
                        (g1 * g2 * pulse).clamp(0.0, 1.0)
                    },
                    TextureQuality::Medium => ((u * 6.0 + v * 6.0 + time * 2.0).sin() * 0.5 + 0.5).clamp(0.0, 1.0),
                    TextureQuality::Low => 0.7,
                };
                
                let glow_dim = Vec3::new(0.8, 0.6, 0.2);
                let glow_bright = Vec3::new(1.0, 0.9, 0.5);
                
                glow_dim.interpolate(glow_bright, intensity)
            },
            
            TextureType::MinecraftObsidian => {
                let noise_val = if let TextureQuality::Low = quality { 0.2 } else { noise(Vec3::new(u * 12.0, v * 12.0, 0.0)) };
                let reflection = if let TextureQuality::High = quality { ((u + v) * 16.0).sin() * 0.5 + 0.5 } else { 0.3 };
                
                let obsidian_base = Vec3::new(0.05, 0.02, 0.1);
                let obsidian_highlight = Vec3::new(0.2, 0.1, 0.3);
                
                let base = obsidian_base.interpolate(obsidian_highlight, noise_val);
                base.interpolate(obsidian_highlight, reflection * 0.3)
            },
        }
    }
}
