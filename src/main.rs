use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "ariane", about = "know your path")]
struct Opt {
    #[structopt(short, long)]
    debug: bool,
    #[structopt(short, long)]
    max_hop: u8,
    #[structopt(short, long)]
    tries: u8,
    #[structopt(short, long)]
    port: u32,
    #[structopt(short, long)]
    icmp: bool,
}

fn main() {
    println!("Hello, world!");
}
