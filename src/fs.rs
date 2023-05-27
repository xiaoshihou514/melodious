use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::path::Path;
use std::sync::{Arc, Mutex};

/// The data is stored in tree files ./tree.yaml, ./index.yaml
/// Which stores fs info and alphabetical order with time stamps, respectively
/// tree.yaml               index.yaml
/// Music                   music1      Thu 23 Mar 2023 17:37:54 CST
///     dir1                music2      Thu 23 Mar 2023 17:37:54 CST
///         music1          music3      Thu 23 Mar 2023 17:37:54 CST
///     dir2
///         music2
///         music3
/// Where created time is sorted using last date modified
///
/// In memory, they are stored with the Tree and Index type
/// Tree:                           Index
/// "/Music" ["/dir1", "/dir2"]     ("music1", "Thu 23 Mar 2023 17:37:54 CST")
/// "/dir1" ["music1"]              ("music2", "Thu 23 Mar 2023 17:37:54 CST")
/// "/dir2" ["music2", "music3"]    ("music3", "Thu 23 Mar 2023 17:37:54 CST")

type Tree = HashMap<String, Vec<String>>;
type AsyncTree = Arc<Mutex<Tree>>;
type Index = Vec<(String, String)>;
type AsyncIndex = Arc<Mutex<Index>>;
type AsyncFile = Arc<Mutex<File>>;

const CACHE_ROOT: &str = "/home/xiaoshihou/.local/share/melodious";
const TREE_PATH: &str = "/home/xiaoshihou/.local/share/melodious/tree.yaml";
const INDEX_PATH: &str = "/home/xiaoshihou/.local/share/melodious/index.yaml";

// helpers
fn get_name(p: &Path) -> String {
    String::from(p.components().last().unwrap().as_os_str().to_str().unwrap())
}

fn open_file(path: &str) -> AsyncFile {
    Arc::new(Mutex::new(
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .unwrap_or_else(|error| panic!("Cannot open yaml file: {}", error)),
    ))
}

// How this file exposes API
pub struct Library {
    tree: AsyncTree,
    index: AsyncIndex,
}

impl Library {
    pub fn new() -> Self {
        Library {
            tree: Arc::new(Mutex::new(HashMap::new())),
            index: Arc::new(Mutex::new(Index::new())),
        }
    }

    // refresh or scan the music dir
    fn init(&mut self) {
        let cache_root = Path::new(CACHE_ROOT);
        let has_conf = cache_root.is_dir();
        if !has_conf {
            let result = fs::create_dir(cache_root);
            if let Err(e) = result {
                panic!("cannot create data dir! {}", e)
            }
        }
        let has_conf = has_conf && Path::new(TREE_PATH).is_file();
        let tree_yaml = open_file(TREE_PATH);
        let index_yaml = open_file(INDEX_PATH);
        // assume all files either all exist or not for simplicity
        let tree: AsyncTree = Arc::new(Mutex::new(HashMap::new()));
        let index = Arc::new(Mutex::new(Index::new()));
        if !has_conf {
            // scan from scratch
            let root = fs::read_dir("/home/xiaoshihou/Music/").unwrap_or_else(|error| {
                panic!("Cannot open the music dir: {}", error);
            });
        } else {
            // refresh
            // TODO: maybe refresh should happen in the background, after app got created
        }
    }

    // returns a tree view of music folders
    pub fn get_formatted_dirs(&self) -> Vec<String> {
        todo!()
    }
}
