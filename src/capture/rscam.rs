use std;
use std::path::Path;
use std::io::{
    Read,
    SeekFrom,
    Cursor,
};

use rscam;
use av::io::{
    AVRead,
    AVSeek,
};

use canvas::renderer::{
    Canvas,
    CanvasError,
};

use glutin::WindowBuilder;

use gfx_device_gl::{
    Resources,
    CommandBuffer,
};

// V4lConfiguration Requires a Path and an rscam::Config
#[derive(Debug, Clone)]
pub struct V4lConfig {
    path: String,
    height: u16,
    width: u16,
    fourcc: String,
    interval: (u32, u32),
}

impl V4lConfig {
//  Create a new V4lConfig with a Path and an rscam::Config
    pub fn new(path: String, height: u16, width: u16, fourcc: String, interval: (u32, u32)) -> V4lConfig {
        V4lConfig { path, height: height, width: width, fourcc, interval }
    }
//  Return the path as String
    pub fn path(&self) -> String {
        self.path.clone()
    }
//  Return the height as u32
    pub fn height(&self) -> u16 {
        self.height.clone()
    }
//  Return the width as u32
    pub fn width(&self) -> u16 {
        self.width.clone()
    }
//  Return the fourcc value as String
    pub fn fourcc(&self) -> String {
        self.fourcc.clone()
    }
//  Return the interval as (u32, u32) (num, den)
    pub fn interval(&self) -> (u32, u32) {
        self.interval.clone()
    }
}
//  Enumerate Runtime States.
pub enum V4lRuntimeState {
//  Is the state after dev.start(&rscam::Config) is called.  It is only during
//  this state that you can receive frames from the device.
    Started(rscam::Camera),
//  Is the state when AVRead returns Some(0) or None
    StoppedReachedEOF,
//  Is the state when rscam returns an error not otherwise handled.
    Error(V4lError),
//  Is the state when there is an io Error.
    IOError(std::io::Error),
//  Is the state before start() has been called on the device.
    Idle,
}
//  Wraps the errors from rscam
pub enum V4lError {
    IOError(std::io::Error),
    Rscam(rscam::Error),
    None,
}
//  Wraps V4lConfig and tracks the V4lRuntimeState
pub struct V4l {
    configuration: V4lConfig,
    pub runtime_state: V4lRuntimeState,
}

impl V4l {
//  Create a new V4l instance by providing a Path and an rscam::Config
    pub fn new(config: V4lConfig) -> V4l {
        V4l {
            configuration: config.clone(),
            runtime_state: V4lRuntimeState::Idle,
        }
    }
//  Initialize the device so that we may receive frames.
    pub fn init(&mut self, path: String) {
        let mut device = rscam::Camera::new(&path).unwrap();
        let format = self.configuration.fourcc();
        let config = rscam::Config {
            interval: self.configuration.interval(),
            resolution: (self.configuration.width() as u32, self.configuration.height() as u32),
            format: format.as_bytes(),
            ..Default::default()
        };
        match device.start(&config) {
            Ok(x)  => { self.runtime_state = V4lRuntimeState::Started(device);              },
            Err(e) => { self.runtime_state = V4lRuntimeState::Error(V4lError::Rscam(e)) },
        }
    }
}
