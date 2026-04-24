/// GPU Control and Gaming Features Tests
///
/// Tests overclocking profiles, fan curves, power management,
/// multi-GPU coordination, gaming integration (Steam/Lutris/GameMode),
/// performance monitoring, upscaling (DLSS/FSR), and VRR/display config.
use nvcontrol::{
    NvControlError,
    advanced_display::{
        ColorDepth, DisplayConfig, DisplayConfigManager, ReflexIntegration, VrrConfig,
        VrrProfileManager,
    },
    advanced_multi_gpu::{GpuProfile, LoadBalancer, MultiGpuCoordinator, PerGpuProfileManager},
    advanced_power::{
        BatteryBoost, DynamicPowerManager, PowerMode, PowerProfile, PowerProfileManager,
    },
    // Advanced GPU control
    enhanced_overclock::{OverclockProfile, OverclockProfileManager},
    // Gaming integration
    gaming_integration::{GameLauncher, GameModeIntegration, LaunchOptimizer, SteamScanner},
    intelligent_fan::{AdvancedFanCurve, FanOptimizer, TempTrend},
    performance_monitoring::{
        FpsCounter, FrameTimeStats, LatencyMonitor, RegressionDetector, RegressionSeverity,
    },
    upscaling_tech::{
        DlssQuality, FsrQuality, ResolutionScaler, UpscalingConfig, UpscalingRecommender,
        UpscalingTech,
    },
};

// ==================== Overclock & Fan Tests ====================

#[test]
fn test_overclock_profile_creation() {
    let profile = OverclockProfile {
        name: "High Performance".to_string(),
        game_exe: Some("game.exe".to_string()),
        gpu_offset_mhz: 150,
        memory_offset_mhz: 500,
        power_limit_watts: Some(400),
        voltage_curve: None,
        fan_curve: None,
        enabled: true,
    };

    assert_eq!(profile.name, "High Performance");
    assert_eq!(profile.gpu_offset_mhz, 150);
    assert_eq!(profile.memory_offset_mhz, 500);
}

#[test]
fn test_overclock_profile_manager() {
    let mut manager = OverclockProfileManager::new();

    let profile1 = OverclockProfile {
        name: "Gaming".to_string(),
        game_exe: Some("cyberpunk.exe".to_string()),
        gpu_offset_mhz: 120,
        memory_offset_mhz: 400,
        power_limit_watts: Some(350),
        voltage_curve: None,
        fan_curve: None,
        enabled: true,
    };

    let profile2 = OverclockProfile {
        name: "Productivity".to_string(),
        game_exe: None,
        gpu_offset_mhz: 50,
        memory_offset_mhz: 200,
        power_limit_watts: Some(300),
        voltage_curve: None,
        fan_curve: None,
        enabled: true,
    };

    manager.set_profile(profile1);
    manager.set_profile(profile2);

    assert_eq!(manager.list_profiles().len(), 2);
    assert!(manager.get_profile("Gaming").is_some());
    assert!(manager.get_profile_by_exe("cyberpunk.exe").is_some());
}

#[test]
fn test_fan_curve_interpolation() {
    let curve = AdvancedFanCurve::balanced();

    let speed_at_50 = curve.get_fan_speed(50, 0);
    let speed_at_70 = curve.get_fan_speed(70, 0);

    assert!(speed_at_50 > 0);
    assert!(speed_at_70 > speed_at_50);
}

#[test]
fn test_fan_curve_presets() {
    let performance = AdvancedFanCurve::performance();
    let silent = AdvancedFanCurve::silent();
    let balanced = AdvancedFanCurve::balanced();

    // Performance should be more aggressive
    assert!(performance.get_fan_speed(70, 0) > balanced.get_fan_speed(70, 0));
    assert!(balanced.get_fan_speed(70, 0) > silent.get_fan_speed(70, 0));
}

#[test]
fn test_fan_optimizer_trend_detection() {
    let mut optimizer = FanOptimizer::new(0);

    // Simulate rising temperature
    for temp in 50..70 {
        optimizer.record_sample(temp, 50, 80);
    }

    let trend = optimizer.predict_temp_trend();
    assert_eq!(trend, TempTrend::Rising);
}

#[test]
fn test_power_mode_detection() {
    use nvcontrol::nvml_backend::MockNvmlBackend;
    use std::sync::Arc;

    let backend = Arc::new(MockNvmlBackend::single_gpu());
    let mut manager = DynamicPowerManager::new(0, backend);

    // Simulate high load
    for _ in 0..15 {
        manager.update(90, 85, 300.0);
    }

    let recommended = manager.recommend_mode();
    assert_eq!(recommended, PowerMode::MaxPerformance);
}

