pub mod ext;

pub use ext::EXT;

// from the playground .. Maybe something like this?
// struct FileSystem {
//     name: &'static str
// }

// static EXT4: FileSystem = FileSystem { name: "EXT4" };

// fn main() {
//     println!("Hello, world!");
//     println!("{}", &EXT4.name );
// }

// Or maybe just a file system enum
// and implment a method call `command` which will match the
// file system name and output the command
// of coarse there are command line options that belong in struct as well
// gonna have to thing on this more
enum FileSystem {
    ext4(EXT);
}