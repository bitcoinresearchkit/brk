use bitcoin::hashes::{Hash, HashEngine};
use derive_more::Deref;

use crate::BlkMetadata;

use super::{BlockHash, CoinbaseTag, Height};

/// Raw block bytes and per-tx offsets for fast txid hashing.
/// Present when block was parsed from blk*.dat files, absent for RPC blocks.
#[derive(Debug)]
struct RawBlockData {
    bytes: Vec<u8>,
    /// Per-tx byte offset within `bytes`.
    tx_offsets: Vec<u32>,
}

#[derive(Debug, Deref)]
pub struct Block {
    height: Height,
    hash: BlockHash,
    #[deref]
    block: bitcoin::Block,
    raw: Option<RawBlockData>,
}

impl Block {
    pub fn height(&self) -> Height {
        self.height
    }

    pub fn hash(&self) -> &BlockHash {
        &self.hash
    }

    /// Compute total_size and weight in a single pass (2N tx serializations
    /// instead of 3N from calling `total_size()` + `weight()` separately,
    /// since `weight()` internally calls both `base_size()` and `total_size()`).
    pub fn total_size_and_weight(&self) -> (usize, usize) {
        let overhead =
            bitcoin::block::Header::SIZE + bitcoin::VarInt::from(self.txdata.len()).size();
        let mut total_size = overhead;
        let mut weight_wu = overhead * 4;
        for (i, tx) in self.txdata.iter().enumerate() {
            let base = tx.base_size();
            let total = self
                .raw_tx_bytes(i)
                .map_or_else(|| tx.total_size(), |raw| raw.len());
            total_size += total;
            weight_wu += base * 3 + total;
        }
        (total_size, weight_wu)
    }

    pub fn set_raw_data(&mut self, bytes: Vec<u8>, tx_offsets: Vec<u32>) {
        self.raw = Some(RawBlockData { bytes, tx_offsets });
    }

    /// Compute txid, base_size, and total_size for the transaction at `index`.
    /// Uses raw bytes (fast path) when available, falls back to re-serialization.
    pub fn compute_tx_id_and_sizes(&self, index: usize) -> (bitcoin::Txid, u32, u32) {
        let tx = &self.txdata[index];
        if let Some(raw) = self.raw_tx_bytes(index) {
            let total_size = raw.len() as u32;
            let is_segwit = raw[4] == 0x00;
            let base_size = if is_segwit {
                tx.base_size() as u32
            } else {
                total_size
            };
            let txid = Self::hash_raw_tx(raw, base_size);
            debug_assert_eq!(txid, tx.compute_txid(), "raw txid mismatch at tx {index}");
            (txid, base_size, total_size)
        } else {
            (
                tx.compute_txid(),
                tx.base_size() as u32,
                tx.total_size() as u32,
            )
        }
    }

    /// Returns raw transaction bytes for the given tx index, if available.
    fn raw_tx_bytes(&self, index: usize) -> Option<&[u8]> {
        let raw = self.raw.as_ref()?;
        let start = raw.tx_offsets[index] as usize;
        let end = raw
            .tx_offsets
            .get(index + 1)
            .map_or(raw.bytes.len(), |&off| off as usize);
        Some(&raw.bytes[start..end])
    }

    /// Hash raw transaction bytes directly (SHA256d), avoiding re-serialization.
    ///
    /// For segwit (`raw[4] == 0x00`): hashes version + inputs/outputs + locktime,
    /// skipping marker, flag, and witness data.
    /// For legacy: hashes entire raw bytes.
    fn hash_raw_tx(raw: &[u8], base_size: u32) -> bitcoin::Txid {
        let mut engine = bitcoin::Txid::engine();
        if raw[4] == 0x00 {
            let io_len = base_size as usize - 8;
            engine.input(&raw[..4]);
            engine.input(&raw[6..6 + io_len]);
            engine.input(&raw[raw.len() - 4..]);
        } else {
            engine.input(raw);
        }
        bitcoin::Txid::from_engine(engine)
    }

    pub fn coinbase_tag(&self) -> CoinbaseTag {
        let bytes = self
            .txdata
            .first()
            .and_then(|tx| tx.input.first())
            .unwrap()
            .script_sig
            .as_bytes();
        CoinbaseTag::from(bytes)
    }
}

impl From<(Height, bitcoin::Block)> for Block {
    #[inline]
    fn from((height, block): (Height, bitcoin::Block)) -> Self {
        Self::from((height, block.block_hash(), block))
    }
}

