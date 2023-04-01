use std::{cmp::Ordering, env, fs, path::Path, process::exit};


fn copy_dir(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    let _ = fs::create_dir_all(&dst);
    let read_dir = fs::read_dir(&src).unwrap();
    for entry in read_dir {
        let entry = entry?;
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            copy_dir(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let (is_dir, src, dest): (bool, String, String) = match args.len().cmp(&3) {
        Ordering::Less => {
            println!("Not enough arguments");
            exit(1);
        }
        Ordering::Greater => {
            if (&args[1] == "-R" || &args[1] == "-r") && args.len() == 4 {
                let src_arg = &args[2];
                let dest_arg = &args[3];
                (true, src_arg.to_string(), dest_arg.to_string())
            } else {
                println!("Too much arguments");
                exit(1);
            }
        }
        Ordering::Equal => {
            let src_arg = &args[1];
            let dest_arg = &args[2];
            (false, src_arg.to_string(), dest_arg.to_string())
        }
    };
    if is_dir {
        let _ = match fs::read_dir(&src) {
            Ok(_) => {
                let _ = match copy_dir(&src, &dest) {
                    Ok(val) => val,
                    Err(err) => {
                        panic!("{}", err);
                    }
                };
            }
            Err(_) => {
                println!("{}: directory not found", src);
            }
        };
    } else {
        let copy_result = fs::copy(&src, &dest);
        let _ = match copy_result {
            Ok(val) => val,
            Err(err) => {
                let errcode = err.raw_os_error();
                let code: i32 = match errcode {
                    None => panic!("{}", err),
                    Some(code) => code,
                };
                if code == 21 {
                    let src_name = Path::new(&src).file_name().unwrap().to_str().unwrap();
                    match fs::copy(&src, format!("{}/{}", &dest, &src_name)) {
                        Ok(val) => val,
                        Err(e) => panic!("{}", e),
                    }
                } else {
                    panic!("{}", err);
                }
            }
        };
    }
}
