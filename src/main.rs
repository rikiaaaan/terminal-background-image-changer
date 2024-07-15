#![allow(unused)]
use std::{env, ffi::OsString, fs, path::{Path, PathBuf}, process::{self, exit}, time::SystemTime};

use uuid::Uuid;
use regex::Regex;
use rand::{self, Rng};

fn main() {

	println!("rikiaaan-terminal-background-image-changer v1.0.0");

	let started_time = SystemTime::now();

	let args = {
		let mut args: Vec<OsString> = env::args_os().collect();
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

	let uuid_v4 = Uuid::new_v4();

	let mut file_counts: usize = 0;

	let current_uuid_regex = Regex::new(uuid_v4.to_string().as_str()).unwrap();

	fs::read_dir(&background_source_dir)
		.unwrap()
		.for_each(|file| {
			let file = file.unwrap();
			let file_name = file.file_name();

			if current_uuid_regex.is_match(file_name.to_str().unwrap()) {
				return ();
			}

			let file_rename_path = (&background_source_dir).join(format!("{}_{}.{}", uuid_v4, file_counts, image_extension));

			fs::rename(file.path(), file_rename_path);

			file_counts += 1;
		});

	if file_counts < 2 {
		eprintln!("[ERROR] at least 2 images are required");
		exit(-1);
	}

	let random_number = rand::thread_rng().gen_range(0..file_counts);

	let image_dist_path = (&background_dir).join(format!("image.{}", image_extension));
	let image_target_path = (&background_source_dir).join(format!("{}_{}.{}", uuid_v4, random_number, image_extension));

	if (&image_dist_path).exists() {
		let last_image_path = background_source_dir.join(format!("last_image.{}", image_extension));
		fs::rename(&image_dist_path, &last_image_path);
	}

	fs::rename(&image_target_path, &image_dist_path);

	let elapsed = SystemTime::now().duration_since(started_time).unwrap();
	println!("finished: {}ms", elapsed.as_millis());
}



mod tests {
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
