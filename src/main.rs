
extern crate jpeg_decoder as jpeg;
extern crate rgb;
extern crate rustdct;



use rustdct::dct2::{DCT2, DCT2ViaFFT};
use rustdct::rustfft::FFTplanner;
use rgb::*;
use std::fs::File;
use std::io::BufReader;
use std::ops::{Index, IndexMut};


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

    let mut matr1 = Vec::new();
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


    for i in 0..hi / 8
    {
        for j in 0..wi / 8
        {
            print!("{} ", Resr[i * wi / 8 + j]);
        }
    }


}