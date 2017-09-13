
extern crate jpeg_decoder as jpeg;

use std::fs::File;
use std::io::BufReader;




fn main()
{
    let file = File::open("hh.jpeg").expect("failed to open file");
    let mut decoder = jpeg::Decoder::new(BufReader::new(file));
    let pixels = decoder.decode().expect("failed to decode image");
    let metadata = decoder.info().unwrap();

    let hi = metadata.height as usize;
    let wi = metadata.width as usize;

    let mut mat1 = Vec::new();
    let mut mat = Vec::new();
    
    for i in 0..hi
    {
        for j in 0..wi
        {
            mat1.push(pixels[i * wi + j]);
        }
        mat.push(mat1.clone());
        mat1.clear();
    }


    for i in 0..hi
    {
        for j in 0..wi
        {
            print!("{}", mat[i][j]);
        }
        println!("");
    }

}