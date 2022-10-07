use std::ffi::CStr;
use user_lib::*;

fn main() {
    println!("hello world (pid:{})", getpid());
    match fork() {
        Ok(ForkResult::Parent { child, .. }) => {
            if let Ok(wc) = wait() {
                if let WaitStatus::Exited(pid, _exitcode) = wc {
                    println!(
                        "hello, I am parent of {}(wc:{}) (pid:{})",
                        child,
                        pid,
                        getpid()
                    );
                }
            }
        }
        Ok(ForkResult::Child) => {
            println!("hello, I am child (pid:{})", getpid());
            let arg0 = CStr::from_bytes_with_nul(b"wc\0").unwrap();
            let arg1 = CStr::from_bytes_with_nul(b"p3.rs\0").unwrap();
            let myargs: [&CStr; 2] = [&arg0, &arg1];
            execvp(myargs[0], &myargs).unwrap();
            println!("this shouldn't print out");
        }
        Err(_) => {
            println!("Fork failed");
            std::process::exit(1);
        }
    }
}
