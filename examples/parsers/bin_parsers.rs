use std::convert::TryInto;

type BinMatch<'a,'b> = (&'a[u8],&'b[u8]);

pub const fn fixed_len(len:usize) -> impl Fn(&[u8]) -> Result<BinMatch,()> {
  move |input|{
    if input.len() >= len {
      Ok((&input[0..len],&input[len..]))
    }
    else {
      Err(())
    }
  }
}

pub const fn pfx(p:&'static [u8]) -> impl Fn(&[u8]) -> Result<BinMatch,()> {
  move |input| {
    input.strip_prefix(p).map(|r|(p,r)).ok_or(())
  }
}

//this is awkward until we have something like concat_idents in std
macro_rules! number_converter {
  ($fn_name:ident, $NT:ty) => {
    pub fn $fn_name(input:&[u8]) -> Result<($NT,&[u8]),()> {
      const SZ : usize = std::mem::size_of::<$NT>();

      let (btz,rest) = input.split_at(SZ);
      let bz = btz.try_into();
      bz.map(<$NT>::from_ne_bytes)
      .map(|u|(u,rest))
      .map_err(|_|())
    }
  };
  
  ($fn_name:ident, $NT:ty, big_endian) => {
    pub fn $fn_name(input:&[u8]) -> Result<($NT,&[u8]),()> {
      const SZ : usize = std::mem::size_of::<$NT>();

      let (btz,rest) = input.split_at(SZ);
      let bz = btz.try_into();
      bz.map(<$NT>::from_be_bytes)
      .map(|u|(u,rest))
      .map_err(|_|())
    }
  };

  ($fn_name:ident, $NT:ty, little_endian) => {
    pub fn $fn_name(input:&[u8]) -> Result<($NT,&[u8]),()> {
      const SZ : usize = std::mem::size_of::<$NT>();

      let (btz,rest) = input.split_at(SZ);
      let bz = btz.try_into();
      bz.map(<$NT>::from_le_bytes)
      .map(|u|(u,rest))
      .map_err(|_|())
    }
  }
}

number_converter!(u8,u8);
number_converter!(u8_be,u8, big_endian);
number_converter!(u8_le,u8, little_endian);

number_converter!(u16,u16);
number_converter!(u16_be,u16, big_endian);
number_converter!(u16_le,u16, little_endian);

number_converter!(u32,u32);
number_converter!(u32_be,u32, big_endian);
number_converter!(u32_le,u32, little_endian);

number_converter!(u64,u64);
number_converter!(u64_be,u64, big_endian);
number_converter!(u64_le,u64, little_endian);

number_converter!(i8,i8);
number_converter!(i8_be,i8, big_endian);
number_converter!(i8_le,i8, little_endian);

number_converter!(i16,i16);
number_converter!(i16_be,i16, big_endian);
number_converter!(i16_le,i16, little_endian);

number_converter!(i32,i32);
number_converter!(i32_be,i32, big_endian);
number_converter!(i32_le,i32, little_endian);

number_converter!(i64,i64);
number_converter!(i64_be,i64, big_endian);
number_converter!(i64_le,i64, little_endian);

number_converter!(f32,f32);
number_converter!(f32_be,f32, big_endian);
number_converter!(f32_le,f32, little_endian);

number_converter!(f64,f64);
number_converter!(f64_be,f64, big_endian);
number_converter!(f64_le,f64, little_endian);
