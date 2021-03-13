use icmp;
use std::net::UdpSocket;
use std::net::{IpAddr, Ipv4Addr};
use std::time::Instant;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "ariane", about = "know your path")]
struct Opt {
    #[structopt(short, long)]
    debug: bool,
    #[structopt(default_value = "64", short, long)]
    max_hop: u8,
    #[structopt(default_value = "3", short, long)]
    tries: u8,
    #[structopt(default_value = "33434", short, long)]
    port: u32,
    #[structopt(short, long)]
    icmp: bool,

    #[structopt(name = "hostname")]
    hostname: String,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
    // increment port for each port
    // this way we know the order for each router response

    let mut hop_count = 0;
    let mut ttl = 0;
    let mut port = opt.port;
    let msg = String::from("LOVELY");
    let msg_bytes = msg.into_bytes();
    let host_addr = format!("{}:{}", opt.hostname, opt.port.to_string());
    let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let icmp_socket = icmp::IcmpSocket::connect(localhost_v4).unwrap();
    let mut stop = false;

    //max hop default to 64
    while !stop && hop_count < opt.max_hop {
        hop_count = hop_count + 1;
        ttl = ttl + 1;
        println!("{} hop #", hop_count);
        let socket = UdpSocket::bind(format!("0.0.0.0:{}", port)).expect("noknok");
        socket.set_ttl(ttl).expect("noknokttl");

        //  3. send request
        socket.connect(&host_addr).expect("can't connect");
        let now = Instant::now();
        socket.send(&msg_bytes).expect("can't send");
        let mut buf: Vec<u8> = vec![0; 1000];
        match icmp_socket.recv_from(&mut buf) {
            Ok((received, rcv_addr)) => {
                let filled_buf = &buf[..received];
                println!(
                    "received {} bytes - rcv_addr {} - icmp_type {}",
                    received, rcv_addr, filled_buf[20],
                );
                //20 octet is icmp type slot in the ipv4 header
                if filled_buf[20] == 3 {
                    stop = true;
                }
            }
            Err(e) => println!("recv function failed: {:?}", e),
        }
        println!("{}ms", now.elapsed().as_millis(),);
        port = port + 1;
    }
}
