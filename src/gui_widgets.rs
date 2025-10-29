// Custom GUI Widgets for nvcontrol
// Fan curves, voltage curves, and monitoring graphs

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurvePoint {
    pub x: f64,
    pub y: f64,
}

impl CurvePoint {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanCurve {
    pub points: Vec<CurvePoint>,
    pub selected_point: Option<usize>,
}

impl Default for FanCurve {
    fn default() -> Self {
        Self {
            points: vec![
                CurvePoint::new(30.0, 30.0),
                CurvePoint::new(50.0, 50.0),
                CurvePoint::new(70.0, 70.0),
                CurvePoint::new(85.0, 100.0),
            ],
            selected_point: None,
        }
    }
}

impl FanCurve {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_point(&mut self, temp: f64, speed: f64) {
        let point = CurvePoint::new(temp, speed);

        // Insert in sorted order by temperature
        let insert_pos = self.points.iter()
            .position(|p| p.x > temp)
            .unwrap_or(self.points.len());

        self.points.insert(insert_pos, point);
    }

    pub fn remove_point(&mut self, index: usize) {
        if self.points.len() > 2 {
            self.points.remove(index);
        }
    }

    pub fn update_point(&mut self, index: usize, temp: f64, speed: f64) {
        if let Some(point) = self.points.get_mut(index) {
            point.x = temp.clamp(0.0, 100.0);
            point.y = speed.clamp(0.0, 100.0);
        }

        // Re-sort by temperature
        self.points.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
    }

    pub fn get_speed_at_temp(&self, temp: f64) -> f64 {
        if self.points.is_empty() {
            return 50.0;
        }

        // Find the two points to interpolate between
        if temp <= self.points[0].x {
            return self.points[0].y;
        }

        for i in 0..self.points.len() - 1 {
            let p1 = &self.points[i];
            let p2 = &self.points[i + 1];

            if temp >= p1.x && temp <= p2.x {
                // Linear interpolation
                let t = (temp - p1.x) / (p2.x - p1.x);
                return p1.y + t * (p2.y - p1.y);
            }
        }

        self.points.last().unwrap().y
    }

    pub fn to_nvcontrol_format(&self) -> Vec<(u32, u32)> {
        self.points
            .iter()
            .map(|p| (p.x as u32, p.y as u32))
            .collect()
    }
}

/// Ring buffer for time-series data
#[derive(Debug, Clone)]
pub struct TimeSeriesData {
    pub timestamps: Vec<f64>,
    pub values: Vec<f64>,
    pub capacity: usize,
}

impl TimeSeriesData {
    pub fn new(capacity: usize) -> Self {
        Self {
            timestamps: Vec::with_capacity(capacity),
            values: Vec::with_capacity(capacity),
            capacity,
        }
    }

    pub fn push(&mut self, timestamp: f64, value: f64) {
        if self.timestamps.len() >= self.capacity {
            self.timestamps.remove(0);
            self.values.remove(0);
        }

        self.timestamps.push(timestamp);
        self.values.push(value);
    }

    pub fn clear(&mut self) {
        self.timestamps.clear();
        self.values.clear();
    }

    pub fn get_points(&self) -> Vec<[f64; 2]> {
        self.timestamps
            .iter()
            .zip(self.values.iter())
            .map(|(&t, &v)| [t, v])
            .collect()
    }

    pub fn latest_value(&self) -> Option<f64> {
        self.values.last().copied()
    }

    pub fn min_value(&self) -> Option<f64> {
        self.values.iter().copied().min_by(|a, b| a.partial_cmp(b).unwrap())
    }

    pub fn max_value(&self) -> Option<f64> {
        self.values.iter().copied().max_by(|a, b| a.partial_cmp(b).unwrap())
    }

