use std::thread;
use std::time::Duration;

// 开始健身，好累，我得发出声音：muuuu...
fn muuuuu(intensity: u32) -> u32 {
    println!("muuuu.....");
    thread::sleep(Duration::from_secs(2));
    intensity
}

//普通函数的实现
fn workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "今天活力满满，先做 {} 个俯卧撑!",
            muuuuu(intensity)
        );
        println!(
            "旁边有妹子在看，俯卧撑太low，再来 {} 组卧推!",
            muuuuu(intensity)
        );
    } else if random_number == 3 {
        println!("昨天练过度了，今天还是休息下吧！");
    } else {
        println!(
            "昨天练过度了，今天干干有氧，跑步 {} 分钟!",
            muuuuu(intensity)
        );
    }
}

//使用闭包的方式实现

fn workout_closue(intensity: u32, random_number: u32){
    let action = || {
        println!("muuuu.....");
        thread::sleep(Duration::from_secs(2));
        intensity
    };

    if intensity < 25 {
        println!(
            "今天活力满满，先做 {} 个俯卧撑!",
            action()
        );
        println!(
            "旁边有妹子在看，俯卧撑太low，再来 {} 组卧推!",
            action()
        );
    } else if random_number == 3 {
        println!("昨天练过度了，今天还是休息下吧！");
    } else {
        println!(
            "昨天练过度了，今天干干有氧，跑步 {} 分钟!",
            action()
        );
    }
}

    fn add_closuer(one: i32, two: i32) -> i32 {
        let data = 10;
        let sum = | x, y | {
            x + y + one + two + data
        };
        sum(one,two)
    }
#[cfg(test)]
mod tests {
    use std::ptr::eq;

    use super::*;
    #[test]
    fn test_workout(){
        workout(10,3);
    }

    #[test]
    fn test_workout_closue() {
        workout_closue(30,7)
    }

    #[test]
    fn test_add_closuer(){
        let res = add_closuer(10, 20);
        assert_eq!(res,70);
        assert_eq!(res,60);
        println!("res:{}", res);
    }
    
}
