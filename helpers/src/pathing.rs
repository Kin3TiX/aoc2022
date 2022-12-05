use std::fs;
use std::env::current_exe;
use std::path::PathBuf;
use relative_path::RelativePath;


pub fn get_root_directory() -> PathBuf {
    let mut executable_path = current_exe().expect(
        "Failed to detect application directory!"
    );
    executable_path.pop();
    return executable_path;
}

pub fn get_input_data(file_name: &str) -> String {
    let root_directory = get_root_directory();
    let input_file: PathBuf = RelativePath::new(file_name)
        .to_path(&root_directory);

    println!("{}", input_file.display());
    return fs::read_to_string(input_file).expect(
        "Filed to read input file!"
    );
}
