use std::{
    env::var,
    net::{Ipv4Addr, SocketAddr},
    // str::FromStr,
};
use warp::{hyper::Uri, Filter};

#[tokio::main]
async fn main() {
    let hello = warp::any().map(move || {
        warp::redirect::temporary(
            Uri::try_from(&match var("REDIR") {
                Ok(s) => s,
                Err(_) => "https://www.next-gen.dev".to_owned(),
            })
            .unwrap(),
        )
    });
    for addr in get_ports() {
        warp::serve(hello).run(addr).await;
    }
}

fn get_ports() -> Vec<SocketAddr> {
    match var("PORTS") {
        Ok(s) => s
            .split(",")
            .map(|string| string.to_string().parse::<u16>().unwrap())
            .collect::<Vec<u16>>(),
        Err(_) => vec![80],
    }
    .into_iter()
    .map(|port| SocketAddr::from((Ipv4Addr::from([127, 0, 0, 1]), port)))
    .collect()
}
