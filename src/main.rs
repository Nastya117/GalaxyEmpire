
extern crate jpeg_decoder as jpeg;
extern crate rgb;
extern crate rustdct;
extern crate image;




use rustdct::dct2::{DCT2, DCT2ViaFFT};
use rustdct::dct3::{DCT3, DCT3ViaFFT};
use rustdct::rustfft::FFTplanner;
use rgb::*;
use std::fs::File;
use std::io::BufReader;
use std::ops::{Index, IndexMut};
use image::jpeg::JPEGEncoder;
use image::ColorType;


struct Maxik2D
{
    V: Vec<f32>,
    wid: usize
}

impl Maxik2D
{
    fn new(V: Vec<f32>, wid: usize) -> Maxik2D
    {
        Maxik2D {V: V, wid: wid}
    }
}

impl Index<usize> for Maxik2D
{
    type Output = [f32];

    fn index(&self, i: usize) -> &[f32]
    {
        return &self.V[i * self.wid..(i + 1) * self.wid]
    }
}


impl IndexMut<usize> for Maxik2D
{
    fn index_mut(&mut self, i: usize) -> &mut[f32]
    {
        return &mut self.V[i * self.wid..(i + 1) * self.wid]
    }
}



fn main()
{
    let file = File::open("hh.jpeg").expect("failed to open file");
    let mut decoder = jpeg::Decoder::new(BufReader::new(file));
    let pixels = decoder.decode().expect("failed to decode image");
    let metadata = decoder.info().unwrap();
    let pixels = pixels.as_slice().as_rgb();

    let mut hi = metadata.height as usize;
    let mut wi = metadata.width as usize;

  /*  let mut matr1 = Vec::new();
    let mut matg1 = Vec::new();
    let mut matb1 = Vec::new();
    

    let a1 = wi % 8;
    let a2 = hi % 8;


    for i in 0..(hi - a2) * wi
    {
        if (i % wi < wi - a1)
        {
            matr1.push(pixels[i].r  as f32);
            matg1.push(pixels[i].g  as f32);
            matb1.push(pixels[i].b  as f32);
        }
    }

    wi -= a1;
    hi -= a2;
    let matr = Maxik2D::new(matr1, wi);
    let matg = Maxik2D::new(matg1, wi);
    let matb = Maxik2D::new(matb1, wi);

    let mut Resr = Vec::new();
    let mut Resg = Vec::new();
    let mut Resb = Vec::new();


    let mut planner = FFTplanner::new(false);
    let fft = planner.plan_fft(64);

    let mut dct = DCT2ViaFFT::new(fft);
    let fft = planner.plan_fft(64);

    let mut a = 0;
    let mut b = 0;

    while a < hi
    {
        while b < wi
        {
            let mut vihodr = vec![0f32; 64];
            let mut vhodr = Vec::new();
            let mut vihodg = vec![0f32; 64];
            let mut vhodg = Vec::new();
            let mut vihodb = vec![0f32; 64];
            let mut vhodb = Vec::new();
            for i in a..a + 8
            {
                for j in b..b + 8
                {
                    vhodr.push(matr[i][j]);
                    vhodg.push(matg[i][j]);
                    vhodb.push(matb[i][j]);
                                                            
                }
            }

            dct.process(&mut vhodr, &mut vihodr);
            dct.process(&mut vhodg, &mut vihodg);
            dct.process(&mut vhodb, &mut vihodb);

            Resr.push(vihodr[0]);
            Resg.push(vihodg[0]);
            Resb.push(vihodb[0]);

            b += 8;
        }
        a += 8;
        b = 0;
    }

    let mut ar = vec![0f32; wi * hi];
    let mut ag = vec![0f32; wi * hi];
    let mut ab = vec![0f32; wi * hi];

    let mut i = 0;
    let mut j = 0;
    while i < hi
    {
        while j < wi
        {
                ar[i * wi + j] = Resr[j / 8 + i / 8 + i];
                ag[i * wi + j] = Resg[j / 8 + i / 8 + i];
                ab[i * wi + j] = Resb[j / 8 + i / 8 + i];
                j += 8;
        }
        i += 8;
        j = 0;
    }


    let mut Resr = Vec::new();
    let mut Resg = Vec::new();
    let mut Resb = Vec::new();

    Resr = vec![0u8; wi * hi];
    Resg = vec![0u8; wi * hi];
    Resb = vec![0u8; wi * hi];

    let mut dct = DCT3ViaFFT::new(fft);

    while a < hi
    {
        while b < wi
        {
            let mut vihodr = vec![0f32; 64];
            let mut vhodr = Vec::new();
            let mut vihodg = vec![0f32; 64];
            let mut vhodg = Vec::new();
            let mut vihodb = vec![0f32; 64];
            let mut vhodb = Vec::new();
            for i in a..a + 8
            {
                for j in b..b + 8
                {
                    vhodr.push(ar[i * wi + j]);
                    vhodg.push(ag[i * wi + j]);
                    vhodb.push(ab[i * wi + j]);
                                                            
                }
            }

            dct.process(&mut vhodr, &mut vihodr);
            dct.process(&mut vhodg, &mut vihodg);
            dct.process(&mut vhodb, &mut vihodb);

            for i in a..a + 8
            {
                for j in b..b + 8
                {
                    Resr[i * wi + j] = vihodr[(i - a) * 8 + (j - b)] as u8;
                    Resg[i * wi + j] = vihodg[(i - a) * 8 + (j - b)] as u8;
                    Resb[i * wi + j] = vihodb[(i - a) * 8 + (j - b)] as u8;                                                       
                }
            }
            b += 8;
        }
        a += 8;
        b = 0;
    }

    let mut Res = Vec::new();
    let mut pixel;

    for i in 0..hi
    {
        for j in 0..wi
        {
            pixel = RGB8 {r: Resr[i * wi + j], g: Resg[i * wi + j], b: Resb[i * wi + j]};
            Res.push(pixel);
        }
    }

    let Res = Res.as_bytes();*/
    let Res = pixels.as_bytes();

    let mut fily = File::create("hhout.jpeg").expect("failed to open file");

    let mut enc = JPEGEncoder::new(&mut fily);
    JPEGEncoder::encode(&mut enc, Res, wi as u32, hi as u32, image::ColorType::RGB(8)).unwrap();

}