mod config_file;
mod host_guard;

use actix_files::Files;
use actix_web::{middleware::Logger, App, HttpServer};
use config_file::ServePath;
use std::net::SocketAddr;
use std::str::FromStr;
use std::{env, process::exit};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = match read_config() {
        Some(c) => c,
        None => exit(1),
    };

    let sock = SocketAddr::from_str(&config.bind)
        .expect(format!("invalid bind address `{}`", &config.bind).as_str());

    let num_routes = config.serve.len();
    if num_routes == 0 {
        eprintln!("no serve routes specified in config file");
        exit(1);
    }

    let mut server = HttpServer::new(move || {
        let mut a = App::new();
        for route in &config.serve {
            a = a.service(make_files_route(route));
        }
        a.wrap(Logger::default())
    });

    if let Some(workers) = config.workers {
        server = server.workers(workers);
    }

    log::info!("starting HTTP server at http://{}", sock);
    log::info!("serving for {} routes", num_routes);
    server.bind(sock)?.run().await
}

/// Reads the configuration file as provided from SERVEBOX_CONFIG_FILE environment
/// variable. If the environment variable is not set, defaults to "config.toml"
fn read_config() -> Option<config_file::ConfigFile> {
    let config_file_path = env::var("SERVEBOX_CONFIG_FILE").unwrap_or(String::from("config.toml"));
    config_file::load_config_file(config_file_path)
}

/// Creates a Files service handler from a ServePath
fn make_files_route(route: &ServePath) -> Files {
    let mut service = Files::new(&route.web_path, &route.file_path);
    if let Some(index_path) = &route.index_file {
        service = service.index_file(index_path);
    }
    if let Some(show) = route.show_index {
        if show {
            service = service.show_files_listing();
        }
    }
    if let Some(host) = &route.host {
        service = service.guard(host_guard::new(host))
    }
    service
}
