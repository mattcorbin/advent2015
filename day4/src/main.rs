extern crate md5;

fn main() {
    let secret_key = "yzbqklnj";
    let mut test: u64 = 0;
    loop {
        let digest = md5::compute(format!("{0}{1}", secret_key, test));
        let hex = format!("{:x}", digest);
        if hex.starts_with("000000") {
            break
        }
        test += 1;
    }
    println!("Answer: {0}", test);
}
