mod repl;

use repl::start;

fn main() {
    start(std::io::stdin(), std::io::stdout())
}
