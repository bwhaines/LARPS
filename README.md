# Lightweight Asynchronous Rust Port Scanner

This program uses Rust and the async-std crate to perform fast, asynchronous port scanning.

*WARNING: This program is still early in development!*

## Usage

`./larps <OPTIONS>`

Options:

| CMD Flag      | Description                                                | Default   |
|---------------|------------------------------------------------------------|-----------|
| -a, --address | URL or IP to scan                                          | 127.0.0.1 |
| -p, --ports   | List of ports to scan                                      | 1-65535   |
| -f, --infile  | Line-separated list of URLs or IPs to scan                 | [NONE]    |
| -o, --outfile | Output to file                                             | [NONE]    |
| -m, --mode    | Scanning mode (either TCP-full, TCP-partial, UDP, or BOTH) | TCP full  |
| -q, --quiet   | Quiet mode (only output open sockets)                      | False     |
| -t, --timeout | Timeout for requests (in seconds)                          | 10 secs   |
| -b, --batch   | Number of requests to try at one time                      | [NONE]    |
| -d, --debug   | Enable info-level logging to stdout                        | False     |

Example:

```
./larps -d -a 192.168.1.165 -p 22,23,25,80,443,5900,8080,8443 -t 5
2021-02-17 20:15:12,288 INFO  [larps] Parsing port list 22,23,25,80,443,5900,8080,8443
2021-02-17 20:15:12,292 INFO  [larps] Starting scan on ports [22, 23, 25, 80, 443, 5900, 8080, 8443]
192.168.1.165:22
192.168.1.165:5900
All done -- 0 seconds elapsed
```

## To-do List

* Scan using partial TCP handshake or UDP connection
* Output active ports to file