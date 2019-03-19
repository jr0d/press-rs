extern crate serde;
pub mod ext;

use serde::Deserialize;

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

#[derive(Debug, Deserialize)]
pub struct FileSystem {
    /// The file system type (ext4, ntfs, etc)
    pub fs_type: String,
    /// The path to the file system command (mke2fs, mkntfs, etc)
    pub command_path: String,
}
