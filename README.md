# Lightweight Asynchronous Rust Port Scanner

This program uses Rust and the async-std crate to perform fast, asynchronous port scanning.

*WARNING: This program is still early in development!*

## Usage

`./larps <OPTIONS>`

Options:

| CMD Flag      | Description                                                |
|---------------|------------------------------------------------------------|
| -a, --address | URL or IP to scan                                          |
| -p, --ports   | List of ports to scan                                      |
| -f, --infile  | Line-separated list of URLs or IPs to scan                 |
| -o, --outfile | Output to file                                             |
| -m, --mode    | Scanning mode (either TCP-full, TCP-partial, UDP, or BOTH) |
| -q, --quiet   | Quiet mode (only output open sockets)                      |
| -t, --timeout | Timeout for requests (in seconds)                          |
| -b, --batch   | Number of requests to try at one time                      |
| -d, --debug   | Enable info-level logging to stdout                        |

## To-do List

* Scan using partial TCP handshake or UDP connection
* Take in file with list of domains or IP addresses
* Output active ports to file