use std::{env, ffi::{OsStr, OsString}, fs::{self, File}, io::{BufRead, BufReader, BufWriter, Write}, path::PathBuf, process::exit, thread, time::{Duration, SystemTime}};

use uuid::Uuid;
use rand::{self, Rng};

// TODO: 選択の重複をなくす
fn main() {

	println!("rikiaaan-terminal-background-image-changer v1.3.0");

	let started_time = SystemTime::now();

	let args = {
		let args: Vec<OsString> = env::args_os().collect();
		if args.len()-1 != 2 {
			eprintln!("[ERROR] invalid arg counts.");
			exit(-1);
		}
		args
	};

	// new
	let background_image_source_dir = {
		let path_buf = PathBuf::from(&args[1]);
		if !path_buf.exists() {
			eprintln!("[ERROR] background image source directory does not exist");
			exit(-1);
		}
		path_buf
	};

	let windows_terminal_settings_json_path = {
		let path_buf = PathBuf::from(&args[2]);
		if !path_buf.exists() {
			eprintln!("[ERROR] settings.json does not exist");
			exit(-1);
		}
		path_buf
	};
	
	let random_unique_name_path = {
		let image_random_change_started_time = SystemTime::now();

		let mut file_entry: Vec<PathBuf> = Vec::new();

		fs::read_dir(&background_image_source_dir)
			.unwrap()
			.for_each(|file| {
				let file = file.unwrap();
				let file_metadata = file.metadata().unwrap();

				// もしファイルじゃないなら
				if !file_metadata.is_file() {
					// continue的なreturn
					return;
				}

				file_entry.push(file.path());
			});


		let image_target_path = {
			let file_entry_len = file_entry.len();
			
			if file_entry_len == 1 {
				eprintln!("[ERROR] image source directory is empty");
				exit(-1);
			}

			let random_number = rand::thread_rng().gen_range(0..file_entry_len);
			&file_entry[random_number]
		};

		let image_random_unique_name_path = {
			let uuid = Uuid::new_v4();
			let target_image_extension = image_target_path.extension().unwrap_or(OsStr::new(""));

			let unique_name = format!("{}_image.{}", uuid, target_image_extension.to_str().unwrap());

			background_image_source_dir.join(unique_name)
		};

		if let Err(err) = fs::rename(image_target_path, &image_random_unique_name_path) {
			eprintln!("{:?}", err);
		}

		let elapsed = SystemTime::now().duration_since(image_random_change_started_time).unwrap();
		println!("image random selection finished: {}ms", elapsed.as_millis());

		image_random_unique_name_path
	};


	let mut settings_file_change_vec: Vec<u8> = Vec::new();
	let mut settings_file_background_image_null_vec: Vec<u8> = Vec::new();

	{
		let settings_file_read_started_time = SystemTime::now();

		let settings_json_file_read = File::open(&windows_terminal_settings_json_path).unwrap();
		let reader = BufReader::new(settings_json_file_read);

		let random_unique_name = random_unique_name_path.to_str().unwrap_or("").replace("\\", "\\\\");

		reader.lines().for_each(|line| {
			let line = {
				let line = line.unwrap_or(String::new());
				format!("{}\n", line)
			};
			let line_bytes = line.as_bytes();

			if (&line).contains("\"backgroundImage\": ") {
				let buf = "\"backgroundImage\": \"\",\n".as_bytes();

				_ = settings_file_background_image_null_vec.write(buf);
				_ = settings_file_change_vec.write_fmt(format_args!("\t\t\t\"backgroundImage\": \"{}\",\n", random_unique_name));
			} else {
				_ = settings_file_background_image_null_vec.write(line_bytes);
				_ = settings_file_change_vec.write(line_bytes);
			}
		});

		let elapsed = SystemTime::now().duration_since(settings_file_read_started_time).unwrap();
		println!("settings.json read finished: {}ms", elapsed.as_millis());
	}



	{
		let settings_json_file_write = File::create(&windows_terminal_settings_json_path).unwrap();
		let mut file_writer = BufWriter::new(settings_json_file_write);

		let background_image_null_buf = settings_file_background_image_null_vec.as_slice();
		_ = file_writer.write_all(background_image_null_buf);
		_ = file_writer.flush();
	}


	// 遅延させることでWindows Terminalにsettings.jsonの変更を検知させる
	println!("settings.jsonバックアップ書き込み遅延開始");
	thread::sleep(Duration::from_millis(200));
	println!("settings.jsonバックアップ書き込み遅延終了");


	{
		let settings_json_file_write = File::create(&windows_terminal_settings_json_path).unwrap();
		let mut file_writer = BufWriter::new(settings_json_file_write);

		let changed_buf = settings_file_change_vec.as_slice();
		_ = file_writer.write_all(changed_buf);
		_ = file_writer.flush();
	}


	let elapsed = SystemTime::now().duration_since(started_time).unwrap();
	println!("finished: {}ms", elapsed.as_millis());
}



mod tests {
	#![allow(unused_imports)]
	use std::{io::{BufRead, BufReader, BufWriter, Write}, os::windows, ptr::read, str::FromStr, thread, time::Duration};

	use fs::File;

	use super::*;

