use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::env;
use std::time::{Instant, Duration};

use serde::{Serialize, Deserialize};
use std::process::exit;
use flate2::write::GzEncoder;

fn main() {
  let x: Root = init();
  let args: Vec<String> = env::args().collect();
  let file_name: &String = &args[1];

  let start_time: Instant = Instant::now();
  // lib::upload_files_to_s3(&files, "output");
  parse_file(file_name, &x);
  let end_time: Instant = Instant::now();
  let elapsed_time: Duration = end_time - start_time;
  println!("Time taken: {:?}", elapsed_time);
}

fn get_log(log_line: &String, x:&Root) -> String {
  let split_vec: Vec<String> = log_line.split(" ").map(|s| s.to_string()).collect();
  let y: &HashMap<String, String> = &x.security_groups;

  let z1: Option<&String> = y.get(&split_vec[4].to_string());
  let z2: Option<&String> = y.get(&split_vec[5].to_string());

  if z1.is_none() || z2.is_none() {
    return "".to_string();
  }

  let ret: Vec<String> = vec![
    split_vec[4].to_string(),
    z1.unwrap().to_string().replace(" ", "_"),
    split_vec[5].to_string(),
    z2.unwrap().to_string().replace(" ", "_"),
    split_vec[7].to_string(),
    split_vec[8].to_string()
  ];
  let ret2: String = ret.join(" ");
  ret2
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Root {
  #[serde(flatten)]
  pub security_groups: HashMap<String, String>,
}

/*
data2.json
{
  "10.25.59.4": "eks-cde-blue-worker-payments-spot",
	"10.25.50.186": "eks-cde-blue-worker-payments-spot"
}
*/
pub fn init() -> Root {
  let file: Result<File, std::io::Error> = File::open("./data2.json");

  if file.is_err() {
    println!("error1");
    exit(1)
  }

  let mut file_data: File = file.unwrap();

  let mut contents: String = String::new();
  file_data.read_to_string(&mut contents);
  
  let dat: Result<Root, serde_json::Error> = serde_json::from_str(&contents);
  if dat.is_err() {
    println!("{:#?}", dat.err());
    exit(1)
  }

  return dat.unwrap();
}

fn parse_file(file_name: &str, x:&Root) {
  let lines: Vec<String> = read_gz_lines(file_name);
  println!("{} lines in log file", lines.len());
  let mut ret: Vec<String> = Vec::new();

  let split_vec: Vec<String> = file_name.split("/").map(|s| s.to_string()).collect();

  let file_path = format!("{}/{}", "output", split_vec[split_vec.len()-1]);
  let ffile = File::create(file_path);
  let mut cnt: i32 = 0;

  if ffile.is_ok() {
    let mut file: File = ffile.unwrap();
    let mut encoder = GzEncoder::new(file, flate2::Compression::default());

    for j in lines {
      if j.contains("ACCEPT") {
        let log: String = get_log(&j, x);

        if log.len() > 0 {
          ret.push(log);
        } else {
          cnt += 1;
          // println!("{}", j);
        }

        // ffile.write_all(log.as_bytes());
        // ffile.write_all(b"\n");
      }
    }
    let rett: String = ret.join("\n");
    println!("ignored: {}", cnt);
    encoder.write_all(rett.as_bytes()).expect("Failed to write parsed flow logs to file");
    encoder.finish().unwrap();
  }

  // write_to_file(&ret);
}

fn read_gz_lines(file_name: &str) -> Vec<String> {
  let f: Result<File, std::io::Error> = File::open(file_name);
  let decoder_new: flate2::read::GzDecoder<File> = flate2::read::GzDecoder::new(f.unwrap());

  let reader: BufReader<flate2::read::GzDecoder<File>> = BufReader::new(decoder_new);

  let mut ret: Vec<String> = Vec::new();
  for line in reader.lines() {
      match line {
          Ok(v) => {
              ret.push(v);
          },
          Err(e) => println!("{}", e),
      }
  }
  ret
}
