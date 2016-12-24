use regex::Regex;

#[derive(PartialEq,Copy,Clone)]
pub struct Node {
    pub x: u32,
    pub y: u32,
    pub size: u32,
    pub used: u32,
    pub available: u32,
}

impl Node {
    pub fn from_str(s: &str) -> Node {
        let re = Regex::new(r"/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+(\d+)%").unwrap();
        let c = re.captures(s.trim()).unwrap();

        Node {
            x: c[1].parse().unwrap(),
            y: c[2].parse().unwrap(),
            size: c[3].parse().unwrap(),
            used: c[4].parse().unwrap(),
            available: c[5].parse().unwrap(),
        }
    }
}
