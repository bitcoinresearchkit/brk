use std::{
    fmt::Debug,
    fs::{self, File},
    io::{BufReader, BufWriter, Cursor},
    path::Path,
};

use bincode::{
    config, decode_from_slice, decode_from_std_read, encode_into_std_write, Decode, Encode,
};
use zstd::decode_all;

const ZST_EXTENSION: &str = "zst";

pub const BIN_EXTENSION: &str = "bin";
pub const COMPRESSED_BIN_EXTENSION: &str = "bin.zst";

enum BinaryType {
    Raw,
    Compressed,
}

pub struct Binary;

impl Binary {
    pub fn import<T>(path: &Path) -> color_eyre::Result<T>
    where
        T: Decode,
    {
        match Self::type_from_path(path) {
            BinaryType::Compressed => Self::import_compressed(path),
            BinaryType::Raw => Self::import_raw(path),
        }
    }

    fn import_raw<T>(path: &Path) -> color_eyre::Result<T>
    where
        T: Decode,
    {
        let config = config::standard();

        let file = File::open(path)?;

        let mut reader = BufReader::new(file);

        let decoded = decode_from_std_read(&mut reader, config)?;

        Ok(decoded)
    }

    fn import_compressed<T>(path: &Path) -> color_eyre::Result<T>
    where
        T: Decode,
    {
        let file = File::open(path).unwrap();

        let reader = BufReader::new(file);

        let decompressed = decode_all(reader).unwrap();

        let config = config::standard();

        let decoded = decode_from_slice::<T, _>(&decompressed, config).unwrap().0;

        Ok(decoded)
    }

    pub fn export<T>(path: &Path, value: &T) -> color_eyre::Result<()>
    where
        T: Debug + Encode,
    {
        // log(&format!("Exporting: {:?}", path));

        match Self::type_from_path(path) {
            BinaryType::Compressed => Self::export_compressed(path, value),
            BinaryType::Raw => Self::export_raw(path, value),
        }
    }

    fn export_raw<T>(path: &Path, value: &T) -> color_eyre::Result<()>
    where
        T: Debug + Encode,
    {
        let config = config::standard();

        let file = File::create(path).inspect_err(|_| {
            dbg!(path, value);
        })?;

        let mut writer = BufWriter::new(file);

        encode_into_std_write(value, &mut writer, config)?;

        Ok(())
    }

    fn export_compressed<T>(path: &Path, value: &T) -> color_eyre::Result<()>
    where
        T: Debug + Encode,
    {
        let config = config::standard();

        let encoded = bincode::encode_to_vec(value, config).unwrap();

        let cursor = Cursor::new(encoded);

        let compressed = zstd::encode_all(cursor, 0).unwrap();

        fs::write(path, compressed).unwrap();

        Ok(())
    }

    pub fn has_correct_extension(path: &Path) -> bool {
        let path = path.to_str().unwrap();
        path.ends_with(BIN_EXTENSION) || path.ends_with(COMPRESSED_BIN_EXTENSION)
    }

    fn type_from_path(path: &Path) -> BinaryType {
        let extension = path.extension();

        if extension.is_none() {
            panic!("Should have extension");
        }

        if !Self::has_correct_extension(path) {
            dbg!(path);
            panic!("Wrong extension")
        }

        let extension = extension.unwrap();

        if extension == ZST_EXTENSION {
            BinaryType::Compressed
        } else {
            BinaryType::Raw
        }
    }
}
