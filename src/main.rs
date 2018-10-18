extern crate chrono;

use std::env;
use std::io;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::error::Error;
use chrono::prelude::*;
use chrono::DateTime;
use chrono::Utc;

mod request;
mod parser;

use request::Request;
use parser::AkamaiParser;

//mod parser;



#[allow(dead_code)]
// struct Simulation
// {
//     cacheTTL: u32,
//     hits: u64,
//     misses: u64,
//     stales: u64
// }

// impl Simulation 
// {
//     pub fn simulate(request:Request) {

//     }
// }


// struct Parser {}

// impl Parser() 
// {
//     pub fn parse(filename:String) {

//     }
// }


// fn parserow(String:row):
//         if len(row) >= 10 and row[4] == "GET":
//             datestr=row[0]
//             timestr=row[1]
//             # strptime is very slow, using faster but stricter parsing.
//             # equivalent to datetime.strptime(datestr + " " + timestr, "%Y-%m-%d %H:%M:%S")
//             date = datetime(int(datestr[:4]), int(datestr[5:7]), int(datestr[8:10]), 
//                 int(timestr[:2]),int(timestr[3:5]),int(timestr[6:8]),0,None) 
//             request = Request(date, row[5], row[6], row[7], row[8])
//             return request

//     def parse(self):
//         with open(self.file, 'r') as csvfile:
//             spamreader = csv.reader(csvfile, delimiter=' ', quotechar='|')
//             for row in spamreader:
//                 yield self.__parserow(row)


// fn parsefile(filename: String)  -> Result<(), Box<Error>> {
//     let file = File::open(filename).unwrap();
//     let reader = BufReader::new(file);
    
//     let mut rdr = csv::ReaderBuilder::new()
//         .delimiter(b' ')
//         .has_headers(true)
//         .from_reader(reader);

//     for result in rdr.records() {
//         // Notice that we need to provide a type hint for automatic
//         // deserialization.
//         let record = result?;
//         println!("Here: {:?}", record);
//     }
//     Ok(())
// }

//2017-07-31 03:37:39 81.132.178.144 - GET /entity/search/v1/skyqstb/home/4101/1/user/programme/e7d0db18-48af-4f1b-ab1c-340f16ea59f7 src=linear&src=svod&src=cup&src=store&src=est&flag=uhd 200 1100 SkyQ_STB/5.99.04.18;%2032B061



fn main() {
	let args: Vec<_> = env::args().collect();
	let files: &[String] = &args[1..];
    let mut processed = 0;

    for filename in files {
    	println!("Loading {} ...", filename);
        
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        
        for l in reader.lines() {

            let line = l.unwrap();
            let row = line.split(" ").collect::<Vec<&str>>();

            if row.len() >= 10 && row[4] == "GET" {
                let datestr = row[0];
                let timestr = row[1];
                
                let date = Utc.ymd(datestr[..4].parse::<i32>().unwrap(), 
                                   datestr[5..7].parse::<u32>().unwrap(), 
                                   datestr[8..10].parse::<u32>().unwrap())
                          .and_hms(timestr[..2].parse::<u32>().unwrap(),
                                   timestr[3..5].parse::<u32>().unwrap(),
                                   timestr[6..8].parse::<u32>().unwrap());
                let req = request::Request {
                    date:date,
                    query: row[1].to_string(),
                    path: row[1].to_string(),
                    status: 200,
                    size: 100
                };

              /*  println!("{}, {} time: {} url:{} {}?{} size:{}", 
                    date, row[0], row[1], row[4], row[5], row[6], row[7]);*/
            }

            
            processed = processed + 1;
            if processed % 100000 == 0 {
                println!("Processing {} requests...", processed)
            }
        }


    	//let x = parsefile(filename.to_string());
        println!("done");

        let x = parser::AkamaiParser {};
	}

}