#[test]
fn test_power_profile_manager() {
    let mut manager = PowerProfileManager::new();

    let profile = PowerProfile {
        name: "Gaming".to_string(),
        executable: Some("game.exe".to_string()),
        power_limit_watts: 350,
        power_mode: PowerMode::MaxPerformance,
        clock_limit_mhz: None,
        memory_clock_limit_mhz: None,
        enabled: true,
    };

    manager.set_profile(profile);

    assert!(manager.get_profile("Gaming").is_some());
    assert!(manager.get_profile_by_exe("game.exe").is_some());
}

#[test]
fn test_battery_boost() {
    use nvcontrol::nvml_backend::MockNvmlBackend;
    use std::sync::Arc;

    let backend = Arc::new(MockNvmlBackend::single_gpu());
    let mut boost = BatteryBoost::new(0, 60, backend);

    assert!(!boost.is_enabled());
    boost.enable().unwrap();
    assert!(boost.is_enabled());
    boost.disable().unwrap();
    assert!(!boost.is_enabled());
}

#[test]
fn test_multi_gpu_coordinator() {
    let coordinator = MultiGpuCoordinator::new();

    match coordinator {
        Ok(coord) => {
            assert_eq!(coord.gpu_ids().len(), coord.gpu_count());
        }
        Err(e) => {
            assert!(matches!(
                e,
                NvControlError::NvmlNotAvailable(_) | NvControlError::GpuQueryFailed(_)
            ));
        }
    }
}

#[test]
fn test_per_gpu_profile_manager() {
    let mut manager = PerGpuProfileManager::new();

    let profile1 = GpuProfile {
        gpu_id: 0,
        name: "GPU 0 Performance".to_string(),
        power_limit_watts: Some(400),
        gpu_offset_mhz: Some(150),
        memory_offset_mhz: Some(500),
        fan_speed_percent: Some(75),
    };

    let profile2 = GpuProfile {
        gpu_id: 1,
        name: "GPU 1 Balanced".to_string(),
        power_limit_watts: Some(300),
        gpu_offset_mhz: Some(100),
        memory_offset_mhz: Some(300),
        fan_speed_percent: Some(60),
    };

    manager.set_profile(profile1);
    manager.set_profile(profile2);

    assert!(manager.get_profile(0).is_some());
    assert!(manager.get_profile(1).is_some());
}

// ==================== Gaming & Performance Tests ====================

#[test]
fn test_steam_scanner() {
    let scanner = SteamScanner::new();
    assert!(
        scanner
            .scan_games()
            .iter()
            .all(|game| game.launcher == GameLauncher::Steam)
    );
}

#[test]
fn test_gamemode_detection() {
    let gamemode = GameModeIntegration::new();

    assert!(!gamemode.is_active() || gamemode.is_available());
}

#[test]
fn test_launch_optimizer() {
    let mut optimizer = LaunchOptimizer::new();

    optimizer.set_gamemode(true);
    optimizer.set_mangohud(true);
    optimizer.set_fsync(true);

    let env = optimizer.build_env_vars();

    assert!(env.contains_key("__GL_THREADED_OPTIMIZATIONS"));
    assert!(env.contains_key("PROTON_NO_FSYNC"));
    assert_eq!(env.get("MANGOHUD"), Some(&"1".to_string()));
    assert_eq!(
        env.get("LD_PRELOAD"),
        Some(&"libgamemodeauto.so".to_string())
    );

    let prefix = optimizer.build_launch_prefix();
    assert!(prefix.contains("gamemoderun"));
    assert!(prefix.contains("mangohud"));
}

#[test]
fn test_fps_counter() {
    use std::time::Duration;

    let mut counter = FpsCounter::new(100);

    // Prime the first frame, then collect a bounded sample window.
    counter.record_frame();
    std::thread::sleep(Duration::from_millis(16));
    for _ in 0..12 {
        counter.record_frame();
        std::thread::sleep(Duration::from_millis(1));
    }

    let fps = counter.current_fps();
    assert!(fps > 0.0);

    let stats = counter.get_stats();
    assert!(stats.avg_fps > 0.0);
    assert!(stats.one_percent_low > 0.0);
    assert!(stats.avg_frame_time_ms > 0.0);
}

