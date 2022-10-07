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
            sleep(1);
        }
        Err(_) => {
            println!("Fork failed");
            std::process::exit(1);
        }
    }
}
