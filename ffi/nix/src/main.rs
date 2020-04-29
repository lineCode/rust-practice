extern crate nix;

use nix::unistd::*;

fn main() {
    match fork() {
        Ok(ForkResult::Parent { child }) => {
            // 在父进程中
            println!("Hello, I am parent thread: {}", getpid());
        }
        Ok(ForkResult::Child) => {
            // 在子进程中
            println!("Hello, I am child thread: {}", getpid());
            println!("My parent thread: {}", getppid());            
        }
        Err(errno) => {
            // fork 创建子进程失败
            println!("Fork creation failed!");
        }
    }  
}