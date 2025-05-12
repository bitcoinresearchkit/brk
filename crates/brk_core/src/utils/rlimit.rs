use std::io;

use rlimit::{Resource, getrlimit};

pub fn setrlimit() -> io::Result<()> {
    let no_file_limit = getrlimit(Resource::NOFILE)?;
    rlimit::setrlimit(
        Resource::NOFILE,
        no_file_limit.0.max(210_000),
        no_file_limit.1,
    )?;

    // let no_stack = getrlimit(Resource::STACK)?;
    // rlimit::setrlimit(Resource::STACK, no_stack.1, no_stack.1)?;

    Ok(())
}
