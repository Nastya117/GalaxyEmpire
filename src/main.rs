
extern crate jpeg_decoder as jpeg;
extern crate rgb;


use rgb::*;
use std::fs::File;
use std::io::BufReader;




fn main()
{
    let file = File::open("hh.jpeg").expect("failed to open file");
    let mut decoder = jpeg::Decoder::new(BufReader::new(file));
    let pixels = decoder.decode().expect("failed to decode image");
    let metadata = decoder.info().unwrap();
    let pixels = pixels.as_slice().as_rgb();

    let hi = metadata.height as usize;
    let wi = metadata.width as usize;

    let mut mat1 = Vec::new();
    let mut mat2 = Vec::new();
    let mut mat3 = Vec::new();

    let mut matr = Vec::new();
    let mut matg = Vec::new();
    let mut matb = Vec::new();
    
    for i in 0..hi
    {
        for j in 0..wi
        {
            mat1.push(pixels[i * wi + j].r);
            mat2.push(pixels[i * wi + j].g);
            mat3.push(pixels[i * wi + j].b);
        }
        matr.push(mat1.clone());
        mat1.clear();
        matg.push(mat2.clone());
        mat2.clear();
        matb.push(mat3.clone());
        mat3.clear();
    }


     for i in 0..hi
    {
        for j in 0..wi
        {
            print!("{} ", matb[i][j]);
        }
        println!("");
    }

}