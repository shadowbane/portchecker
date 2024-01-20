use std::env;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::process::exit;
use std::time::{Duration, Instant};

use app::dto::ping_result::{PingResult, PingResults};

use crate::app::traits::check_result::CheckResult;

mod app;

const APP_NAME: &str = "PortChecker";
const VERSION: f64 = 0.1;
const DEVELOPER: &str = "Adly Shadowbane <adly.shadowbane@gmail.com>";

fn main() {
    let args: Vec<String> = env::args().collect();

    // set default timeout to 3s
    let mut timeout: u8 = 3;
    let mut tries: u8 = 3;
    let host: &str;
    let port: u16;

    match &args.len() {
        2 => {
            match String::from(&args[1]).as_str() {
                "--help" => {
                    version();
                    tagline();
                    help();
                    exit(0);
                }
                "--version" => {
                    version();
                    exit(0);
                }
                _ => {
                    println!("Invalid number of argument supplied");
                    help();
                    exit(1);
                }
            }
        }
        3 => {
            host = &args[1];
            port = parse_port(&args[2]);
        }
        4 => {
            host = &args[1];
            port = parse_port(&args[2]);
            timeout = parse_timeout(&args[3]);
        }
        5 => {
            host = &args[1];
            port = parse_port(&args[2]);
            timeout = parse_timeout(&args[3]);
            tries = parse_tries(&args[4]);
        }
        _ => {
            println!("Invalid number of argument supplied");
            help();
            exit(1);
        }
    }

    println!("Host is {} and port is {} with timeout of {}s for {} number of tries", &host, &port, &timeout, &tries);

    let ping_results: PingResults = is_port_open(
        host,
        port,
        timeout,
        tries,
    );

    // println!("Check Result");
    println!("{:<15} {:>10}", "Success", ping_results.received());
    println!("{:<15} {:>10}", "Failed", ping_results.failed());
    println!("{:<15} {:>10}", "Total", ping_results.transmitted());
    println!("{:<15} {:>10}", "Average (ms)", ping_results.average());
    println!("{:<15} {:>10}", "Max (ms)", ping_results.max());
    println!("{:<15} {:>10}", "Min (ms)", ping_results.min());
}

fn is_port_open(host: &str, port: u16, timeout: u8, tries: u8) -> PingResults {
    // let host = "103.124.196.181";
    // let host = "login.microsoftonline.com";
    // let port: u16 = 443;
    let mut ping_results: PingResults = PingResults {
        results: Vec::new(),
    };

    let addr = prepare_host(host, port);
    println!("Checking connectivity to {} on port {} ...", host, port);

    for iter in 0..tries {
        let mut result: PingResult = PingResult {
            host: addr.to_string(),
            domain: host.parse().unwrap(),
            port,
            sequence: iter,
            ttl: Default::default(),
            success: false,
        };
        // start the check
        let start_time = Instant::now();

        match TcpStream::connect_timeout(&addr, Duration::from_secs(timeout as u64)) {
            Ok(_) => {
                let end_time = Instant::now();
                let response_time = end_time - start_time;
                result.ttl = response_time;
                result.success = true;
                ping_results.results.push(result);
            }
            Err(_) => {
                result.success = false;
                ping_results.results.push(result);
                println!("error when connecting to {addr}")
            }
        }
    }

    return ping_results;
}

fn prepare_host(host: &str, port: u16) -> SocketAddr {
    let address = format!("{}:{}", host, port);

    let resolved: Option<SocketAddr> = match address.to_socket_addrs() {
        Ok(mut addrs) => addrs.next(),
        Err(_) => None,
    };

    let addr = resolved.unwrap();

    return addr;
}

fn parse_port(number: &str) -> u16 {
    let parsed_num: u16 = match number.parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("error: port must be a valid port");
            help();
            exit(1);
        }
    };

    return parsed_num;
}

fn parse_timeout(number: &str) -> u8 {
    let parsed_num: u8 = match number.parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("error: timeout must be an integer value between 1-15");
            help();
            exit(1);
        }
    };

    if parsed_num > 15 || parsed_num < 1 {
        eprintln!("error: timeout must be an integer value between 1-15");
        help();
        exit(2);
    }

    return parsed_num;
}

fn parse_tries(number: &str) -> u8 {
    let parsed_num: u8 = match number.parse() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("error: retry must be an integer value between 1-10");
            help();
            exit(1);
        }
    };

    if parsed_num > 10 || parsed_num < 1 {
        eprintln!("error: retry must be an integer value between 1-10");
        help();
        exit(2);
    }

    return parsed_num;
}

fn version() {
    println!("{} v{} by {}", APP_NAME, VERSION, DEVELOPER);
}

fn tagline() {
    println!("Quickly verify port accessibility");
}

fn help() {
    println!("
Usage
  portchecker HOST PORT [TIMEOUT] [RETRY]

Parameters:
  HOST                🌐 The IP address or domain name you want to check.
  PORT                🚪 The specific port to probe (1-65535).
  TIMEOUT (optional)  ⏱️ Time in seconds before a request times out (1-15, default: 10).
  RETRY (optional)    🔁 Number of times to retry the test (1-10, default: 3).
"
    );
}
