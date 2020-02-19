//#[macro_use]
//extern crate serde_derive;

use std::time;
use std::error::Error;
use std::fs::File;
use dotenv::dotenv;
use std::env;
use encoding_rs::GB18030;
use encoding_rs_io::DecodeReaderBytesBuilder;
use std::collections::{HashMap, HashSet};
use chrono::prelude::*;

const THRESHOLD: f64 = 0.2;
const DATA_DIR: &str = "./data/";

fn main() {
    dotenv().ok();
    let threshold = env::var("threshold").unwrap_or("0.2".to_string()).parse().unwrap_or(THRESHOLD);
    let data_dir = env::var("data_dir").unwrap_or(DATA_DIR.to_string());
    let mut cluster_map = HashMap::<String, Cluster>::new(); //
    let mut reverse_map = HashMap::<String, HashSet<String>>::new(); // word -> cluster key collection

    let begin: DateTime<Utc> = Utc::now();
    println!("UTC now is: {}", begin);
    let paths = std::fs::read_dir(data_dir).unwrap();
    for path in paths {
        let mut doc_map = HashMap::<String, Doc>::new();
        let now = time::Instant::now();

        if let Ok(d) = path {
            if d.path().is_file() {
//                let file = File::open(d.path()).unwrap();
                read_scv(&d.path().display().to_string(), &mut doc_map);

                // for (id, doc) in &doc_map {
                //     println!("{}", doc.features.len());
                // }
                //
                // println!("{} ms", now.elapsed().as_millis());

//                let mut count = 0;

                for (doc_id, doc) in &doc_map {
//                    count += 1;
//        println!("{}, {}", count, doc_id);
                    let words = doc.features.clone();
                    let mut orphan = true;
                    for cluster_id in get_candidate_cluster(&words, &reverse_map) {
                        if let Some(cluster) = cluster_map.get_mut(&cluster_id)
                        {
                            if is_similar(&cluster, &doc, &doc_map, threshold) {
                                cluster.members.insert(doc_id.clone());
                                orphan = false;
                                break;
                            }
                        }
                    };
                    if orphan {
                        let new_cluster_id = format!("cluster_{}", cluster_map.len());
                        let mut members = HashSet::<String>::new();
                        members.insert(doc_id.clone());
                        cluster_map.insert(new_cluster_id.clone(), Cluster {
                            id: new_cluster_id.clone(),
                            doc_id: doc_id.clone(),
                            members,
                        });

                        for word in words {
                            if !reverse_map.contains_key(&word) {
                                reverse_map.insert(word.clone(), HashSet::<String>::new());
                            };
                            reverse_map.get_mut(&word).unwrap().insert(new_cluster_id.clone());
                        }
                    }
                }
                println!("cluster len = {}", cluster_map.len());
                println!("reverse map len = {}", reverse_map.len());
                println!("{} ms", now.elapsed().as_millis());
            }
        }
    }

    let end: DateTime<Utc> = Utc::now();
    println!("UTC now is: {}", end);
}


fn read_scv(file_path: &str, doc_map: &mut HashMap<String, Doc>) -> Result<(), Box<dyn Error>> {
    println!("read file: {} ......", file_path);
    let file = File::open(file_path)?;
    let transcoded = DecodeReaderBytesBuilder::new()
        .encoding(Some(GB18030))
        .build(file);
    let mut rdr = csv::Reader::from_reader(transcoded);
    let mut count = 0;
    // for result in rdr.deserialize::<Record>() {
    for record in rdr.records() {
        count += 1;
        let result = record.unwrap();
        let features = &result[1];
        let x = features.trim_end_matches(']').trim_start_matches('[')
            .split(',')
            .map(|s| { s.trim().trim_start_matches('\'').trim_end_matches('\'') })
            .map(|s| s.to_string())
            .collect::<HashSet<String>>();
        let title = result[6].to_string();
        let id = result[8].to_string();
        // if count <= 6 { println!("{:?}", x) };
        // if count <= 6 { println!("{:?}", title) }
        // if count <= 6 { println!("{:?}", id) };
        doc_map.insert(id.clone(), Doc {
            id,
            title,
            features: x,
        });
    }
    println!("read count = {:?}", count);
    println!("map len = {:?}", doc_map.len());
    println!("read file: {} ended.", file_path);
    Ok(())
}


fn is_similar(cluster: &Cluster, doc: &Doc, doc_map: &HashMap<String, Doc>, threshold: f64) -> bool {
    let id = cluster.doc_id.clone();
    let cf = if let Some(d) = doc_map.get(&id) {
        d.features.clone()
    } else { HashSet::<String>::new() };
    let df = doc.features.clone();
//    println!("cf len = {:?}, df len = {:?}", cf.len(), df.len() );
    let x =
        df.intersection(&cf).collect::<HashSet<&String>>().len() as f64
            / df.union(&cf).collect::<HashSet<&String>>().len() as f64;
//    println!("{:?}", x);
    x > threshold
}

fn get_candidate_cluster(words: &HashSet<String>, reverse_map: &HashMap::<String, HashSet<String>>) -> HashSet<String> {
    let mut ans = HashSet::<String>::new();
    for word in words {
        if let Some(m) = reverse_map.get(word) {
            for x in m {
                ans.insert(x.clone());
            }
        }
    }
//    if ans.len() > 0  { println!("{:?}", ans); }
    ans
}

#[derive(Debug)]
struct Doc {
    id: String,
    title: String,
    features: HashSet<String>,
}

#[derive(Debug)]
struct Cluster {
    id: String,
    doc_id: String,
    // 第一篇文档ID
    members: HashSet<String>,  // 文档IDs
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufReader, BufRead};

    #[test]
    fn test_file() {
        let paths = std::fs::read_dir("./").unwrap();
        for path in paths {
            if let Ok(d) = path {
                if d.path().is_file() {
                    let file = File::open(d.path()).unwrap();
                    let reader = BufReader::new(file);
                    for (index, line) in reader.lines().enumerate() {
                        let line = line.unwrap(); // Ignore errors.
                        // Show the line and its number.
                        println!("{}. {}", index + 1, line);
                    }
                }
            }
//            println!("Name: {}", path.unwrap().path().display())
        };
    }
}

// https://github.com/baiziyuandyufei/text_classification/blob/master/Chap3/%E5%A2%9E%E9%87%8F%E8%81%9A%E7%B1%BB.py
// https://zhuanlan.zhihu.com/p/91007237?utm_source=ZHShareTargetIDMore&utm_medium=social&utm_oi=885424648970436608
// https://zhuanlan.zhihu.com/p/65366198?utm_source=ZHShareTargetIDMore&utm_medium=social&utm_oi=885424648970436608
