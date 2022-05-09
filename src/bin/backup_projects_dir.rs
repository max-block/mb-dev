use clap::Parser;
use mb_dev::shell;

#[derive(Parser, Debug)]
struct Args {
    src: String,
    dest: String,
}

fn main() {
    let args = Args::parse();
    shell(&format!("rsync -azvhP --exclude=.venv/ --exclude=target/ --exclude=node_modules/ {} {}", args.src, args.dest));
}
