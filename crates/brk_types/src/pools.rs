use std::{slice::Iter, sync::OnceLock};

use allocative::Allocative;

use crate::{JSONPool, PoolId};

use super::Pool;

const POOL_COUNT: usize = 158;

#[derive(Debug, Allocative)]
pub struct Pools([Pool; POOL_COUNT]);

impl Pools {
    pub fn find_from_coinbase_tag(&self, coinbase_tag: &str) -> Option<&Pool> {
        let coinbase_tag = coinbase_tag.to_lowercase();
        self.0.iter().find(|pool| {
            pool.tags_lowercase
                .iter()
                .any(|pool_tag| coinbase_tag.contains(pool_tag))
        })
    }

    pub fn find_from_address(&self, address: &str) -> Option<&Pool> {
        self.0.iter().find(|pool| pool.addresses.contains(&address))
    }

    pub fn get_unknown(&self) -> &Pool {
        &self.0[0]
    }

    pub fn get(&self, id: PoolId) -> &Pool {
        let i: u8 = id.into();
        &self.0[i as usize]
    }

    pub fn iter(&self) -> Iter<'_, Pool> {
        self.0.iter()
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

pub fn pools() -> &'static Pools {
    static POOLS: OnceLock<Pools> = OnceLock::new();
    POOLS.get_or_init(|| {
        Pools::from([
            JSONPool {
                name: "Unknown",
                addresses: Box::new([]),
                tags: Box::new([]),
                link: "",
            },
            // Source:
            // https://github.com/mempool/mining-pools/blob/master/pools-v2.json
            JSONPool {
                name: "BlockFills",
                addresses: Box::new(["1PzVut5X6Nx7Mv4JHHKPtVM9Jr9LJ4Rbry"]),
                tags: Box::new(["/BlockfillsPool/"]),
                link: "https://www.blockfills.com/mining",
            },
            JSONPool {
                name: "ULTIMUSPOOL",
                addresses: Box::new([
                    "1EMVSMe1VJUuqv7D7SFzctnVXk4KdjXATi",
                    "3C9sAKXrBVpJVe3b738yik4LPHpPmceBgd",
                ]),
                tags: Box::new(["/ultimus/"]),
                link: "https://www.ultimuspool.com",
            },
            JSONPool {
                name: "Terra Pool",
                addresses: Box::new([
                    "32P5KVSbZYAkVmSHxDd2oBXaSk372rbV7L",
                    "3Qqp7LwxmSjPwRaKkDToysJsM3xA4ThqFk",
                    "bc1q39dled8an7enuxtmjql3pk7ny8kzvsxhd924sl",
                ]),
                tags: Box::new(["terrapool.io", "Validated with Clean Energy"]),
                link: "https://terrapool.io",
            },
            JSONPool {
                name: "Luxor",
                addresses: Box::new([
                    "1MkCDCzHpBsYQivp8MxjY5AkTGG1f2baoe",
                    "39bitUyBcUu3y3hRTtYprKbTp712t4ZWqK",
                    "32BfKjhByDSxx3BM5vUkQ3NQq9csZR6nt6",
                ]),
                tags: Box::new(["/LUXOR/", "Luxor Tech"]),
                link: "https://mining.luxor.tech",
            },
            JSONPool {
                name: "1THash",
                addresses: Box::new([
                    "147SwRQdpCfj5p8PnfsXV2SsVVpVcz3aPq",
                    "15vgygQ7ZsWdvZpctmTZK4673QBHsos6Sh",
                ]),
                tags: Box::new(["/1THash&58COIN/", "/1THash/"]),
                link: "https://www.1thash.top",
            },
            JSONPool {
                name: "BTC.com",
                addresses: Box::new([
                    "1Bf9sZvBHPFGVPX71WX2njhd1NXKv5y7v5",
                    "34qkc2iac6RsyxZVfyE2S5U5WcRsbg2dpK",
                    "3EhLZarJUNSfV6TWMZY1Nh5mi3FMsdHa5U",
                    "3NA8hsjfdgVkmmVS9moHmkZsVCoLxUkvvv",
                    "bc1qjl8uwezzlech723lpnyuza0h2cdkvxvh54v3dn",
                ]),
                tags: Box::new(["/BTC.COM/", "/BTC.com/", "btccom"]),
                link: "https://pool.btc.com",
            },
            JSONPool {
                name: "Bitfarms",
                addresses: Box::new(["3GvEGtnvgeBJ3p3EpdZhvUkxY4pDARkbjd"]),
                tags: Box::new(["BITFARMS"]),
                link: "https://www.bitfarms.io",
            },
            JSONPool {
                name: "Huobi.pool",
                addresses: Box::new([
                    "18Zcyxqna6h7Z7bRjhKvGpr8HSfieQWXqj",
                    "1EepjXgvWUoRyNvuLSAxjiqZ1QqKGDANLW",
                    "1MvYASoHjqynMaMnP7SBmenyEWiLsTqoU6",
                    "3HuobiNg2wHjdPU2mQczL9on8WF7hZmaGd",
                ]),
                tags: Box::new(["/HuoBi/", "/Huobi/"]),
                link: "https://www.hpt.com",
            },
            JSONPool {
                name: "WAYI.CN",
                addresses: Box::new([]),
                tags: Box::new(["/E2M & BTC.TOP/"]),
                link: "https://www.easy2mine.com",
            },
            JSONPool {
                name: "CanoePool",
                addresses: Box::new(["1GP8eWArgpwRum76saJS4cZKCHWJHs9PQo"]),
                tags: Box::new(["/CANOE/", "/canoepool/"]),
                link: "https://btc.canoepool.com",
            },
            JSONPool {
                name: "BTC.TOP",
                addresses: Box::new(["1Hz96kJKF2HLPGY15JWLB5m9qGNxvt8tHJ"]),
                tags: Box::new(["/BTC.TOP/"]),
                link: "https://btc.top",
            },
            JSONPool {
                name: "Bitcoin.com",
                addresses: Box::new([]),
                tags: Box::new(["pool.bitcoin.com"]),
                link: "https://www.bitcoin.com",
            },
            JSONPool {
                name: "175btc",
                addresses: Box::new([]),
                tags: Box::new(["Mined By 175btc.com"]),
                link: "https://www.175btc.com",
            },
            JSONPool {
                name: "GBMiners",
                addresses: Box::new([]),
                tags: Box::new(["/mined by gbminers/"]),
                link: "https://gbminers.com",
            },
            JSONPool {
                name: "A-XBT",
                addresses: Box::new(["1MFsp2txCPwMMBJjNNeKaduGGs8Wi1Ce7X"]),
                tags: Box::new(["/A-XBT/"]),
                link: "https://www.a-xbt.com",
            },
            JSONPool {
                name: "ASICMiner",
                addresses: Box::new([]),
                tags: Box::new(["ASICMiner"]),
                link: "https://www.asicminer.co",
            },
            JSONPool {
                name: "BitMinter",
                addresses: Box::new(["19PkHafEN18mquJ9ChwZt5YEFoCdPP5vYB"]),
                tags: Box::new(["BitMinter"]),
                link: "https://bitminter.com",
            },
            JSONPool {
                name: "BitcoinRussia",
                addresses: Box::new([
                    "14R2r9FkyDmyxGB9xUVwVLdgsX9YfdVamk",
                    "165GCEAx81wce33FWEnPCRhdjcXCrBJdKn",
                ]),
                tags: Box::new(["/Bitcoin-Russia.ru/"]),
                link: "https://bitcoin-russia.ru",
            },
            JSONPool {
                name: "BTCServ",
                addresses: Box::new([]),
                tags: Box::new(["btcserv"]),
                link: "https://btcserv.net",
            },
            JSONPool {
                name: "simplecoin.us",
                addresses: Box::new([]),
                tags: Box::new(["simplecoin"]),
                link: "https://simplecoin.us",
            },
            JSONPool {
                name: "BTC Guild",
                addresses: Box::new([]),
                tags: Box::new(["BTC Guild"]),
                link: "https://www.btcguild.com",
            },
            JSONPool {
                name: "Eligius",
                addresses: Box::new([]),
                tags: Box::new(["Eligius"]),
                link: "https://eligius.st",
            },
            JSONPool {
                name: "OzCoin",
                addresses: Box::new([]),
                tags: Box::new(["ozco.in", "ozcoin"]),
                link: "https://ozcoin.net",
            },
            JSONPool {
                name: "EclipseMC",
                addresses: Box::new([
                    "15xiShqUqerfjFdyfgBH1K7Gwp6cbYmsTW",
                    "18M9o2mXNjNR96yKe7eyY6pfP6Nx4Nso3d",
                ]),
                tags: Box::new(["EMC ", "EMC:"]),
                link: "https://eclipsemc.com",
            },
            JSONPool {
                name: "MaxBTC",
                addresses: Box::new([]),
                tags: Box::new(["MaxBTC"]),
                link: "https://maxbtc.com",
            },
            JSONPool {
                name: "TripleMining",
                addresses: Box::new([]),
                tags: Box::new(["Triplemining.com", "triplemining"]),
                link: "https://www.triplemining.com",
            },
            JSONPool {
                name: "CoinLab",
                addresses: Box::new([]),
                tags: Box::new(["CoinLab"]),
                link: "https://coinlab.com",
            },
            JSONPool {
                name: "50BTC",
                addresses: Box::new([]),
                tags: Box::new(["50BTC"]),
                link: "https://www.50btc.com",
            },
            JSONPool {
                name: "GHash.IO",
                addresses: Box::new(["1CjPR7Z5ZSyWk6WtXvSFgkptmpoi4UM9BC"]),
                tags: Box::new(["ghash.io"]),
                link: "https://ghash.io",
            },
            JSONPool {
                name: "ST Mining Corp",
                addresses: Box::new([]),
                tags: Box::new(["st mining corp"]),
                link: "https://bitcointalk.org/index.php?topic=77000.msg3207708#msg3207708",
            },
            JSONPool {
                name: "Bitparking",
                addresses: Box::new([]),
                tags: Box::new(["bitparking"]),
                link: "https://mmpool.bitparking.com",
            },
            JSONPool {
                name: "mmpool",
                addresses: Box::new([]),
                tags: Box::new(["mmpool"]),
                link: "https://mmpool.org/pool",
            },
            JSONPool {
                name: "Polmine",
                addresses: Box::new([
                    "13vWXwzNF5Ef9SUXNTdr7de7MqiV4G1gnL",
                    "16cv7wyeG6RRqhvJpY21CnsjxuKj2gAoK2",
                    "17kkmDx8eSwj2JTTULb3HkJhCmexfysExz",
                    "1AajKXkaq2DsnDmP8ZPTrE5gH1HFo1x3AU",
                    "1JrYhdhP2jCY6JwuVzdk9jUwc4pctcSes7",
                    "1Nsvmnv8VcTMD643xMYAo35Aco3XA5YPpe",
                ]),
                tags: Box::new(["by polmine.pl", "bypmneU"]),
                link: "https://polmine.pl",
            },
            JSONPool {
                name: "KnCMiner",
                addresses: Box::new([]),
                tags: Box::new(["KnCMiner"]),
                link: "https://portal.kncminer.com/pool",
            },
            JSONPool {
                name: "Bitalo",
                addresses: Box::new(["1HTejfsPZQGi3afCMEZTn2xdmoNzp13n3F"]),
                tags: Box::new(["Bitalo"]),
                link: "https://bitalo.com/mining",
            },
            JSONPool {
                name: "F2Pool",
                addresses: Box::new([
                    "1KFHE7w8BhaENAswwryaoccDb6qcT6DbYY",
                    "bc1qf274x7penhcd8hsv3jcmwa5xxzjl2a6pa9pxwm",
                ]),
                tags: Box::new(["七彩神仙鱼", "F2Pool", "🐟"]),
                link: "https://www.f2pool.com",
            },
            JSONPool {
                name: "HHTT",
                addresses: Box::new([]),
                tags: Box::new(["HHTT"]),
                link: "https://hhtt.1209k.com",
            },
            JSONPool {
                name: "MegaBigPower",
                addresses: Box::new(["1K7znxRfkS8R1hcmyMvHDum1hAQreS4VQ4"]),
                tags: Box::new(["megabigpower.com"]),
                link: "https://megabigpower.com",
            },
            JSONPool {
                name: "Mt Red",
                addresses: Box::new([]),
                tags: Box::new(["/mtred/"]),
                link: "https://mtred.com",
            },
            JSONPool {
                name: "NMCbit",
                addresses: Box::new([]),
                tags: Box::new(["nmcbit.com"]),
                link: "https://nmcbit.com",
            },
            JSONPool {
                name: "Yourbtc.net",
                addresses: Box::new([]),
                tags: Box::new(["yourbtc.net"]),
                link: "https://yourbtc.net",
            },
            JSONPool {
                name: "Give Me Coins",
                addresses: Box::new([]),
                tags: Box::new(["Give-Me-Coins"]),
                link: "https://give-me-coins.com",
            },
            JSONPool {
                name: "Braiins Pool",
                addresses: Box::new([
                    "1AqTMY7kmHZxBuLUR5wJjPFUvqGs23sesr",
                    "1CK6KHY6MHgYvmRQ4PAafKYDrg1ejbH1cE",
                ]),
                tags: Box::new(["/slush/"]),
                link: "https://braiins.com/pool",
            },
            JSONPool {
                name: "AntPool",
                addresses: Box::new([
                    "12dRugNcdxK39288NjcDV4GX7rMsKCGn6B",
                    "15kiNKfDWsq7UsPg87UwxA8rVvWAjzRkYS",
                    "16MdTdqmXusauybtXTmFEW4GNFPPgGxQYE",
                    "16kUc5B48qnASbxeZTisCqTNx6G3DPXuKn",
                    "17gVZssumiJqYMCHozHKXGyaAvyu6NCX6V",
                    "1AJQ3jXhUF8WiisEcuVd8Xmfq4QJ7n1SdL",
                    "1B7ZBX2C39b26M9chHLURGSFTJA6DDQkZv",
                    "1BWW3pg5jb6rxebrNeo9TATarwJ1rthnoe",
                    "1CBqo1w3hmm9SCmbu2Yg6Ls4uLfkUqZJsx",
                    "1CZHhV67Qos4xXb8uYqvAGjK8Wq52woPi5",
                    "1CyB8GJNEsNVXtPutB36nrDY3fMXBTzXSX",
                    "1D4UZG4qo8bF1MuZHSEyBHRZaxT8inatXS",
                    "1D9jw3QHNankXxtcGVihsDK7Z7THN6j7Pg",
                    "1DDXyKUT6q3H9e5QXm2Gv6BNNWgztFG55g",
                    "1DQaDTefKPjHz3beLuo8KHRZF9t2Sc6foP",
                    "1Dek9ArRHb9tyWb9gaaX8SWmkfi5V7U5Y6",
                    "1DyR7HPQWjM6Zrnk7SzHVY2GEpXRGNNH9o",
                    "1FdJkPdpXtK3t5utZHJAop3saLZWfPfgak",
                    "1FrHkVsW7csAYYaRbUUcrKSmv91hcQnsqQ",
                    "1GEG1JR81jvUXs7TMAuo3SPBHZrpJijcjt",
                    "1GRcX882sdBYCAWyG99iF2oz7j3nYzXhLM",
                    "1GT2N4dCufvbnTKMbS61QrQPN4SexCAFiH",
                    "1Gp7iCzDGMZiV55Kt8uKsux6VyoHe1aJaN",
                    "1H3u6R813MHGYhmGW6v86EYYriawRtACYD",
                    "1H6ckqNWikmVT3wpN3X1BQ6b156Xc9nT2L",
                    "1JBVrhSSDrZrRmm4RnoWouqgGGqJMvWHi8",
                    "1JwUDWVSbAY5NeCBJhxQk1E8AfETfZuPj4",
                    "1K8PNogxBZ6ts532DZnzxdbjgzJLjLdXqz",
                    "1KmgBTL7cFmFFYTD7HcdkMcZXRcTkh2WwS",
                    "1LTGvTjDxiy5S9YcKEE9Lb7xSpZcPSqinw",
                    "1MiQrT5sEKTUGNMbd9WS3yPPkSjWdpYA2r",
                    "1NS4gbx1G2D5rc9PnvVsPys12nKxGiQg72",
                    "1Nh7uHdvY6fNwtQtM1G5EZAFPLC33B59rB",
                    "1Pzf7qT7bBGouvnjRvtRD8VhTyqjX1NrJT",
                    "1Sjj2cPC3rTWcSTEYDeu2f3BavLosog4T",
                    "1jLVpwtNMfXWaHY4eiLDmGuBxokYLgv1X",
                    "3FaYVQF6wCMUB9NCeRe4tUp1zZx8qqM7H1",
                ]),
                tags: Box::new(["/AntPool/", "Mined By AntPool", "Mined by AntPool"]),
                link: "https://www.antpool.com",
            },
            JSONPool {
                name: "MultiCoin.co",
                addresses: Box::new([]),
                tags: Box::new(["Mined by MultiCoin.co"]),
                link: "https://multicoin.co",
            },
            JSONPool {
                name: "bcpool.io",
                addresses: Box::new([]),
                tags: Box::new(["bcpool.io"]),
                link: "https://bcpool.io",
            },
            JSONPool {
                name: "Cointerra",
                addresses: Box::new(["1BX5YoLwvqzvVwSrdD4dC32vbouHQn2tuF"]),
                tags: Box::new(["cointerra"]),
                link: "https://cointerra.com",
            },
            JSONPool {
                name: "KanoPool",
                addresses: Box::new([]),
                tags: Box::new(["Kano"]),
                link: "https://kano.is",
            },
            JSONPool {
                name: "Solo CK",
                addresses: Box::new([]),
                tags: Box::new(["/solo.ckpool.org/"]),
                link: "https://solo.ckpool.org",
            },
            JSONPool {
                name: "CKPool",
                addresses: Box::new([]),
                tags: Box::new(["/ckpool.org/"]),
                link: "https://ckpool.org",
            },
            JSONPool {
                name: "NiceHash",
                addresses: Box::new([]),
                tags: Box::new(["/NiceHashSolo", "/NiceHash/"]),
                link: "https://www.nicehash.com",
            },
            JSONPool {
                name: "BitClub",
                addresses: Box::new(["155fzsEBHy9Ri2bMQ8uuuR3tv1YzcDywd4"]),
                tags: Box::new(["/BitClub Network/"]),
                link: "https://bitclubpool.com",
            },
            JSONPool {
                name: "Bitcoin Affiliate Network",
                addresses: Box::new([]),
                tags: Box::new(["bitcoinaffiliatenetwork.com"]),
                link: "https://mining.bitcoinaffiliatenetwork.com",
            },
            JSONPool {
                name: "BTCC",
                addresses: Box::new(["152f1muMCNa7goXYhYAQC61hxEgGacmncB"]),
                tags: Box::new(["/BTCC/", "BTCChina Pool", "BTCChina.com", "btcchina.com"]),
                link: "https://pool.btcc.com",
            },
            JSONPool {
                name: "BWPool",
                addresses: Box::new(["1JLRXD8rjRgQtTS9MvfQALfHgGWau9L9ky"]),
                tags: Box::new(["BW Pool", "BWPool"]),
                link: "https://bwpool.net",
            },
            JSONPool {
                name: "EXX&BW",
                addresses: Box::new([]),
                tags: Box::new(["xbtc.exx.com&bw.com"]),
                link: "https://xbtc.exx.com",
            },
            JSONPool {
                name: "Bitsolo",
                addresses: Box::new(["18zRehBcA2YkYvsC7dfQiFJNyjmWvXsvon"]),
                tags: Box::new(["Bitsolo Pool"]),
                link: "https://bitsolo.net",
            },
            JSONPool {
                name: "BitFury",
                addresses: Box::new([
                    "14yfxkcpHnju97pecpM7fjuTkVdtbkcfE6",
                    "1AcAj9p6zJn4xLXdvmdiuPCtY7YkBPTAJo",
                ]),
                tags: Box::new(["/BitFury/", "/Bitfury/"]),
                link: "https://bitfury.com",
            },
            JSONPool {
                name: "21 Inc.",
                addresses: Box::new([
                    "15rQXUSBQRubShPpiJfDLxmwS8ze2RUm4z",
                    "1CdJi2xRTXJF6CEJqNHYyQDNEcM3X7fUhD",
                    "1GC6HxDvnchDdb5cGkFXsJMZBFRsKAXfwi",
                ]),
                tags: Box::new(["/pool34/"]),
                link: "https://21.co",
            },
            JSONPool {
                name: "digitalBTC",
                addresses: Box::new(["1MimPd6LrPKGftPRHWdfk8S3KYBfN4ELnD"]),
                tags: Box::new(["/agentD/"]),
                link: "https://digitalbtc.com",
            },
            JSONPool {
                name: "8baochi",
                addresses: Box::new(["1Hk9gD8xMo2XBUhE73y5zXEM8xqgffTB5f"]),
                tags: Box::new(["/八宝池 8baochi.com/"]),
                link: "https://8baochi.com",
            },
            JSONPool {
                name: "myBTCcoin Pool",
                addresses: Box::new(["151T7r1MhizzJV6dskzzUkUdr7V8JxV2Dx"]),
                tags: Box::new(["myBTCcoin Pool"]),
                link: "https://mybtccoin.com",
            },
            JSONPool {
                name: "TBDice",
                addresses: Box::new(["1BUiW44WuJ2jiJgXiyxJVFMN8bc1GLdXRk"]),
                tags: Box::new(["TBDice"]),
                link: "https://tbdice.org",
            },
            JSONPool {
                name: "HASHPOOL",
                addresses: Box::new([]),
                tags: Box::new(["HASHPOOL"]),
                link: "https://hashpool.com",
            },
            JSONPool {
                name: "Nexious",
                addresses: Box::new(["1GBo1f2tzVx5jScV9kJXPUP9RjvYXuNzV7"]),
                tags: Box::new(["/Nexious/"]),
                link: "https://nexious.com",
            },
            JSONPool {
                name: "Bravo Mining",
                addresses: Box::new([]),
                tags: Box::new(["/bravo-mining/"]),
                link: "https://www.bravo-mining.com",
            },
            JSONPool {
                name: "HotPool",
                addresses: Box::new(["17judvK4AC2M6KhaBbAEGw8CTKc9Pg8wup"]),
                tags: Box::new(["/HotPool/"]),
                link: "https://hotpool.co",
            },
            JSONPool {
                name: "OKExPool",
                addresses: Box::new([]),
                tags: Box::new(["/www.okex.com/"]),
                link: "https://www.okex.com",
            },
            JSONPool {
                name: "BCMonster",
                addresses: Box::new(["1E18BNyobcoiejcDYAz5SjbrzifNDEpM88"]),
                tags: Box::new(["/BCMonster/"]),
                link: "https://www.bcmonster.com",
            },
            JSONPool {
                name: "1Hash",
                addresses: Box::new(["1F1xcRt8H8Wa623KqmkEontwAAVqDSAWCV"]),
                tags: Box::new(["Mined by 1hash.com"]),
                link: "https://www.1hash.com",
            },
            JSONPool {
                name: "Bixin",
                addresses: Box::new([
                    "13hQVEstgo4iPQZv9C7VELnLWF7UWtF4Q3",
                    "1KsFhYKLs8qb1GHqrPxHoywNQpet2CtP9t",
                ]),
                tags: Box::new(["/Bixin/", "/HaoBTC/", "HAOBTC"]),
                link: "https://haopool.com",
            },
            JSONPool {
                name: "TATMAS Pool",
                addresses: Box::new([]),
                tags: Box::new(["/ViaBTC/TATMAS Pool/"]),
                link: "https://tmsminer.com",
            },
            JSONPool {
                name: "ViaBTC",
                addresses: Box::new([]),
                tags: Box::new(["/ViaBTC/", "viabtc.com deploy"]),
                link: "https://viabtc.com",
            },
            JSONPool {
                name: "ConnectBTC",
                addresses: Box::new(["1KPQkehgYAqwiC6UCcbojM3mbGjURrQJF2"]),
                tags: Box::new(["/ConnectBTC - Home for Miners/"]),
                link: "https://www.connectbtc.com",
            },
            JSONPool {
                name: "BATPOOL",
                addresses: Box::new(["167ApWWxUSFQmz2jdz9xop3oAKdLejvMML"]),
                tags: Box::new(["/BATPOOL/"]),
                link: "https://www.batpool.com",
            },
            JSONPool {
                name: "Waterhole",
                addresses: Box::new(["1FLH1SoLv4U68yUERhDiWzrJn5TggMqkaZ"]),
                tags: Box::new(["/WATERHOLE.IO/"]),
                link: "https://btc.waterhole.io",
            },
            JSONPool {
                name: "DCExploration",
                addresses: Box::new([]),
                tags: Box::new(["/DCExploration/"]),
                link: "https://dcexploration.cn",
            },
            JSONPool {
                name: "DCEX",
                addresses: Box::new([]),
                tags: Box::new(["/DCEX/"]),
                link: "https://dcexploration.cn",
            },
            JSONPool {
                name: "BTPOOL",
                addresses: Box::new([]),
                tags: Box::new(["/BTPOOL/"]),
                link: "",
            },
            JSONPool {
                name: "58COIN",
                addresses: Box::new(["199EDJoCpqV672qESEkfFgEqNT1iR2gj3t"]),
                tags: Box::new(["/58coin.com/"]),
                link: "https://www.58coin.com",
            },
            JSONPool {
                name: "Bitcoin India",
                addresses: Box::new(["1AZ6BkCo4zgTuuLpRStJH8iNsehXTMp456"]),
                tags: Box::new(["/Bitcoin-India/"]),
                link: "https://bitcoin-india.org",
            },
            JSONPool {
                name: "shawnp0wers",
                addresses: Box::new([
                    "12znnESiJ3bgCLftwwrg9wzQKN8fJtoBDa",
                    "18HEMWFXM9UGPVZHUMdBPD3CMFWYn2NPRX",
                ]),
                tags: Box::new(["--Nug--"]),
                link: "https://www.brainofshawn.com",
            },
            JSONPool {
                name: "PHash.IO",
                addresses: Box::new([]),
                tags: Box::new(["/phash.cn/", "/phash.io/"]),
                link: "https://phash.io",
            },
            JSONPool {
                name: "RigPool",
                addresses: Box::new(["1JpKmtspBJQVXK67DJP64eBJcAPhDvJ9Er"]),
                tags: Box::new(["/RigPool.com/"]),
                link: "https://www.rigpool.com",
            },
            JSONPool {
                name: "HAOZHUZHU",
                addresses: Box::new(["19qa95rTbDziNCS9EexUbh2hVY4viUU9tt"]),
                tags: Box::new(["/haozhuzhu/"]),
                link: "https://haozhuzhu.com",
            },
            JSONPool {
                name: "7pool",
                addresses: Box::new(["1JLc3JxvpdL1g5zoX8sKLP4BkJQiwnJftU"]),
                tags: Box::new(["/$Mined by 7pool.com/"]),
                link: "https://7pool.com",
            },
            JSONPool {
                name: "MiningKings",
                addresses: Box::new([
                    "1ApE99VM5RJzMRRtwd2JMgmkGabtJqoMEz",
                    "1EowSPumj9D9AMTpE64Jr7vT3PJDNopVcz",
                    "1KGbsDDAgJN2HDNBjmMHp9828qATo5B9c9",
                ]),
                tags: Box::new(["/mined by poopbut/"]),
                link: "https://miningkings.com",
            },
            JSONPool {
                name: "HashBX",
                addresses: Box::new([]),
                tags: Box::new(["/Mined by HashBX.io/"]),
                link: "https://hashbx.io",
            },
            JSONPool {
                name: "DPOOL",
                addresses: Box::new(["1ACAgPuFFidYzPMXbiKptSrwT74Dg8hq2v"]),
                tags: Box::new(["/DPOOL.TOP/"]),
                link: "https://www.dpool.top",
            },
            JSONPool {
                name: "Rawpool",
                addresses: Box::new([
                    "1FbBbv5oYqFKwiPm4CAqvAy8345n8AQ74b",
                    "35y82tEPDa2wm6tzkEacMG8GPPW7zbMj83",
                    "3CLigLYNkrtoNgNcUwTaKoUSHCwr9W851W",
                    "3QYvfQoG9Gs9Vfvbpw6947muSqhoGagvF6",
                    "bc1q8ej2g5uxdsg0jwl0mpl606qfjxgkyv3p29yf37",
                    "bc1qnnl503n04cqacpwvhr89qr70metxr79ht3n380",
                    "bc1qru8mtv3e3u7ms6ecjmwgeakdakclemvhnw00q9",
                    "bc1qwlrsvgtn99rqp3fgaxq6f6jkgms80rnej0a8tc",
                ]),
                tags: Box::new(["/Rawpool.com/"]),
                link: "https://www.rawpool.com",
            },
            JSONPool {
                name: "haominer",
                addresses: Box::new([]),
                tags: Box::new(["/haominer/"]),
                link: "https://haominer.com",
            },
            JSONPool {
                name: "Helix",
                addresses: Box::new([]),
                tags: Box::new(["/Helix/"]),
                link: "",
            },
            JSONPool {
                name: "Bitcoin-Ukraine",
                addresses: Box::new([]),
                tags: Box::new(["/Bitcoin-Ukraine.com.ua/"]),
                link: "https://bitcoin-ukraine.com.ua",
            },
            JSONPool {
                name: "Poolin",
                addresses: Box::new([
                    "14sA8jqYQgMRQV9zUtGFvpeMEw7YDn77SK",
                    "17tUZLvy3X2557JGhceXRiij2TNYuhRr4r",
                    "1E8CZo2S3CqWg1VZSJNFCTbtT8hZPuQ2kB",
                    "1GNgwA8JfG7Kc8akJ8opdNWJUihqUztfPe",
                    "36n452uGq1x4mK7bfyZR8wgE47AnBb2pzi",
                    "3JQSigWTCHyBLRD979JWgEtWP5YiiFwcQB",
                    "3KJrsjfg1dD6CrsTeHdHVH3KqMpvL2XWQn",
                ]),
                tags: Box::new(["/poolin.com", "/poolin/"]),
                link: "https://www.poolin.com",
            },
            JSONPool {
                name: "SecretSuperstar",
                addresses: Box::new([]),
                tags: Box::new(["/SecretSuperstar/"]),
                link: "",
            },
            JSONPool {
                name: "tigerpool.net",
                addresses: Box::new([]),
                tags: Box::new(["/tigerpool.net"]),
                link: "",
            },
            JSONPool {
                name: "Sigmapool.com",
                addresses: Box::new(["12cKiMNhCtBhZRUBCnYXo8A4WQzMUtYjmR"]),
                tags: Box::new(["/Sigmapool.com/"]),
                link: "https://sigmapool.com",
            },
            JSONPool {
                name: "okpool.top",
                addresses: Box::new([]),
                tags: Box::new(["/www.okpool.top/"]),
                link: "https://www.okpool.top",
            },
            JSONPool {
                name: "Hummerpool",
                addresses: Box::new([]),
                tags: Box::new(["HummerPool", "Hummerpool"]),
                link: "https://www.hummerpool.com",
            },
            JSONPool {
                name: "Tangpool",
                addresses: Box::new(["12Taz8FFXQ3E2AGn3ZW1SZM5bLnYGX4xR6"]),
                tags: Box::new(["/Tangpool/"]),
                link: "https://www.tangpool.com",
            },
            JSONPool {
                name: "BytePool",
                addresses: Box::new(["39m5Wvn9ZqyhYmCYpsyHuGMt5YYw4Vmh1Z"]),
                tags: Box::new(["/bytepool.com/"]),
                link: "https://www.bytepool.com",
            },
            JSONPool {
                name: "SpiderPool",
                addresses: Box::new([
                    "125m2H43pwKpSZjLhMQHneuTwTJN5qRyYu",
                    "38u1srayb1oybVB43UWKBJsrwJbdHGtPx2",
                    "1BM1sAcrfV6d4zPKytzziu4McLQDsFC2Qc",
                ]),
                tags: Box::new(["SpiderPool"]),
                link: "https://www.spiderpool.com",
            },
            JSONPool {
                name: "NovaBlock",
                addresses: Box::new(["3Bmb9Jig8A5kHdDSxvDZ6eryj3AXd3swuJ"]),
                tags: Box::new(["/NovaBlock/"]),
                link: "https://novablock.com",
            },
            JSONPool {
                name: "MiningCity",
                addresses: Box::new(["11wC5KcbgrWRBb43cwADdVrxgyF8mndVC"]),
                tags: Box::new(["MiningCity"]),
                link: "https://www.miningcity.com",
            },
            JSONPool {
                name: "Binance Pool",
                addresses: Box::new([
                    "122pN8zvqTxJaA8fRY1PDBu4QYodqE5m2X",
                    "16moWjUJVRnDQKqhoCdcszfJg9wzBdoTHw",
                    "1DSh7vX6ed2cgTeKPwufV5i4hSi4pp373h",
                    "1JvXhnHCi6XqcanvrZJ5s2Qiv4tsmm2UMy",
                    "3L8Ck6bm3sve1vJGKo6Ht2k167YKSKi8TZ",
                    "bc1qx9t2l3pyny2spqpqlye8svce70nppwtaxwdrp4",
                    "3G7jcEELKh38L6kaSV8K35pTqsh5bgZW2D",
                ]),
                tags: Box::new(["/Binance/", "binance"]),
                link: "https://pool.binance.com",
            },
            JSONPool {
                name: "Minerium",
                addresses: Box::new([]),
                tags: Box::new(["/Mined in the USA by: /Minerium.com/", "/Minerium.com/"]),
                link: "https://www.minerium.com",
            },
            JSONPool {
                name: "Lubian.com",
                addresses: Box::new(["34Jpa4Eu3ApoPVUKNTN2WeuXVVq1jzxgPi"]),
                tags: Box::new(["/Buffett/", "/lubian.com/"]),
                link: "https://www.lubian.com",
            },
            JSONPool {
                name: "OKKONG",
                addresses: Box::new(["16JHXJ7M2MubWNX9grnqbjUqJ5PHwcCWw2"]),
                tags: Box::new(["/hash.okkong.com/"]),
                link: "https://hash.okkong.com",
            },
            JSONPool {
                name: "AAO Pool",
                addresses: Box::new(["12QVFmJH2b4455YUHkMpEnWLeRY3eJ4Jb5"]),
                tags: Box::new(["/AAOPOOL/"]),
                link: "https://btc.tmspool.top",
            },
            JSONPool {
                name: "EMCDPool",
                addresses: Box::new(["1BDbsWi3Mrcjp1wdop3PWFNCNZtu4R7Hjy"]),
                tags: Box::new(["/EMCD/", "/one_more_mcd/", "get___emcd", "emcd"]),
                link: "https://pool.emcd.io",
            },
            JSONPool {
                name: "Foundry USA",
                addresses: Box::new([
                    "12KKDt4Mj7N5UAkQMN7LtPZMayenXHa8KL",
                    "1FFxkVijzvUPUeHgkFjBk2Qw8j3wQY2cDw",
                    "bc1qxhmdufsvnuaaaer4ynz88fspdsxq2h9e9cetdj",
                    "bc1p8k4v4xuz55dv49svzjg43qjxq2whur7ync9tm0xgl5t4wjl9ca9snxgmlt",
                ]),
                tags: Box::new(["/2cDw/", "Foundry USA Pool"]),
                link: "https://foundrydigital.com",
            },
            JSONPool {
                name: "SBI Crypto",
                addresses: Box::new([]),
                tags: Box::new(["/SBICrypto.com Pool/", "SBI Crypto", "SBICrypto"]),
                link: "https://sbicrypto.com",
            },
            JSONPool {
                name: "ArkPool",
                addresses: Box::new(["1QEiAhdHdMhBgVbDM7zUXWGkNhgEEJ6uLd"]),
                tags: Box::new(["/ArkPool/"]),
                link: "https://www.arkpool.com",
            },
            JSONPool {
                name: "PureBTC.COM",
                addresses: Box::new([]),
                tags: Box::new(["/PureBTC.COM/"]),
                link: "https://purebtc.com",
            },
            JSONPool {
                name: "MARA Pool",
                addresses: Box::new([
                    "15MdAHnkxt9TMC2Rj595hsg8Hnv693pPBB",
                    "1A32KFEX7JNPmU1PVjrtiXRrTQcesT3Nf1",
                ]),
                tags: Box::new(["MARA Pool", "MARA Made in USA"]),
                link: "https://marapool.com",
            },
            JSONPool {
                name: "KuCoinPool",
                addresses: Box::new(["1ArTPjj6pV3aNRhLPjJVPYoxB98VLBzUmb"]),
                tags: Box::new(["KuCoinPool"]),
                link: "https://www.kucoin.com/mining-pool",
            },
            JSONPool {
                name: "Entrust Charity Pool",
                addresses: Box::new([]),
                tags: Box::new(["Entrustus"]),
                link: "pool.entustus.org",
            },
            JSONPool {
                name: "OKMINER",
                addresses: Box::new(["15xcAZ2HfaSwYbCV6GGbasBSAekBRRC5Q2"]),
                tags: Box::new(["okminer.com/euz"]),
                link: "https://okminer.com",
            },
            JSONPool {
                name: "Titan",
                addresses: Box::new(["14hLEtxozmmih6Gg5xrGZLfx51bEMj21NW"]),
                tags: Box::new(["Titan.io"]),
                link: "https://titan.io",
            },
            JSONPool {
                name: "PEGA Pool",
                addresses: Box::new(["1BGFwRzjCfRR7EvRHnzfHyFjGR8XiBDFKa"]),
                tags: Box::new(["/pegapool/"]),
                link: "https://www.pega-pool.com",
            },
            JSONPool {
                name: "BTC Nuggets",
                addresses: Box::new(["1BwZeHJo7b7M2op7VDfYnsmcpXsUYEcVHm"]),
                tags: Box::new([]),
                link: "https://104.197.8.250",
            },
            JSONPool {
                name: "CloudHashing",
                addresses: Box::new(["1ALA5v7h49QT7WYLcRsxcXqXUqEqaWmkvw"]),
                tags: Box::new([]),
                link: "https://cloudhashing.com",
            },
            JSONPool {
                name: "digitalX Mintsy",
                addresses: Box::new(["1NY15MK947MLzmPUa2gL7UgyR8prLh2xfu"]),
                tags: Box::new([]),
                link: "https://www.mintsy.co",
            },
            JSONPool {
                name: "Telco 214",
                addresses: Box::new([
                    "13Sd8Y7nUao3z4bJFkZvCRXpFqHvLy49YY",
                    "14M1pQ5KKeqmDrmqKyZEnaxAGJfBPrfWvQ",
                    "18hvMLisvfc58PvA5rHH7NsLN9CV5ddB2x",
                    "18ikmzPqk721ZNvWhDos1UL4H29w352Kj5",
                    "1AsEJU4ht5wR7BzV6xsNQpwi5qRx4qH1ac",
                    "1BUhwvF9oo3qkaSjjPpWrUzQxXNjkHdMZF",
                    "1CNq2FAw6S5JfBiDkjkYJUVNQwjoeY4Zfi",
                    "1DXRoTT67mCbhdHHL1it4J1xsSZHHnFxYR",
                    "1GaKSh2t396nfSg5Ku2J3Yn1vfVsXrGuH5",
                    "1LXWA3EEEwPixQcyFWXKX2hWHpkDoLknZW",
                    "1MoYfV4U61wqTPTHCyedzFmvf2o3uys2Ua",
                    "1P4B6rx1js8TaEDXvZvtrkiEb9XrJgMQ19",
                ]),
                tags: Box::new([]),
                link: "https://www.telco214.com",
            },
            JSONPool {
                name: "BTC Pool Party",
                addresses: Box::new(["1PmRrdp1YSkp1LxPyCfcmBHDEipG5X4eJB"]),
                tags: Box::new([]),
                link: "https://btcpoolparty.com",
            },
            JSONPool {
                name: "Multipool",
                addresses: Box::new(["1MeffGLauEj2CZ18hRQqUauTXb9JAuLbGw"]),
                tags: Box::new([]),
                link: "https://www.multipool.us",
            },
            JSONPool {
                name: "transactioncoinmining",
                addresses: Box::new(["1qtKetXKgqa7j1KrB19HbvfRiNUncmakk"]),
                tags: Box::new([]),
                link: "https://sha256.transactioncoinmining.com",
            },
            JSONPool {
                name: "BTCDig",
                addresses: Box::new(["15MxzsutVroEE9XiDckLxUHTCDAEZgPZJi"]),
                tags: Box::new([]),
                link: "https://btcdig.com",
            },
            JSONPool {
                name: "Tricky's BTC Pool",
                addresses: Box::new(["1AePMyovoijxvHuKhTqWvpaAkRCF4QswC6"]),
                tags: Box::new([]),
                link: "https://pool.wemine.uk",
            },
            JSONPool {
                name: "BTCMP",
                addresses: Box::new(["1jKSjMLnDNup6NPgCjveeP9tUn4YpT94Y"]),
                tags: Box::new([]),
                link: "https://www.btcmp.com",
            },
            JSONPool {
                name: "Eobot",
                addresses: Box::new([
                    "16GsNC3q6KgVXkUX7j7aPxSUdHrt1sN2yN",
                    "1MPxhNkSzeTNTHSZAibMaS8HS1esmUL1ne",
                ]),
                tags: Box::new([]),
                link: "https://eobot.com",
            },
            JSONPool {
                name: "UNOMP",
                addresses: Box::new(["1BRY8AD7vSNUEE75NjzfgiG18mWjGQSRuJ"]),
                tags: Box::new([]),
                link: "https://199.115.116.7:8925",
            },
            JSONPool {
                name: "Patels",
                addresses: Box::new([
                    "197miJmttpCt2ubVs6DDtGBYFDroxHmvVB",
                    "19RE4mz2UbDxDVougc6GGdoT4x5yXxwFq2",
                ]),
                tags: Box::new([]),
                link: "https://patelsminingpool.com",
            },
            JSONPool {
                name: "GoGreenLight",
                addresses: Box::new(["18EPLvrs2UE11kWBB3ABS7Crwj5tTBYPoa"]),
                tags: Box::new([]),
                link: "https://www.gogreenlight.se",
            },
            JSONPool {
                name: "EkanemBTC",
                addresses: Box::new(["1Cs5RT9SRk1hxsdzivAfkjesNmVVJqfqkw"]),
                tags: Box::new([]),
                link: "https://ekanembtc.com",
            },
            JSONPool {
                name: "CANOE",
                addresses: Box::new(["1Afcpc2FpPnREU6i52K3cicmHdvYRAH9Wo"]),
                tags: Box::new([]),
                link: "https://www.canoepool.com",
            },
            JSONPool {
                name: "tiger",
                addresses: Box::new(["1LsFmhnne74EmU4q4aobfxfrWY4wfMVd8w"]),
                tags: Box::new([]),
                link: "",
            },
            JSONPool {
                name: "1M1X",
                addresses: Box::new(["1M1Xw2rczxkF3p3wiNHaTmxvbpZZ7M6vaa"]),
                tags: Box::new([]),
                link: "",
            },
            JSONPool {
                name: "Zulupool",
                addresses: Box::new(["1ZULUPooLEQfkrTgynLV4uHyMgQYx71ip"]),
                tags: Box::new(["ZULUPooL", "ZU_test"]),
                link: "https://beta.zulupool.com/",
            },
            JSONPool {
                name: "SECPOOL",
                addresses: Box::new(["3Awm3FNpmwrbvAFVThRUFqgpbVuqWisni9"]),
                tags: Box::new(["SecPool"]),
                link: "https://www.secpool.com",
            },
            JSONPool {
                name: "OCEAN",
                addresses: Box::new(["37dvwZZoT3D7RXpTCpN2yKzMmNs2i2Fd1n"]),
                tags: Box::new(["OCEAN.XYZ"]),
                link: "https://ocean.xyz/",
            },
            JSONPool {
                name: "WhitePool",
                addresses: Box::new(["14VkxDwSAUWrzYTxV49HnYhKLWTJ3pCoUS"]),
                tags: Box::new(["WhitePool"]),
                link: "https://whitebit.com/mining-pool",
            },
            JSONPool {
                name: "wk057",
                addresses: Box::new(["1WizkidqARMLvjGUpfDQFRcEbnHpL55kK"]),
                tags: Box::new(["wizkid057's block"]),
                link: "",
            },
            JSONPool {
                name: "FutureBit Apollo Solo",
                addresses: Box::new([]),
                tags: Box::new(["Apollo", "/mined by a Solo FutureBit Apollo/"]),
                link: "https://www.futurebit.io",
            },
            JSONPool {
                name: "Carbon Negative",
                addresses: Box::new([
                    "33SAB6pzbhEGPbfY6NVgRDV7jVfspZ3A3Z",
                    "3KZDwmJHB6QJ13QPXHaW7SS3yTESFPZoxb",
                ]),
                tags: Box::new([]),
                link: "https://github.com/bitcoin-data/mining-pools/issues/48",
            },
            JSONPool {
                name: "Portland.HODL",
                addresses: Box::new([]),
                tags: Box::new(["Portland.HODL"]),
                link: "",
            },
            JSONPool {
                name: "Phoenix",
                addresses: Box::new([
                    "37cGvBD4qufoZQHopGS7XstxRUzx5cNuy1",
                    "bc1q2zcenaujmmdv8sqgf723cug4vjnphkvvf8zpst",
                    "1Ld6okoaLNDbSnougAyQTrchxRn9ELnTJg",
                ]),
                tags: Box::new(["/Phoenix/"]),
                link: "https://phoenixpool.com",
            },
            JSONPool {
                name: "Neopool",
                addresses: Box::new(["1HCAb2h89bUinm6QZrAPpfbk4ySBrT2V4w"]),
                tags: Box::new(["/Neopool/"]),
                link: "https://neopool.com/",
            },
            JSONPool {
                name: "MaxiPool",
                addresses: Box::new(["36r3YqAXWpyqNcczjCBdHrYZ3m8X56WDzx"]),
                tags: Box::new(["/MaxiPool/"]),
                link: "https://maxipool.org/",
            },
            JSONPool {
                name: "BitFuFuPool",
                addresses: Box::new(["3JP3zF7LoeoAotqkNGdvX5szUyNPwd937d"]),
                tags: Box::new(["/BitFuFu/"]),
                link: "https://www.bitfufu.com/pool",
            },
            JSONPool {
                name: "luckyPool",
                addresses: Box::new(["1DnPPFQPrfyNTiHPXhDFyqNnW9T62GEhB1"]),
                tags: Box::new(["Lucky pool"]),
                link: "",
            },
            JSONPool {
                name: "Mining-Dutch",
                addresses: Box::new(["1AfPSq5ZbqBaxU5QAayLQJMcXV8HZt92eq"]),
                tags: Box::new(["/Mining-Dutch/"]),
                link: "https://www.mining-dutch.nl/",
            },
            JSONPool {
                name: "Public Pool",
                addresses: Box::new([]),
                tags: Box::new(["Public-Pool", "Public Pool on Umbrel"]),
                link: "https://web.public-pool.io/",
            },
            JSONPool {
                name: "Mining Squared",
                addresses: Box::new([
                    "3GdjWJdkJhtkxRZ3Ns1LstaoHNMBW8XsvU",
                    "3AvXzTUat4p6Qf6ZLnRNvB3mDLjp3fNmjJ",
                ]),
                tags: Box::new(["MiningSquared", "BSquared Network", "/bsquared/"]),
                link: "https://pool.bsquared.network/",
            },
            JSONPool {
                name: "Innopolis Tech",
                addresses: Box::new(["bc1q75t4wewkmf3l9qg097zvtlh05v5pdz6699kv8k"]),
                tags: Box::new(["Innopolis", "Innopolis.tech"]),
                link: "https://innopolis.tech/",
            },
            JSONPool {
                name: "BTCLab",
                addresses: Box::new([]),
                tags: Box::new(["BTCLab", "BTCLab.dev"]),
                link: "https://btclab.dev/",
            },
            JSONPool {
                name: "Parasite",
                addresses: Box::new([]),
                tags: Box::new(["parasite"]),
                link: "https://parasite.space",
            },
        ])
    })
}

impl From<[JSONPool; POOL_COUNT]> for Pools {
    #[inline]
    fn from(value: [JSONPool; POOL_COUNT]) -> Self {
        Pools(
            value
                .into_iter()
                .enumerate()
                .map(|tuple| tuple.into())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        )
    }
}
