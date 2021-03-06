
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


/*struct Maxik2D
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

*/


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
            vihod[i * 8 + j] = 0f32;
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
            matr1.push(pixels[i].r);
            matg1.push(pixels[i].g);
            matb1.push(pixels[i].b);
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






let mut i = 0;
let mut j = 0;

let mut vhodr = Vec::new();
vhodr = vec![0f32; 8 * 8];

let mut vhodg = Vec::new();
vhodg = vec![0f32; 8 * 8];

let mut vhodb = Vec::new();
vhodb = vec![0f32; 8 * 8];

let mut vihodr = Vec::new();
vihodr = vec![0f32; 8 * 8];
let mut vihodg = Vec::new();
vihodg = vec![0f32; 8 * 8];
let mut vihodb = Vec::new();
vihodb = vec![0f32; 8 * 8];









    let src = r#"
        __kernel void add(__global unsigned char* A, __global float* B, int m) 
        {
            __local float Blo[64];
            int x = get_local_id(0);
            int y = get_local_id(1);
            int i = get_global_id(0);
            int j = get_global_id(1);

            Blo[x * 8 + y] = (float)A[i * m + j];




            barrier(CLK_LOCAL_MEM_FENCE);

            float BB = 0;

            for (int xx = 0; xx < 8; ++xx)
                for (int yy = 0; yy < 8; ++yy)
                {
    
                        float c = (2 * xx + 1) * x * 3.1415926535 / 16;
                        float cc = (2 * yy + 1) * y * 3.1415926535 / 16;
                        c = cos(c);
                        cc = cos(cc);
                        BB += Blo[xx * 8 + yy] * c * cc;

                }

            float Ci, Cj;
            if (x == 0)
                Ci = 1 / 1.4142135623;
            else
                Ci = 1;

            if (y == 0)
                Cj = 1 / 1.4142135623;
            else
                Cj = 1;
            if (x < 2 && y < 2)
                B[i * m + j] = Ci * Cj / 4 * BB;
            else
                B[i * m + j] = 0;
        }
    "#;

    

    let src1 = r#"
        __kernel void add1(__global float* A, __global float* BBB, __global float* B, int m, int n) 
        {
            __local float Blo[64];
            int x = get_local_id(0);
            int y = get_local_id(1);
            int i = get_global_id(0);
            int j = get_global_id(1);
            int k = get_global_id(2);
            i += k / 8;
            j += k % 8;

            if (i >= n || j >= m) return;


            Blo[x * 8 + y] = A[i * m + j];




            barrier(CLK_LOCAL_MEM_FENCE);

            float BB = 0;

            for (int xx = 0; xx < 8; ++xx)
                for (int yy = 0; yy < 8; ++yy)
                {
    
                        float c = (2 * xx + 1) * x * 3.1415926535 / 16;
                        float cc = (2 * yy + 1) * y * 3.1415926535 / 16;
                        c = cos(c);
                        cc = cos(cc);
                        BB += Blo[xx * 8 + yy] * c * cc;

                }

            float Ci, Cj;
            if (x == 0)
                Ci = 1 / 1.4142135623;
            else
                Ci = 1;

            if (y == 0)
                Cj = 1 / 1.4142135623;
            else
                Cj = 1;
            B[k * m * n + i * m + j] = Ci * Cj / 4 * BB;

            barrier(CLK_LOCAL_MEM_FENCE);

            i = get_global_id(0);
            j = get_global_id(1);

            float summ = 0;
            for (int ii = 0; ii < 64; ++ii)
                summ += B[ii * m * n + i * m + j];
            BBB[i * m + j] = summ / 64;
            
        }
    "#;







    let pro_que = ProQue::builder().src(src).dims((hi, wi)).build().unwrap();

   let matr = Buffer::builder()
        .queue(pro_que.queue().clone())
        .flags(MemFlags::new().read_only().use_host_ptr())
        .dims((hi, wi))
        .host_data(&matr1)
        .build().unwrap();

        let matg = Buffer::builder()
        .queue(pro_que.queue().clone())
        .flags(MemFlags::new().read_only().use_host_ptr())
        .dims((hi, wi))
        .host_data(&matg1)
        .build().unwrap();

        let matb = Buffer::builder()
        .queue(pro_que.queue().clone())
        .flags(MemFlags::new().read_only().use_host_ptr())
        .dims((hi, wi))
        .host_data(&matb1)
        .build().unwrap();


        let resr1 = Buffer::builder()
        .queue(pro_que.queue().clone())
        .flags(MemFlags::new().read_only().use_host_ptr())
        .dims((hi, wi))
        .host_data(&vec![0f32; hi * wi])
        .build().unwrap();

        let resg1 = Buffer::builder()
        .queue(pro_que.queue().clone())
        .flags(MemFlags::new().read_only().use_host_ptr())
        .dims((hi, wi))
        .host_data(&vec![0f32; hi * wi])
        .build().unwrap();

        let resb1 = Buffer::builder()
        .queue(pro_que.queue().clone())
        .flags(MemFlags::new().read_only().use_host_ptr())
        .dims((hi, wi))
        .host_data(&vec![0f32; hi * wi])
        .build().unwrap();

        


    let mut kernel;
    {
            let hi = hi as i32;
            let wi = wi as i32;
            kernel = pro_que.create_kernel("add").unwrap().arg_buf(&matr).arg_buf(&resr1).arg_scl(wi);
            kernel.lws((8, 8)).enq().unwrap();
            kernel = pro_que.create_kernel("add").unwrap().arg_buf(&matg).arg_buf(&resg1).arg_scl(wi);
            kernel.lws((8, 8)).enq().unwrap();
            kernel = pro_que.create_kernel("add").unwrap().arg_buf(&matb).arg_buf(&resb1).arg_scl(wi);
            kernel.lws((8, 8)).enq().unwrap();
    }

    println!("Закончелъ прямое");







    resr1.read(&mut Resr).enq().unwrap();
    resg1.read(&mut Resg).enq().unwrap();
    resb1.read(&mut Resb).enq().unwrap();