#[test]
fn test_frame_time_statistics() {
    use std::time::Duration;

    let mut counter = FpsCounter::new(100);

    counter.record_frame();
    std::thread::sleep(Duration::from_millis(16));
    for _ in 0..16 {
        counter.record_frame();
        std::thread::sleep(Duration::from_millis(1));
    }

    let stats = counter.get_stats();

    assert!(stats.avg_fps > 0.0);
    assert!(stats.one_percent_low > 0.0);
    assert!(stats.median_frame_time_ms >= stats.min_frame_time_ms);
    assert!(stats.max_frame_time_ms >= stats.min_frame_time_ms);
}

#[test]
fn test_latency_monitor() {
    use std::time::Duration;

    let mut monitor = LatencyMonitor::new(100);

    for i in 0..100 {
        monitor.record_latency(Duration::from_millis(10 + i % 5));
    }

    let avg = monitor.avg_latency_ms();
    assert!(avg > 0.0);
    assert!(monitor.percentile_latency_ms(0.95) >= avg);
}

#[test]
fn test_regression_detector() {
    let mut detector = RegressionDetector::new(10.0);

    let baseline = FrameTimeStats {
        avg_fps: 100.0,
        avg_frame_time_ms: 10.0,
        one_percent_low: 90.0,
        zero_one_percent_low: 80.0,
        min_frame_time_ms: 8.0,
        max_frame_time_ms: 15.0,
        median_frame_time_ms: 10.0,
    };

    detector.set_baseline(baseline.clone());

    let regressed = FrameTimeStats {
        avg_fps: 80.0, // 20% drop
        ..baseline
    };

    let report = detector.check_regression(&regressed);
    let report = report.expect("expected regression report");
    assert_eq!(report.severity, RegressionSeverity::Critical);
    assert!(report.fps_drop_percent >= 20.0);
}

#[test]
fn test_dlss_quality_scaling() {
    let quality = DlssQuality::Performance;
    let (render_w, render_h) = quality.render_resolution(3840, 2160);

    // Performance mode: 2x scaling
    assert_eq!(render_w, 1920);
    assert_eq!(render_h, 1080);
}

#[test]
fn test_fsr_quality_scaling() {
    let quality = FsrQuality::Balanced;
    let (render_w, render_h) = quality.render_resolution(2560, 1440);

    assert!(render_w < 2560);
    assert!(render_h < 1440);
}

#[test]
fn test_upscaling_config() {
    let dlss_config = UpscalingConfig::new_dlss("Game 1".to_string(), DlssQuality::Quality);
    let fsr_config = UpscalingConfig::new_fsr("Game 2".to_string(), FsrQuality::Performance);

    assert_eq!(dlss_config.tech, UpscalingTech::DLSS);
    assert_eq!(fsr_config.tech, UpscalingTech::FSR);
}

#[test]
fn test_resolution_scaler() {
    let scaler = ResolutionScaler::new(3840, 2160);

    let config = UpscalingConfig::new_dlss("Test".to_string(), DlssQuality::Performance);

    let (render_w, render_h) = scaler.calculate_render_resolution(&config);
    assert_eq!(render_w, 1920);
    assert_eq!(render_h, 1080);

    let gain = scaler.estimate_performance_gain(&config);
    assert!(gain > 200.0);
}

#[test]
fn test_upscaling_recommender() {
    let mut recommender = UpscalingRecommender::new(60);

    recommender.update_fps(40.0); // Below target

    let recommended = recommender.recommend_dlss_quality();
    assert!(recommended.is_some());
}

#[test]
fn test_vrr_config() {
    let config = VrrConfig::new("Test Game".to_string(), 48, 165);

    assert!(config.enabled);
    assert_eq!(config.min_refresh_hz, 48);
    assert_eq!(config.max_refresh_hz, 165);
}

#[test]
fn test_display_config() {
    let config = DisplayConfig {
        game: "Test Game".to_string(),
        resolution: (3840, 2160),
        refresh_rate: 144,
        vrr_enabled: true,
        hdr_enabled: false,
        color_depth: ColorDepth::Bit10,
    };

    assert_eq!(config.resolution, (3840, 2160));
    assert_eq!(config.refresh_rate, 144);
    assert_eq!(config.color_depth.bits(), 10);
}

#[test]
fn test_reflex_integration() {
    let reflex = ReflexIntegration::new(0);

    assert!(!reflex.is_enabled());
}

