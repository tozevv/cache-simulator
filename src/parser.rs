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

use request::Request;

pub struct AkamaiParser {
	req: Request
}


/*impl AkamaiParser {
	fn parse(filename:String) -> request::Request {
		let req = request::Request {
	        date:date,
	        query: row[1].to_string(),
	        path: row[1].to_string(),
	        status: 200,
	        size: 100
		};
		return req;
	}
}
*/
/*
trait Parser {
    fn parse(filename:String) -> Iterator<Item=request::Request>;
}



impl Parser for AkamaiParser  {
	fn parse(filename:String) -> Iterator<Item=request::Request> {
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
            }
        }
	}
}*/
