use std::{fs::File, io};

pub fn add(left: usize, right: usize) -> usize {
    println!("add line: {:?} left: {:?} right: {:?}", line!(), left,right);
    left + right
}

pub fn sub(left: i32, right: i32) -> i32 {
    println!("sub line: {:?} left: {:?} right: {:?}", line!(), left,right);
    // let val = left-right;
    // panic!("crash and burn");
    left-right
}

// pub fn mul(left: i32, right: i32) -> Result<T,E> {
//     println!("mul line: {:?} left: {:?} right: {:?}", line!(), left, right);
//     if left == right {
//         Result::Ok(())
//     } else {
//         Result::Err(())
//     }
// }

pub fn open_file(path: &str) -> Result<File, io::Error> {
    println!("open file: {}", path);
    let mut f = File::open("path.txt");
    let f = match f {
        Ok(file) => return Ok(file),
        Err(e) => return Err(e),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn it_works_01() {
        let result = add(2, 2);
        println!("line:{} run test!!", std::line!());
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore = "reason"]
    pub fn it_works_02() {
        let result = sub(5, 2);
        // panic!("sub(1, 2)");
        println!("line:{} run test!!", std::line!());
        assert_eq!(result, 3);
    }

    #[test]
    pub fn test_open_file() {
        let path = "abc.txt";
        println!("line:{} run test open file!!", std::line!());
        // open_file(path).unwrap();
        let res =  open_file(path);
        // if res == Ok(File) {

        // }

        open_file(path).expect("系统找不到abc.txt");

    }
}
