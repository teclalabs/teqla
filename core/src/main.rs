mod dag;
mod pouw;
mod net;

fn main() {
    println!("TEQLA Core Node — DAG + PoUW (skeleton)");
    // TODO:
    // - initialize p2p (libp2p/gossipsub) [see net.rs]
    // - load DAG state [see dag.rs]
    // - main loop: receive tx, verify PQC sig (future), verify PoUW, attach to DAG, propagate
}
