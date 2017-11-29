
extern crate jpeg_decoder as jpeg;
extern crate rgb;
extern crate image;
extern crate ocl;




use rgb::*;
use std::fs::File;
use std::io::BufReader;
use std::ops::{Index, IndexMut};
use image::jpeg::JPEGEncoder;
use image::ColorType;
use ocl::ProQue;


struct Maxik2D
{
    V: Vec<f32>,
    wid: usize
}

struct Resa
{
    R: Vec<u8>,
    G: Vec<u8>,
    B: Vec<u8>
}

impl Resa
{
    fn new(V1: Vec<u8>, V2: Vec<u8>, V3: Vec<u8>) -> Resa
    {
        Resa {R: V1, G: V2, B: V3}
    }
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




fn C(x: usize) -> f32
{
    if x == 0
    {
        return 1.0 / 1.4142135623;
    }
    else
    {
        return 1.0;
    }
}


fn dct1(vhod :&mut[f32], vihod :&mut[f32])
{
    for i in 0..8
    {
        for j in 0..8
        {
            for x in 0..8
            {
                for y in 0..8
                {
                    let mut c = (2.0 * x as f32 + 1.0) * i as f32 * ::std::f32::consts::PI / 16.0;
                    let mut cc = (2.0 * y as f32 + 1.0) * j as f32 * ::std::f32::consts::PI / 16.0;
                    c = c.cos();
                    cc = cc.cos();
                    vihod[i * 8 + j] += vhod[x * 8 + y] * c * cc;
                }
            }
            vihod[i * 8 + j] *= C(i) * C(j) / 4.0;
        }
    }
}





fn dct(vhod :&mut[f32], vihod :&mut[f32])
{
    let mut a1 = 0.0;
    let mut a2 = 0.0;
    let mut a3 = 0.0;
    let mut a4 = 0.0;
    for i in 0..4
    {
        for j in 0..4
        {
            a1 += vhod[i * 8 + j];
            a2 += vhod[i * 8 + (j + 4)];
            a3 += vhod[(i + 4) * 8 + j];
            a4 += vhod[(i + 4) * 8 + (j + 4)];
        }
    }
    a1 /= 16.0;
    a2 /= 16.0;
    a3 /= 16.0;
    a4 /= 16.0;
    vihod[0] = a1;
    vihod[1] = a2;
    vihod[2] = a3;
    vihod[3] = a4;
}


fn tcd(vhod :&mut[f32], vihod :&mut[f32])
{
    for i in 0..4
    {
        for j in 0..4
        {
            vihod[i * 8 + j] += vhod[0];
            vihod[i * 8 + (j + 4)] = vhod[1];
            vihod[(i + 4) * 8 + j] = vhod[8];
            vihod[(i + 4) * 8 + (j + 4)] = vhod[9];
        }
    }
}





fn mani(x: usize, y: usize, wi: usize, hi: usize, matr1: Vec<f32>, matg1: Vec<f32>, matb1: Vec<f32>) -> Resa
{
    let mut Resr0 = Vec::new();
    let mut Resg0 = Vec::new();
    let mut Resb0 = Vec::new();

    let mut Resr1 = Vec::new();
    let mut Resg1 = Vec::new();
    let mut Resb1 = Vec::new();

    let mut Resr2 = Vec::new();
    let mut Resg2 = Vec::new();
    let mut Resb2 = Vec::new();

    let mut Resr3 = Vec::new();
    let mut Resg3 = Vec::new();
    let mut Resb3 = Vec::new();


    let mut a = 0;
    let mut b = 0;

    let mut xx = 0;
    let mut yy = 0;
    if (x != 0) 
    {
        xx = 8;
    }
    if (y != 0) 
    {
        yy = 8;
    }


    while a < hi - xx
    {
        while b < wi - yy
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
                    vhodr.push(matr1[(i + x) * wi + (j + y)]);
                    vhodg.push(matg1[(i + x) * wi + (j + y)]);
                    vhodb.push(matb1[(i + x) * wi + (j + y)]);
                                                            
                }
            }


            dct(&mut vhodr, &mut vihodr);
            dct(&mut vhodg, &mut vihodg);
            dct(&mut vhodb, &mut vihodb);


            Resr0.push(vihodr[0]);
            Resr1.push(vihodr[1]);
            Resr2.push(vihodr[2]);
            Resr3.push(vihodr[3]);

            Resg0.push(vihodg[0]);
            Resg1.push(vihodg[1]);
            Resg2.push(vihodg[2]);
            Resg3.push(vihodg[3]);

            Resb0.push(vihodb[0]);
            Resb1.push(vihodb[1]);
            Resb2.push(vihodb[2]);
            Resb3.push(vihodb[3]);

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
    let mut k = 0;




    while i < hi - xx
    {
        while j < wi - yy
        {
                ar[(i + x) * wi + (j + y)] = Resr0[k];
                ag[(i + x) * wi + (j + y)] = Resg0[k];
                ab[(i + x) * wi + (j + y)] = Resb0[k];
                j += 8;
                k += 1;
        }
        i += 8;
        j = 0;
    }



