use clap::Parser;
use dioxus_cli::{*, plugin::{PluginManager, PluginConfig}};
use std::process::exit;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    set_up_logging();

    let plugin_state = PluginManager::init(&PluginConfig {
        available: true,
        required: vec![],
    });
    
    if !plugin_state {
        log::error!("🚫 Plugin system initialization failed");
        exit(1);
    }

    match args.action {
        Commands::Translate(opts) => {
            if let Err(e) = opts.translate() {
                log::error!("🚫 Translate failed: {}", e);
                exit(1);
            }
        }

        Commands::Build(opts) => {
            if let Err(e) = opts.build() {
                log::error!("🚫 Build project failed: {}", e);
                exit(1);
            }
        }

        Commands::Clean(opts) => {
            if let Err(e) = opts.clean() {
                log::error!("🚫 Clean project failed: {}", e);
                exit(1);
            }
        }

        Commands::Serve(opts) => {
            if let Err(e) = opts.serve().await {
                log::error!("🚫 Serve startup failed: {}", e);
                exit(1);
            }
        }

        Commands::Create(opts) => {
            if let Err(e) = opts.create() {
                log::error!("🚫 Create project failed: {}", e);
                exit(1);
            }
        }

        Commands::Config(opts) => {
            if let Err(e) = opts.config() {
                log::error!("config error: {}", e);
                exit(1);
            }
        }

        Commands::Plugin(opts) => {
            if let Err(e) = opts.plugin().await {
                log::error!("tool error: {}", e);
            }
        }
    }

    Ok(())
}
