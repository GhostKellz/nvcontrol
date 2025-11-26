use crate::overclocking::OverclockProfile;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuModelSpec {
    pub model_name: String,
    pub architecture: String,
    pub cuda_cores: u32,
    pub memory_bus_width: u32,
    pub memory_type: String,
    pub default_tdp: u32,        // Watts
    pub max_tdp: u32,            // Watts
    pub default_temp_limit: u8,  // Celsius
    pub max_temp_limit: u8,      // Celsius
    pub safe_gpu_offset: i32,    // MHz
    pub safe_memory_offset: i32, // MHz
    pub boost_clock: u32,        // MHz
}

/// Get GPU specifications based on model name
pub fn get_gpu_spec(model_name: &str) -> Option<GpuModelSpec> {
    let model_lower = model_name.to_lowercase();

    // RTX 50 Series (Blackwell)
    if model_lower.contains("rtx 5090") {
        // Check for ASUS ROG Astral variant
        let (tdp, max_tdp, boost, safe_offset) = if model_lower.contains("asus") || model_lower.contains("rog") || model_lower.contains("astral") {
            // ASUS ROG Astral OC Edition: Higher clocks, beefier 4-fan cooling
            (600, 630, 2610, 175) // Factory OC'd
        } else {
            // Reference 5090
            (575, 600, 2580, 150)
        };

        return Some(GpuModelSpec {
            model_name: if model_lower.contains("asus") || model_lower.contains("astral") {
                "RTX 5090 (ASUS ROG Astral)".to_string()
            } else {
                "RTX 5090".to_string()
            },
            architecture: "Blackwell".to_string(),
            cuda_cores: 21760,
            memory_bus_width: 512,
            memory_type: "GDDR7".to_string(),
            default_tdp: tdp,
            max_tdp: max_tdp,
            default_temp_limit: 90,
            max_temp_limit: 92,
            safe_gpu_offset: safe_offset,
            safe_memory_offset: 1500, // GDDR7 can handle more
            boost_clock: boost,
        });
    } else if model_lower.contains("rtx 5080") {
        return Some(GpuModelSpec {
            model_name: "RTX 5080".to_string(),
            architecture: "Blackwell".to_string(),
            cuda_cores: 10752,
            memory_bus_width: 256,
            memory_type: "GDDR7".to_string(),
            default_tdp: 360,
            max_tdp: 400,
            default_temp_limit: 88,
            max_temp_limit: 90,
            safe_gpu_offset: 175,
            safe_memory_offset: 1500,
            boost_clock: 2625,
        });
    } else if model_lower.contains("rtx 5070 ti") {
        return Some(GpuModelSpec {
            model_name: "RTX 5070 Ti".to_string(),
            architecture: "Blackwell".to_string(),
            cuda_cores: 8960,
            memory_bus_width: 256,
            memory_type: "GDDR7".to_string(),
            default_tdp: 300,
            max_tdp: 350,
            default_temp_limit: 87,
            max_temp_limit: 89,
            safe_gpu_offset: 175,
            safe_memory_offset: 1500,
            boost_clock: 2500,
        });
    } else if model_lower.contains("rtx 5070") {
        return Some(GpuModelSpec {
            model_name: "RTX 5070".to_string(),
            architecture: "Blackwell".to_string(),
            cuda_cores: 6144,
            memory_bus_width: 192,
            memory_type: "GDDR7".to_string(),
            default_tdp: 250,
            max_tdp: 285,
            default_temp_limit: 86,
            max_temp_limit: 88,
            safe_gpu_offset: 200,
            safe_memory_offset: 1500,
            boost_clock: 2500,
        });
    } else if model_lower.contains("rtx 5060 ti") {
        return Some(GpuModelSpec {
            model_name: "RTX 5060 Ti".to_string(),
            architecture: "Blackwell".to_string(),
            cuda_cores: 4608,
            memory_bus_width: 128,
            memory_type: "GDDR7".to_string(),
            default_tdp: 200,
            max_tdp: 220,
            default_temp_limit: 85,
            max_temp_limit: 87,
            safe_gpu_offset: 200,
            safe_memory_offset: 1500,
            boost_clock: 2475,
        });
    } else if model_lower.contains("rtx 5060") {
        return Some(GpuModelSpec {
            model_name: "RTX 5060".to_string(),
            architecture: "Blackwell".to_string(),
            cuda_cores: 3584,
            memory_bus_width: 128,
            memory_type: "GDDR6".to_string(),
            default_tdp: 170,
            max_tdp: 185,
            default_temp_limit: 83,
            max_temp_limit: 85,
            safe_gpu_offset: 225,
            safe_memory_offset: 1200,
            boost_clock: 2400,
        });
    }

    // RTX 40 Series (Ada Lovelace)
    if model_lower.contains("rtx 4090") {
        return Some(GpuModelSpec {
            model_name: "RTX 4090".to_string(),
            architecture: "Ada Lovelace".to_string(),
            cuda_cores: 16384,
            memory_bus_width: 384,
            memory_type: "GDDR6X".to_string(),
            default_tdp: 450,
            max_tdp: 480,
            default_temp_limit: 88,
            max_temp_limit: 90,
            safe_gpu_offset: 150,
            safe_memory_offset: 1000,
            boost_clock: 2520,
        });
    } else if model_lower.contains("rtx 4080 super") {
        return Some(GpuModelSpec {
            model_name: "RTX 4080 Super".to_string(),
            architecture: "Ada Lovelace".to_string(),
            cuda_cores: 10240,
            memory_bus_width: 256,
            memory_type: "GDDR6X".to_string(),
            default_tdp: 320,
            max_tdp: 350,
            default_temp_limit: 86,
            max_temp_limit: 88,
            safe_gpu_offset: 175,
            safe_memory_offset: 1000,
            boost_clock: 2550,
        });
    } else if model_lower.contains("rtx 4080") {
        return Some(GpuModelSpec {
            model_name: "RTX 4080".to_string(),
            architecture: "Ada Lovelace".to_string(),
            cuda_cores: 9728,
            memory_bus_width: 256,
            memory_type: "GDDR6X".to_string(),
            default_tdp: 320,
            max_tdp: 350,
            default_temp_limit: 86,
            max_temp_limit: 88,
            safe_gpu_offset: 175,
            safe_memory_offset: 1000,
            boost_clock: 2505,
        });
    } else if model_lower.contains("rtx 4070 ti super") {
        return Some(GpuModelSpec {
            model_name: "RTX 4070 Ti Super".to_string(),
            architecture: "Ada Lovelace".to_string(),
            cuda_cores: 8448,
            memory_bus_width: 256,
            memory_type: "GDDR6X".to_string(),
            default_tdp: 285,
            max_tdp: 320,
            default_temp_limit: 85,
            max_temp_limit: 87,
            safe_gpu_offset: 175,
            safe_memory_offset: 1000,
            boost_clock: 2610,
        });
    } else if model_lower.contains("rtx 4070 ti") {
        return Some(GpuModelSpec {
            model_name: "RTX 4070 Ti".to_string(),
            architecture: "Ada Lovelace".to_string(),
            cuda_cores: 7680,
            memory_bus_width: 192,
            memory_type: "GDDR6X".to_string(),
            default_tdp: 285,
            max_tdp: 320,
            default_temp_limit: 85,
            max_temp_limit: 87,
            safe_gpu_offset: 175,
            safe_memory_offset: 1000,
            boost_clock: 2610,
        });
    } else if model_lower.contains("rtx 4070 super") {
        return Some(GpuModelSpec {
            model_name: "RTX 4070 Super".to_string(),
            architecture: "Ada Lovelace".to_string(),
            cuda_cores: 7168,
            memory_bus_width: 192,
            memory_type: "GDDR6X".to_string(),
            default_tdp: 220,
            max_tdp: 240,
            default_temp_limit: 84,
            max_temp_limit: 86,
            safe_gpu_offset: 200,
            safe_memory_offset: 1000,
            boost_clock: 2475,
        });
    } else if model_lower.contains("rtx 4070") {
        return Some(GpuModelSpec {
            model_name: "RTX 4070".to_string(),
            architecture: "Ada Lovelace".to_string(),
            cuda_cores: 5888,
            memory_bus_width: 192,
            memory_type: "GDDR6X".to_string(),
            default_tdp: 200,
            max_tdp: 220,
            default_temp_limit: 83,
            max_temp_limit: 85,
            safe_gpu_offset: 200,
            safe_memory_offset: 1000,
            boost_clock: 2475,
        });
    } else if model_lower.contains("rtx 4060 ti") {
        return Some(GpuModelSpec {
            model_name: "RTX 4060 Ti".to_string(),
            architecture: "Ada Lovelace".to_string(),
            cuda_cores: 4352,
            memory_bus_width: 128,
            memory_type: "GDDR6".to_string(),
            default_tdp: 160,
            max_tdp: 175,
            default_temp_limit: 82,
            max_temp_limit: 84,
            safe_gpu_offset: 225,
            safe_memory_offset: 1200,
            boost_clock: 2535,
        });
    } else if model_lower.contains("rtx 4060") {
        return Some(GpuModelSpec {
            model_name: "RTX 4060".to_string(),
            architecture: "Ada Lovelace".to_string(),
            cuda_cores: 3072,
            memory_bus_width: 128,
            memory_type: "GDDR6".to_string(),
            default_tdp: 115,
            max_tdp: 135,
            default_temp_limit: 80,
            max_temp_limit: 82,
            safe_gpu_offset: 250,
            safe_memory_offset: 1200,
            boost_clock: 2460,
        });
    }

    // RTX 30 Series (Ampere)
    if model_lower.contains("rtx 3090 ti") {
        return Some(GpuModelSpec {
            model_name: "RTX 3090 Ti".to_string(),
            architecture: "Ampere".to_string(),
            cuda_cores: 10752,
            memory_bus_width: 384,
            memory_type: "GDDR6X".to_string(),
            default_tdp: 450,
            max_tdp: 480,
            default_temp_limit: 88,
            max_temp_limit: 90,
            safe_gpu_offset: 100,
            safe_memory_offset: 800,
            boost_clock: 1860,
        });
    } else if model_lower.contains("rtx 3090") {
        return Some(GpuModelSpec {
            model_name: "RTX 3090".to_string(),
            architecture: "Ampere".to_string(),
            cuda_cores: 10496,
            memory_bus_width: 384,
            memory_type: "GDDR6X".to_string(),
            default_tdp: 350,
            max_tdp: 390,
            default_temp_limit: 85,
            max_temp_limit: 87,
            safe_gpu_offset: 100,
            safe_memory_offset: 800,
            boost_clock: 1695,
        });
    } else if model_lower.contains("rtx 3080 ti") {
        return Some(GpuModelSpec {
            model_name: "RTX 3080 Ti".to_string(),
            architecture: "Ampere".to_string(),
            cuda_cores: 10240,
            memory_bus_width: 384,
            memory_type: "GDDR6X".to_string(),
            default_tdp: 350,
            max_tdp: 390,
            default_temp_limit: 85,
            max_temp_limit: 87,
            safe_gpu_offset: 100,
            safe_memory_offset: 800,
            boost_clock: 1665,
        });
    } else if model_lower.contains("rtx 3080") {
        return Some(GpuModelSpec {
            model_name: "RTX 3080".to_string(),
            architecture: "Ampere".to_string(),
            cuda_cores: 8704,
            memory_bus_width: 320,
            memory_type: "GDDR6X".to_string(),
            default_tdp: 320,
            max_tdp: 350,
            default_temp_limit: 83,
            max_temp_limit: 85,
            safe_gpu_offset: 125,
            safe_memory_offset: 800,
            boost_clock: 1710,
        });
    } else if model_lower.contains("rtx 3070 ti") {
        return Some(GpuModelSpec {
            model_name: "RTX 3070 Ti".to_string(),
            architecture: "Ampere".to_string(),
            cuda_cores: 6144,
            memory_bus_width: 256,
            memory_type: "GDDR6X".to_string(),
            default_tdp: 290,
            max_tdp: 320,
            default_temp_limit: 82,
            max_temp_limit: 84,
            safe_gpu_offset: 125,
            safe_memory_offset: 800,
            boost_clock: 1770,
        });
    } else if model_lower.contains("rtx 3070") {
        return Some(GpuModelSpec {
            model_name: "RTX 3070".to_string(),
            architecture: "Ampere".to_string(),
            cuda_cores: 5888,
            memory_bus_width: 256,
            memory_type: "GDDR6".to_string(),
            default_tdp: 220,
            max_tdp: 240,
            default_temp_limit: 80,
            max_temp_limit: 82,
            safe_gpu_offset: 150,
            safe_memory_offset: 1000,
            boost_clock: 1725,
        });
    } else if model_lower.contains("rtx 3060 ti") {
        return Some(GpuModelSpec {
            model_name: "RTX 3060 Ti".to_string(),
            architecture: "Ampere".to_string(),
            cuda_cores: 4864,
            memory_bus_width: 256,
            memory_type: "GDDR6".to_string(),
            default_tdp: 200,
            max_tdp: 220,
            default_temp_limit: 80,
            max_temp_limit: 82,
            safe_gpu_offset: 150,
            safe_memory_offset: 1000,
            boost_clock: 1665,
        });
    } else if model_lower.contains("rtx 3060") {
        return Some(GpuModelSpec {
            model_name: "RTX 3060".to_string(),
            architecture: "Ampere".to_string(),
            cuda_cores: 3584,
            memory_bus_width: 192,
            memory_type: "GDDR6".to_string(),
            default_tdp: 170,
            max_tdp: 190,
            default_temp_limit: 78,
            max_temp_limit: 80,
            safe_gpu_offset: 175,
            safe_memory_offset: 1200,
            boost_clock: 1777,
        });
    }

    None
}

