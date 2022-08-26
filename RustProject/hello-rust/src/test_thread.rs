use std::thread;
use std::time::Duration;

fn thread_func() {
    thread::spawn(||{
        for i in 1..10 {
            println!("thread:{}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thread_func(){
        thread_func();
        for i in 1..10 {
            println!("hi number{}", i);
            thread::sleep(Duration::from_millis(1));
        }
    }

}