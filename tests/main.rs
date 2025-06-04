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
