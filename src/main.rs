use std::{fs::File, io::Write};

use comb_filter::CombFilter;

mod comb_filter;

fn show_info() {
    eprintln!("MUSI-6106 Assignment Executable");
    eprintln!("(c) 2024 Stephen Garrett & Ian Clester");
}

fn main() {
   show_info();

    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input wave filename> <output text filename>", args[0]);
        return
    }

    // Open the input wave file
    let mut reader = hound::WavReader::open(&args[1]).unwrap();
    let spec = reader.spec();
    let channels = spec.channels;

    // TODO: Modify this to process audio in blocks using your comb filter and write the result to an audio file.
    //       Use the following block size:
    
    let block_size = 1024;
    //let mut data: &mut [f32] = &mut (vec![0.0 as f32; block_size])[..];
    //let mut in_buff: &mut [&mut [f32]] =  &mut (data;channels as usize)[..];
    let mut in_buffer = &mut (vec![0.0; channels as usize * block_size])[..];
    let mut in_ref = &mut in_buffer[..];
    let mut out_buffer: Vec<f32> = vec![0.0; channels as usize * block_size];
    let mut out_ref = &mut out_buffer[..];

    let mut combs: Vec<CombFilter> = Vec::with_capacity(channels as usize);
    for i in 0..channels {
        combs.push(CombFilter::new(comb_filter::FilterType::FIR, 1.0, 44100.0, channels as usize));
    }

    //let mut filter: CombFilter = CombFilter::new(comb_filter::FilterType::FIR, 1.0, 44100.0, channels as usize);
    // Read audio data and write it to the output text file (one column per channel)
    let mut out = File::create(&args[2]).expect("Unable to create file");
    for (i, sample) in reader.samples::<i16>().enumerate() {
        let sample = sample.unwrap() as f32 / (1 << 15) as f32;
        
        in_ref[i % (block_size*channels as usize)] = sample;
        if i % (block_size * channels as usize) == 0 {
            
            for j in 0..channels as usize {
                
                comb_filter::CombFilter::process(&mut combs[j],&[in_ref],&mut [out_ref], j as usize);
            }
            
            for j in 0..out_ref.len() {
                write!(out, "{}{}", out_ref[j], if j % channels as usize == (channels - 1).into() { "\n" } else { " " }).unwrap();
            }

        }
        //for i in 0..block_size {
        
        
    }
}
