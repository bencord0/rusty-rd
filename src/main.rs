
use std::{
    error::Error,
    io::{self, IsTerminal},
    os::unix::process::CommandExt,
    process::{self, Command},
    thread,
    time::Duration,
};

use nix::{
    mount,
    sys::reboot::{reboot, RebootMode},
};

fn main() {
    match run() {
        Ok(_) => {
            shutdown();
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            rescue_shell();
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let stdin = io::stdin();
    println!("Is tty: {}", stdin.is_terminal());

    setup_mounts()?;

    Ok(())
}

fn setup_mounts() -> Result<(), Box<dyn Error>> {
    //mount -t proc none /proc
    mount::mount(Option::<&str>::None, "/proc", Some("proc"), mount::MsFlags::empty(), Option::<&str>::None)?;

    //mount -t sysfs none /sys
    mount::mount(Option::<&str>::None, "/sys", Some("sysfs"), mount::MsFlags::empty(), Option::<&str>::None)?;

    //mount -t devtmpfs none /dev
    mount::mount(Option::<&str>::None, "/dev", Some("devtmpfs"), mount::MsFlags::empty(), Option::<&str>::None)?;

    Ok(())
}

fn shutdown() {
    println!("Shutting down");

    let wait = Duration::from_secs(5);
    thread::sleep(wait);

    let mode = RebootMode::RB_POWER_OFF;
    let _ = reboot(mode); // infallible
}

fn rescue_shell() -> !{
    println!("Rescue shell");
    let error = Command::new("/usr/bin/setsid")
        .args(["-c", "/bin/bash"])
        .exec();

    // If bash executes, we should never reach this point
    eprintln!("Error: {:?}", error);
    println!("looping");
    loop {};
}
