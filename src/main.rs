use std::time::Duration;
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::str::FromStr;
use std::io;
use std::io::Write;


//I'm gonna make a lot of comments since this is a learning project so just know that
fn main() {

    //GET IP FOR SCANNING!!
    let ip = loop{ //We're gonna loop here to ensure we get input
        print!("Enter ip address: ");
        io::stdout().flush().unwrap(); //forces the statement to be printed

        let mut user_input = String::new(); //define new string
        io::stdin().read_line(&mut user_input).unwrap(); //take the input and put it in user_input

        match IpAddr::from_str(user_input.trim()){ //Checks if the IP is valid to be turned into IpAddr from the string
            Ok(ip) => break ip, //if good, break loop and return ip to line 9 ip
            Err(_) => {println!("invalid ip address\n"); continue;} //if it no work, tell them and continue
        }
    };

    //GET START PORT
    let start_port: u16 = loop { //start port is gonna be any u16 since that's what ports use
        print!("Enter the starting port: ");
        io::stdout().flush().unwrap();

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();

        match user_input.trim().parse() { //trim to get rid of whitespace, parse turns it to an u16
            Ok(port) => break port,
            Err(_) => {println!("invalid starting port, please enter a number between 0 and 65535.\n"); continue;}
        }
    };

    //GET END PORT
    let end_port: u16 = loop { //same as start port
        print!("Enter the starting port: ");
        io::stdout().flush().unwrap();

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();

        match user_input.trim().parse() {
            Ok(port) => break port,
            Err(_) => {println!("invalid ending port, please enter a number between 0 and 65535.\n"); continue;}
        }
    };

    

}
