use std::io::BufRead;

fn tail(name: String) {
    let file = std::fs::File::open(&name).unwrap();
    let pbuf = std::path::PathBuf::from(&name);
    let show = pbuf
        .file_name()
        .and_then(|s: &std::ffi::OsStr| s.to_str())
        .or(Some("nameless"))
        .unwrap();
    let mut reader = std::io::BufReader::new(file);
    loop {
        let mut s = String::new();
        let n = reader.read_line(&mut s).unwrap();
        if n == 0 {
            std::thread::sleep(std::time::Duration::from_secs(1));
            continue;
        }
        if s.ends_with("\n") {
            s.pop();
            if s.ends_with("\r") {
                s.pop();
            }
        }
        let parsed: serde_json::Value = serde_json::from_str(&s).unwrap();
        let mut obj: serde_json::map::Map<String, serde_json::Value> = parsed.as_object().unwrap().clone();
        obj.insert(String::from("file"), serde_json::Value::String(String::from(show)));
        println!("{:?}", serde_json::to_string(&obj).unwrap());
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        return;
    }
    let c = &args[1..args.len()];
    for n in c {
        let n = n.clone();
        std::thread::spawn(move || {
            tail(n);
        });
    }
    loop {
        std::thread::sleep(std::time::Duration::from_secs(3600));
    }
}
