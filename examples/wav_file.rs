use pekoms::Parser;

mod bin_parsers;

const RIFF_HEADER : &[u8] = &[0x52, 0x49, 0x46, 0x46];
const WAVE_HEADER : &[u8] = &[0x57, 0x41, 0x56, 0x45];
const FMT_HEADER : &[u8] = &[0x66, 0x6D, 0x74, 0x20];
const DATA_HEADER : &[u8] = &[0x64, 0x61, 0x74, 0x61];


fn chunk(header:&'static [u8]) -> impl Fn(&[u8]) -> Option<(&[u8],&[u8])> {
  |input| {
    input.strip_prefix(header)
    .and_then(|r| bin_parsers::u32(r))
    .and_then(|(sz,r)|bin_parsers::fixed_len(sz as usize).parse(r))
  }
}

fn riff_chunk(input:&[u8]) -> Option<(&[u8],&[u8])> {
  chunk(RIFF_HEADER).parse(input)
}

#[derive(Debug)]
pub struct WavInfo {
  pub fmt:u16,
  pub channels:u16,
  pub samps_per_sec:u32,
  pub bytes_per_sec:u32,
  pub block_alignment:u16,
  pub bits_per_sample:u16
}

impl From<(u16,u16,u32,u32,u16,u16)> for WavInfo {
  fn from((f,c,sps,byps,blka,bps):(u16,u16,u32,u32,u16,u16)) -> Self {
    Self {
      fmt:f,
      channels:c,
      samps_per_sec:sps,    
      bytes_per_sec:byps,  
      block_alignment:blka,
      bits_per_sample:bps
    }
  }
}

//this exists mostly to shut up a "very complex type" warning if you give it
//a tuple with an info struct and the data bytes.
type WavData<'a> = (WavInfo,&'a[u8]);

fn fmt_chunk(input:&[u8]) -> Option<(WavInfo,&[u8])> {
  use bin_parsers::{
    u16_le,
    u32_le
  };

  chunk(FMT_HEADER)
  .and_then(|c|{
    let seq = (u16_le,u16_le,u32_le,u32_le,u16_le,u16_le);
    seq.parse(c).map(|(nfo,_rem)|nfo.into())
  })
  .parse(input)
}

fn data_chunk(input:&[u8]) -> Option<(&[u8],&[u8])> {
  chunk(DATA_HEADER).parse(input)
}

fn wave_chunk(input:&[u8]) -> Option<(WavInfo,&[u8])> {
  let seq = (bin_parsers::pfx(WAVE_HEADER),fmt_chunk,data_chunk);
  seq.map(|(_,fmt,data)|(fmt,data)).parse(input).map(|((f,d),_r)|(f,d))
}

fn parse_wav(input:&[u8]) -> Option<(WavData,&[u8])> {
  riff_chunk.and_then(wave_chunk).parse(input)
}

fn get_wav() -> Vec<u8> {
  use std::io::prelude::*;
  use std::fs::File;

  let mut f = File::open("./examples/assets/neusnare.wav").expect("couldn't open file");
  let mut buffer = Vec::new();

  f.read_to_end(&mut buffer).expect("couldn't read file");
  buffer
}

fn main() {
  let buff = get_wav();
  
  if let Some(((nfo,data),rest)) = parse_wav(&buff[..]) {
    println!("wow it worked");
    println!("wave nfo {:?}",nfo);
    println!("how much data?: {} bytes",data.len());
    println!("anything left over? {:?}",rest);
  }
  else {
    println!("it didn't work");
  }
}
