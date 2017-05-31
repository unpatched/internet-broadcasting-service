extern crate av;
extern crate glutin;
extern crate gfx;
extern crate gfx_device_gl;
extern crate portaudio_rs as portaudio;

extern crate internetbroadcastingservice as ibs;
//  We can put all of these use statements into a prelude module so it's less "verbose".
use ibs::capture::rscam::{
    V4lConfig,
    V4lRuntimeState,
    V4l,
};

use ibs::capture::portaudio::{
    PortAudioConfig,
    PortAudio,
};
use ibs::canvas::renderer::Canvas;
use ibs::transcoder::configuration::{
    EncoderConfig,
    DecoderConfig,
    ChannelLayout,
    AudioEncoderInfo,
    VideoEncoderInfo,
    TranscodingMetaData,
    Transcoder,
    Muxers,
};

use std::thread;
use std::time::{
    Instant,
    Duration,
};
use av::common::{
    Packet,
};
use av::common::ts::{
    Ts,
};
use std::fs::{
    OpenOptions,
    File,
};
use std::sync::mpsc;

use av::video;

use av::ffi::{
    AVRational,
    AVCodecID,
    AVPixelFormat,
    AVSampleFormat,
};

use av::io::{
    AVRead,
    AVWrite,
};

use av::format::{
    Muxer,
    OutputFormat,
};

use av::ffi::AVSampleFormat::AV_SAMPLE_FMT_FLTP;

use av::audio::constants::{
    CHANNEL_LAYOUT_MONO,
    CHANNEL_LAYOUT_STEREO,
};

use gfx::{
    Device,
    Resources,
    CommandBuffer,
    Factory,
    Bind,
};

use gfx::memory::Usage;
use gfx::buffer::Role;
const CLEAR_COLOR: [f32; 4] = [1.0, 0.0, 1.0, 0.5];

#[derive(Debug, Copy, Clone)]
pub enum Status {
    None,
    Recording,
    Stopping,
}

//  Used to convert f32 slices to u8 slices.
//  useful for audio from portaudio -> ffmpeg.
fn f32_slice_as_u8_slice(slice: &[f32]) -> &[u8] {
    unsafe {
        ::std::slice::from_raw_parts(slice.as_ptr() as _, slice.len() * 4)
    }
}

