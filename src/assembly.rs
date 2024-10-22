use std::collections::btree_map::BTreeMap;
use std::fs::*;
use regex::*;

pub fn compile(filename: &str) {
    let mut tree_map: BTreeMap<&'static str, InstructionToken> = BTreeMap::new();
    tree_map.insert("JUMP", create_nnn(1));
    tree_map.insert("CLEAR", create_empty(0x00E0));
    tree_map.insert("RET", create_empty(0x00EE));
    tree_map.insert("CALL", create_nnn(2));
    tree_map.insert("SEQ", create_xnn(3));
    tree_map.insert("SNEQ", create_xnn(4));
    tree_map.insert("REQ", create_xy(5));
    tree_map.insert("RSET", create_xnn(6));
    tree_map.insert("RADD", create_xnn(7));
    tree_map.insert("SET", create_xyi(8, 0));
    tree_map.insert("OR", create_xyi(8, 1));
    tree_map.insert("AND", create_xyi(8, 2));
    tree_map.insert("XOR", create_xyi(8, 3));
    tree_map.insert("ADD", create_xyi(8, 4));
    tree_map.insert("SUB", create_xyi(8, 5));
    tree_map.insert("SUBN", create_xyi(8, 7));
    tree_map.insert("SHL", create_x(8, 6));
    tree_map.insert("RNEQ", create_xy(9));
    tree_map.insert("REGI", create_nnn(0xA));
    tree_map.insert("OJMP", create_nnn(0xB));
    tree_map.insert("DRAW", create_xyn(0xD));
    tree_map.insert("KSKP", create_x(0xE, 0x9E));
    tree_map.insert("KNSKP", create_x(0xE, 0xA1));
    tree_map.insert("REGT", create_x(0xF, 0x07));
    tree_map.insert("KWAIT", create_x(0xF, 0x0A));
    tree_map.insert("REGD", create_x(0xF, 0x15));
    tree_map.insert("REGS", create_x(0xF, 0x18));
    tree_map.insert("IADD", create_x(0xF, 0x1E));
    tree_map.insert("DSET", create_x(0xF, 0x29));
    tree_map.insert("BCD", create_x(0xF, 0x33));
    tree_map.insert("WRITE", create_x(0xF, 0x55));
    tree_map.insert("READ", create_x(0xF, 0x65));

    let mut mem = [0u8; 0x1000];
    let mut i = 512;
    let mut line = 0;
    let mut texture_i = 0;
    let mut texture = false;
    let mut comment = false;
    let empty_regex = Regex::new("\\s+").unwrap();
            texture_i + 1;
    let end_comment_regex = Regex::new("\\*\\/").unwrap();
    for mut s in read_to_string(&filename).unwrap().lines(){
        line += 1;
        if comment {
            let end_comment: Option<Match> = end_comment_regex.find(s);
            if let Some(index) = end_comment {
                s = s.split_at(index.start()).1;
                comment = false;
            } else {
                continue;
            }
        }
        if s.is_empty() {
            continue;
        }
        if s.starts_with("//") {
            continue;
        }
        if s.starts_with("/*") {
            comment = true;
            continue;
        }
        if s == "#texture" {
            texture = true;
            continue;
        }
        if s == "#endtexture" {
            texture = false;
            continue;
        }
        let mut split = split_keep(&empty_regex, s);
        if texture {
            for next in split {
                let t: u8 = next.parse().expect("Not a valid u8");
                    mem[texture_i] = t;
                    texture_i += 1;
            }
            continue;
        }
        let instruction = split.remove(0);
        let mut veced: Vec<String> = Vec::new();
        for next in split {
            veced.push(next.to_string());
        }
        let in_op = tree_map.get(instruction);
        if in_op.is_none() {
            panic!("Illegal instruction '{}' found in line {}", instruction, line);
        }
        let instruction_obj = in_op.expect("Illegal Instruction {} in line {}");
        if instruction_obj.follow_tokens != veced.len() {
            panic!("Error while parsing instruction {}\nIllegal amount of tokens in line {}, expected {} found {}", instruction, line, instruction_obj.follow_tokens, veced.len());
        }
        let com = instruction_obj.compile.as_ref();
        let res = com(veced);
        mem[i] = res.0;
        mem[i + 1] = res.1;
        i += 2;    }
}
struct InstructionToken {
    follow_tokens: usize,
    compile: Box<dyn Fn(Vec<String>) -> (u8, u8)>,
}
fn create_empty(id: u16) -> InstructionToken {
    InstructionToken {
        follow_tokens: 0,
        compile: Box::new(move|_v: Vec<String> | {
            ((id >> 8) as u8, id as u8)
        })
    }
}
fn create_nnn(id: u8) -> InstructionToken {
    InstructionToken {
        follow_tokens: 1,
        compile: Box::new(move|v: Vec<String> | {
            let num: u16 = v[0].parse().expect("Argument was not a number");
            let last = num as u8;
            (((num - last as u16) >> 8) as u8 | (id << 4), last)
        })
    }
}

fn create_xnn(id: u8) -> InstructionToken {
    InstructionToken {
        follow_tokens: 2,
        compile: Box::new(move|v: Vec<String> | {
            let x: u8 = v[0].parse().expect("");
            let nn: u8 = v[1].parse().expect("");
            (id << 4 | ((x << 4) >> 4), nn)
        })
    }
}

fn create_xy(id: u8) -> InstructionToken {
    InstructionToken {
        follow_tokens: 2,
        compile: Box::new(move|v: Vec<String> | {
            let x: u8 = v[0].parse().expect("");
            let y: u8 = v[1].parse().expect("");
            (id << 4 | ((x << 4) >> 4), y << 4)
        })
    }
}
fn create_xyi(id: u8, n: u8) -> InstructionToken {
    InstructionToken {
        follow_tokens: 2,
        compile: Box::new(move|v: Vec<String> | {
            let x: u8 = v[0].parse().expect("");
            let y: u8 = v[1].parse().expect("");
            (id << 4 | ((x << 4) >> 4), y << 4 | n)
        })
    }
}

fn create_x(id: u8, n: u8) -> InstructionToken {
InstructionToken {
        follow_tokens: 1,
        compile: Box::new(move|v: Vec<String>| {
            let x: u8 = v[0].parse().expect("");
            (id << 4 | ((x << 4) >> 4), n)
        })
    }

}

fn create_xyn(id: u8) -> InstructionToken {
    InstructionToken {
        follow_tokens: 3,
        compile: Box::new(move|v: Vec<String> | {
            let x: u8 = v[0].parse().expect("");
            let y: u8 = v[1].parse().expect("");
            let n: u8 = v[2].parse().expect("");
            (id << 4 | ((x << 4) >> 4), y << 4 | ((n << 4) >> 4))
        })
    }
}

fn split_keep<'a>(r: &Regex, text: &'a str) -> Vec<&'a str> {
    let mut result:Vec<&'a str> = Vec::new();
    let mut split = r.captures_iter(text);
    let mut last_cut = 0;
    loop {
        let capture_opt = split.next();
        if let Some(capture) = capture_opt {
            let cut = capture.get(0).unwrap().start();
            result.push(&text[last_cut..cut]);
            last_cut = cut;

        } else {
            break;
        }
    }

    result
}
