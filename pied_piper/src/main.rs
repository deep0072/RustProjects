//compress the file 

// fist take file and read it
// then create output file
// encode and the compress


extern crate flate2; 

use flate2::write::GzEncoder;                   

use std::fs::File;
use std::env::args;
use std::io::BufReader; // use to read files
use std::io::copy; // copy encoded file to output file

use flate2::Compression; 
use std::time::Instant; // will used to calculate the elapsed time





fn main() {

    if args().len() !=3{
        eprintln!("Usage:`source` `target`");
        return;
    }
  

    // args().nth(index) ==> it will get the args index value 
    // at first index usafe
    // at 2nd  source file
    // at 3rd index output file which are we going to save

    let src_file = File::open(args().nth(1).unwrap()).unwrap();
    // now read opened file using buffreader

    let mut input = BufReader::new(src_file);

    // create output file
    let output_file = File::create(args().nth(2).unwrap()).unwrap();

    // GzEncoder is useful when you want to compress data before writing it to a destination 
    //such as a file or network connection

    // this will create instance of struct 
    let mut encoder = GzEncoder::new(output_file,Compression::default());
    // now start time 

    let start = Instant::now();

    // now copy  data from src file  to encoder which compress data 

    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap(); // finalise the compression
    println!("SOurce len: {:?}", input.get_ref().metadata().unwrap().len());
    println!("Output  len: {:?}", output.metadata().unwrap().len());
    println!("Elapsed: {:?}", start.elapsed());




}
