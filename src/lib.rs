#[macro_use]
extern crate gfx;

#[macro_use]
extern crate error_chain;

extern crate gfx_device_gl;
extern crate gfx_window_glutin;
extern crate glutin;

extern crate rscam;
extern crate av;

pub mod transcoder;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
