
// mod mata_infos;

// use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::prelude::*;
// use tokio::time::{Duration, delay_for};
use tokio::net::{TcpStream, TcpListener};
use tokio::stream::StreamExt;
// use pyo3::prelude::*;
use httparse::Request;
use std::collections::HashMap;

// use pyo3::wrap_pyfunction;

fn construction_environ(req: Request) -> Box<HashMap<String, Option<String>>> {
    let mut environ: Box<HashMap<String, Option<String>>> = Box::new(HashMap::new());
    environ.insert("REQUEST_METHOD".to_string(), req.method.map(|v| v.to_string()));
    environ.insert("SERVER_PROTOCOL".to_string(), req.version.map(|v| v.to_string()));
    // environ.insert("PATH_INFO".to_string(), req.path.map(|v| v.to_string()));
    // println!("method: {:?}", req.method);
    // println!("path: {:?}", req.path);
    let _header_map: HashMap<String, Vec<u8>> =
        req.headers
            .iter()
            .map(|h| (h.name.to_string(), h.value.to_vec()))
            .collect();
    return environ;
}

async fn get_request(mut stream: TcpStream) -> Box<HashMap<String, Option<String>>> {
    loop {
        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut req = httparse::Request::new(&mut headers);
        let mut buffer = [0; 1024];

        stream.read(&mut buffer).await.unwrap();

        let s = req.parse(&buffer).unwrap();
        if s.is_complete() {
            return construction_environ(req);
        }
    }
}


#[tokio::main]
async fn main() {
    let address = "127.0.0.1";
    let port = "6142";
    let addr = format!("{}:{}", address, port);
    let mut listener = TcpListener::bind(addr.clone()).await.unwrap();

    println!("Server running on {}", addr);

    let mut incoming = listener.incoming();
    while let Some(stream) = incoming.next().await {
        let stream = stream.unwrap();
        let _environ = get_request(stream).await;
    }
}


// use std::io::prelude::*;
// use std::net::TcpStream;
// use std::net::TcpListener;
//
// fn main() {
//     let address = "127.0.0.1";
//     let port = "6142";
//     let addr = format!("{}:{}", address, port);
//     let listener = TcpListener::bind(addr.clone()).unwrap();
//
//     println!("Server running on {}", addr);
//
//     for stream in listener.incoming() {
//         let stream = stream.unwrap();
//
//         handle_connection(stream);
//     }
// }
//
// fn handle_connection(mut stream: TcpStream) {
//     let mut buffer = [0; 512];
//
//     stream.read(&mut buffer).unwrap();
//
//     println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
// }