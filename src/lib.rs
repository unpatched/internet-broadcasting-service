#[macro_use]
extern crate gfx;

#[macro_use]
extern crate error_chain;

extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;

extern crate rscam;
extern crate portaudio_rs;
extern crate av;

pub mod transcoder;
pub mod canvas;
pub mod capture;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

//  Incase anybody is wondering, we <3 the OBS crew.
