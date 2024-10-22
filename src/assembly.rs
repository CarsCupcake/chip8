use std::collections::btree_map::BTreeMap;
use std::fs::*;

fn compile(filename: &str) {
    let mut tree_map: BTreeMap<&'static str, InstructionToken> = BTreeMap::new();
    tree_map.insert("JUMP", InstructionToken {
        follow_tokens: 1,
        compile: (|v| {
            let jump_to: u16 = v[0].parse().expect("Some");
            let last = jump_to as u8;
            (((jump_to - last as u16) >> 8) as u8 | 0x10, last)
        })
    });
    let mut mem = [0u8; 0x1000];
    let mut i = 512;
    for s in read_to_string(&filename).unwrap().lines(){
        if s.is_empty(){
            continue;
        }
        let mut split = s.split("\\s+");
        let instruction = split.next().expect("");
        let mut veced: Vec<String> = Vec::new();
        loop {
            if let Some(next) = split.next() {
            veced.push(next.to_string());
            } else {
                break;
            }
        }
        let com = tree_map.get(instruction).expect("").compile;
        let res = com(veced);
        mem[i] = res.0;
        mem[i + 1] = res.1;
        i += 2;
    }
}
struct InstructionToken {
    follow_tokens: usize,
    compile: fn (std::vec::Vec<String>) -> (u8, u8),
}
