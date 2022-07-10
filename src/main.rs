use std::io;
use std::convert::TryFrom;
use std::os::unix::io::RawFd;
use std::thread;
use libc::c_int;
use nix::fcntl::OFlag;
use nix::unistd::pipe2;
use tokio_fd::AsyncFd;
use tokio::io::{AsyncReadExt, AsyncWrite, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio::time::{Duration, sleep};

#[link(name = "gomodule")]
extern "C" {
    fn Hello(fd: c_int);
}

async fn read(mut rh: ReadHalf<AsyncFd>) -> io::Result<()> {
    let mut buf = vec![0; 1024];
    println!("before read");
    while let Ok(n) = rh.read(&mut buf).await {
        let msg = String::from_utf8_lossy(&buf[..n]);
        println!("reading from go: {}", msg);
        tokio::time::sleep(Duration::from_secs(3)).await;
    }
    Ok(())
}

async fn write(mut wfd: RawFd) -> io::Result<()> {
    let mut fd = AsyncFd::try_from(wfd)?;
    loop {
        println!("before write");
        fd.write_all("hello".as_bytes()).await?;
        sleep(Duration::from_secs(2)).await;
    }
}

fn write_in_another_thread(fd: RawFd) {
    let _th = thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let _ = rt.block_on(write(fd));
    });
}

pub async fn run() -> io::Result<()> {
    let (mut fd1, mut fd2) = pipe2(OFlag::O_CLOEXEC | OFlag::O_NONBLOCK)?;
    let mut afd1 = AsyncFd::try_from(fd1)?;
    let (mut afd1_rh, mut afd1_wh) = tokio::io::split(afd1);

    tokio::spawn(read(afd1_rh));

    //write_in_another_thread(fd2);
    unsafe { Hello(fd2);}

    sleep(Duration::from_secs(1000000)).await;
    Ok(())
}


fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let _ = rt.block_on(run());
}
