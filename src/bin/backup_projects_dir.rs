use clap::Parser;
use mb_dev::shell;

#[derive(Parser, Debug)]
struct Args {
    src: String,
    dest: String,
}

const EXCLUDE_DIRS: &[&str] = &[".venv", "target", "node_modules", ".mypy_cache", ".pytest_cache"];

fn main() {
    let args = Args::parse();
    let exclude = EXCLUDE_DIRS.iter().map(|d| format!("--exlude={}/", d)).collect::<Vec<String>>().join(" ");
    shell(&format!("rsync -azvhP {} {} {}", exclude, args.src, args.dest));
}
