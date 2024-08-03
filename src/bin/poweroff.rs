use nix::sys::reboot::{reboot, RebootMode};

fn main() {
    println!("Shutting down!");
    shutdown();
}

fn shutdown() {
    let mode = RebootMode::RB_POWER_OFF;
    let _ = reboot(mode); // infallible
}