	#[test]
	fn test() {
		println!("{:?}", std::env::current_dir().unwrap());
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

	#[test]
	fn test3() {
		let path = PathBuf::from_str(r"C:\Users\[user_name]\test.txt").unwrap();
		let path2 = PathBuf::from_str(r"C:\Users\[user_name]\test2.txt").unwrap();
		println!("{:?}", path);
		println!("{:?}", path2);
		let file = File::open(path).unwrap();
		let file2 = File::create(path2).unwrap();

		let reader = BufReader::new(file);
		let mut writer = BufWriter::new(file2);

		reader.lines().for_each(|line| {
			let line = line.unwrap();
			// writer.write(&line.as_bytes()).unwrap();
			writer.write_fmt(format_args!("{}\n", line)).unwrap();
			println!("{}", line);
		});

		

		_ = writer.flush();
	}

	#[test]
	fn test4() {
		let path = PathBuf::from_str(r"C:\Users\[user_name]\AppData\Local\Packages\Microsoft.WindowsTerminal_8wekyb3d8bbwe\LocalState\settings.json").unwrap();
		let path_test = PathBuf::from_str(r"C:\Users\[user_name]\AppData\Local\Packages\Microsoft.WindowsTerminal_8wekyb3d8bbwe\LocalState\sett_test.json").unwrap();

		println!("{:?}", path);
		assert!(path.exists());

		// let mut file_backup_writer = BufWriter::new(Vec::new());
		// let mut file_background_image_nuller = BufWriter::new(Vec::new());

		let mut file_backup_vec: Vec<u8> = Vec::new();
		let mut file_background_image_null_vec: Vec<u8> = Vec::new();

	
		{
			let settings_file_read = File::open(&path).unwrap();
			let reader = BufReader::new(settings_file_read);

			reader.lines().for_each(|line| {
				let line = {
					let line = line.unwrap_or(String::new());
					format!("{}\n", line)
				};
				let line_bytes = line.as_bytes();

				// file_backup_writer.write(line_bytes).unwrap();
				file_backup_vec.write(line_bytes).unwrap();

				if (&line).contains("\"backgroundImage\": ") {
					let buf = "\"backgroundImage\": \"\",\n".as_bytes();

					println!("{:?}", buf.iter().map(|u8a| *u8a as char).collect::<Vec<char>>());

					file_background_image_null_vec.write(buf).unwrap();
				} else {
					file_background_image_null_vec.write(line_bytes).unwrap();
				}
			});
		}


		let nuller_buf = file_background_image_null_vec.as_slice();
		let backup_buf = file_backup_vec.as_slice();

		{
			let settings_file_write = File::create(&path).unwrap();
			let mut file_writer = BufWriter::new(settings_file_write);
			file_writer.write_all(nuller_buf).unwrap();
			file_writer.flush().unwrap();
		}

		thread::sleep(Duration::from_millis(200));

		{
			let settings_file_write = File::create(&path).unwrap();
			let mut file_writer = BufWriter::new(settings_file_write);
			file_writer.write_all(backup_buf).unwrap();
			file_writer.flush().unwrap();

			let file_test = File::create(&path_test).unwrap();
			let mut test_file_writer = BufWriter::new(file_test);

			test_file_writer.write_all(backup_buf).unwrap();
			test_file_writer.flush().unwrap();
		}
	}
	#[test]
	fn test5() {
		let test_file = PathBuf::from_str(r"C:\Users\[user_name]\AppData\Local\Packages\Microsoft.WindowsTerminal_8wekyb3d8bbwe\LocalState\settings_test.json").unwrap();
		let null_path = PathBuf::from_str(r"C:\Users\[user_name]\AppData\Local\Packages\Microsoft.WindowsTerminal_8wekyb3d8bbwe\LocalState\settings_null.json").unwrap();

		let org_file = PathBuf::from_str(r"C:\Users\[user_name]\AppData\Local\Packages\Microsoft.WindowsTerminal_8wekyb3d8bbwe\LocalState\settings.json").unwrap();

		let file = File::open(&org_file).unwrap();
		let reader = BufReader::new(file);

		let mut test_buf: Vec<u8> = Vec::new();
		let mut nuller: Vec<u8> = Vec::new();

		reader.lines().for_each(|line| {
			let line = {
				let line = line.unwrap_or(String::new());
				format!("{}\n", line)
			};

			if (&line).contains("\"backgroundImage\": ") {
				test_buf.write_fmt(format_args!("\t\t\t\"backgroundImage\": \"{}\",\n", "HelloWorld.jpg")).unwrap();
				nuller.write(b"\t\t\t\"backgroundImage\": \"\",\n").unwrap();
			} else {
				test_buf.write(line.as_bytes()).unwrap();
				nuller.write(line.as_bytes()).unwrap();
			}
		});

		{
			let settings_test_file_write = File::create(&test_file).unwrap();
			let mut file_writer = BufWriter::new(settings_test_file_write);
			file_writer.write_all(&test_buf).unwrap();
			file_writer.flush().unwrap();
		}

		{
			let settings_test_file_write = File::create(&null_path).unwrap();
			let mut file_writer = BufWriter::new(settings_test_file_write);
			file_writer.write_all(&nuller).unwrap();
			file_writer.flush().unwrap();
		}
	}
}