impl From<(Height, bitcoin::BlockHash, bitcoin::Block)> for Block {
    #[inline]
    fn from((height, hash, block): (Height, bitcoin::BlockHash, bitcoin::Block)) -> Self {
        Self::from((height, BlockHash::from(hash), block))
    }
}

impl From<(Height, BlockHash, bitcoin::Block)> for Block {
    #[inline]
    fn from((height, hash, block): (Height, BlockHash, bitcoin::Block)) -> Self {
        Self {
            height,
            hash,
            block,
            raw: None,
        }
    }
}

impl From<ReadBlock> for Block {
    #[inline]
    fn from(value: ReadBlock) -> Self {
        value.block
    }
}

#[derive(Debug, Deref)]
pub struct ReadBlock {
    #[deref]
    block: Block,
    metadata: BlkMetadata,
    tx_metadata: Vec<BlkMetadata>,
}

impl From<(Block, BlkMetadata, Vec<BlkMetadata>)> for ReadBlock {
    #[inline]
    fn from((block, metadata, tx_metadata): (Block, BlkMetadata, Vec<BlkMetadata>)) -> Self {
        Self {
            block,
            metadata,
            tx_metadata,
        }
    }
}

impl ReadBlock {
    pub fn metadata(&self) -> &BlkMetadata {
        &self.metadata
    }

    pub fn tx_metadata(&self) -> &Vec<BlkMetadata> {
        &self.tx_metadata
    }

    pub fn inner(self) -> Block {
        self.block
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitcoin::{Transaction, consensus::Decodable};

    fn decode_hex(s: &str) -> Vec<u8> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
            .collect()
    }

