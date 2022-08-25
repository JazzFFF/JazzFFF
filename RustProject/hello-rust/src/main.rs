mod TestMethod;
pub use crate::TestMethod::*;

mod test_generics;
pub use crate::test_generics::*;

mod test_box;
pub use test_box::*;

mod test_closure;

#[derive(Debug)]
struct User {
    age:    i32,
    name:   String,
    email:  String,
}
#[derive(Debug)]
enum Action {
    Name(String),
    Point(i32,i32),
    Color(i32,i32,i32),
}

fn main() {
    //for enum 聚会与赋值
    // TestEnum();

    //测试Option
    // TestIfOption();

    //2.6.4 测试match
    // TestMatch();

    //2.7 测试方法methods

    //2.12 测试包引用
    // let cir = TestMethod::Circle::new(1.0, 2.0, 3.0);
    // println!("{}",  cir.area());

    if false {//Test 泛型
        let gen_point = test_generics::Point::new(1.0, 2.0);
        let val = gen_point.get_x() + gen_point.get_y();
        println!("{}",  val);
    }
    if true {//Test 特征
        //测试为结构体接口定义特征
        let post = test_generics::Post{
            title:String::from("中国人"),
            author:String::from("zzf"),
            content:String::from("abcdef"),
        };
        // post.summaryize();

        let weibo = test_generics::Weibo{
            name:String::from("日本人"),
            content:String::from("jjho"),
        };
        // weibo.summaryize();
        
        if true {//测试特征作为函数参数 使用
            println!("测试特征作为函数参数 使用 {}", line!());
            test_generics::test_trait_summary(&post);
            test_generics::test_trait_summary(&weibo);
            println!("====================== 使用 {}", line!());
            test_generics::test_trait_summary_display(&post);
            // test_generics::test_trait_summary_display(&weibo);
        }

    }

    {   //测试智能指针 box

    }

}

//for enum 遍历与赋值
fn TestEnum() {
    let mut val: [Action; 3] = [
        Action::Name(String::from("zzf")),
        Action::Point(1,2),
        Action::Color(1,2,3),
    ];
    val[0] = Action::Name(String::from("test"));

    for v in &mut val{
        match v {
            Action::Name(string) => {
                println!("{:?}", v);
                *v = Action::Name(String::from("ccc"));
            },
            Action::Point(x,y) => println!("Name: {}, Position: {}", x, y),
            Action::Color(x,y,z) => println!("Color: {}, Color: {}, Color {}", x, y, z),
            _ => println!("Action: {:?}", v),
        }
    }

    for v in &val{
        match v {
            Action::Name(string) => println!("{:?}", v),
            Action::Point(x,y) => println!("Name: {}, Position: {}", x, y),
            // Action::Color(x,y,z) => println!("Color: {}, Color: {}, Color {}", x, y, z),
            _ => println!("Action: {:?}", v),
        }
    }
}

//2.6.2 测试Option Some 结构
fn TestIfOption() {
    let re = plus_add(Option::Some(32));
    println!("line:{} re(x):{:?}",  std::line!(), re);
    if let re = Some(1){
        println!("line:{} re(x):{:?}",  std::line!(), re);
    } else if let re=Some(33) { //这行 永远不会 执行？？，无意义
        println!("line:{} re(x):{:?}",  std::line!(), re);
    } else {
        println!("line:{} re(x):{:?}",  std::line!(), re);//这行 永远不会 执行？？，无意义
    }
}

fn plus_add(x: Option<i32>) -> Option<i32>{
    println!("{:?}", x);
    match x {
        None => {
            println!("None");
            None
        },
        Some(y) => {
            println!("Some(x):{} {} {}", y, std::line!(), std::file!());
            Some(y+1)   
        }
    }
}

//2.6.1 测试If Let1
fn TestIfLet() {
    let mut val: [Action; 3] = [
        Action::Name(String::from("zzf")),
        Action::Point(1,2),
        Action::Color(1,2,3),
    ];
    if let 3 = 5{
        println!("Action: {:?} equals 3", val);
    } else {
        println!("Action: {:?} equals 5", val);
    }
    // if let Action::Name(string) = String::from("zzf"){
    //     println!("{:?}", val);
    // }
}

