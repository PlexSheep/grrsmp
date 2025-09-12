#[derive(Debug)]
pub enum Connection {
    P2P(P2PConnection),
}

#[derive(Debug)]
pub struct P2PConnection {
    t: i32,
}
