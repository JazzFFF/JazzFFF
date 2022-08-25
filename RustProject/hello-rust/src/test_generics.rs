use std::{process::Output, sync::Arc, fmt::Display};
use std::ops::Add;


//Test01 结构体泛型
// #[derive(Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

pub enum Data<T> {
    One,
    Two(T),
    Point(T, T),
}
// impl<T: std::fmt::Debug> Point<T>{
impl<T:std::fmt::Debug> Point<T>{
    pub fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }

    pub fn get_x(&self) -> &T {
        println!("line:{} x:{:?} y:{:?}", std::line!(), self.x, self.y);
        &self.x
    }

    pub fn get_mut_x(&mut self) -> &mut T {
        &mut self.x
    }

    pub fn get_y(&self) -> &T {
        println!("line:{} x:{:?} y:{:?}", std::line!(), self.x, self.y);
        &self.y
    }

    pub fn get_mut_y(&mut self) -> &mut T {
        &mut self.y
    }

    // pub fn x_y_add(&self) -> T {
    //     &self.x + &self.y
    // }

    // pub fn add(&self) -> &T {
    //     // self.x + self.y
    //     // &self.x
    //     // Result::err()
    // }
}

//Test02 测试特征
pub trait Summary<T> {
    fn summarysize_author(&self) -> &T; //特征中的接口定义，需要被 实现
    // fn summaryize(&self) -> String;
    fn summaryize(&self) -> &T{     //特征中的接口默认实现 ，可以被 重载
        println!("默认方法的调用 line:{}", std::line!());
        self.summarysize_author()
    }
}

#[derive(Debug)]
pub struct Post<T> {
    pub title:T,
    pub author:T,
    pub content:T,
}

impl Summary<String> for Post<String> {//为post结构 定义Summary特征
    // pub fn new(title:T, author:T, content:T) -> Post<T> {
    //     Post {
    //             title,
    //             author,
    //             content,
    //     }
    // }

    fn summarysize_author(&self) -> &String {   //特征中的接口的实现
        println!("line:{} x:{:?} y:{:?} y:{:?}", std::line!(), self.title,self.author,self.content);
        &self.content
        // String::from("test summary")
    }

    fn summaryize(&self) -> &String {   //特征中的接口重载的实现
        println!("我自己实现特征中默认方法的重载 line:{}", std::line!());
        self.summarysize_author()
    }
}

impl Display for Post<String> { //为post结构定义Display特征
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from("post"))
    }
}

//
impl <T: Add<T,Output = T>> Add for Post<T> {
    type Output = Self;
    fn add(self, p: Post<T>) -> Post<T> { 
        Post { title: self.title + p.title, author: self.author+p.author, content: self.content+p.content}
    }
}

pub struct Weibo<T> {
    pub name: T,
    pub content: T,
}

impl Summary<String> for Weibo<String> {
    fn summarysize_author(&self) -> &String {//特征中的接口的实现
        println!("line:{} x:{:?} y:{:?} y:{:?}", std::line!(), self.name, self.content, self.content);
        &self.name
    }
}

pub struct Message<T> {
    username:T,
    content: T,
}

// impl<T: Summary> for Message<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{} {}", self.username, self.content)
//     }
// }
//Test03 特征作为函数参数
pub fn test_trait_summary<T>(item: &impl Summary<T>) -> &T{
    println!("line:{} 测试特征作为函数参数来使用 ", std::line!());
    item.summaryize()
}

pub fn test_trait_summary_display<T>(item: &(impl Summary<T>+Display)) -> &T{
    println!("line:{} 测试特征作为函数参数来使用,同时可以输出item item:{}", std::line!(), item);
    item.summaryize()
}