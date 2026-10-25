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
            Err(_) => {println!("invalid ip address"); continue;} //if it no work, tell them and continue
        }
    };


}
