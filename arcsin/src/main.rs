// ギャンブルに潜む逆正弦法則【勝ち越す人と負け越す人】
// https://www.youtube.com/watch?v=4iMIydZM2RE

use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    // コインを12回投げる時の全パターン（2^12）を確認
    for i in 0..4096 {
        // 獲得ポイント、勝ち越していた期間、負け越していた期間
        let (points, over0, under0) = random_walk(i);

        println!("{} {:#012b} {} {} {}", i, i, points, over0, under0);

        // 勝ち越している期間と負け越している期間の差
        let count = map.entry(over0 - under0).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

fn random_walk(i: i32) -> (i32, i32, i32) {
    let mut over0 = 0;
    let mut under0 = 0;

    let mut now = 0;

    println!("{} {:#012b}", i, i);

    for j in 0..12 {
        if ((i >> j) % 2) == 1 {
            now += 1;
        } else {
            now -= 1;
        }

        match now {
            1.. => over0 += 1,
            core::i32::MIN..=-1 => under0 += 1,
            0 => (),
        }
    }

    (now, over0, under0)
}
