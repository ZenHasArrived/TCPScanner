# Simple TCP Scanner made in Rust
I made this scanner to familiarize myself with the Rust language as well as make a cyber project

## Using this on Nmap's http://scanme.nmap.org/
```
Enter ip address: 45.33.32.156
Enter the starting port: 20
Enter the ending port: 80
Scanning...
Port: 22 is open
Port: 80 is open
Scan complete!
Process finished with exit code 0
```
This lines up with the open ports on the website using nmap
```
Nmap scan report for scanme.nmap.org (45.33.32.156)
Host is up (0.073s latency).
Not shown: 992 closed tcp ports (reset)
PORT     STATE    SERVICE
22/tcp   open     ssh
25/tcp   filtered smtp
80/tcp   open     http
135/tcp  filtered msrpc
445/tcp  filtered microsoft-ds
593/tcp  filtered http-rpc-epmap
646/tcp  filtered ldp
9929/tcp open     nping-echo

Nmap done: 1 IP address (1 host up) scanned in 3.18 seconds
```