    i = 0;
    j = 1;
    k = 0;
    while i < hi - xx
    {
        while j < wi - yy
        {
                ar[(i + x) * wi + (j + y)] = Resr1[k];
                ag[(i + x) * wi + (j + y)] = Resg1[k];
                ab[(i + x) * wi + (j + y)] = Resb1[k];
                j += 8;
                k += 1;
        }
        i += 8;
        j = 1;
    }






    i = 1;
    j = 0;
    k = 0;
    while i < hi - xx
    {
        while j < wi - yy
        {
                ar[(i + x) * wi + (j + y)] = Resr2[k];
                ag[(i + x) * wi + (j + y)] = Resg2[k];
                ab[(i + x) * wi + (j + y)] = Resb2[k];
                j += 8;
                k += 1;
        }
        i += 8;
        j = 0;
    }


    i = 1;
    j = 1;
    k = 0;
    while i < hi - xx
    {
        while j < wi - yy
        {
                ar[(i + x) * wi + (j + y)] = Resr3[k];
                ag[(i + x) * wi + (j + y)] = Resg3[k];
                ab[(i + x) * wi + (j + y)] = Resb3[k];
                j += 8;
                k += 1;
        }
        i += 8;
        j = 1;
    }


    let mut Resr = Vec::new();
    let mut Resg = Vec::new();
    let mut Resb = Vec::new();

    Resr = vec![0u8; wi * hi];
    Resg = vec![0u8; wi * hi];
    Resb = vec![0u8; wi * hi];


    a = 0;
    b = 0;
    while a < hi - xx
    {
        while b < wi - yy
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
                    vhodr.push(ar[(i + x) * wi + (j + y)]);
                    vhodg.push(ag[(i + x) * wi + (j + y)]);
                    vhodb.push(ab[(i + x) * wi + (j + y)]);
                                                            
                }
            }

              

              


            tcd(&mut vhodr, &mut vihodr);
            tcd(&mut vhodg, &mut vihodg);
            tcd(&mut vhodb, &mut vihodb);

                    //println!("{:?}", vihodr);

            for i in a..a + 8
            {
                for j in b..b + 8
                {
                    Resr[(i + x) * wi + (j + y)] = vihodr[(i - a) * 8 + (j - b)] as u8;
                    Resg[(i + x) * wi + (j + y)] = vihodg[(i - a) * 8 + (j - b)] as u8;
                    Resb[(i + x) * wi + (j + y)] = vihodb[(i - a) * 8 + (j - b)] as u8;                                                       
                }
            }
            b += 8;
        }
        a += 8;
        b = 0;
    }

    let mut R = Resa::new(Resr, Resg, Resb);

    return R;

}











fn main()
{








/*



    let src = r#"
        __kernel void add(__global float* buffer, float scalar) {
            buffer[get_global_id(0)] += scalar;
        }
    "#;

    let pro_que = ProQue::builder().src(src).dims(1 << 20).build().unwrap();

    let buffer = pro_que.create_buffer::<f32>().unwrap();

    let kernel = pro_que.create_kernel("add").unwrap().arg_buf(&buffer).arg_scl(117.0f32);

    kernel.enq().unwrap();

    let mut vec = vec![0.0f32; buffer.len()];
    buffer.read(&mut vec).enq().unwrap();

    println!("The value at index [{}] is now '{}'!", 200007, vec[200007]);


*/




















    let file = File::open("oo.jpeg").expect("failed to open file");
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
            matr1.push(pixels[i].r as f32);
            matg1.push(pixels[i].g as f32);
            matb1.push(pixels[i].b as f32);
        }
    }

    wi -= a1;
    hi -= a2;


    let mut Resr = Vec::new();
    Resr = vec![0u64; wi * hi];
    let mut Resg = Vec::new();
    Resg = vec![0u64; wi * hi];
    let mut Resb = Vec::new();
    Resb = vec![0u64; wi * hi];


    println!("PreMani");

    for i in 0..8
    {
        for j in 0..8
        {
            let R = mani(i, j, wi, hi, matr1.clone(), matg1.clone(), matb1.clone());
            for ii in 0..hi
            {
                for jj in 0..wi
                {
                    Resr[ii * wi + jj] += R.R[ii * wi + jj] as u64;
                    Resg[ii * wi + jj] += R.G[ii * wi + jj] as u64;
                    Resb[ii * wi + jj] += R.B[ii * wi + jj] as u64;
                }
            }
        }
    }


    for i in 0..hi
    {
        for j in 0..wi
        {
            Resr[i * wi + j] /= 64;
            Resg[i * wi + j] /= 64;
            Resb[i * wi + j] /= 64;
        }
    }



    let mut Res = Vec::new();
    let mut pixel;

    for i in 0..hi
    {
        for j in 0..wi
        {
            pixel = RGB8 {r: Resr[i * wi + j] as u8, g: Resg[i * wi + j] as u8, b: Resb[i * wi + j] as u8};
            Res.push(pixel);
        }
    }

    let Res = Res.as_bytes();

    let mut fily = File::create("hhout.jpeg").expect("failed to open file");

    let mut enc = JPEGEncoder::new(&mut fily);
    JPEGEncoder::encode(&mut enc, Res, wi as u32, hi as u32, image::ColorType::RGB(8)).unwrap();

}