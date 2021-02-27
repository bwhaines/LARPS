use async_std::io::timeout;
use async_std::net::TcpStream;
use async_std::stream::StreamExt;
use futures::stream::FuturesUnordered;
use regex::Regex;
use std::fs::OpenOptions;
use std::io::{BufRead,BufReader,ErrorKind};
use std::time::{Duration,SystemTime};
use simple_logger::SimpleLogger;

extern crate futures;
extern crate log;

#[async_std::main]
async fn main() {

    // Set up time tracking
    let start_time = SystemTime::now();

    // Set up default options
    let mut quiet_mode : bool = false;
    let mut timeout : u64 = 10;
    let mut target_addrs : Vec<String> = Vec::new();
    let mut port_list : Vec<u16> = (0..u16::MAX).map(|x| x+1).collect();

    // Parse user args
    let args: Vec<String> = std::env::args().collect();
    for index in 0..args.len() {
        match args[index].as_str() {
            "-a" | "--address" => target_addrs.push(args[index + 1].clone()),
            "-p" | "--ports"   => port_list = parse_port_list(&args[index + 1]),
            "-f" | "--infile"  => parse_addr_list(&args[index + 1], &mut target_addrs),
            "-o" | "--outfile" => println!("-o is not supported yet!"),
            "-m" | "--mode"    => println!("-m is not supported yet!"),
            "-q" | "--quiet"   => quiet_mode = true,
            "-t" | "--timeout" => timeout = args[index + 1].parse::<u64>().unwrap(),
            "-b" | "--batch"   => println!("-b is not supported yet!"),
            "-d" | "--debug"   => SimpleLogger::new().with_level(log::LevelFilter::Info).init().unwrap(),
            _ => (),
        }
    }

    // Fill the FuturesUnordered list with scan_addr calls
    let mut futures = target_addrs.iter().map(|target| scan_addr(target, &port_list, timeout)).collect::<FuturesUnordered<_>>();

    // Run through list of scan_addr calls in the FuturesUnordered
    while let Some(_) = futures.next().await { }

    if !quiet_mode {
        println!("All done -- {} seconds elapsed", start_time.elapsed().unwrap().as_secs());
    }
}


// Read a list of addresses from a file
fn parse_addr_list(filename : &String, addr_list : &mut Vec<String>) {

    log::info!("Reading addresses from file at {}", filename);

    // Try to open the file for reading
    let file = match OpenOptions::new().read(true).open(filename) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => panic!("Could not find file at {}", filename),
            ErrorKind::PermissionDenied => panic!("You don't have permission to read {}", filename),
            _ => panic!("An unknown file error occurred"),
        },
    };

    // Push each line from file into addr_list vector
    for addr in BufReader::new(file).lines() {
        log::info!("Adding {:?} to address list", &addr);
        addr_list.push(addr.unwrap());
    }
}


// Create a list of ports, either from a comma-separated list or a range
fn parse_port_list(port_string : &str) -> Vec<u16> {

    log::info!("Parsing port list {}", port_string);

    // Comma separated list -- match to any number of 1-5 digits followed by a 
    // comma, then a final group of 1-5 digits
    if Regex::new(r"^(\d{1,5},)*\d{1,5}$").unwrap().is_match(port_string) {
        return port_string.split(",").map(|x| x.parse::<u16>().unwrap()).collect();
    }

    // Range -- match to two groups of 1-5 digits separated by two '.' chars or a '-'
    if Regex::new(r"^\d{1,5}(\.\.|-)\d{1,5}$").unwrap().is_match(port_string) {
        let delim = Regex::new(r"\.\.|-").unwrap();
        let bounds : Vec<u16> = delim.split(port_string).map(|x| x.parse::<u16>().unwrap()).collect();
        return (bounds[0]..bounds[1]).collect::<Vec<u16>>();
    }

    // Else default to all ports
    println!("Couldn't understand port list...");
    return (0..u16::MAX).map(|x| x+1).collect::<Vec<u16>>();
}


// Scan a given list of ports on a given address
async fn scan_addr(address : &str, port_list : &Vec<u16>, timeout_value : u64) {

    log::info!("Starting scan on ports {:?}", port_list);

    // For each port, try a connection
    for port_num in port_list {
        log::trace!("Scanning port {}", port_num);
        async move {
            let stream = TcpStream::connect((address, port_num.clone()));
            if let Ok(_) = timeout(
                Duration::from_secs(timeout_value),
                stream
            ).await { println!("{}:{}", address, port_num); }
        }.await;
    }
}