#[test]
fn test_vrr_profile_manager() {
    let mut manager = VrrProfileManager::new();

    let config1 = VrrConfig::new("Game 1".to_string(), 48, 144);
    let config2 = VrrConfig::new("Game 2".to_string(), 60, 165);

    manager.set_profile(config1);
    manager.set_profile(config2);

    assert_eq!(manager.list_profiles().len(), 2);
    assert!(manager.get_profile("Game 1").is_some());
}

#[test]
fn test_display_config_manager() {
    let mut manager = DisplayConfigManager::new();

    let config = DisplayConfig {
        game: "Test".to_string(),
        resolution: (2560, 1440),
        refresh_rate: 144,
        vrr_enabled: true,
        hdr_enabled: false,
        color_depth: ColorDepth::Bit8,
    };

    manager.set_config(config);

    assert!(manager.get_config("Test").is_some());
}

// ==================== Integration Tests ====================

#[test]
fn test_full_gaming_session_simulation() {
    // 1. Detect game
    let steam = SteamScanner::new();
    assert!(
        steam
            .scan_games()
            .iter()
            .all(|game| game.launcher == GameLauncher::Steam)
    );

    // 2. Apply overclock profile
    let mut oc_manager = OverclockProfileManager::new();
    let oc_profile = OverclockProfile {
        name: "Cyberpunk High".to_string(),
        game_exe: Some("cyberpunk.exe".to_string()),
        gpu_offset_mhz: 150,
        memory_offset_mhz: 500,
        power_limit_watts: Some(400),
        voltage_curve: None,
        fan_curve: None,
        enabled: true,
    };
    oc_manager.set_profile(oc_profile);
    assert!(oc_manager.get_profile("Cyberpunk High").is_some());

    // 3. Configure upscaling
    let upscaling_config = UpscalingConfig::new_dlss("Cyberpunk".to_string(), DlssQuality::Quality);
    assert_eq!(upscaling_config.tech, UpscalingTech::DLSS);

    // 4. Enable VRR
    let vrr_config = VrrConfig::new("Cyberpunk".to_string(), 48, 144);
    assert_eq!(vrr_config.min_refresh_hz, 48);
    assert_eq!(vrr_config.max_refresh_hz, 144);

    // 5. Monitor performance
    let mut fps_counter = FpsCounter::new(100);
    use std::time::Duration;
    fps_counter.record_frame();
    std::thread::sleep(Duration::from_millis(16));
    for _ in 0..12 {
        fps_counter.record_frame();
        std::thread::sleep(Duration::from_millis(1));
    }
    let stats = fps_counter.get_stats();
    assert!(stats.avg_fps > 0.0);
    assert!(stats.one_percent_low > 0.0);
}

#[test]
fn test_multi_gpu_load_balancing() {
    let balancer = LoadBalancer::new();

    match balancer {
        Ok(mut bal) => {
            bal.update_load(0, 80);
            bal.update_load(1, 40);

            let distribution = bal.get_load_distribution();
            assert!(
                distribution
                    .iter()
                    .any(|(gpu_id, load)| *gpu_id == 0 && *load == 80.0)
            );
            if distribution.iter().any(|(gpu_id, _)| *gpu_id == 1) {
                assert!(
                    distribution
                        .iter()
                        .any(|(gpu_id, load)| *gpu_id == 1 && *load == 40.0)
                );
                assert_eq!(bal.get_least_loaded_gpu(), Some(1));
                assert!(!bal.is_balanced());
            } else {
                assert_eq!(distribution.len(), 1);
                assert_eq!(bal.get_least_loaded_gpu(), Some(0));
            }
        }
        Err(e) => {
            assert!(matches!(
                e,
                NvControlError::NvmlNotAvailable(_) | NvControlError::GpuQueryFailed(_)
            ));
        }
    }
}

#[test]
fn test_regression_detector_records_history() {
    let mut detector = RegressionDetector::new(10.0);
    let stats = FrameTimeStats {
        avg_fps: 120.0,
        avg_frame_time_ms: 8.3,
        one_percent_low: 100.0,
        zero_one_percent_low: 95.0,
        min_frame_time_ms: 7.8,
        max_frame_time_ms: 12.0,
        median_frame_time_ms: 8.1,
    };

    detector.record_session("Cyberpunk 2077".to_string(), stats.clone());

    assert_eq!(detector.get_history().len(), 1);
    assert_eq!(detector.get_history()[0].game, "Cyberpunk 2077");
    assert_eq!(detector.get_history()[0].stats.avg_fps, stats.avg_fps);
}
