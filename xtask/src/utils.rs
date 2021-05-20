use std::{
    env::current_dir,
    io::{
        Error,
        ErrorKind,
        Result,
    },
    path::PathBuf,
};

pub fn try_find_path_to_terrikon_hak() -> Result<PathBuf> {
    current_dir()?.ancestors().find(|path| path.ends_with("Terrikon/hak")).map_or_else(
        || {
            Err(Error::new(
                ErrorKind::NotFound,
                "Expected Terrikon/hak path before as xtask\n
                    help: `Try https://github.com/Terrikonn/Terrikon.git \
                 --recursive`",
            ))
        },
        |path| Ok(PathBuf::from(path)),
    )
}
