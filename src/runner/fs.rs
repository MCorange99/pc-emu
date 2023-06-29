// use std::{collections::HashMap, hash::Hash, path::PathBuf};
// use color_eyre::{Result, eyre::eyre};

// #[derive(Debug, Clone)]
// struct FsEntry {
//     files: Option<HashMap<String, FsEntry>>,
//     typ: FileType,
//     data: Vec<u8>
// }
// #[derive(Debug, Clone)]
// enum FileType {
//     Folder(Box<FsEntry>),
//     File
// }


// ///
// /// Folders end with /
// /// ! CURRENTLY BROKEN
// #[derive(Debug, Clone)]
// pub struct Fs {
//     files: HashMap<String, FsEntry>
// }

// impl Fs {
//     pub fn new() -> Self{
//         Self {
//             files: HashMap::new(),
//         }
//     }

//     // pub fn follow_path(&mut self, path: String) -> &mut FsEntry {
//     //     let mut path_parts: Vec<&str> = path.split("/").collect();
//     //     path_parts.reverse();
//     //     path_parts.pop();

//     //     let mut f_ptr = &self.files;

//     //     for p in path_parts {
//     //         if let Some(f) = f_ptr.get(&format!("{p}/")) {
//     //             f_ptr = if let Some(f) = &f.files {f} else {
//     //                 return Err(eyre!("Folder is a file"));
//     //             };
//     //         } else {
//     //             return Err(eyre!("Folder does not exist"));
//     //         }
//     //     };
//     // }

//     pub fn new_file(&mut self, path: String, name: String) -> Result<()> {
//         let mut path_parts: Vec<&str> = path.split("/").collect();
//         path_parts.reverse();
//         path_parts.pop();

//         let mut f_ptr = &self.files;

//         for p in path_parts {
//             if let Some(f) = f_ptr.get(&format!("{p}/")) {
//                 f_ptr = if let Some(f) = &f.files {f} else {
//                     return Err(eyre!("Folder is a file"));
//                 }
//             } else {
//                 return Err(eyre!("Folder does not exist"));
//             }
//         }

//         self.files.insert(name, FsEntry {
//             files: None,
//             typ: FileType::File,
//             data: Vec::new()
//         });
//         Ok(())
//     }
// }