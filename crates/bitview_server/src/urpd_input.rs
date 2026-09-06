use bitcoin::hashes::{Hash, HashEngine, sha256};
use bitview_query::ResolvedUrpd;

/// Identity of every captured representation input, not proof of valid data.
pub fn identity(input: &ResolvedUrpd) -> sha256::Hash {
    let mut engine = sha256::Hash::engine();
    engine.input(b"urpd1\0");
    engine.input(input.cohort.as_bytes());
    engine.input(&[0]);
    engine.input(&input.date.year().to_le_bytes());
    engine.input(&[
        input.date.month(),
        input.date.day(),
        input.aggregation as u8,
        input.weight as u8,
    ]);
    engine.input(&input.scalar.to_bits().to_le_bytes());
    engine.input(&u64::from(input.close).to_le_bytes());
    input.for_each_section(|bytes| {
        engine.input(&(bytes.len() as u64).to_le_bytes());
        engine.input(bytes);
    });
    sha256::Hash::from_engine(engine)
}
