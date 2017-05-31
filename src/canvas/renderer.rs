use canvas;

use gfx;
use gfx::format::{
    U8Norm,
    Srgba8,
    Swizzle,
    Format,
    ChannelType,
    DepthStencil,
    Rgba8,
    Srgb,
    R8_G8_B8_A8,
    SurfaceType,
    Uint,
};
use gfx::buffer::{
    Role,
    CreationError,
};
use gfx::memory::Usage;
use gfx::{
    Resources,
    Factory,
    Bind,
    CommandBuffer,
    Device,
    Rect,
};
use gfx::traits::FactoryExt;
use gfx::memory::Typed;
use gfx::handle::Buffer;
use gfx::texture::{
    FilterMethod,
    WrapMode,
    SamplerInfo,
    ImageInfoCommon,
    Kind,
    AaMode,
};
use gfx::pso::{
    InitError,
    PipelineData,
};
use gfx::pso::bundle::Bundle;
use glutin;
use glutin::{
    WindowBuilder,
};
use gfx_device_gl;
use gfx_window_glutin;

use std::path::Path;
use std::fmt::{
    Debug,
    Display,
};
// Standard fullscreen quad.
const SCREEN: [Vertex; 6] = [
    Vertex { pos: [-1.0, -1.0], tex: [0.0, 1.0] },
    Vertex { pos: [ 1.0, -1.0], tex: [1.0, 1.0] },
    Vertex { pos: [ 1.0,  1.0], tex: [1.0, 0.0] },
    Vertex { pos: [-1.0, -1.0], tex: [0.0, 1.0] },
    Vertex { pos: [-1.0,  1.0], tex: [0.0, 0.0] },
    Vertex { pos: [ 1.0,  1.0], tex: [1.0, 0.0] },
];
//  Define a constant clear color.
const CLEAR_COLOR: [f32; 4] = [1.0, 0.0, 1.0, 0.5];
//  This macro provided by gfx-rs. Thanks all!
gfx_defines!{
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        tex: [f32; 2] = "a_Tex",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex>              = (),
        tex:  gfx::TextureSampler<[f32; 4]>          = "u_Source",
        out:  gfx::RenderTarget<Rgba8>               = "Target0",
    }
}
// Holds everything needed to render sources with gfx-rs.
pub struct Canvas<R: gfx::Resources>{
    pub window:  glutin::Window,
    pub device:  gfx_device_gl::Device,
    pub factory: gfx_device_gl::Factory, 
    pub encoder: gfx::Encoder<gfx_device_gl::Resources,gfx_device_gl::CommandBuffer>,
    pub e_loop:  glutin::EventsLoop,
    pub height:  u16,
    pub width:   u16,
    pub tex:     gfx::handle::Texture<gfx_device_gl::Resources, R8_G8_B8_A8>,
    pub iic:     gfx::texture::ImageInfoCommon<Format>,
    pub bundle:  Bundle<R, pipe::Data<R>>,
}

#[derive(Debug)]
pub enum CanvasError {
    None,
    UnableToCreatePSO(String),
    UnableToCreateUBuf(CreationError),
    UnableToUpdateUBuf,
    UnableToCreateDBuf(CreationError),
    UnableToCreateTexture(gfx::texture::CreationError),
    UnableToViewTextureAsShaderResource(gfx::ResourceViewError),
}

impl<R> Canvas<R> where R: gfx::Resources {
//  Create a new Canvas with a specific size.  
    pub fn new(height: u16, width: u16, wb: WindowBuilder) -> Result<Canvas<gfx_device_gl::Resources>, CanvasError> {

        let e_loop = glutin::EventsLoop::new();
        let (window, device, mut factory, rtv, stv) =
            gfx_window_glutin::init::<Rgba8, DepthStencil>(wb, &e_loop);
        let pso = match factory.create_pipeline_simple(
            include_bytes!("shaders/screen_150.glslv"),
            include_bytes!("shaders/screen_150.glslf"),
            pipe::new()
        )                                                                                                             {
            Ok(x)  => x,
            Err(e) => { return Err(CanvasError::UnableToCreatePSO(e.to_string())) },
        };
        let encoder: gfx::Encoder<_,_> = factory.create_command_buffer().into();
//      TODO: Make sure this is right.
//      vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
        let tex = match factory.create_texture::<R8_G8_B8_A8>(
            Kind::D2(width, height, AaMode::Single),
            1,
            Bind::all(),
            Usage::Dynamic,
            Some(ChannelType::Srgb)
        )                                                                                                             {
            Ok(x)  => x,
            Err(e) => { return Err(CanvasError::UnableToCreateTexture(e)) },
        };
        let iic: ImageInfoCommon<Format> = ImageInfoCommon                                                            {
            xoffset: 0,
            yoffset: 0,
            zoffset: 0,
            width:   width,
            height:  height,
            depth:   0,
            format:  Format(SurfaceType::R8_G8_B8_A8, ChannelType::Srgb),
            mipmap:  0,
        };
        let srv = match factory.view_texture_as_shader_resource::<(R8_G8_B8_A8, Srgb)>(&tex, (0,0), Swizzle::new())   {
            Ok(x)  => x,
            Err(e) => { return Err(CanvasError::UnableToViewTextureAsShaderResource(e)) },
        };
        let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&SCREEN, ());
        let data = pipe::Data                                                                                         {
            vbuf: vertex_buffer,
            tex:  (srv, factory.create_sampler_linear()),
            out:  rtv,
        };
        let bundle = Bundle { slice, pso, data };
        Ok(Canvas { window, device, factory, encoder, e_loop, height, width, tex, iic, bundle     })
    }
//  Update the texture with the upload buffer.  Should probably be called after update_ubuf().
    pub fn update_tex(&mut self, upload_buf: &Buffer<gfx_device_gl::Resources,u8>) {
        self.encoder.copy_buffer_to_texture_raw(upload_buf.raw(), 0, &self.tex.raw(), None, self.iic.clone());
    }
//  Updates the dbuf with the texture.  If using the Canvas for an editor you might consider updating this after
//  calling update_tex.
    pub fn update_dbuf(&mut self, download_buf: &Buffer<gfx_device_gl::Resources,u8>) {
        self.encoder.copy_texture_to_buffer_raw(&self.tex.raw(), None, self.iic.clone(), download_buf.raw(), 0);
    }
}
