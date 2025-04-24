//use std::io::prelude::*;
//use std::net::TcpStream;
use tokio::net::TcpSocket;
use tokio::net::TcpStream;
//use std::net::ToSocketAddrs;
use std::net::SocketAddr;
use tokio::io::{AsyncWriteExt, AsyncReadExt};

//use anyhow::Error;
//use byteorder::{BigEndian, WriteBytesExt};

pub async fn connect_tcp_stream (s:String) -> std::io::Result<tokio::net::TcpStream> {

    /*
    for addr in tokio::net::lookup_host("127.0.0.1:5000").await? {
        println!("socket address isssssss {}", addr);
    }

    let mut addrs = s.to_socket_addrs()?;
    //let _ = addrs.next();

    */

    let socket: TcpSocket = TcpSocket::new_v4()?;

    let server_addr: SocketAddr = s.parse().unwrap();

    let maybe_client_addr_string: Option<String> = None;  //Some ("127.0.0.1:5025".to_string());



    match maybe_client_addr_string {
        Some (client_addr_string) => {

            let client_addr: SocketAddr = client_addr_string.parse().unwrap();
            socket.set_reuseaddr(true)?;
            socket.bind(client_addr)?;

            println!("\n{}{}{}{}", "Trying to connect to server at address:  ", server_addr, " from FIXED client address: ", client_addr);
            let stream = socket.connect(server_addr).await?;
            Ok(stream)

        }
        None => {
            println!("\n{}{}{}", "Trying to connect to server at address:  ", server_addr, " from EPHEMERAL (OS-chosen) client address");
            let stream = socket.connect(server_addr).await?;
            Ok(stream)

        }

    }

    /*
    let client_addr: SocketAddr = "127.0.0.1:5025".parse().unwrap();

    let server_addr: SocketAddr = s.parse().unwrap();

    let socket = TcpSocket::new_v4()?;
    socket.set_reuseport(true)?; 
    socket.bind(client_addr)?;

    //println!("\n{}{}", "Server ADDR:  ", server_addr);
    println!("\n{}{}{}{}", "Trying to connect to server at address:  ", server_addr, " from client address: ", client_addr);
    let stream = socket.connect(server_addr).await?;
    Ok(stream)
    */

    /*

    let maybe_server_addr  = addrs.next(); //"127.0.0.1:5000".parse().unwrap(); //s.parse().unwrap();
    match maybe_server_addr {    
        Some (server_addr) => { 
            println!("\n{}{}", "Server ADDR:  ", server_addr);
            let stream = socket.connect(server_addr).await?;
            Ok(stream)
        }
        None => {panic!("Could not parse server UUID");}
    }
    */
    /*
    let stream = TcpStream::connect(s);
    stream
    */


    /*
    let addr = "127.0.0.1:8080".parse().unwrap();

    let socket = TcpSocket::new_v4()?;
    socket.set_reuseaddr(true)?;
    assert!(socket.reuseaddr().unwrap());
    socket.bind(addr)?;
    */
}

#[allow(non_snake_case)]
pub async fn am_sendRec_string (s:String, mut stream:TcpStream) -> std::io::Result<String> {
    let sbytes = s.as_bytes();
    let sbytes_len: u32 = sbytes.len().try_into().unwrap();

    // Write a buffer of bytes representing the (u32) size of the string to be sent
    let mut wtr = vec![];
    AsyncWriteExt::write_u32(& mut wtr,sbytes_len).await?;//.unwrap();
    stream.write_all(&wtr).await?;

    // Write the string as bytes
    stream.try_write(s.as_bytes())?;


    
    // This is a hack to read 4 bytes from the stream (peeling off the response buffer size)
    // TODO:  We should probably use/decode this value in the future if we keep this approach
    let mut x:[u8; 4] = [0u8;4];
    stream.read_exact(&mut x).await?;
    //stream.try_read(&mut x)?;

    // Read in response string from stream
    let mut str_in : String = String::new();
    stream.read_to_string(&mut str_in).await?;

    // Clone and return response string
    let str_out : String = str_in.clone();
    Ok (str_out)
}