use std::env;
mod iohelper;

fn main() {
    // let id = Uuid::new_v4();
    // println!("uuid = {}", id)

    let args: Vec<String> = env::args().collect();
    // // println!("{:?}", args)
    let filename = &args[1];
    // println!("Syncing {}", filename);



    iohelper::test(filename.to_string());
}
