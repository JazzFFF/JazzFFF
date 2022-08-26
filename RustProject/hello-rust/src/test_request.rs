use std::collections::HashMap;
use tokio::time::Duration;
use reqwest::Client;
use mini_redis::{client,Result};

#[tokio::main]
async fn request_client_func() -> Result<()>{
    println!("request_client_func() called with client2222");
    if false {
        // let resp = reqwest::get("www.baidu.com")
        // .await?
        // .json::<HashMap<String,String>>()
        // .await?;
        // println!("{:#?}", resp);
        // ok(());
    }

     // 建立与mini-redis服务器的连接
     let mut client = client::connect("127.0.0.1:6379").await?;

     // 设置 key: "hello" 和 值: "world"
     client.set("hello", "world".into()).await?;
 
     // 获取"key=hello"的值
     let result = client.get("hello").await?;
 
     println!("从服务器端获取到结果={:?}", result);
 
     Ok(())
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_request_client_func(){
        println!("request_client_func() called with client1111");
        request_client_func();
    }
    
}