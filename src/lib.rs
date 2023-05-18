// use aws_config::meta::region::RegionProviderChain;
// use aws_sdk_s3::{config::Region, meta::PKG_VERSION, Client, Error};
// use clap::Parser;
// use rand::prelude::*;
// use tokio::io::AsyncReadExt;
// use std::{fs::File, io::Write, path::PathBuf, process::exit};

// #[derive(Debug, Parser)]
// struct Opt {
//     /// The AWS Region.
//     #[structopt(short, long)]
//     region: Option<String>,

//     /// The name of the bucket.
//     #[structopt(short, long)]
//     bucket: String,

//     /// Whether to display additional information.
//     #[structopt(short, long)]
//     verbose: bool,

//     #[structopt(short, long)]
//     prefix: String,
// }

// #[tokio::main]
// pub async fn get() -> Result<Vec<String>, Error> {
//     let Opt {
//       region,
//       bucket,
//       verbose,
//       prefix,
//     } = Opt::parse();
//     // let config = aws_config::load_from_env().await;
//     // let client = s3::Client::new(&config);
//     let region_provider = RegionProviderChain::first_try(region.map(Region::new))
//         .or_default_provider()
//         .or_else(Region::new("us-west-2"));
//     let shared_config = aws_config::from_env().region(region_provider).load().await;
//     let client = Client::new(&shared_config);

//     // let region = "ap-south-1";
//     let bucket = "rzp-prod-vpc-flow-logs";
//     // let prefix = "AWSLogs/14112890/vpcflowlogs/ap-south-1/2023/05/16/";

//     let mut ret = Vec::new();
//     let mut resp = client.list_objects_v2().max_keys(1000).bucket(bucket).prefix(&prefix).send().await?;
//     // let mut resp = client.list_objects_v2().bucket(bucket).prefix(&prefix).send().await?;
//     for object in resp.contents().unwrap_or_default() {
//       ret.push(object.key().unwrap_or_default().to_string());
//     }
//     let mut cnt=0;
//     while resp.next_continuation_token().is_some() {
//       cnt+=1;
//       let tok = resp.next_continuation_token().unwrap();
//       resp = client.list_objects_v2().max_keys(1000).bucket(bucket).prefix(&prefix).continuation_token(tok).send().await?;
//       for object in resp.contents().unwrap_or_default() {
//         ret.push(object.key().unwrap_or_default().to_string());
//       }
//     }

//     println!("API Calls: {}", cnt);
//     println!("Number of Files: {:#?}", ret.len());
//     let ret_rand = pick_random_n(1000, ret);

//     Ok(ret_rand)
// }

// fn pick_random_n(n: i32, mut vv: Vec<String>) -> Vec<String> {
//   let mut rng = rand::thread_rng();

//   let mut chosen_strings: Vec<String> = Vec::new();

//   for _ in 0..n {
//       let index = rng.gen_range(0..vv.len());
//       chosen_strings.push(vv.remove(index));
//   }
//   chosen_strings
// }
