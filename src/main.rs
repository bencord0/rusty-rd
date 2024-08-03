use std::{
    error::Error,
    ffi::CString,
    fs::File,
    io::{self, IsTerminal},
    os::unix::process::CommandExt,
    process::Command,
    thread,
    time::Duration,
};
use futures::stream::TryStreamExt;

use nix::{
    kmod::{finit_module, ModuleInitFlags},
    mount,
    sys::reboot::{reboot, RebootMode},
};

use rtnetlink::new_connection;
use netlink_packet_route::link::LinkAttribute;

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

    // Run under tokio
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .build()?;

    let future = setup_network(&rt);
    rt.block_on(future)?;

    let future = show_links(&rt);
    rt.block_on(future)?;

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

async fn setup_network(rt: &tokio::runtime::Runtime) -> Result<(), Box<dyn Error>> {
    // Open a netlink socket to the kernel
    let (connection, handle, _) = new_connection()?;
    rt.spawn(connection);

    //modprobe e1000
    let e1000 = File::open("/lib/modules/e1000.ko")?;
    finit_module(&e1000, &CString::from(c""), ModuleInitFlags::empty())?;

    //ip link set lo up
    let lo = String::from("lo");
    let mut links = handle.link().get().match_name(lo).execute();
    if let Some(link) = links.try_next().await? {
        let index = link.header.index;
        handle.link().set(index).up().execute().await?;
    }

    //ip link set eth0 up
    // This is enough to permit IPv6 networking to the internet under qemu.
    // You need to bring your own resolver, and TLS stack; but these are available to rust.
    //
    // If you need IPv4, you will need to run DHCP.
    let eth0 = String::from("eth0");
    let mut links = handle.link().get().match_name(eth0).execute();
    if let Some(link) = links.try_next().await? {
        let index = link.header.index;
        handle.link().set(index).up().execute().await?;
    }

    Ok(())
}

async fn show_links(rt: &tokio::runtime::Runtime) -> Result<(), Box<dyn Error>> {
    // Open a netlink socket to the kernel
    let (connection, handle, _) = new_connection()?;
    rt.spawn(connection);

    // ip link show
    let mut links = handle.link().get().execute();

    while let Some(msg) = links.try_next().await? {
        let header = msg.header;
        for link_attr in msg.attributes.into_iter() {
            if let LinkAttribute::IfName(name) = link_attr {
                println!("Found Network Link {}: {}", header.index, name);
            }
        }
    }

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
