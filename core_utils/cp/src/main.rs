use std::{cmp::Ordering, env, fs, process::exit, path::Path};
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
    let (src, dest): (String, String) = match args.len().cmp(&3) {
        Ordering::Less => {
            println!("Not enough arguments");
            exit(1);
        }
        Ordering::Greater => {
            println!("Too much arguments");
            exit(1);
        }
        Ordering::Equal => {
            let arg1 = &args[1];
            let arg2 = &args[2];
            (arg1.to_string(), arg2.to_string())
        }
    };
    let copy_result = fs::copy(&src, &dest);
    let _ = match copy_result {
        Ok(val) => val,
        Err(_) => match fs::read_dir(&src) {
            Ok(_) => {
                let res = copy_dir(src, dest);
                let _ = match res {
                    Ok(val) => val,
                    Err(err) => {
                        panic!("{}", err);
                    }
                };
                exit(0);
            }
            Err(_) => {
                println!("File doesn't exist");
                exit(1);
            }
        },
    };
}
