/* std use */

/* crate use */

/* project use */

fn main() {
    let buffer = std::env::args().nth(1).unwrap();
    let json_path = std::env::args().nth(2).unwrap();
    println!("{} {}", buffer, json_path);
    let mut count = 0;
    if buffer == "1" {
        let value: serde_json::Value =
            serde_json::from_reader(niffler::from_path(json_path).unwrap().0).unwrap();
        count += value.as_object().unwrap().len();
    } else {
        let value: serde_json::Value = serde_json::from_reader(
            niffler::get_reader(Box::new(std::fs::File::open(json_path).expect("file")))
                .expect("niffler")
                .0,
        )
        .expect("json");

        count += value.as_object().unwrap().len();
    }

    println!("{}", count)
}
