
// mod mata_infos;

use std::collections::HashMap;
// use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::prelude::*;
// use tokio::time::{Duration, delay_for};
use tokio::net::{TcpStream, TcpListener};
use tokio::stream::StreamExt;
// use pyo3::prelude::*;
use httparse::Request;
use url::Url;


// use pyo3::wrap_pyfunction;

fn construction_environ(req: Request) -> Box<HashMap<String, Option<String>>> {
    let rurl = req.path
        .and_then(|v| { Url::parse(v).ok() })
        .unwrap();

    let other_header_map: HashMap<&str, String> =
        req.headers
            .iter()
            .map(|h| (h.name, String::from_utf8_lossy(h.value).to_string()))
            .collect();
    let host_url = other_header_map
        .get("Host")
        .and_then(|v| { Url::parse(v).ok() })
        .unwrap();

    let mut environ: Box<HashMap<String, Option<String>>> = Box::new(HashMap::new());
    environ.insert("REQUEST_METHOD".to_string(), req.method.map(&str::to_string));
    environ.insert("SCRIPT_NAME".to_string(),
                   rurl.path_segments().iter().last().map(|v| v.clone().collect::<String>()));
    environ.insert("PATH_INFO".to_string(), Some(rurl.path().to_string()));
    environ.insert("QUERY_STRING".to_string(), rurl.query().map(&str::to_string));
    environ.insert("CONTENT_TYPE".to_string(), other_header_map.get("Content-Type").cloned());
    environ.insert("CONTENT_LENGTH".to_string(), other_header_map.get("Content-Length").cloned());
    environ.insert("SERVER_NAME".to_string(), host_url.host_str().map(&str::to_string));
    environ.insert("SERVER_PORT".to_string(), host_url.port().map(|v| v.to_string()));
    environ.insert("SERVER_PROTOCOL".to_string(), req.version.map(|v| v.to_string()));

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
