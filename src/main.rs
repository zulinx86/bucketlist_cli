use clap::Parser;
use std::error::Error;

mod app;

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

    /// Same as `add`.
    Incr {
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

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let args = Args::parse();
    log::info!("args: {:?}", args);

    let mut items = app::read_file()?;

    match args.action {
        Action::Add { name } => {
            log::info!("Add action");
            items = app::add_or_incr(items, name)?;
        },
        Action::Incr { name } => {
            log::info!("Incr action");
            items = app::add_or_incr(items, name)?;
        },
        Action::Note { name, note} => {
            log::info!("Note action");
            items = app::note(items, name, note)?;
        }
        Action::Del { name } => {
            log::info!("Del action");
            items = app::del(items, name)?;
        },
        Action::Ls { all } => {
            log::info!("Ls action");
            items = app::ls(items, all)?;
        }
    }

    log::debug!("{:#?}", items);
    app::save_file(items)?;

    Ok(())
}