    pub fn avg_value(&self) -> Option<f64> {
        if self.values.is_empty() {
            return None;
        }
        Some(self.values.iter().sum::<f64>() / self.values.len() as f64)
    }
}

/// Voltage/Frequency curve for undervolting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoltageCurve {
    pub points: Vec<CurvePoint>, // x = frequency (MHz), y = voltage (mV)
    pub selected_point: Option<usize>,
}

impl Default for VoltageCurve {
    fn default() -> Self {
        Self {
            points: vec![
                CurvePoint::new(800.0, 700.0),
                CurvePoint::new(1200.0, 850.0),
                CurvePoint::new(1600.0, 950.0),
                CurvePoint::new(2000.0, 1050.0),
            ],
            selected_point: None,
        }
    }
}

impl VoltageCurve {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_point(&mut self, freq: f64, voltage: f64) {
        let point = CurvePoint::new(freq, voltage);

        let insert_pos = self.points.iter()
            .position(|p| p.x > freq)
            .unwrap_or(self.points.len());

        self.points.insert(insert_pos, point);
    }

    pub fn remove_point(&mut self, index: usize) {
        if self.points.len() > 2 {
            self.points.remove(index);
        }
    }

    pub fn update_point(&mut self, index: usize, freq: f64, voltage: f64) {
        if let Some(point) = self.points.get_mut(index) {
            point.x = freq.clamp(0.0, 3000.0);
            point.y = voltage.clamp(500.0, 1200.0);
        }

        self.points.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());
    }

    pub fn get_voltage_at_freq(&self, freq: f64) -> f64 {
        if self.points.is_empty() {
            return 850.0;
        }

        if freq <= self.points[0].x {
            return self.points[0].y;
        }

        for i in 0..self.points.len() - 1 {
            let p1 = &self.points[i];
            let p2 = &self.points[i + 1];

            if freq >= p1.x && freq <= p2.x {
                let t = (freq - p1.x) / (p2.x - p1.x);
                return p1.y + t * (p2.y - p1.y);
            }
        }

        self.points.last().unwrap().y
    }
}

/// Multi-metric dashboard data
pub struct MonitoringDashboard {
    pub temperature: TimeSeriesData,
    pub power: TimeSeriesData,
    pub gpu_utilization: TimeSeriesData,
    pub memory_utilization: TimeSeriesData,
    pub fan_speed: TimeSeriesData,
    pub gpu_clock: TimeSeriesData,
    pub memory_clock: TimeSeriesData,
    pub start_time: std::time::Instant,
}

impl MonitoringDashboard {
    pub fn new(capacity: usize) -> Self {
        Self {
            temperature: TimeSeriesData::new(capacity),
            power: TimeSeriesData::new(capacity),
            gpu_utilization: TimeSeriesData::new(capacity),
            memory_utilization: TimeSeriesData::new(capacity),
            fan_speed: TimeSeriesData::new(capacity),
            gpu_clock: TimeSeriesData::new(capacity),
            memory_clock: TimeSeriesData::new(capacity),
            start_time: std::time::Instant::now(),
        }
    }

    pub fn update(&mut self, stats: &GpuStats) {
        let elapsed = self.start_time.elapsed().as_secs_f64();

        self.temperature.push(elapsed, stats.temperature as f64);
        self.power.push(elapsed, stats.power_draw as f64);
        self.gpu_utilization.push(elapsed, stats.utilization as f64);
        self.fan_speed.push(elapsed, stats.fan_speed as f64);
    }

    pub fn clear_all(&mut self) {
        self.temperature.clear();
        self.power.clear();
        self.gpu_utilization.clear();
        self.memory_utilization.clear();
        self.fan_speed.clear();
        self.gpu_clock.clear();
        self.memory_clock.clear();
        self.start_time = std::time::Instant::now();
    }
}

/// GPU stats struct (for compatibility with GUI)
pub struct GpuStats {
    pub temperature: f32,
    pub power_draw: f32,
    pub utilization: f32,
    pub fan_speed: u32,
}
