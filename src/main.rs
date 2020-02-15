use jieba_rs::{Jieba, TFIDF, KeywordExtract};
use std::time;

//#[global_allocator]
//static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

// static WEICHENG_TXT: &str = include_str!("weicheng.txt");

fn main() {
    let jieba = Jieba::new();
    let words = jieba.cut("我们中出了一个叛徒", false);
    assert_eq!(words, vec!["我们", "中", "出", "了", "一个", "叛徒"]);

    let keyword_extractor = TFIDF::new_with_jieba(&jieba);
    let now = time::Instant::now();
    for _ in 0..1000 {
        let _top_k = keyword_extractor.extract_tags(
            "今天纽约的天气真好啊，京华大酒店的张尧经理吃了一只北京烤鸭。后天纽约的天气不好，昨天纽约的天气也不好，北京烤鸭真好吃",
            3,
            vec![],
        );
    }
    println!("{} ms", now.elapsed().as_millis());
//    assert_eq!(top_k, vec!["北京烤鸭", "纽约", "天气"]);
//    let lines: Vec<&str> = WEICHENG_TXT.split('\n').collect();
//    let now = time::Instant::now();
//    for _ in 0..50 {
//        for line in &lines {
//            let _ = jieba.cut(line, true);
//        }
//    }
//    println!("{}ms", now.elapsed().as_millis());
}