fn main() {
//  While u32 these need to be <= u16::MAX.
//  This is so terrible.
    let height = 720;
//  This is so terrible.
    let width  = 1280;
//  I am ashamed.
//  Requested output format
    let output_format = "mpegts";
//  The device path to the camera.
    let device_path = "/dev/video0".to_string();
    let interval: (u32, u32) = (100, 3000);
    let format = "RGB3".to_string();
//  The encoders configuration
    let h264_enc = EncoderConfig::Video(VideoEncoderInfo::new(AVCodecID::AV_CODEC_ID_H264, (interval.0 as i32, interval.1 as i32), width as u16, height as u16, output_format.to_string()));
    let aac_enc  = EncoderConfig::Audio(AudioEncoderInfo::new(AVCodecID::AV_CODEC_ID_AAC, ChannelLayout::Stereo, 48000, AV_SAMPLE_FMT_FLTP, output_format.to_string())); 
    let mut encoderconfig_vec = Vec::<EncoderConfig>::new();
    let mut decoderconfig_vec = Vec::<DecoderConfig>::new();
    encoderconfig_vec.push(h264_enc);
    encoderconfig_vec.push(aac_enc);
//  Note: Do not worry about seperating Audio and Video encoders, this is handled for you, 
//  all encoders go in one vec while all decoders go into another vec.  It's not pretty but 
//  it "works", you must also remember that you need to pass the stream number of the encoder used 
//  to create the packet to the muxer itself.  This is not handled automagically, but it ought to be.
    let mut transcoder_metadata: TranscodingMetaData = TranscodingMetaData::prepare_transcoder("Session ID".to_string(), &encoderconfig_vec, &decoderconfig_vec).unwrap();
//  For now our destination is a file.
    let destination = OpenOptions::new().create(true).write(true).open("/home/unpatched/test3.mpegts").unwrap();
    let mut mb = match transcoder_metadata.open_mux(output_format.to_string(), destination) {
        Ok(muxer_builder) => { muxer_builder     },
        Err(e)            => { panic!("{:?}", e) },
    };
    let mut stream_num: usize = 0;

//  Now we have to configure the capture device. interval is in (num, den) format, this is 
//  basically fps (1,30) being 30fps.
    let capture_device_config = V4lConfig::new(device_path.clone(), height as u16, width as u16, format, (interval.0, interval.1));
//  Create a new v4l capture device.
    let mut capture_device_1 = V4l::new(capture_device_config);

/*  Below are the channels the threads use to communicate with one another.  It is easiest to
    think of it as a pipeline made of threads.  Each step is contained in it's own thread and they use
    "channels" to communicate.  A channel is cross thread queue so everything is guaranteed to be in
    in order.
*/  
//  The channel the capture device we just configured will use to communicate with the canvas.
    let (capture1_canvas_tx, capture1_canvas_rx) = mpsc::sync_channel::<(Instant, Vec<u8>)>(1);
//  The channel the video capture device we just configured will use to communicate with the encoders.
    let (capture1_encoder_tx, capture1_encoder_rx) = mpsc::sync_channel::<(Instant, Vec<u8>)>(1);
//  The channel the video encoder uses to communicate with the muxer.  usize is the stream id of the encoder.
    let (video_muxer_tx, video_muxer_rx) = mpsc::sync_channel::<(usize, Packet)>(1);
//  The channel the audio capture device we just configured will use to communicate with the encoders.
    let (audio_tx, audio_rx) = mpsc::sync_channel::<(AVRational, Instant, Vec<u8>)>(1);
//  The channel the audio _encoder uses to communicate with the muxer. usize is the stream id of the encoder.
    let (audio_muxer_tx, audio_muxer_rx) = mpsc::sync_channel::<(usize, Packet)>(1);

//  Get the start time of the capture, this is used to calculate the index of the frames.
    let start_time = Instant::now();
    println!("Stream Started at: {:?}", start_time);

//  Spawn a Capture Thread.
    thread::spawn(move || {
//  You must keep track of the start time of the capture.  You can also keep track of the start time of the encoder
//  you need somewhere to anchor the Ts temporally.  Ts is REQUIRED to be monotonically increasing.
//  The index of the frame at the encoder and the muxer is REQUIRED to only increment.
//  If you do not follow those REQUIREDS FFmpeg will bring the pain. No bueno.
        let capture1_start_time = Instant::now();
//  V4l devices are required to be initialized before running capture().
        capture_device_1.init(device_path);
        match capture_device_1.runtime_state {
             V4lRuntimeState::Started(mut camera) => {
                let mut swizzle_vec: Vec<u8> = Vec::<u8>::new();
                'capture_loop: loop {
//  capture() is blocking.  So this works. Should probably put this loop to sleep. TODO: Calculate duration to put thread to
//  sleep.
                    if start_time.elapsed() >= Duration::new(10,0) {
                        camera.stop();
                        break 'capture_loop
                    }
 
                    if let Ok(capture) = camera.capture() {
//                      Admittedly this for loop is not optimal, but because capture() is blocking and it's /just/
//                      heat it's not a priority to fix this.  What it does is convert RGB24 to RGBA.  It needs to be
//                      RGBA for the Canvas.  FFMPEG takes RGBA too so I just sent it to both.
                        for chunks in capture[..].chunks(3) {
                            swizzle_vec.append(&mut chunks.to_vec());
                            swizzle_vec.push(255);
                        }

                        match capture1_canvas_tx.send((capture1_start_time, swizzle_vec.as_slice().to_vec())) {
                            Ok(x)  => (),
                            Err(e) => { break 'capture_loop },
                        };
                        match capture1_encoder_tx.send((capture1_start_time, swizzle_vec.as_slice().to_vec())) {
                            Ok(x)  => { swizzle_vec.clear() },
                            Err(e) => { break 'capture_loop },
                        };
                    }
                }
                drop(capture1_canvas_tx);
                drop(capture1_encoder_tx);
            },
            _ => { panic!("Capture device was not successfully started") },
        }
    });

//  Spawn a Canvas Thread for drawing what our Capture thread sends to the sync_channel.
    thread::spawn(move || {
//      Protip: You can specify which screen to display this window on.
        let wb = glutin::WindowBuilder::new().with_title("Preview").with_dimensions(width, height);
//   Canvas is limited to u16 width/height because texture::Kind from gfx-rs,ogl,glutin etc only support
//   textures of that width/height.  The compiler will not let you compile if you don't remember this
//   but that's the reason for the casting. This cast is also not realistically unsafe but if height is >= u16::MAX
//   then you're going to have some issues.
        let mut canvas1 = match Canvas::<gfx_device_gl::Resources>::new(height as u16, width as u16, wb) {
            Ok(x)  => x,
            Err(e) => { panic!("{:?}", e) },
        };
//      We can use the upload_buffer as a "Vector to the GPU", that's Vec<u8>(Rust Vec) not vec4(...)(GPU vec).
//      It's nice and handy because you can simply convert the image you want to RGBA (Srgb colorspace) and it'll be drawn on the canvas.
//      To play video, you simply update the canvas in a loop as shown.
        let upload_buffer = match canvas1.factory.create_buffer::<u8>((height * width * 4) as usize, Role::Staging, Usage::Dynamic, Bind::all()) {
            Ok(x) => x,
            Err(e) => { panic!("{:?}", e) },
        };
//      We can use the download_buffer as a "Vector from the GPU".  It's nice because you can perfrom some blending operation and send the texture
//      as Vec<u8> to the GPU.  So it's fairly simple to interact with the GPU in this manner.
        let download_buffer = match canvas1.factory.create_buffer::<u8>((height * width * 4) as usize, Role::Staging, Usage::Dynamic, Bind::all()) {
            Ok(x)  => x,
            Err(e) => { panic!("{:?}", e) },
        };
        'canvas_loop: loop {
            match capture1_canvas_rx.recv() {
                Ok(capture) => {
                    println!("{:?}", capture.1.len());
                    canvas1.encoder.clear(&canvas1.bundle.data.out, CLEAR_COLOR);
                    canvas1.encoder.update_buffer(&upload_buffer, &capture.1, 0);
                    canvas1.update_tex(&upload_buffer);
                    canvas1.encoder.draw(&canvas1.bundle.slice, &canvas1.bundle.pso, &canvas1.bundle.data);
                    canvas1.encoder.flush(&mut canvas1.device);
                    canvas1.window.swap_buffers().unwrap();
                },
                Err(e) => { break 'canvas_loop },
            }
        }
        canvas1.device.cleanup();
    });
//  Sad! unwrap().unwrap() haha. All or nothing.  It's not a problem because this is popping a config which is gauranteed to not fail.
//  Let's revel in it's ghoulish uglyness though, and maybe point and laugh.
    let mut video_encoders = transcoder_metadata.video_encoders.unwrap().unwrap();
    let mut h264_encoder = video_encoders.pop().unwrap();
    mb.add_stream_from_encoder(&h264_encoder).unwrap();
//  TODO: Make this thread more automagic.
    thread::spawn(move || {
        let mut frame = match video::Frame::new(width as usize, height as usize, AVPixelFormat::AV_PIX_FMT_RGBA, 32) {
            Ok(x)  => x,
            Err(e) => { panic!("{:?}", e) },
        };
        let stream_id = stream_num;
        stream_num += 1;
        'video_encoding_loop: loop {
            let mut ts = Ts::new(0, AVRational { num: 1, den: 30 });
            ts.calc_index_since(start_time);
            println!("{:?}", ts.index());
            frame.set_pts(ts.index());
            if let Ok(x) = capture1_encoder_rx.recv() {
                frame.fill_channel(0, &x.1);
                if let Ok(pkts) = h264_encoder.encode(&mut frame) {
//                         ^^^^ This is an interator, in Rust, iterators are lazy
//                             You must call them in a loop or .iter() over it's contents.
//                             This effectively means that the for loop below is where the /actual/
//                             encoding takes place.
                    for pkt in pkts {
                        match video_muxer_tx.send((stream_id, pkt.unwrap())) {
                            Ok(x)  => (),
                            Err(e) => { break 'video_encoding_loop },
                        }
                    }
                }
            }
        }
        if let Ok(pkts) = h264_encoder.flush() {
            for pkt in pkts {
                if let Ok(x) = video_muxer_tx.send((stream_id, pkt.unwrap())) {
                    
                }
            }
        }
    });
    let audio_avr = AVRational { num: 1, den: 48000 };
    let pac = PortAudioConfig::new(audio_avr.clone(), 48_000.0, true, 2, 2, 0);
    thread::spawn(move || {
        let mut ts = Ts::new(0, pac.av_rational());
        let pa = match portaudio::initialize() {
            Ok(x)  => x,
            Err(e) => { panic!("Failed to Initialize PortAudio"); },
        };
        let stream = match portaudio::stream::Stream::open_default(pac.input_channel(),
                                                                   pac.output_channel(),
                                                                   pac.sample_rate(),
                                                                   0, // Set to 0, so pa can decide
                                                                   None) {
            Ok(x)  => x,
            Err(e) => { panic!("{:?}", e) },
        };
        if let Ok(ok) = stream.start() {
            'audio_read: loop {
                match stream.read(stream.num_read_available().unwrap()) {
                    Ok(x) => {
        //                println!("{:?}", x);
                        let x = ::f32_slice_as_u8_slice(&x).to_vec();
                        ts.calc_index_since(start_time);
                        match audio_tx.send((audio_avr, Instant::now(), x.to_vec())) {
                            Ok(x) => {},
                            Err(e) => {},
                            
                        }
                    },
                    Err(e) => {},
                }
            }
        }
    });
    let aac_enc = transcoder_metadata.audio_encoders.unwrap().unwrap().pop().unwrap();
//    mb.add_stream_from_encoder(&aac_enc).unwrap();
    let mut muxer = match mb.open() {
        Ok(x)  => x,
        Err(e) => { panic!("{:?}", e) },
    };
    muxer.dump_info();
    'muxer_recv_loop: loop {
        let (stream_id, packet) = match video_muxer_rx.recv() {
            Ok(recv) => { println!("Recieved Packet"); (recv.0, recv.1)    },
            Err(e)   => { muxer.close(); break 'muxer_recv_loop},
        };
        muxer.mux(packet, stream_id);
    }
}
