use std::env;
use rand::thread_rng;
use rand::prelude::SliceRandom;

const DATA_JSON: &str = include_str!("data/data.json");

fn main() {
    // 解析 JSON 数据
    let data: Vec<String> = serde_json::from_str(DATA_JSON).expect("Failed to parse JSON");

    // 从命令行参数中获取连击数量
    let args: Vec<String> = env::args().collect();
    let n: usize = if args.len() >= 2 {
        args[1].parse().expect("Invalid number")
    } else {
        1 // 默认为1
    };

    // 从数据中随机选择n个项
    let mut rng = thread_rng();
    let mut result = Vec::new();
    for _ in 0..n {
        let random_item = data.choose(&mut rng).expect("No items found");
        result.push(random_item.clone());
    }

    // 将结果转换为 JSON 并打印输出
    let json_output = serde_json::to_string_pretty(&result).expect("Failed to convert to JSON");
    println!("{}", json_output);
}
