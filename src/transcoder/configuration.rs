use std::fs::File;
use std::sync::Arc;

use av::codec::Codec;
use av::format::{
    Muxer,
    OutputFormat,
};
use av::ffi::AVPixelFormat;
use av::ffi::AVSampleFormat;
use av::ffi::AVCodecID;
use av::ffi::AVRational;
use av::ffi;

use av::audio::constants::{
    CHANNEL_LAYOUT_MONO,
    CHANNEL_LAYOUT_STEREO,
};

use std::cmp::min;
use av::{
    audio,
    video,
};

use av;
use av::format::{ Demuxer, MuxerBuilder};
use av::generic::{Encoder, Decoder,Frame};
use av::errors::ResultExt;
use av::common::Ts;
use av::errors::{ Error, ErrorKind };

use av::io::{
    AVWrite,
    AVRead,
};

#[derive(Debug, Clone)]
pub enum EncoderConfig {
    Video(VideoEncoderInfo),
    Audio(AudioEncoderInfo),
    None,
}

#[derive(Debug, Clone)]
pub enum DecoderConfig {
    None,
}

#[derive(Debug, Clone)]
pub struct VideoEncoderInfo {
    pub codec_id: AVCodecID,
    pub time_base: (i32, i32),
    pub height: u16,
    pub width:  u16,
    pub output_format: String,
}

