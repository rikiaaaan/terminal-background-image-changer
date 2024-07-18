use std::{env, ffi::OsString, fs, path::PathBuf, process::exit, time::SystemTime};

use uuid::Uuid;
use regex::Regex;
use rand::{self, Rng};

fn main() {

	println!("rikiaaan-terminal-background-image-changer v1.1.0");

	let started_time = SystemTime::now();

	let args = {
		let args: Vec<OsString> = env::args_os().collect();
		if args.len() != 3 {
			eprintln!("[ERROR] invalid arg counts.");
			exit(-1);
		}
		args
	};

	let background_dir = {
		let path_buf = PathBuf::from(&args[1]);
		if !path_buf.exists() {
			eprintln!("[ERROR] background directory does not exist");
			exit(-1);
		}
		path_buf
	};

	let image_extension = *(&args[2].to_str().unwrap());

	let background_source_dir = {
		let path_buf = (&background_dir).join("s");
		if !path_buf.exists() {
			eprintln!("[ERROR] background souce directory does not exitst");
			exit(-1);
		}
		path_buf
	};

	let mut file_entry: Vec<PathBuf> = Vec::new();

	// /\.jpg$/ とか /\.png$/ とか
	let extension_regex = Regex::new(format!("\\.{}$", image_extension).as_str()).unwrap();

	fs::read_dir(&background_source_dir)
		.unwrap()
		.for_each(|file| {
			let file = file.unwrap();
			let file_name = file.file_name();

			// もし指定した拡張子に合っていなかったら
			if !extension_regex.is_match(file_name.to_str().unwrap()) {
				// continue的なreturn
				return;
			}

			file_entry.push(file.path());
		});

	let file_entry_len = file_entry.len();

	if file_entry_len < 2 {
		eprintln!("[ERROR] at least 2 images are required");
		exit(-1);
	}

	let random_number = rand::thread_rng().gen_range(0..file_entry_len);

	let image_dist_path = (&background_dir).join(format!("image.{}", image_extension));
	let image_target_path = &file_entry[random_number];

	if (&image_dist_path).exists() {
		let uuid = Uuid::new_v4();
		let last_image_path = background_source_dir.join(format!("{}_last_image.{}", uuid, image_extension));
		if let Err(err) = fs::rename(&image_dist_path, &last_image_path) {
			eprintln!("{:?}", err);
		}
	}

	if let Err(err) = fs::rename(image_target_path, &image_dist_path) {
		eprintln!("{:?}", err);
	}

	let elapsed = SystemTime::now().duration_since(started_time).unwrap();
	println!("finished: {}ms", elapsed.as_millis());
}



mod tests {
	#![allow(unused_imports)]
    use std::str::FromStr;

    use super::*;

	#[test]
	fn test() {
		assert!(env::args().count() == 1);
	}

	#[test]
	fn test1() {
		let uuid_v4 = Uuid::new_v4();
		println!("{}", uuid_v4);
	}

	#[test]
	fn test2() {
		let dir = PathBuf::from_str("C:\\wallpepers\\t\\").unwrap();
		let dir2 = PathBuf::from_str("C:/wallpapers/t").unwrap();
		let dir3 = PathBuf::from_str("C:/wdllpaptrs/t").unwrap();

		println!("{:?}", dir);
		println!("{:?}", dir2);
		println!("{:?}", dir3);

		// assert!(dir.exists());
		assert!(dir2.exists());
		assert!(!dir3.exists());
	}
}
