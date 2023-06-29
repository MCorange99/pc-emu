

// struct HgPath(PathBuf);

// impl HgPath {
//     pub fn new() -> Self {
//         Self(PathBuf::new())
//     }

//     pub fn join<P: AsRef<Path>>(mut self, p: P) -> Self {
//         // let host_path = std::env::current_dir().unwrap();
//         // let p = p.as_ref();
//         // if p.is_absolute() {
//         //     let p = p.to_str().unwrap();
//         //     p = format!(".{p}").as_str();
            
//         // }
        
//         // self.join(p)
//         self.0 = self.0.join(p);
//         self
//     }


// }

// impl From<PathBuf> for HgPath {
//     fn from(value: PathBuf) -> Self {
//         HgPath(value)
//     }
// }

// impl From<String> for HgPath {
//     fn from(value: String) -> Self {
//         HgPath(PathBuf::from(value))
//     }
// }

// impl From<&str> for HgPath {
//     fn from(value: &str) -> Self {
//         HgPath(PathBuf::from(value))
//     }
// }

// impl From<&Path> for HgPath {
//     fn from(value: &Path) -> Self {
//         HgPath(PathBuf::from(value))
//     }
// }

use std::path::PathBuf;

pub trait HgPath{
    fn get_host_path(&mut self) -> PathBuf;
    fn simplify(&mut self) -> PathBuf;
}


impl HgPath for PathBuf {
    fn get_host_path(&mut self) -> PathBuf {
        let host_path = std::env::current_dir().unwrap().join("fs");
        if self.is_absolute() {
            let p = &self.to_str().unwrap()[1..];
            return host_path.join(p);
        } else {
            let p = PathBuf::from(&self.simplify().to_str().unwrap());

            return host_path.join(p);
        }
    }

    fn simplify(&mut self) -> PathBuf {
        let mut buf = PathBuf::new();
        for p in self.components() {
            let p = p.as_os_str().to_str().unwrap();
            if p == "." {
                // do nothin
            } else
            if p == ".." {
                buf.pop();
            } else {
                buf.push(p);
            }
        }
        buf
    }
}