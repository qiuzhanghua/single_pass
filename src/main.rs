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


// https://github.com/baiziyuandyufei/text_classification/blob/master/Chap3/%E5%A2%9E%E9%87%8F%E8%81%9A%E7%B1%BB.py
// https://zhuanlan.zhihu.com/p/91007237?utm_source=ZHShareTargetIDMore&utm_medium=social&utm_oi=885424648970436608
// https://zhuanlan.zhihu.com/p/65366198?utm_source=ZHShareTargetIDMore&utm_medium=social&utm_oi=885424648970436608
