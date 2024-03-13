use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;

fn get_input(query: &str) -> std::io::Result<String> {
    println!("{}", query);
    std::io::stdout().flush()?;

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_owned())
}

fn organize_dir(dir_path: PathBuf) {
    if !dir_path.exists() {
        println!("Dir \"{}\" doest exist", dir_path.display());
    }else {
        let dir_files = match dir_path.read_dir() {
            Ok(dir_files) => dir_files,
            Err(err) => {
                println!("Error opening dir \"{}\": \"{}\"", dir_path.display(), err);
                return;
            }
        };
        for file in dir_files  {
            if let Ok(file) = file {
                if file.path().is_dir() {
                    println!("Path {} is dir, skip", file.path().display());
                    continue;
                }
                 let file_extension = match file.path().extension() {
                     None => {
                         println!("Cant get extension of the file: \"{}\"",file.path().display());
                         continue;
                     },
                     Some(extension) => match extension.to_str() {
                         None => continue,
                         Some(extension) =>extension.to_lowercase()
                     }
                 };
                let extension_dir = PathBuf::from(dir_path.join(file_extension));

                create_dir_if_not_exist(&extension_dir);
                move_file(&file.path(), &extension_dir.join(file.file_name()));
            }
        }
    }
}
fn create_dir_if_not_exist(dir_path: &PathBuf) {
    if !dir_path.exists() {
        if let Err(err) = fs::create_dir(dir_path) {
            println!("Error creating dir \"{}\": \"{}\"", dir_path.display(), err);
        }
    }
}
fn move_file(from: &PathBuf, to: &PathBuf) {
    if let Err(err) = fs::rename(from, to) {
        println!("Error moving file \"{}\" to  \"{}\": \"{}\"", from.display(), to.display(), err);
    }
}

fn main() {
    loop {
        let dir_path = match get_input("Enter the path to the dir you want organize: ") {
            Ok(dir_path) => dir_path,
            Err(err) => {
                println!("Error getting user input: {err}");
                continue;
            }
        };
        //time check
        let now = Instant::now();
        organize_dir(PathBuf::from(dir_path));
        println!("Tine to organize: {}s",now.elapsed().as_secs_f64());
    }
}
