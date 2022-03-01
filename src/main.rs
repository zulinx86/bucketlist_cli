use clap::Parser;
use main_error::MainError;

mod app;

#[derive(Parser, Debug)]
#[clap(
    version,
    about = "Bucket List CLI",
    long_about = "Provides an bucket list prioritizing items automatically."
)]
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

fn main() -> Result<(), MainError> {
    env_logger::init();

    let args = Args::parse();
    log::info!("args: {:?}", args);

    let items = app::read_file()?;

    let items = match args.action {
        Action::Add { name } => app::add_or_incr(items, name),
        Action::Incr { name } => app::add_or_incr(items, name),
        Action::Note { name, note } => app::note(items, name, note),
        Action::Del { name } => app::del(items, name),
        Action::Ls { all } => app::ls(items, all),
    }?;

    log::debug!("{:#?}", items);
    app::save_file(items)?;

    Ok(())
}