impl VideoEncoderInfo {
    pub fn new(codec_id: AVCodecID, time_base: (i32, i32), width: u16, height: u16, output_format: String) -> VideoEncoderInfo {
        VideoEncoderInfo {
            codec_id: codec_id,
            time_base: time_base,
            height: height,
            width: width,
            output_format: output_format,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ChannelLayout {
    Mono,
    Stereo,
}

#[derive(Debug, Clone)]
pub struct AudioEncoderInfo {
    pub codec_id: AVCodecID,
    pub chan_layout: ChannelLayout,
    pub sample_rate: u32,
    pub sample_fmt:  AVSampleFormat,
    pub output_format: String
}

impl AudioEncoderInfo {
    pub fn new(codec_id: AVCodecID, chan_layout: ChannelLayout, sample_rate: u32, sample_fmt: AVSampleFormat, output_format: String) -> AudioEncoderInfo {
        AudioEncoderInfo {
            codec_id,
            chan_layout,
            sample_rate,
            sample_fmt,
            output_format,
        }
    }
}

pub trait Muxers<W: AVWrite> {
    fn open_mux(&mut self, output_format: String, destinaton: W) -> av::errors::Result<MuxerBuilder>;
}
pub trait Demuxers<R: AVRead> {
    fn open_demux(source: R) -> av::errors::Result<Demuxer>;
}
#[derive(Debug)]
pub enum TranscoderError {
    InvalidEncoderConfiguration,
    InvalidAudioEncoder(av::Error),
    InvalidAudioFormat(av::Error),
    InvalidPixelFormat,
    InvalidVideoEncoder(av::Error),
}

pub struct TranscodingMetaData {
    pub name:           String,
    pub audio_encoders: Option<Result<Vec<audio::Encoder>, TranscoderError>>,
    pub video_encoders: Option<Result<Vec<video::Encoder>, TranscoderError>>,
    pub audio_decoders: Option<Result<Vec<audio::Decoder>, TranscoderError>>,
    pub video_decoders: Option<Result<Vec<video::Decoder>, TranscoderError>>,
}

pub trait Transcoder {
    fn prepare_transcoder(name: String, e_configs: &[EncoderConfig], d_configs: &[DecoderConfig]) -> Result<TranscodingMetaData, TranscoderError>;
    fn create_decoders(demux: Demuxer) -> Result<Vec<Decoder>, Error>;
}

impl Transcoder for TranscodingMetaData{
//  Prepare a transcoding session
    fn prepare_transcoder(name: String, e_configs: &[EncoderConfig], d_configs: &[DecoderConfig]) -> Result<TranscodingMetaData, TranscoderError> {
       let mut audio_enc_vec: Vec<audio::Encoder> = Vec::<audio::Encoder>::new();
       let mut video_enc_vec: Vec<video::Encoder> = Vec::<video::Encoder>::new();
       let mut audio_dec_vec: Vec<audio::Decoder> = Vec::<audio::Decoder>::new();
       let mut video_dec_vec: Vec<video::Decoder> = Vec::<video::Decoder>::new();
//     TODO: Parallelize this loop.       
       for config in e_configs {
           match config {
               &EncoderConfig::Audio(ref x) => {
                   if let Ok(ac) = Codec::find_encoder_by_id(x.codec_id) {
                       if let Some(output_format) = OutputFormat::from_name(&x.output_format) {
                           match audio::Encoder::from_codec(ac) {
                               Ok(mut ae) => {
//                                 TODO: Move this to another function
//                                                   vvvvvvvvvvvvvvvvvvvvv
                                   let chan_layout = match x.chan_layout {
                                       ChannelLayout::Mono   => CHANNEL_LAYOUT_MONO,
                                       ChannelLayout::Stereo => CHANNEL_LAYOUT_STEREO,
                                   };
                                   ae.sample_rate(x.sample_rate);
                                   ae.sample_format(x.sample_fmt);
                                   ae.channel_layout(chan_layout);
                                   match ae.open(output_format) {
                                       Ok(x)  => { audio_enc_vec.push(x); },
                                       Err(e) => { return Err(TranscoderError::InvalidAudioFormat(e)) },
                                   }
                               },
                               Err(e) => { return Err(TranscoderError::InvalidAudioEncoder(e)) },
                           };
                       };
                   };
               },
              &EncoderConfig::Video(ref x) => {
                   if let Ok(vc) = Codec::find_encoder_by_id(x.codec_id) {
                       if let Some(output_format) = OutputFormat::from_name(&x.output_format) {
                           match video::Encoder::from_codec(vc) {
                               Ok(mut ve) => {
                                   ve.width(x.width as usize);
                                   ve.height(x.height as usize);
                                   let pf = match vc.pixel_formats().first() {
                                       Some(x) => x,
                                       None    => {
                                           return Err(TranscoderError::InvalidPixelFormat)
                                       }
                                   };
                                   ve.pixel_format(*pf);
                                   ve.time_base(x.time_base);
                                   match ve.open(output_format) {
                                       Ok(x)  => { video_enc_vec.push(x); },
                                       Err(e) => { return Err(TranscoderError::InvalidVideoEncoder(e)) },
                                   }
                               },
                               Err(e) => {
                                   return Err(TranscoderError::InvalidVideoEncoder(e))
                               },
                           };
                       };
                   };
               }
               _                            => { return Err(TranscoderError::InvalidEncoderConfiguration) },
           }
        }
//      TODO: Add loop for configuring Decoders, also parallelize it.
        let audio_encoders = match audio_enc_vec.len() > 0 {
            true  => { Some(Ok(audio_enc_vec)) },
            false => { None                    },
        };
        let video_encoders = match video_enc_vec.len() > 0 {
            true  => { Some(Ok(video_enc_vec)) },
            false => { None                    },
        };
        let audio_decoders = match audio_dec_vec.len() > 0 {
            true  => { Some(Ok(audio_dec_vec)) },
            false => { None                    },
        };
        let video_decoders = match video_dec_vec.len() > 0 {
            true  => { Some(Ok(video_dec_vec)) },
            false => { None                    },
        };

        Ok(TranscodingMetaData {
            name,
            audio_encoders,
            video_encoders,
            audio_decoders,
            video_decoders,
        })
    }
    fn create_decoders(demux: Demuxer) -> Result<Vec<Decoder>, Error> {
        demux.streams().map(|stream| Decoder::from_stream(&stream)).collect::<av::Result<Vec<Decoder>>>()
    }
}

impl<W> Muxers<W> for TranscodingMetaData where W: AVWrite {
    fn open_mux(&mut self, output_format: String, destination: W) -> av::errors::Result<MuxerBuilder> {
        match OutputFormat::from_name(&output_format) {
            Some(x) => {
                    Muxer::new(x, destination)
            },
            None => { Err(Error::from_kind(ErrorKind::Msg("Unable to use output format.".to_string()))) },
        }
    }
}

impl<R> Demuxers<R> for TranscodingMetaData where R: AVRead {
    fn open_demux(source: R) -> av::errors::Result<Demuxer> {
        Demuxer::open(source)
    }
}
                                                                            
