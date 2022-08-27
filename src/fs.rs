use eyre::Context as _;
use serde::{de::DeserializeOwned, Serialize};
use std::path::{Path, PathBuf};

pub(crate) fn read_to_string(path: impl AsRef<Path>) -> eyre::Result<String> {
    let path = path.as_ref();
    std::fs::read_to_string(path).with_context(|| format!("could not read `{}`", path.display()))
}

pub(crate) fn read_json<T: DeserializeOwned, P: AsRef<Path>>(path: P) -> eyre::Result<T> {
    let path = path.as_ref();
    serde_json::from_str(&read_to_string(path)?)
        .with_context(|| format!("could not parse the JSON file at `{}`", path.display()))
}

pub(crate) fn read_yaml<T: DeserializeOwned, P: AsRef<Path>>(path: P) -> eyre::Result<T> {
    let path = path.as_ref();
    serde_yaml::from_str(&read_to_string(path)?)
        .with_context(|| format!("could not parse the YAML file at `{}`", path.display()))
}

pub(crate) fn write(path: impl AsRef<Path>, content: impl AsRef<[u8]>) -> eyre::Result<()> {
    let path = path.as_ref();
    std::fs::write(path, content).with_context(|| format!("could not write `{}`", path.display()))
}

pub(crate) fn write_json(path: impl AsRef<Path>, content: impl Serialize) -> eyre::Result<()> {
    write(path, serde_json::to_string(&content)?)
}

pub(crate) fn create_dir_all(path: impl AsRef<Path>) -> eyre::Result<()> {
    let path = path.as_ref();
    std::fs::create_dir_all(path).with_context(|| format!("could not create `{}`", path.display()))
}

pub(crate) fn copy(from: impl AsRef<Path>, to: impl AsRef<Path>) -> eyre::Result<u64> {
    let (from, to) = (from.as_ref(), to.as_ref());
    std::fs::copy(from, to)
        .with_context(|| format!("failed to copy `{}` to `{}`", from.display(), to.display()))
}

pub(crate) fn read_dir(path: impl AsRef<Path>) -> eyre::Result<Vec<PathBuf>> {
    let path = path.as_ref();
    std::fs::read_dir(path)?
        .map(|e| e.map(|e| e.path()))
        .collect::<Result<_, _>>()
        .with_context(|| format!("could not list files in `{}`", path.display()))
}
