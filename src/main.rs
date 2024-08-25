#![allow(deprecated)]
use std::{env, ffi::{OsStr, OsString}, fs::{self, File}, io::{self, BufRead, BufReader, BufWriter, Write}, path::PathBuf, thread, time::{Duration, SystemTime}};

use uuid::Uuid;
use rand::{self, Rng};


#[derive(Debug)]
enum AppError {
    NotEnoughArgs,
    ImageSourceNotExist,
    ImageSourceIsNotDirectory,
    ImageSourceEmpty,
    SettingsJsonNotExist,
    IoError(io::Error),
}


fn parse_args() -> std::result::Result<Vec<PathBuf>, AppError> {
    let args = env::args_os().collect::<Vec<OsString>>();
    if args.len() - 1 < 2 {
        return Err(AppError::NotEnoughArgs);
    }

    let background_image_source_dir = PathBuf::from(&args[1]);
    if !background_image_source_dir.exists() {
        return Err(AppError::ImageSourceNotExist);
    }
    if !background_image_source_dir.is_dir() {
        return Err(AppError::ImageSourceIsNotDirectory);
    }
    

    let settings_json_path = PathBuf::from(&args[2]);
    if !settings_json_path.exists() {
        return Err(AppError::SettingsJsonNotExist);
    }
    
    Ok(vec![background_image_source_dir, settings_json_path])
}


fn choose_random_image_path(background_image_source_dir: &PathBuf) -> std::result::Result<PathBuf, AppError> {
    let mut file_entry = Vec::new();

    match fs::read_dir(background_image_source_dir) {
        Ok(dir_entry) => dir_entry.for_each(|file| {
            let file = file.unwrap();

            if !file.metadata().unwrap().is_file() {
                return;
            }

            file_entry.push(file.path());
        }),
        Err(err) => return Err(AppError::IoError(err)),
    };

    let file_entry_len = file_entry.len();
    if file_entry_len == 0 {
        return Err(AppError::ImageSourceEmpty);
    }

    let random_number = rand::thread_rng().gen_range(0..file_entry_len);
    Ok((&file_entry[random_number]).to_owned())
}


fn generate_unique_image_name(image_extension: &OsStr, image_source_directory: &PathBuf) -> PathBuf {
    let uuid = Uuid::new_v4();

    image_source_directory.join(
        format!("{}_image.{}", uuid, image_extension.to_str().unwrap())
    )
}

fn generate_settings_json_data(windows_terminal_settings_json_path: &PathBuf, random_unique_image_path: &PathBuf) -> std::result::Result<(Vec<u8>, Vec<u8>), AppError> {
    let settings_json_file_reader = match File::open(windows_terminal_settings_json_path) {
        Ok(file) => BufReader::new(file),
        Err(err) => return Err(AppError::IoError(err)),
    };
    
    let mut settings_file_changed_vec = Vec::new();
    let mut settings_file_background_image_null_vec = Vec::new();


    for line in settings_json_file_reader.lines() {
        let line = line.unwrap_or(String::new());

        if (&line).contains("\"backgroundImage\": ") {
            let buf = "\"backgroundImage\": \"\",\n".as_bytes();
            let random_unique_image_name = random_unique_image_path.to_str().unwrap_or("").replace("\\", "\\\\");
            
            if let Err(err) = settings_file_changed_vec.write_fmt(
                format_args!("\t\t\t\"backgroundImage\": \"{}\",\n", random_unique_image_name)
            ) {
                return Err(AppError::IoError(err));
            }
            if let Err(err) = settings_file_background_image_null_vec.write(buf) {
                return Err(AppError::IoError(err));
            }
        } else {
            let line = format!("{}\n", line);
            let line_bytes = line.as_bytes();

            if let Err(err) = settings_file_changed_vec.write(line_bytes) {
                return Err(AppError::IoError(err));
            }
            if let Err(err) = settings_file_background_image_null_vec.write(line_bytes) {
                return Err(AppError::IoError(err));
            }
        }
    }

    Ok((settings_file_background_image_null_vec, settings_file_changed_vec))
}

fn write_to_settings_json(windows_terminal_settings_json_path: &PathBuf, data: &Vec<u8>) -> std::result::Result<(), AppError> {
    let mut settings_json_file_writer = match File::create(windows_terminal_settings_json_path) {
        Ok(file) => BufWriter::new(file),
        Err(err) => return Err(AppError::IoError(err)),
    };

    let data_slice = data.as_slice();

    if let Err(err) = settings_json_file_writer.write_all(data_slice) {
        return Err(AppError::IoError(err));
    }

    Ok(())
}


// TODO: 選択の重複をなくす
fn main() -> std::result::Result<(), AppError> {

	let started_time = SystemTime::now();


	println!("rikiaaan-terminal-background-image-changer v1.3.1");


    let app_args = parse_args()?;

    let image_target_path = choose_random_image_path(&app_args[0])?;
    let image_extension = image_target_path.extension().unwrap_or(OsStr::new(""));
    let random_unique_image_path = generate_unique_image_name(image_extension, &app_args[0]);

    if let Err(err) = fs::rename(&image_target_path, &random_unique_image_path) {
        return Err(AppError::IoError(err));
    }


    let (settings_file_background_image_null_vec, settings_file_changed_vec) = generate_settings_json_data(&app_args[1], &random_unique_image_path)?;

    write_to_settings_json(&app_args[1], &settings_file_changed_vec)?;

    println!("settings.json backup writing delay started");
	thread::sleep(Duration::from_millis(200));
	println!("settings.json backup writing delay finished");

    write_to_settings_json(&app_args[1], &settings_file_changed_vec)?;


	let elapsed = SystemTime::now().duration_since(started_time).unwrap();
	println!("finished: {}ms", elapsed.as_millis());

    Ok(())
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
