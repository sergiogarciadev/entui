use crate::entropy::analyze_file;
use anyhow::Result;
use std::path::PathBuf;

pub struct App {
    #[allow(dead_code)]
    pub file_path: PathBuf,
    pub entropy_data: Vec<(f64, f64)>, // (offset_ratio, entropy) or (offset, entropy)?
    // Using (f64, f64) for Chart: (offset, entropy)
    pub window_start: f64,
    pub window_width: f64, // The range of x-axis showed
    pub block_size: usize,
    pub should_quit: bool,
    pub total_size: f64,
}

impl App {
    pub fn new(file_path: PathBuf, block_size: usize) -> Result<Self> {
        let raw_data = analyze_file(&file_path, block_size)?;
        let total_size = raw_data.last().map(|(off, _)| *off as f64).unwrap_or(0.0) + block_size as f64;
        
        // Convert to f64 for charts
        let entropy_data = raw_data.into_iter().map(|(x, y)| (x as f64, y)).collect();

        Ok(Self {
            file_path,
            entropy_data,
            window_start: 0.0,
            window_width: total_size,
            block_size,
            should_quit: false,
            total_size,
        })
    }

    #[allow(dead_code)]
    pub fn on_tick(&mut self) {
        // Handle tick if needed
    }

    pub fn on_left(&mut self) {
        let step = self.window_width * 0.1;
        self.window_start = (self.window_start - step).max(0.0);
    }

    pub fn on_right(&mut self) {
        let step = self.window_width * 0.1;
        self.window_start = (self.window_start + step).min(self.total_size - self.window_width);
    }
    
    pub fn on_zoom_in(&mut self) {
        let new_width = (self.window_width * 0.9).max(self.block_size as f64 * 10.0); // Min zoom
        // Center zoom
        let center = self.window_start + self.window_width / 2.0;
        self.window_start = (center - new_width / 2.0).max(0.0);
        self.window_width = new_width;
    }

    pub fn on_zoom_out(&mut self) {
        let new_width = (self.window_width * 1.1).min(self.total_size);
        let center = self.window_start + self.window_width / 2.0;
        self.window_start = (center - new_width / 2.0).max(0.0).min(self.total_size - new_width);
        self.window_width = new_width;
    }

    pub fn on_quit(&mut self) {
        self.should_quit = true;
    }
}
