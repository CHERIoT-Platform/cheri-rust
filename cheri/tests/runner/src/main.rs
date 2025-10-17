use clap::Parser;

mod runner;

fn main() -> anyhow::Result<()> {
    runner::App::parse().run()
}