#[derive(Debug)]
enum Message{
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    Clolor(i32, i32, i32, i32, i32, i32),
}
struct Point {
    x: i32,
    y: i32,
}
//2.6.4 测试match
fn TestMatch() {
    {//测试match 01
        let x = Some(10);
        let y = 100;

        match(x) {
            Some(50) =>{//匹配字面值
                println!("line:{} Action: {:?} matches x",std::line!(), x);
            },
            Some(y) =>{//匹配命名变量
                println!("line:{} Action: {:?} matches y",std::line!(), y);
            },
            _ => {
                println!("line:{} Action: {:?} not found",std::line!(), x);
            }
        }

        match(x) {
            Some(10) =>{//匹配字面值
                println!("line:{} Action: {:?} matches x",std::line!(), x);
            },
            Some(y) =>{//匹配命名变量
                println!("line:{} Action: {:?} matches y",std::line!(), y);
            },
            _ => {
                println!("line:{} Action: {:?} not found",std::line!(), x);
            }
        }

        match(None) {
            Some(10) =>{//匹配字面值
                println!("line:{} Action: {:?} matches x",std::line!(), x);
            },
            Some(y) =>{//匹配命名变量
                println!("line:{} Action: {:?} matches y",std::line!(), y);
            },
            _ => {
                println!("line:{} Action: {:?} {:?} not found",std::line!(), x, y);
            }
        }

        match(Some(50)) {
            Some(10) =>{//匹配字面值
                println!("line:{} Action: {:?} matches x",std::line!(), x);
            },
            Some(y) =>{//匹配命名变量
                println!("line:{} Action: {:?} matches y",std::line!(), y);
            },
            _ => {
                println!("line:{} Action: {:?} not found",std::line!(), x);
            }
        }
        
        let p = Point { x: 0, y: 7 };
        match p {
            Point { x, y: 0 } => println!("On the x axis at {}", x),
            Point { x: 0, y } => println!("On the y axis at {}", y),
            Point { x, y } => println!("On neither axis: ({}, {})", x, y),
        }
    }

    {//测试match 02
        {//Test for Enum
            let testMsg = [
                Message::Quit,
                Message::Move { x: 10, y: 20 },
                Message::Write(String::from("test write")),
                Message::Clolor(111, 222, 333,444,555,666),
                ];
            for item in testMsg.iter(){
                match item {
                    Message::Quit =>{
                        println!("line:{} Quit",std::line!());
                    },
                    Message::Move { x: 20, y } =>{
                        println!("line:{} Move",std::line!());
                    },
                    Message::Move { x, y } =>{
                        println!("line:{} Move",std::line!());
                    },
                    Message::Write(x) =>{
                        println!("line:{} Write",std::line!());
                    },
                    Message::Clolor(111, y, 30,40,50,60) =>{//即匹配了类型，又匹配了字面值
                        println!("line:{} Clolor",std::line!());
                    },
                    Message::Clolor(112, ..) =>{//用 .. 忽略剩余值
                        println!("line:{} Clolor",std::line!());
                    },
                    Message::Clolor(first, 200, 40,50,60, last) =>{//
                        println!("line:{} Clolor",std::line!());
                    },
                    Message::Clolor(x, .., y) if x==y =>{//匹配第一个和最后一个类型  && x==y
                        println!("line:{} Clolor first:{} last:{}",std::line!(), x, y);
                    },
                    
                    Message::Clolor(_, y, 30,40,50,60) =>{//使用 _ 忽略整个值
                        println!("line:{} Clolor",std::line!());
                    },
                    // Message::Clolor(first, .., last) =>{//匹配第一个和最后一个类型  ,first last 只是一个临时变量而已
                    //     println!("line:{} Clolor first:{} last:{}",std::line!(), first, last);
                    // },
                    Message::Clolor(x, .., 555, y) =>{//匹配第一个和最后一个类型  + 倒数第二个字面值
                        println!("line:{} Clolor first:{} last:{}",std::line!(), x, y);
                    },
                    Message::Clolor(x, y, z,m,n,p) =>{//匹配类型任意值
                        println!("line:{} Clolor",std::line!());
                    },
                    _ => println!("line:{} None: {:?} equals",std::line!(),item),
                }
            }
        }  
    }
}