let pro_que = ProQue::builder().src(src1).dims((hi, wi, 64)).build().unwrap();


   let matr11 = Buffer::builder()
        .queue(pro_que.queue().clone())
        .flags(MemFlags::new().read_only().use_host_ptr())
        .dims((hi, wi))
        .host_data(&Resr)
        .build().unwrap();

        let matg11 = Buffer::builder()
        .queue(pro_que.queue().clone())
        .flags(MemFlags::new().read_only().use_host_ptr())
        .dims((hi, wi))
        .host_data(&Resg)
        .build().unwrap();

        let matb11 = Buffer::builder()
        .queue(pro_que.queue().clone())
        .flags(MemFlags::new().read_only().use_host_ptr())
        .dims((hi, wi))
        .host_data(&Resb)
        .build().unwrap();

        let bor1 = Buffer::builder()
        .queue(pro_que.queue().clone())
        .flags(MemFlags::new().read_write().use_host_ptr())
        .dims((hi, wi, 64))
        .host_data(&vec![0f32; hi * wi * 64])
        .build().unwrap();

        let bog1 = Buffer::builder()
        .queue(pro_que.queue().clone())
        .flags(MemFlags::new().read_write().use_host_ptr())
        .dims((hi, wi, 64))
        .host_data(&vec![0f32; hi * wi * 64])
        .build().unwrap();

        let bob1 = Buffer::builder()
        .queue(pro_que.queue().clone())
        .flags(MemFlags::new().read_write().use_host_ptr())
        .dims((hi, wi, 64))
        .host_data(&vec![0f32; hi * wi * 64])
        .build().unwrap();


    let resr11 = pro_que.create_buffer::<f32>().unwrap();
    let resg11 = pro_que.create_buffer::<f32>().unwrap();
    let resb11 = pro_que.create_buffer::<f32>().unwrap();

    let mut kernel;
    {
            let wi = wi as i32;
            let hi = hi as i32;
            kernel = pro_que.create_kernel("add1").unwrap().arg_buf(&matr11).arg_buf(&resr11).arg_buf(&bor1).arg_scl(wi).arg_scl(hi);
            kernel.lws((8, 8)).enq().unwrap();
            kernel = pro_que.create_kernel("add1").unwrap().arg_buf(&matg11).arg_buf(&resg11).arg_buf(&bog1).arg_scl(wi).arg_scl(hi);
            kernel.lws((8, 8)).enq().unwrap();
            kernel = pro_que.create_kernel("add1").unwrap().arg_buf(&matb11).arg_buf(&resb11).arg_buf(&bob1).arg_scl(wi).arg_scl(hi);
            kernel.lws((8, 8)).enq().unwrap();
    }

    resr11.read(&mut Resr).enq().unwrap();
    resg11.read(&mut Resg).enq().unwrap();
    resb11.read(&mut Resb).enq().unwrap();





















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