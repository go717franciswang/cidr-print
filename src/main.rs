use std::io;
use std::io::BufRead;

fn ip2u(ip: &str) -> u32 {
    let digits: Vec<u32> = ip.splitn(4, '.').map(|n| n.parse::<u32>().unwrap()).collect();
    digits.iter().fold(0u32, |s,n| s*256+n)
}

fn u2ip(mut u: u32) -> String {
    let n0 = u % 256u32;
    u = u / 256u32;
    let n1 = u % 256u32;
    u = u / 256u32;
    let n2 = u % 256u32;
    u = u / 256u32;
    format!("{}.{}.{}.{}", u, n2, n1, n0)
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let v: Vec<&str> = line.splitn(2, '/').collect();
        let ip = v[0];
        let length = v[1].parse::<i32>().unwrap();

        let shift = 32 - length;
        let base = (ip2u(ip) >> shift) << shift;

        for x in 0..2u32.pow(shift as u32) {
            println!("{}", u2ip(base+x));
        }
    }
}
