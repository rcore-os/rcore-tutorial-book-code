use user_lib::*;

fn main() {
    println!("hello world (pid:{})", getpid());
    match fork() {
        Ok(ForkResult::Parent { child, .. }) => {
            println!("hello, I am parent of {} (pid:{})", child, getpid());
        }
        Ok(ForkResult::Child) => println!("hello, I am child (pid:{})", getpid()),
        Err(_) => {
            println!("Fork failed");
            std::process::exit(1);
        }
    }
}
