#[macro_use]
extern crate libtest_mimic_collect;

use libtest_mimic::Arguments;
use libtest_mimic_collect::TestCollection;
use rusty_docker_compose::DockerComposeCmd;

mod utils;
use utils::{client, time};
mod services;

fn init_docker() -> DockerComposeCmd {
    DockerComposeCmd::new("tests/docker/docker-compose.yml", "tests/docker/logs")
}

mod doc {
    use paper_plane::clients::{Client as CT, reqwest::lite::Client};
    use paper_plane::{auth::Auth, services};

    // services for your application
    struct Services {
        // access to the "Documents" service
        paperless: Box<dyn services::Documents>,
    }

    fn get_prod_services() -> Services {
        let url = std::env::var("PAPERLESS_URL").unwrap();
        let tok = std::env::var("PAPERLESS_TOKEN").unwrap();
        let paperless_client = Client::new(url, Auth::Token(tok.into()));

        Services {
            paperless: Box::new(paperless_client),
        }
    }
}

pub fn main() {
    let docker_service = init_docker();

    println!(">>> starting docker...");
    time::measure("--- done", || {
        let _gag = gag::Gag::stdout();
        docker_service.up();
        time::wait(client::PAPERLESS_URL)
    });

    println!(">>> running tests");
    let args = Arguments::from_args();
    let result = libtest_mimic::run(&args, TestCollection::collect_tests());

    println!(">>> shutting down docker...");
    time::measure("--- done", || {
        let _gag = gag::Gag::stdout();
        docker_service.down();
    });

    result.exit()
}
