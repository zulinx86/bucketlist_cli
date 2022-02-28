use thiserror::Error;
use indexmap::IndexMap;
use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::time::{SystemTime, UNIX_EPOCH};

const CONFIG_DIRNAME: &str = ".bucketlist";
const DATA_FILENAME: &str = "data.json";
const DECAY: f32 = 0.925;
const SEC_OF_DECAY: u64 = 86400;
const ACTIVE_THRESHOLD: f32 = 0.1;

#[derive(Debug, Error)]
pub enum BucketListError {
    #[error("No such an item (name: `{0}`).")]
    NotFoundError(String),
    #[error("Failed to convert home directory to str.")]
    HomeDirError,
    #[error("I/O related error happened.")]
    IoError(#[from] std::io::Error),
    #[error("Time related error happened.")]
    TimeError(#[from] std::time::SystemTimeError),
    #[error("Serde related error happened.")]
    SeredeError(#[from] serde_json::Error),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    prio: f32,
    last: u64,
    active: bool,
    note: String,
}

pub fn read_file() -> Result<IndexMap<String, Info>, BucketListError> {
    let dir = match dirs::home_dir() {
        None => format!("./{}", CONFIG_DIRNAME),
        Some(home) => format!(
            "{}/{}",
            home.to_str().ok_or(BucketListError::HomeDirError)?,
            CONFIG_DIRNAME
        ),
    };
    std::fs::create_dir_all(&dir)?;

    match File::open(format!("{}/{}", dir, DATA_FILENAME)) {
        Err(_) => Ok(IndexMap::new()),
        Ok(file) => {
            let reader = BufReader::new(file);

            let mut items: IndexMap<String, Info> = serde_json::from_reader(reader)?;

            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)?
                .as_secs();

            for (_, info) in items.iter_mut() {
                let days = (now - info.last) / SEC_OF_DECAY;
                info.prio *= DECAY.powi(days as i32);
                info.active = info.prio >= ACTIVE_THRESHOLD;
            }

            Ok(items)
        },
    }
}

pub fn save_file(items: IndexMap<String, Info>) -> Result<(), BucketListError> {
    let dir = match dirs::home_dir() {
        None => format!("./{}", CONFIG_DIRNAME),
        Some(home) => format!(
            "{}/{}",
            home.to_str().ok_or(BucketListError::HomeDirError)?,
            CONFIG_DIRNAME
        ),
    };

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(format!("{}/{}", dir, DATA_FILENAME))?;

    serde_json::to_writer(&file, &items)?;

    Ok(())
}

pub fn add_or_incr(mut items: IndexMap<String, Info>, name: String) -> Result<IndexMap<String, Info>, BucketListError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs();

    match items.get_mut(&name) {
        Some(info) => {
            info.prio += 1.0;
            info.last = now;
            info.active = true;
            println!("The priority of `{}` gets higher.", name);
        },
        None => {
            items.insert(name.clone(), Info {
                prio: 1.0,
                last: now,
                active: true,
                note: String::from(""),
            });
            println!("A new item `{}` is added.", name);
        }
    }
    Ok(items)
}

pub fn note(mut items: IndexMap<String, Info>, name: String, note: String) -> Result<IndexMap<String, Info>, BucketListError> {
    match items.get_mut(&name) {
        None => Err(BucketListError::NotFoundError(name)),
        Some(info) => {
            info.note = note;
            println!("The note of `{}` is upated.", name);
            Ok(items)
        },
    }
}

pub fn del(mut items: IndexMap<String, Info>, name: String) -> Result<IndexMap<String, Info>, BucketListError> {
    match items.remove(&name) {
        None => Err(BucketListError::NotFoundError(name)),
        Some(info) => {
            println!("`{}` is deleted.", name);
            log::info!("{:#?}", info);
            Ok(items)
        },
    }
}

pub fn ls(mut items: IndexMap<String, Info>, all: bool) -> Result<IndexMap<String, Info>, BucketListError> {
    items.sort_by(
        |_, v1, _, v2| 
            v1.prio.partial_cmp(&v2.prio).unwrap().reverse()
        // TODO: change partial_cmp() to total_cmp() when total_cmp() become stable in order to remove unwrap().
        // https://github.com/rust-lang/rust/issues/72599
    );

    for (k, v) in &items {
        if all == false && v.prio <= ACTIVE_THRESHOLD {
            break
        }
        println!("{}: {:#?}", k, v);
    }

    Ok(items)
}