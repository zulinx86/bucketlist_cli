use clap::Parser;
use std::error::Error;
use std::fs::{OpenOptions};
use std::io::BufReader;
use indexmap::IndexMap;
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

const CONFIG_DIRNAME: &str = ".bucketlist";
const DATA_FILENAME: &str = "data.json";
const DECAY: f32 = 0.925;
const SEC_OF_DECAY: u64 = 86400;
const ACTIVE_THRESHOLD: f32 = 0.1;

#[derive(Parser, Debug)]
#[clap(
    version,
    about = "Bucket List CLI", 
    long_about = "Provides an bucket list prioritizing items automatically.")]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand, Debug)]
enum Action {
    /// Add a new item or raise priority of an existing item.
    Add {
         /// Name of the item
        name: String,
    },

    /// Add a note to an existing item.
    Note {
        /// Name of the item
        name: String,

        /// Note to be added
        note: String,
    },

    /// Delete an item.
    Del {
        /// Name of the item
        name: String,
    },

    /// List items
    Ls {
        /// Shows also inactive items.
        #[clap(long, short)]
        all: bool,
    },
}

#[derive(Serialize, Deserialize, Debug)]
struct Info {
    prio: f32,
    last: u64,
    active: bool,
    note: String,
}

fn read_file() -> Result<IndexMap<String, Info>, Box<dyn Error>> {
    // create "~/.bucketlist" directory if not exist.
    let dir = match dirs::home_dir() {
        None => format!("./{}", CONFIG_DIRNAME),
        Some(home) => {
            let dir = format!("{}/{}", home.to_str().unwrap(), CONFIG_DIRNAME);
            std::fs::create_dir_all(&dir)?;
            dir
        },
    };

    // create "data.json" file if not exists.
    let file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(format!("{}/{}", dir, DATA_FILENAME))?;
    let reader = BufReader::new(file);

    match serde_json::from_reader(reader) {
        // when succeeds, returns deserialized items.
        Ok::<IndexMap<String, Info>, _>(mut items) => {
            let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
            for (_, info) in items.iter_mut() {
                let days = (now - info.last) / SEC_OF_DECAY;
                info.prio *= DECAY.powi(days as i32);
                info.active = info.prio >= ACTIVE_THRESHOLD;
            }

            Ok(items)
        },

        // when fails, returns empty HashMap.
        Err(_) => Ok(IndexMap::new())
    }
}

fn save_file(items: IndexMap<String, Info>) -> Result<(), Box<dyn Error>> {
    let dir = match dirs::home_dir() {
        None => format!("./{}", CONFIG_DIRNAME),
        Some(home) => format!("{}/{}", home.to_str().unwrap(), CONFIG_DIRNAME),
    };

    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(format!("{}/{}", dir, DATA_FILENAME))?;

    serde_json::to_writer(&file, &items)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let args = Args::parse();
    log::info!("args: {:?}", args);

    let mut items = read_file()?;

    match args.action {
        Action::Add { name } => {
            log::info!("Add action");

            let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

            match items.get_mut(&name) {
                Some(info) => {
                    info.prio += 1.0;
                    info.last = now;
                    info.active = true;
                    println!("The priority of `{}` is raised.", name);
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
            
            log::debug!("{:#?}", items);
            save_file(items)?;
        },
        Action::Note { name, note} => {
            log::info!("Note action");

            match items.get_mut(&name) {
                Some(info) => {
                    info.note = note;
                    println!("The note of `{}` is upated.", name);
                },
                None => println!("No such an item (name: `{}`)", name)
            }

            log::debug!("{:#?}", items);
            save_file(items)?;
        }
        Action::Del { name } => {
            log::info!("Del action");

            match items.remove(&name) {
                Some(info) => {
                    println!("`{}` is deleted.", name);
                    log::info!("{:#?}", info);
                },
                None => println!("No such an item (name: `{}`)", name),
            };

            log::debug!("{:#?}", items);
            save_file(items)?;
        },
        Action::Ls { all } => {
            log::info!("Ls action");

            items.sort_by(
                |_, v1, _, v2| v1.prio.partial_cmp(&v2.prio).unwrap().reverse()
            );

            for (k, v) in &items {
                if all == false && v.prio <= ACTIVE_THRESHOLD {
                    break
                }
                println!("{}: {:#?}", k, v);
            }

            log::debug!("{:#?}", items);
            save_file(items)?;
        }
    }

    Ok(())
}
