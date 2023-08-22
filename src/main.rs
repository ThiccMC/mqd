use log::info;
use std::{
  collections::VecDeque,
  fs,
  io::{Read, Write},
  net::TcpListener,
};

use clap::Parser;
use gamedig::games::mc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct PingConfig {
  pub host: String,
  pub port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
  pub bind: String,
  pub popts: PingConfig,
}

#[derive(Parser, Debug)]
struct Args {
  #[arg(short, long, default_value_t = String::from("./config.ron"))]
  pub confile: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct QueryResult {
  pub e: u8,
  pub current: u32,
  pub max: u32,
}

fn ping(bucket: &mut VecDeque<QueryResult>, opts: &PingConfig, ret: Option<u8>) -> Option<()> {
  let r = mc::query(&(opts.host).parse().unwrap(), Some(opts.port));
  if r.is_ok() {
    let u = r.unwrap();
    bucket.push_back(QueryResult {
      e: 0,
      current: u.players_online,
      max: u.players_maximum,
    });
    Some(())
  } else {
    let u = ret.unwrap_or(0);
    if u > 3 {
      return None;
    } else {
      ping(bucket, opts, Some(u + 1))
    }
  }
}

fn save(bucket: &VecDeque<QueryResult>) {
  info!("Saving the database");
  let r = bincode::serialize(bucket).unwrap();
  fs::write("./db.u344", r).unwrap();
}

fn load(bucket: &mut VecDeque<QueryResult>) {
  info!("Loading the database");
  let r = fs::read("./db.u344").unwrap();
  let d: VecDeque<QueryResult> = bincode::deserialize(&r).unwrap();
  *bucket = d;
}

fn main() {
  env_logger::init();

  let args = Args::parse();

  let config_file_buffer = fs::read(args.confile).unwrap();
  let config_data = std::str::from_utf8(&config_file_buffer).unwrap();
  let conf: Config = ron::from_str(config_data).unwrap();

  let mut bucket: VecDeque<QueryResult> = VecDeque::new();
  if fs::metadata("./u344").is_ok() {
    load(&mut bucket)
  };
  let listener = TcpListener::bind(conf.bind).unwrap();
  info!("Server started on port 8080");
  for stream in listener.incoming() {
    match stream {
      Ok(mut stream) => {
        let mut buffer = [0; 64];
        loop {
          let nbytes = stream.read(&mut buffer).unwrap();
          if nbytes == 0 {
            break;
          }
          let input = std::str::from_utf8(&buffer[..nbytes]).unwrap();
          let response = match input.trim() {
            "query" => {
              let mut r: String = String::new();
              for ele in &bucket {
                r.push_str(&format!(":{},{},{}", ele.e, ele.current, ele.max));
              }
              r
            }
            "cron" => {
              info!("Cron awoked");
              if bucket.len() >= 32 {
                bucket.pop_front();
              }
              if ping(&mut bucket, &conf.popts, None).is_some() {
                "ok".to_string()
              } else {
                "nope".to_string()
              }
            }
            "freeze" => {
              save(&bucket);
              "ok".to_string()
            }
            "yeet" => {
              save(&bucket);
              break;
            }
            _ => "undefined".to_string(),
          };
          stream.write(response.as_bytes()).unwrap();
        }
      }
      Err(e) => {
        eprintln!("connection failed: {}", e);
      }
    }
  }
}
