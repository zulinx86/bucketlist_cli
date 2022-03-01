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

type Result<T> = std::result::Result<T, BucketListError>;

#[derive(Debug, Error)]
pub enum BucketListError {
    #[error("No such an item (name: `{0}`).")]
    NotFound(String),
    #[error("Failed to get home directory.")]
    HomeDir,
    #[error("I/O related error happened.")]
    Io(#[from] std::io::Error),
    #[error("Time related error happened.")]
    Time(#[from] std::time::SystemTimeError),
    #[error("Serde related error happened.")]
    Serde(#[from] serde_json::Error),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    prio: f32,
    last: u64,
    active: bool,
    note: String,
}

fn get_bucketlist_dir() -> Result<String> {
    Ok(
        format!(
            "{}/{}",
            dirs::home_dir().ok_or(BucketListError::HomeDir)?
                .to_str().ok_or(BucketListError::HomeDir)?,
            CONFIG_DIRNAME
        )
    )
}

pub fn read_file() -> Result<IndexMap<String, Info>> {
    let dir = get_bucketlist_dir()?;
    std::fs::create_dir_all(&dir)?;

    match File::open(format!("{}/{}", dir, DATA_FILENAME)) {
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::NotFound => Ok(IndexMap::new()),
                _ => Err(BucketListError::from(e)),
            }
        },
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

pub fn save_file(items: IndexMap<String, Info>) -> Result<()> {
    let dir = get_bucketlist_dir()?;

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(format!("{}/{}", dir, DATA_FILENAME))?;

    serde_json::to_writer(&file, &items)?;

    Ok(())
}

pub fn add_or_incr(mut items: IndexMap<String, Info>, name: String) -> Result<IndexMap<String, Info>> {
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

pub fn note(mut items: IndexMap<String, Info>, name: String, note: String) -> Result<IndexMap<String, Info>> {
    match items.get_mut(&name) {
        None => Err(BucketListError::NotFound(name)),
        Some(info) => {
            info.note = note;
            println!("The note of `{}` is upated.", name);
            Ok(items)
        },
    }
}

pub fn del(mut items: IndexMap<String, Info>, name: String) -> Result<IndexMap<String, Info>> {
    match items.remove(&name) {
        None => Err(BucketListError::NotFound(name)),
        Some(info) => {
            println!("`{}` is deleted.", name);
            log::info!("{:#?}", info);
            Ok(items)
        },
    }
}

pub fn ls(mut items: IndexMap<String, Info>, all: bool) -> Result<IndexMap<String, Info>> {
    items.sort_by(
        |_, v1, _, v2| v1.prio.partial_cmp(&v2.prio).unwrap().reverse()
        // TODO: change partial_cmp() to total_cmp() when total_cmp() become stable in order to remove unwrap().
        // https://github.com/rust-lang/rust/issues/72599
    );

    for (k, v) in &items {
        if !all && v.prio <= ACTIVE_THRESHOLD {
            break
        }
        println!("{}: {:#?}", k, v);
    }

    Ok(items)
}