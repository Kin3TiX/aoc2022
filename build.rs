use fs_extra::copy_items;
use fs_extra::dir::CopyOptions;


fn main() {

    println!("cargo:rerun-if-changed=resources/*");

    let mut options = CopyOptions::new();
    let mut from_path = Vec::new();
    let out_path = "../target/debug";
    options.overwrite = true;
    from_path.push("resources");
    copy_items(&from_path, &out_path, &options)
        .expect(&format!("Could not copy {:?} to {:?}", from_path, out_path));

}
