// 特意的将数据分配在堆上
// 数据较大时，又不想在转移所有权时进行数据拷贝
// 类型的大小在编译期无法确定，但是我们又需要固定大小的类型时
// 特征对象，用于说明对象实现了一个特征，而不是某个特定的类型

fn box_01() {
    println!("box_01 line: {} file: {}", std::line!(), std::file!());
    let arr0 = [1;1000];    //数组，在栈空间上
    let arr1 = arr0;    //这是值拷贝，并没有转移所有权

    println!("0:{} 1:{}", arr0.len(), arr1.len());
    println!("0:{:?} 1:{:?}", arr0, arr1);

    let arr2 = Box::new(['a';100]); //字符动态数组，分配在堆上
    let arr3 = arr2;        //  这里是指针的转移，所以arr2已经不再有所有权，并非值拷贝

    println!("3:{}", arr3.len());
    println!("3:{:?}", arr3);
    // println!("0:{} 1:{}", arr2.len(), arr3.len());
    // println!("0:{:?} 1:{:?}", arr2, arr3);
}

fn box_02(){
    println!("box_02 line: {} file: {}", std::line!(), std::file!());
    let s1 = String::from("abcdef");
    let s2 = Box::new(s1);
    let s3 = Box::new(s2);
    println!("box_02 s2:{:?}" ,s3);
    // println!("s1:{} s2:{} s3:{}",s1,s2,s3)
}

fn rc_01(){
    use std::rc::Rc;

    println!("box_02 line: {} file: {}", std::line!(), std::file!());
    let s1 = String::from("abcdef");
    let s2 = Rc::new(s1);
    println!("count: {:?}",Rc::strong_count(&s2));
    let s3 = Rc::clone(&s2);

    println!("s2 {:?}  s3{:?}", s2,s3);
    // println!("s1{:?} s2 {:?}  s3{:?}", s1,s2,s3);
    println!("count: {:?}",Rc::strong_count(&s2));
    println!("count: {:?}",Rc::strong_count(&s3));
    assert_eq!(2, Rc::strong_count(&s2));
    assert_eq!(Rc::strong_count(&s2), Rc::strong_count(&s3))
}

#[cfg(test)]
mod sub_tests_box {
    use super::*;
    #[test]
    pub fn test_box_01() {
        println!("test_box_01 line: {} file: {}", std::line!(), std::file!());
        box_01();
    }

    #[test]
    pub fn test_box_02(){
        println!("test_box_02 line: {} file: {}", std::line!(), std::file!());
        box_02();
    }

    #[test]
    pub fn test_rc_01(){
        println!("test_rc_01 line: {} file: {}", std::line!(), std::file!());
        rc_01();
    }
}