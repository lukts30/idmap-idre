use std::{fs, cmp, env};
use std::error::Error;

#[derive(Debug)]
struct Mapping {
    uid: u32,
    loweruid: u32,
    count: u32,
}

impl Mapping {
    fn new(uid: u32,loweruid: u32,count: u32) -> Self {
        Mapping {
            uid,
            loweruid,
            count,
        }
    }
    fn sort(m: &mut [Mapping]) {
        m.sort_by(|a,b| a.uid.cmp(&b.uid));
    }

    fn process(m: &mut [Mapping],mut container_uid: u32,mut on_disk_uid: u32,mut amount: u32) {
        Mapping::sort(m);
        for line in m.iter() {
            if(std::ops::Range { start: line.uid, end: line.uid+line.count}.contains(&container_uid)) {
                let remaining_cap = (line.uid+line.count).saturating_sub(container_uid);
                
                let k = cmp::min(remaining_cap, amount);
                
                println!("{} {} {}",on_disk_uid,line.loweruid-line.uid+container_uid,k);
                on_disk_uid = on_disk_uid + k;
                container_uid = container_uid + k;
                amount = amount - k;

                if amount <= 0 {
                    break;
                }
            }
        }
        if amount > 0 {
            eprintln!("Could not map id {} {}",container_uid,amount);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() > 4);
    let map = match args[1].as_str() {
        "uid" => fs::read_to_string("/proc/self/uid_map")?,
        "gid" => fs::read_to_string("/proc/self/gid_map")?,
        _ => {
            panic!("Args error!")
        },
    };

    let mut v = helper(&map);
    let a = (args.len()-2) / 3;
    let mut k = 2;
    for i in 0..a {
        dbg!(i);
        Mapping::process(&mut v, args[k].parse()?, args[k+1].parse()?, args[k+2].parse()?);
        k = k+3;
    }

    Ok(())
}

fn helper(input: &str) -> Vec<Mapping>{
    let mut v = Vec::new();
    for lines in input.lines() {
        let mut num: [u32;3] = [0;3];
        for (i,n) in lines.split_ascii_whitespace().enumerate() {
            num[i] = n.parse().unwrap();
            assert!(i < 3);
        }
        let m = Mapping::new(num[0], num[1], num[2]);
        v.push(m);
    }
    v
}