/// Generate optimized profiles based on GPU model
pub fn generate_gaming_profiles(model_name: &str) -> Vec<OverclockProfile> {
    let spec = match get_gpu_spec(model_name) {
        Some(s) => s,
        None => return vec![OverclockProfile::default()],
    };

    vec![
        // Stock profile
        OverclockProfile {
            name: "Stock".to_string(),
            gpu_clock_offset: 0,
            memory_clock_offset: 0,
            voltage_offset: 0,
            power_limit: 100,
            temp_limit: spec.default_temp_limit,
            fan_curve: vec![
                (30, 25),
                (40, 35),
                (50, 45),
                (60, 55),
                (70, 65),
                (spec.default_temp_limit - 5, 85),
                (spec.default_temp_limit, 100),
            ],
        },
        // Performance profile (safe OC)
        OverclockProfile {
            name: "Performance".to_string(),
            gpu_clock_offset: spec.safe_gpu_offset,
            memory_clock_offset: spec.safe_memory_offset,
            voltage_offset: 0,
            power_limit: ((spec.max_tdp as f32 / spec.default_tdp as f32) * 100.0) as u8,
            temp_limit: spec.max_temp_limit,
            fan_curve: vec![
                (30, 30),
                (40, 40),
                (50, 50),
                (60, 60),
                (70, 75),
                (spec.max_temp_limit - 5, 90),
                (spec.max_temp_limit, 100),
            ],
        },
        // Quiet profile (reduced clocks, lower temps)
        OverclockProfile {
            name: "Quiet".to_string(),
            gpu_clock_offset: -100,
            memory_clock_offset: -200,
            voltage_offset: 0,
            power_limit: 85,
            temp_limit: spec.default_temp_limit - 5,
            fan_curve: vec![
                (30, 15),
                (40, 20),
                (50, 30),
                (60, 40),
                (70, 55),
                (spec.default_temp_limit - 10, 70),
                (spec.default_temp_limit - 5, 85),
            ],
        },
        // Max Performance (aggressive OC)
        OverclockProfile {
            name: "Max Performance".to_string(),
            gpu_clock_offset: (spec.safe_gpu_offset as f32 * 1.2) as i32,
            memory_clock_offset: (spec.safe_memory_offset as f32 * 1.1) as i32,
            voltage_offset: 0,
            power_limit: ((spec.max_tdp as f32 / spec.default_tdp as f32) * 100.0) as u8,
            temp_limit: spec.max_temp_limit,
            fan_curve: vec![
                (30, 40),
                (40, 50),
                (50, 60),
                (60, 70),
                (70, 80),
                (spec.max_temp_limit - 5, 95),
                (spec.max_temp_limit, 100),
            ],
        },
    ]
}

