use std::{
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};
use md5::{Md5, Digest}; 

const SHA1_HEX_STRING_LENGTH: usize = 40;
const MD5_HEX_STRING_LENGTH: usize = 32;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage:");
        println!("hash_cracker: <wordlist.txt> <hash>");
        println!("Example: ./hash_cracker my_wordlist.txt 23db6982caef9e9152f1a5b2589e6ca3");
        return Ok(());
    }

    let wordlist_file = File::open(&args[1])?;
    let reader = BufReader::new(&wordlist_file);

    let hash_to_crack = args[2].trim();
    if hash_to_crack.len() == SHA1_HEX_STRING_LENGTH {
        println!("Cracking SHA1 hash");
        for line in reader.lines() {
            let line = line?;
            let common_password = line.trim();
            if hash_to_crack == &hex::encode(sha1::Sha1::digest(common_password.as_bytes())) {
                println!("Password found: {}", &common_password);
                return Ok(());   
            }
        }
        println!("password not found in wordlist :(");
    } else if hash_to_crack.len() == MD5_HEX_STRING_LENGTH {
        println!("Cracking MD5 hash");
        for line in reader.lines() {
            let line = line?;
            let common_password = line.trim();
            let mut hasher = Md5::new();
            hasher.update(common_password.as_bytes());
            let result = hasher.finalize();
            if hash_to_crack == &hex::encode(result) {
                println!("Password found: {}", &common_password);
                return Ok(());
            }
        }
        println!("password not found in wordlist :(");
    } else {
        return Err("Hash type not supported, try MD5 or SHA1".into());
    }

    // as almost everything is an expression, this is equivalent to return Ok(());
    Ok(())
}