    #[test]
    fn hash_raw_tx_matches_compute_txid_for_segwit_tx() {
        let hex = "0100000000010ad027250a29ef00b4fbab679d6d910054826a5ee679c70f69500fc2df02f87aea0400000000ffffffff93c9f683b46da97d2c631dccba17cb7c9dc5bd3c1afab3320bc4aa03369c201b0000000000ffffffff1e8170f5ad6680adcacb14ebce26d8584050b59b50fb83d5a1f958e99250b16e00000000fdfd000047304402200328e9ce339c90f5211a9c7c3315b0db27cbf554b383397bf0d50cfd4c5a150d022020f439553ed1712c63429073aa5811198e6d0db0606229540836380054d6ddcb014830450221009ed16332377d8bec8ea2cf59f7f4f6eb72eb0d9ff7fe12693cd400d84bde6eca0220563dd7fc2cd248d695ff38380efc7e27d99eb599815008b9baae7c4600e84284014c69522102a30752dcbbde99be02549ef88bc4d48122f0622c14aebe2dc20fe3f97101270a210256edf7f7bec24b7e4d11e49494a2147dc83c7dfd5bb3cc452181b2215c160bdd21033f5794f9a4640d35f42b132b552487d2ba0c5c3888380b66a3b16d2f366ce51b53aeffffffff09ad2196dd3dc106ac773f31e8918aa0a533daa1d3ad4db0cdb4df08750b061b77010000fdfd0000483045022100de89da280706a5cecb6472647df34c81f24c34f37f32c3fca07d64ce495e94f0022044048eeb71740e3b90540303688341584bc1b5f2459e104f1c52fdf2b047b4590147304402202a3ff42d0650d311e180f80019d7cc141e27c4706e9a79a1b2ff2f83bdc840eb02207cc28198d76bd2b5e7043b59025ef269301b337a0c5468ce9f7c332ce302d608014c69522102831ed7a372b37b803bd28cf987fde03a76ac873e0c801ba98309a08d043f9987210375adeb1ac333d7d0776ed8663e567bb129f76ad9d590435354307ab396350d082103407713929ce2e1b1f6f66d314a96db5b0aead3b47a2136a6e9feb4aed237972253aeffffffffaacd884cd7719ad1f1f49c6fed68449e50dfc11c7264559171a0f9220d998fab0100000000ffffffffa994c3b8c0acffbc5cd948fb01239ea3e89079be768822e56e98a0d868f9e46300000000fdfd0000483045022100ac6b08eab37b9e603951109f46ba54c69c3e399e5943015fdea76af263028c87022045fb7894781d8ce7f015bba655a9aa32fe54e16ace9911fad73ca39541f3126401473044022067654eaacbdb49b9c1ad55ca3319eb6a15d49019ffddc0329959e89160bfbba2022066943b9a869145940da4db58db36fe8c1bb8510827bd09327edbb938120c9bd5014c6952210393d7f2b624b51d8807be5dd1111f69b42d6422a2d073621684a011ce9743c7792102d5627dd55b9b5f99de1d52554e512022a424c360fe1c069a6c13c6cc5376effc2103747cb24063c911b5b31e94b058d04e17d86422c52c07d6c47db6b74d12c060ea53aeffffffffbb395a98a515b625ec35b41dcfc0cb9a225816abd676cf4fd950120e70e387120000000000ffffffff62872b622e250f6a067b0ef1e63ee6fa958b51f90c78dde02f5e8da90f8066d500000000fdfd0000483045022100cd1b0dd9404279b862d6a155ffbe894cd5e2285ec04f6eab61a17d4244049c9a02203ca5b49c55ef08ee6f25cfc329ae2acb7b60be46a72e3bb00485d2feba425cc101473044022018d643afccb3418c33b84eefe86de6baec695c5a5481c31e41fa22aedaed067402204529a1382d8d3e1ad7ace4709466f5df72c6e069166b28284c64c4e5278839d2014c695221037a06cea95b189aafe1e15b1eb931db5cf559beaf394cbc32ae5856b3e9814c132102e22967dd22d7740cd2c9d7fbd39f436ff611dc66d8043e077fec4732da704e052102035b82c21f2819595d72a665f4f4b5cb755a4756b4aa07812e4b825473e3748753aeffffffff127d9aa6f11a32f29d89be6e2837564162d911e3ab955be66c50c7fff8ad9bee00000000fc0047304402203988842a765e875f3931b764ec27b0dc9c5681188145a1a0dcf98fca44b7a93b02200d315a3f3648bec8a44c4db2edf2d9875197cc380ae9f1ccbf04e211af178c950147304402202e26adf91def55e8e54d32dbd339a7ef7a22e263518f3910073960fbdf2fee880220372c5dc1c6db8a36aa68dc2d9c0d300bacc882a61503bff4af7b4ebca92bd8c4014c69522103dbf9d0d0ecf6c1999c11c455677e8ebae5de2ffbab90954632aab0a9a257f21f21037480f7a31eb7a60db9c7e9f0a99b14bffd73014a3902a1d501820c3cdd51fc532103aa1e66aebe111353ff9febe0aa3c03f3acb7d339fa1c0811271ae951dd8bdc6353aeffffffff7487c6c1463948fe863431a1da6a4ff91baaa44606bc575fe980b1e86c9e9be00800000000ffffffff11705d010000000000225120ce4ab52fc4e6eea74c7db277bac6d9b9325fde9253039513b1de9782f452587ac0273500000000001976a9144210f6f3929a7fc2b25e96636c740cd7935b883d88ac4897e11100000000160014ea688c32414aebe4556dcd42301112dc06fa7290f661ad0000000000160014224b6524062a25b67cb5c1df918443b7c2e129689d211d0000000000225120cb8f606f894c983d1a644e60787e1ebef78ebc4ba760a3e230a7cd0cdfbec450144704000000000017a91462122c29b2ca069322c2daf6b19711a3e0ea595687bb5444020000000022512087f33c6febffa9ff60af09216a5897c5ab8638caaa3322779f02842862e3ac995ac90000000000001600148e5ab09208d65672994de460f70fc932feb269795809030000000000160014a310664053baef6ebda995ae758322f7c9ab2db56cdc0f000000000022512089ab10c68d292f5a00acca284b4f8712f0e6482dad4a3d0ad67c60a52086ef96012807000000000016001455621c3abb99d39577c19184ee816d10b578d6d5a08e030000000000160014dcf2b597675b3b375fb74735dfc44cadba18f19100b60200000000001600143ad99e76508462f0c119ab9c174937c56fede6ab2eaa430200000000220020ef90c2de160bb4939dfbcb13b530aef51b83f8ccf045709cc7ff8ed534dbb3db17472700000000001976a914d6065a5aad907d0f3f22e2962475355e6406dd7288ac1f610100000000001600148d0fd4724281d72f84774d99f4878cd0681e9ae631d0960300000000220020e5c7c00d174631d2d1e365d6347b016fb87b6a0c08902d8e443989cb771fa7ec04004730440220011789ebda70dec9c8bffec5a362ddf29cee2be3f5f9180b514af376ce0d077302205a38b773555e4795dc156286a4dab915b9a5eb60dbd3e2ba192ab35512290c7b0147304402206753092d5902dc26a93deb06c19a4b434563f8d94d718ced82ff3806cc38b59902205d1b604e9cb15728c7980b68ff29fe87ec380dc973756d9aa34e4cffb05a38360169522102ee39e562700f3793caa280d583e7be8dcae6b93fba64ee36204ea1b2ff39f24f2102249d182ac68b5379e9c31bc4773a6d150f437ae8077bd54fa9d7e62f4afbeb762103b1a1f84d381a9807a6ab1d7be0d7a5389cdaf226bdb68c23266c700281937c6f53ae0400483045022100a48324f241bc59374b54e3d1dacef1b1e713366c3af2bfeb438ab3e5d7822247022076be22d14fc628ed9123a38dbdc603073b79a531a2a0da3fefa6b4c29f3dc34d01473044022038546446cbbae00a41b14b2849556add6cf8b858bc445565fec4901922a1512d0220055c3f8dd44a00ccd3d3dee938f4d33a63411de71aea196aed7b223279f3d5eb016952210362c6286246c0dc9ff357fd98c298ce2da405e7d27536e4bc260586b678b3d8592103eec4a13ef4a65c5eae1602e2338e22a32997a25b6534698c606cb0560cba15ad210299f5b9d4f378005d67b9d77f762abb6070d02b7983895499efefe6f2341c579253ae0000040047304402202dcc5baba068b9ae48652af80d1c2ca2b445347db01cbf1f794d208c8087868c02200bb0cbc83f1d3075ed931c46688636152aaf87ad328e561a4220f3c690f9657f01473044022020bf84afc514f50222e5e7df62f158c56fad064c0c993a1ca021f22b48f41b6a0220546b24c9133c3b16cdb9289314b9b0ef012f33b276a8ef4f2b80206cdd1f2bb601695221023f0345f574652e6952fa9d288b3ba38cc15c09277decaa0e06ef1771f515f38721024274e29f6bb7569841c384b52828209e77bb0db0569c969e7aa386ce2768dc37210398d63ca1f2eed732371d7a36e8f41da050c14031a9adee29c8c2f246dbee06d653ae000400483045022100c1b84e2d9f2c83ea19668881f453caaf40856e2fa6b86361ced62644ac387522022022b8787394809f5406f9188478537128ad9a5e02677afca22a80f136b941de8301483045022100e6caad435ebffe06576446c95e5900f79eda75dcccf6a177442d2c82f947836202205fb7473dbc444d7dd436497da60533397d2468d0a30edb6b3ed514b466995dba016952210205ce9f2ad24fe1221ae503f67bb0ecc759e0c5945c8df2158d8ba5bf262c570c21023a05a46bd80a1a5c6c727704d9e9ddba74590041ae52524090b5ebc779d84a462102543352c636506f63617f7c6d6ec44cc808e36ebb8af4bd04dc61961fe7531cbc53ae00000400473044022025d75591d74390dd3fa267b9dd2bb2f8d1adfb4bab787a3f6ad33d0958a15f6c022039352e6caeae77fc8a0c5ef5e27fe414488c145420f01ab62cf03373cd7fdd3701483045022100eaac10703e5aab0b6db6c127f723c2be88f5c0790e1377727ab7250c6c1b29ec022012b6386a067b9e244698d1972f4fbe8d4d04859b69a6f8311406cdf325502ee901695221026064e5b88c4fff7dba7dc0300db8dbfc1faff14f9ddbaacbcaa4f70124de0e93210331870350912385ca9a9d537e9cf9d80c6c9558e31d654f82f3164fdc5955e9642103c7b133a0f463a501d8c58c8eb8c7b6e9e4ddfb7d4a7bf6365a4732201569bc8353ae00000000";
        let raw = decode_hex(hex);
        let tx = Transaction::consensus_decode(&mut &raw[..]).unwrap();

        let expected_txid = tx.compute_txid();
        assert_eq!(
            expected_txid.to_string(),
            "3595e70d423e7a28c6a865322d30e49e98d377cd93e0b53d133ca7397d9d752e"
        );

        let base_size = tx.base_size() as u32;
        let total_size = raw.len() as u32;
        assert_ne!(base_size, total_size, "should be segwit");
        assert_eq!(raw[4], 0x00, "segwit marker");

        let raw_txid = Block::hash_raw_tx(&raw, base_size);
        assert_eq!(
            raw_txid, expected_txid,
            "hash_raw_tx must match compute_txid"
        );
    }
}
