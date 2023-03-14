use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;

enum UnzipResult {
    PwMismatch,
    ERROR,
    SUCCESS,
    IsDir,
}

fn try_unzip(
    zip: &mut zip::ZipArchive<File>,
    password: &String,
    index: usize,
    write_output: bool,
) -> UnzipResult {
    match zip.by_index_decrypt(index, password.as_bytes()).unwrap() {
        Ok(mut file) => {
            // try to extract the first file
            let mut buf = Vec::new();
            let file_name = format!("output/{}", file.name());
            if write_output && !file.is_dir() {
                println!("extracting: {file_name}");
            }
            match file.read_to_end(&mut buf) {
                Ok(_) => {
                    if write_output {
                        if file.is_dir() {
                            println!("creating dir: {file_name}");
                            fs::create_dir_all(file_name).unwrap();
                        } else {
                            fs::create_dir_all("output/").unwrap();
                            let mut outfile = File::create(file_name).unwrap();
                            outfile.write_all(&buf).unwrap();
                        }
                    }
                    if file.is_dir() {
                        UnzipResult::IsDir
                    } else {
                        UnzipResult::SUCCESS
                    }
                }
                Err(_) => UnzipResult::ERROR,
            }
        }
        Err(_) => UnzipResult::PwMismatch,
    }
}

// brute force zip file password using mask attack. run in release mode for best performance
fn main() {
    let file = File::open("test.zip").unwrap();
    let mut zip = zip::ZipArchive::new(file).unwrap();
    let mut trials = 0;
    let start = 0;
    let end = 1000000000;
    let mask_prefix = "";
    let mask_suffix = "";
    let write_output = true;
    let len = zip.len();
    println!("len: {len}");

    // first, choose non-directory entry to test password correctly
    let mut index = 0;

    for i in start..end {
        if index >= len {
            break;
        }
        // report progress every 1000000
        if i % 1000000 == 0 {
            println!("progress: {i}");
        }
        let mut password = String::new();
        password.push_str(mask_prefix);
        password.push_str(&i.to_string());
        password.push_str(mask_suffix);
        let success = match try_unzip(&mut zip, &password, index, false) {
            UnzipResult::SUCCESS => {
                println!("password found: {password}");
                true
            }
            UnzipResult::PwMismatch => false,
            UnzipResult::ERROR => {
                trials += 1;
                if trials % 1000 == 0 {
                    println!("trials: {trials}");
                    println!("trying: {password}");
                }
                false
            }
            UnzipResult::IsDir => {
                index += 1;
                false
            }
        };
        if success && write_output {
            // iterate over all files (except selected index) in the zip archive
            for i in 0..len {
                match try_unzip(&mut zip, &password, i, write_output) {
                    UnzipResult::SUCCESS => {}
                    UnzipResult::PwMismatch => {
                        println!("password mismatch: {password}");
                    }
                    UnzipResult::ERROR => {
                        println!("error extracting: {i}");
                    }
                    UnzipResult::IsDir => {}
                }
            }
            break;
        }
    }
}
