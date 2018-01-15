
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
use ocl::{util, ProQue, Buffer, MemFlags};


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


fn dct(vhod :&mut[f32], vihod :&mut[f32])
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






/*fn mani(x: usize, y: usize, wi: usize, hi: usize, matr1: Vec<f32>, matg1: Vec<f32>, matb1: Vec<f32>) -> Resa
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

              

              


            dct(&mut vhodr, &mut vihodr);
            dct(&mut vhodg, &mut vihodg);
            dct(&mut vhodb, &mut vihodb);

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

}*/











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
            matr1.push(pixels[i].r as f32);
            matg1.push(pixels[i].g as f32);
            matb1.push(pixels[i].b as f32);
        }
    }

    wi -= a1;
    hi -= a2;

    let mut Resr = Vec::new();
    Resr = vec![0f32; wi * hi];
    let mut Resg = Vec::new();
    Resg = vec![0f32; wi * hi];
    let mut Resb = Vec::new();
    Resb = vec![0f32; wi * hi];






 /*   let src = r#"
        __kernel void add(__global float* A, __global float* B, int n, int m, int cx, int cy) 
        {
            int xb = get_group_id(0);
            int yb = get_group_id(1);
            int kbm = m / 8;
            int kbn = n / 8;
            int i = get_global_id(0);
            int j = get_global_id(1);
            int bn = 8;
            int bm = 8;
            int BB = 0;

            if (i + cx + bn < n && j + cy + bm < m)
            {
            for (int x = 0; x < bn; ++x)
                for (int y = 0; y < bm; ++y)
                {

                        float c = (2 * (x + cx) + 1) * i * M_PI / 16;
                        float cc = (2 * (y + cy) + 1) * j * M_PI / 16;
                        c = cos(c);
                        cc = cos(cc);
                        BB += A[(xb * kbm * bm * bn) + ((x + cx) * m) + (xb * bm) + (y + cy)] * c * cc;

                }
            }
                    else
                    BB += 0;

        float Ci, Cj;
        if (get_local_id(0) - cx == 0)
            Ci = 1 / 1.4142135623;
            else
            Ci = 1;

        if (get_local_id(1) - cy == 0)
            Cj = 1 / 1.4142135623;
            else
            Cj = 1;
            if (i + cx < n && j + cy < m)
            {
                B[(i + cx) * m + j + cy] += Ci * Cj / 4 * BB;
            }

    }

    "#;
*/
    




let src = r#"
        __kernel void add(__global float* A, __global float* B, int n, int m, int cx, int cy)
        {
            int i = get_global_id(0);
            int j = get_global_id(1);
            B[i * m + j] = get_group_id(1);
    }

    "#;





    let pro_que = ProQue::builder().src(src).dims((hi, wi)).build().unwrap();

   let matr = Buffer::builder()
        .queue(pro_que.queue().clone())
        .flags(MemFlags::new().read_write().copy_host_ptr())
        .dims((hi, wi))
        .host_data(&matr1)
        .build().unwrap();

        let matg = Buffer::builder()
        .queue(pro_que.queue().clone())
        .flags(MemFlags::new().read_write().copy_host_ptr())
        .dims((hi, wi))
        .host_data(&matg1)
        .build().unwrap();

        let matb = Buffer::builder()
        .queue(pro_que.queue().clone())
        .flags(MemFlags::new().read_write().copy_host_ptr())
        .dims((hi, wi))
        .host_data(&matb1)
        .build().unwrap();


    let resr1 = pro_que.create_buffer::<f32>().unwrap();
    let resg1 = pro_que.create_buffer::<f32>().unwrap();
    let resb1 = pro_que.create_buffer::<f32>().unwrap();

    let mut kernel;
    
    for i in 0..8
    {
        for j in 0..8
        {
            kernel = pro_que.create_kernel("add").unwrap().arg_buf(&matr).arg_buf(&resr1).arg_scl(hi).arg_scl(wi).arg_scl(i).arg_scl(j);
            kernel.lws((1, 1)).enq().unwrap();
            kernel = pro_que.create_kernel("add").unwrap().arg_buf(&matg).arg_buf(&resg1).arg_scl(hi).arg_scl(wi).arg_scl(i).arg_scl(j);
            kernel.lws((1, 1)).enq().unwrap();
            kernel = pro_que.create_kernel("add").unwrap().arg_buf(&matb).arg_buf(&resb1).arg_scl(hi).arg_scl(wi).arg_scl(i).arg_scl(j);
            kernel.lws((1, 1)).enq().unwrap();

        }
    }











    /*let src = r#"
        __kernel void add1(__global float* A, __global float* B, int n, int m, int cx, int cy) 
        {
            int xb = get_group_id(0);
            int yb = get_group_id(1);
            int kbm = m / 8;
            int kbn = n / 8;
            int i = get_global_id(0);
            int j = get_global_id(1);
            int bn = 8;
            int bm = 8;
            int BB = 0;

            for (int x = 0; x < bn; ++x)
                for (int y = 0; y < bm; ++y)
                {
                    if (i + cx + bn < n && j + cy + bm < m)
                    {
                        float c = (2 * (x + cx) + 1) * i * M_PI / 16;
                        float cc = (2 * (y + cy) + 1) * j * M_PI / 16;
                        c = cos(c);
                        cc = cos(cc);
                        BB += A[(xb * kbm * bm * bn) + ((x + cx) * m) + (xb * bm) + (y + cy)] * c * cc;
                    }
                    else
                    BB += 0;
                }

        float Ci, Cj;
        if (get_local_id(0) - cx == 0)
            Ci = 1 / 1.4142135623;
            else
            Ci = 1;

        if (get_local_id(1) - cy == 0)
            Cj = 1 / 1.4142135623;
            else
            Cj = 1;
            if (i + cx < n && j + cy < m)
            {
                B[(i + cx) * m + j + cy] += Ci * Cj / 4 * BB;
            }

    }

    "#;

    

    let pro_que = ProQue::builder().src(src).dims((8, 8)).build().unwrap();



    let resrr1 = pro_que.create_buffer::<f32>().unwrap();
    let resgg1 = pro_que.create_buffer::<f32>().unwrap();
    let resbb1 = pro_que.create_buffer::<f32>().unwrap();


    let mut kernel;
    
    for i in 0..8
    {
        for j in 0..8
        {
            kernel = pro_que.create_kernel("add1").unwrap().arg_buf(&resr1).arg_buf(&resrr1).arg_scl(hi).arg_scl(wi).arg_scl(i).arg_scl(j);
            kernel.enq().unwrap();
            kernel = pro_que.create_kernel("add1").unwrap().arg_buf(&resg1).arg_buf(&resgg1).arg_scl(hi).arg_scl(wi).arg_scl(i).arg_scl(j);
            kernel.enq().unwrap();
            kernel = pro_que.create_kernel("add1").unwrap().arg_buf(&resb1).arg_buf(&resbb1).arg_scl(hi).arg_scl(wi).arg_scl(i).arg_scl(j);
            kernel.enq().unwrap();
        }
    }


*/
















    resr1.read(&mut Resr).enq().unwrap();
    resg1.read(&mut Resg).enq().unwrap();
    resb1.read(&mut Resb).enq().unwrap();


























/*

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
*/

  /*  for i in 0..hi
    {
        for j in 0..wi
        {
            Resr[i * wi + j] /= 64.0;
            Resg[i * wi + j] /= 64.0;
            Resb[i * wi + j] /= 64.0;
        }

    }
*/
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