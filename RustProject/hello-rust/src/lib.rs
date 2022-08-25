pub struct Song {
    author: String,
    name: String,
}

pub async fn learn_song() -> Song {
    Song {
        author: "曲婉婷".to_string(),
        name: String::from("《我的歌声里》"),
    }
}

pub async fn sing_song(song: Song) {
    println!(
        "给大家献上一首{}的{} ~ {}",
        song.author, song.name, "你存在我深深的脑海里~ ~"
    );
}

pub async fn dance() {
    println!("唱到情深处，身体不由自主的动了起来~ ~");
}

#[cfg(test)]
mod tests {
    // use super::*;
    // #[test]
    // fn learn_and_sing() {
    //     // 这里使用`.await`来等待学歌的完成，但是并不会阻塞当前线程，该线程在学歌的任务`.await`后，完全可以去执行跳舞的任务
    //     let song = learn_song().await;

    //     // 唱歌必须要在学歌之后
    //     sing_song(song).await;
    // }

    // fn async_main() {
    //     let f1 = learn_and_sing();
    //     let f2 = dance();

    //     // `join!`可以并发的处理和等待多个`Future`，若`learn_and_sing Future`被阻塞，那`dance Future`可以拿过线程的所有权继续执行。若`dance`也变成阻塞状态，那`learn_and_sing`又可以再次拿回线程所有权，继续执行。
    //     // 若两个都被阻塞，那么`async main`会变成阻塞状态，然后让出线程所有权，并将其交给`main`函数中的`block_on`执行器
    //     futures::join!(f1, f2);
    // }
    // #[test]
    // fn async_main_abc() {
    //     println!("main");
    // }
}