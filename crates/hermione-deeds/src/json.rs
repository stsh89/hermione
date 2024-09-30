use serde::{de::DeserializeOwned, Serialize};
use std::{
    fs::{File, OpenOptions},
    io::{BufReader, Write},
    path::Path,
};

pub fn read_collection<T, P>(path: P) -> eyre::Result<Vec<T>>
where
    T: DeserializeOwned,
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let collection = serde_json::from_reader(reader)?;

    Ok(collection)
}

pub fn write_collection<S, P>(path: P, collection: Vec<S>) -> eyre::Result<()>
where
    S: Serialize,
    P: AsRef<Path>,
{
    let mut file = OpenOptions::new().write(true).truncate(true).open(path)?;

    serde_json::to_writer(&mut file, &collection)?;

    Ok(())
}

pub fn prepare_collection(path: impl AsRef<Path>) -> anyhow::Result<()> {
    if path.as_ref().try_exists()? {
        return Ok(());
    }

    let mut file = std::fs::File::create(path)?;
    file.write_all(b"[]")?;

    Ok(())
}