/// Print GPU specifications
pub fn print_gpu_info(model_name: &str) {
    if let Some(spec) = get_gpu_spec(model_name) {
        println!("\n=== GPU Specifications: {} ===", spec.model_name);
        println!("Architecture: {}", spec.architecture);
        println!("CUDA Cores: {}", spec.cuda_cores);
        println!("Memory Type: {}", spec.memory_type);
        println!("Memory Bus: {}-bit", spec.memory_bus_width);
        println!("Default TDP: {}W (Max: {}W)", spec.default_tdp, spec.max_tdp);
        println!(
            "Temp Limits: {}°C (Max: {}°C)",
            spec.default_temp_limit, spec.max_temp_limit
        );
        println!("Boost Clock: {} MHz", spec.boost_clock);
        println!("Safe GPU Offset: +{} MHz", spec.safe_gpu_offset);
        println!("Safe Memory Offset: +{} MHz", spec.safe_memory_offset);
    } else {
        println!("GPU model not found in database: {}", model_name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rtx_5090_spec() {
        let spec = get_gpu_spec("RTX 5090").unwrap();
        assert_eq!(spec.cuda_cores, 21760);
        assert_eq!(spec.architecture, "Blackwell");
        assert_eq!(spec.memory_type, "GDDR7");
        assert_eq!(spec.default_tdp, 575);
    }

    #[test]
    fn test_rtx_4090_spec() {
        let spec = get_gpu_spec("RTX 4090").unwrap();
        assert_eq!(spec.cuda_cores, 16384);
        assert_eq!(spec.architecture, "Ada Lovelace");
        assert_eq!(spec.default_tdp, 450);
    }

    #[test]
    fn test_profile_generation() {
        let profiles = generate_gaming_profiles("RTX 5090");
        assert_eq!(profiles.len(), 4);
        assert!(profiles.iter().any(|p| p.name == "Stock"));
        assert!(profiles.iter().any(|p| p.name == "Performance"));
        assert!(profiles.iter().any(|p| p.name == "Quiet"));
        assert!(profiles.iter().any(|p| p.name == "Max Performance"));
    }
}
