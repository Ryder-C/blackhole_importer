mod app;
mod config;

use anyhow::{bail, Result};
use clap::Parser;
use config::Config;
use magnet_url::Magnet;

const APP_NAME: &str = "blackhole_importer";
const CONFIG_NAME: &str = "config";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The magnet link
    #[arg(short, long)]
    magnet_link: String,

    /// Output file name
    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let cfg: Config = confy::load(APP_NAME, CONFIG_NAME).unwrap();

    if cfg.instance.is_empty() {
        bail!("Empty config found, edit your config file at {}", confy::get_configuration_file_path(APP_NAME, CONFIG_NAME)?.to_str().unwrap());
    }

    let mut terminal = ratatui::init();
    terminal.clear()?;

    let Ok(magnet) = Magnet::new(&args.magnet_link) else {
        bail!("Invalid magnet link");
    };

    if args.output.is_none() && magnet.dn.is_none() {
        bail!("No output file name provided and no name found in magnet link. Please use --output to specify a file name");
    }

    let app = app::App::new(cfg, magnet, args.output);
    app.run(terminal)?;

    ratatui::restore();
    Ok(())
}
