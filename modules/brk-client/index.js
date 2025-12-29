// Auto-generated BRK JavaScript client
// Do not edit manually

// Constants

export const VERSION = "v0.1.0-alpha.1";

export const INDEXES = /** @type {const} */ ([
  "dateindex",
  "decadeindex",
  "difficultyepoch",
  "emptyoutputindex",
  "halvingepoch",
  "height",
  "txinindex",
  "monthindex",
  "opreturnindex",
  "txoutindex",
  "p2aaddressindex",
  "p2msoutputindex",
  "p2pk33addressindex",
  "p2pk65addressindex",
  "p2pkhaddressindex",
  "p2shaddressindex",
  "p2traddressindex",
  "p2wpkhaddressindex",
  "p2wshaddressindex",
  "quarterindex",
  "semesterindex",
  "txindex",
  "unknownoutputindex",
  "weekindex",
  "yearindex",
  "loadedaddressindex",
  "emptyaddressindex",
]);

export const POOL_ID_TO_POOL_NAME = /** @type {const} */ ({
  pool175btc: "175btc",
  onehash: "1Hash",
  onem1x: "1M1X",
  onethash: "1THash",
  twentyoneinc: "21 Inc.",
  pool50btc: "50BTC",
  fiftyeightcoin: "58COIN",
  sevenpool: "7pool",
  eightbaochi: "8baochi",
  axbt: "A-XBT",
  aaopool: "AAO Pool",
  antpool: "AntPool",
  arkpool: "ArkPool",
  asicminer: "ASICMiner",
  batpool: "BATPOOL",
  bcmonster: "BCMonster",
  bcpoolio: "bcpool.io",
  binancepool: "Binance Pool",
  bitalo: "Bitalo",
  bitclub: "BitClub",
  bitcoinaffiliatenetwork: "Bitcoin Affiliate Network",
  bitcoinindia: "Bitcoin India",
  bitcoinukraine: "Bitcoin-Ukraine",
  bitcoincom: "Bitcoin.com",
  bitcoinrussia: "BitcoinRussia",
  bitfarms: "Bitfarms",
  bitfufupool: "BitFuFuPool",
  bitfury: "BitFury",
  bitminter: "BitMinter",
  bitparking: "Bitparking",
  bitsolo: "Bitsolo",
  bixin: "Bixin",
  blockfills: "BlockFills",
  braiinspool: "Braiins Pool",
  bravomining: "Bravo Mining",
  btcguild: "BTC Guild",
  btcnuggets: "BTC Nuggets",
  btcpoolparty: "BTC Pool Party",
  btccom: "BTC.com",
  btctop: "BTC.TOP",
  btcc: "BTCC",
  btcdig: "BTCDig",
  btclab: "BTCLab",
  btcmp: "BTCMP",
  btcserv: "BTCServ",
  btpool: "BTPOOL",
  bwpool: "BWPool",
  bytepool: "BytePool",
  canoe: "CANOE",
  canoepool: "CanoePool",
  carbonnegative: "Carbon Negative",
  ckpool: "CKPool",
  cloudhashing: "CloudHashing",
  coinlab: "CoinLab",
  cointerra: "Cointerra",
  connectbtc: "ConnectBTC",
  dcex: "DCEX",
  dcexploration: "DCExploration",
  digitalbtc: "digitalBTC",
  digitalxmintsy: "digitalX Mintsy",
  dpool: "DPOOL",
  eclipsemc: "EclipseMC",
  ekanembtc: "EkanemBTC",
  eligius: "Eligius",
  emcdpool: "EMCDPool",
  entrustcharitypool: "Entrust Charity Pool",
  eobot: "Eobot",
  exxbw: "EXX&BW",
  f2pool: "F2Pool",
  foundryusa: "Foundry USA",
  futurebitapollosolo: "FutureBit Apollo Solo",
  gbminers: "GBMiners",
  ghashio: "GHash.IO",
  givemecoins: "Give Me Coins",
  gogreenlight: "GoGreenLight",
  haominer: "haominer",
  haozhuzhu: "HAOZHUZHU",
  hashbx: "HashBX",
  hashpool: "HASHPOOL",
  helix: "Helix",
  hhtt: "HHTT",
  hotpool: "HotPool",
  hummerpool: "Hummerpool",
  huobipool: "Huobi.pool",
  innopolistech: "Innopolis Tech",
  kanopool: "KanoPool",
  kncminer: "KnCMiner",
  kucoinpool: "KuCoinPool",
  lubiancom: "Lubian.com",
  luckypool: "luckyPool",
  luxor: "Luxor",
  marapool: "MARA Pool",
  maxbtc: "MaxBTC",
  maxipool: "MaxiPool",
  megabigpower: "MegaBigPower",
  minerium: "Minerium",
  miningsquared: "Mining Squared",
  miningdutch: "Mining-Dutch",
  miningcity: "MiningCity",
  miningkings: "MiningKings",
  mmpool: "mmpool",
  mtred: "Mt Red",
  multicoinco: "MultiCoin.co",
  multipool: "Multipool",
  mybtccoinpool: "myBTCcoin Pool",
  neopool: "Neopool",
  nexious: "Nexious",
  nicehash: "NiceHash",
  nmcbit: "NMCbit",
  novablock: "NovaBlock",
  ocean: "OCEAN",
  okexpool: "OKExPool",
  okkong: "OKKONG",
  okminer: "OKMINER",
  okpooltop: "okpool.top",
  ozcoin: "OzCoin",
  parasite: "Parasite",
  patels: "Patels",
  pegapool: "PEGA Pool",
  phashio: "PHash.IO",
  phoenix: "Phoenix",
  polmine: "Polmine",
  poolin: "Poolin",
  portlandhodl: "Portland.HODL",
  publicpool: "Public Pool",
  purebtccom: "PureBTC.COM",
  rawpool: "Rawpool",
  rigpool: "RigPool",
  sbicrypto: "SBI Crypto",
  secpool: "SECPOOL",
  secretsuperstar: "SecretSuperstar",
  shawnp0wers: "shawnp0wers",
  sigmapoolcom: "Sigmapool.com",
  simplecoinus: "simplecoin.us",
  solock: "Solo CK",
  spiderpool: "SpiderPool",
  stminingcorp: "ST Mining Corp",
  tangpool: "Tangpool",
  tatmaspool: "TATMAS Pool",
  tbdice: "TBDice",
  telco214: "Telco 214",
  terrapool: "Terra Pool",
  tiger: "tiger",
  tigerpoolnet: "tigerpool.net",
  titan: "Titan",
  transactioncoinmining: "transactioncoinmining",
  trickysbtcpool: "Tricky's BTC Pool",
  triplemining: "TripleMining",
  ultimuspool: "ULTIMUSPOOL",
  unknown: "Unknown",
  unomp: "UNOMP",
  viabtc: "ViaBTC",
  waterhole: "Waterhole",
  wayicn: "WAYI.CN",
  whitepool: "WhitePool",
  wk057: "wk057",
  yourbtcnet: "Yourbtc.net",
  zulupool: "Zulupool",
});

// Cohort names

export const TERM_NAMES = /** @type {const} */ ({
  "short": {
    "id": "sth",
    "short": "STH",
    "long": "Short Term Holders"
  },
  "long": {
    "id": "lth",
    "short": "LTH",
    "long": "Long Term Holders"
  }
});

export const EPOCH_NAMES = /** @type {const} */ ({
  "_0": {
    "id": "epoch_0",
    "short": "Epoch 0",
    "long": "Epoch 0"
  },
  "_1": {
    "id": "epoch_1",
    "short": "Epoch 1",
    "long": "Epoch 1"
  },
  "_2": {
    "id": "epoch_2",
    "short": "Epoch 2",
    "long": "Epoch 2"
  },
  "_3": {
    "id": "epoch_3",
    "short": "Epoch 3",
    "long": "Epoch 3"
  },
  "_4": {
    "id": "epoch_4",
    "short": "Epoch 4",
    "long": "Epoch 4"
  }
});

export const YEAR_NAMES = /** @type {const} */ ({
  "_2009": {
    "id": "year_2009",
    "short": "2009",
    "long": "Year 2009"
  },
  "_2010": {
    "id": "year_2010",
    "short": "2010",
    "long": "Year 2010"
  },
  "_2011": {
    "id": "year_2011",
    "short": "2011",
    "long": "Year 2011"
  },
  "_2012": {
    "id": "year_2012",
    "short": "2012",
    "long": "Year 2012"
  },
  "_2013": {
    "id": "year_2013",
    "short": "2013",
    "long": "Year 2013"
  },
  "_2014": {
    "id": "year_2014",
    "short": "2014",
    "long": "Year 2014"
  },
  "_2015": {
    "id": "year_2015",
    "short": "2015",
    "long": "Year 2015"
  },
  "_2016": {
    "id": "year_2016",
    "short": "2016",
    "long": "Year 2016"
  },
  "_2017": {
    "id": "year_2017",
    "short": "2017",
    "long": "Year 2017"
  },
  "_2018": {
    "id": "year_2018",
    "short": "2018",
    "long": "Year 2018"
  },
  "_2019": {
    "id": "year_2019",
    "short": "2019",
    "long": "Year 2019"
  },
  "_2020": {
    "id": "year_2020",
    "short": "2020",
    "long": "Year 2020"
  },
  "_2021": {
    "id": "year_2021",
    "short": "2021",
    "long": "Year 2021"
  },
  "_2022": {
    "id": "year_2022",
    "short": "2022",
    "long": "Year 2022"
  },
  "_2023": {
    "id": "year_2023",
    "short": "2023",
    "long": "Year 2023"
  },
  "_2024": {
    "id": "year_2024",
    "short": "2024",
    "long": "Year 2024"
  },
  "_2025": {
    "id": "year_2025",
    "short": "2025",
    "long": "Year 2025"
  },
  "_2026": {
    "id": "year_2026",
    "short": "2026",
    "long": "Year 2026"
  }
});

export const SPENDABLE_TYPE_NAMES = /** @type {const} */ ({
  "p2pk65": {
    "id": "p2pk65",
    "short": "P2PK65",
    "long": "Pay to Public Key (65 bytes)"
  },
  "p2pk33": {
    "id": "p2pk33",
    "short": "P2PK33",
    "long": "Pay to Public Key (33 bytes)"
  },
  "p2pkh": {
    "id": "p2pkh",
    "short": "P2PKH",
    "long": "Pay to Public Key Hash"
  },
  "p2ms": {
    "id": "p2ms",
    "short": "P2MS",
    "long": "Pay to Multisig"
  },
  "p2sh": {
    "id": "p2sh",
    "short": "P2SH",
    "long": "Pay to Script Hash"
  },
  "p2wpkh": {
    "id": "p2wpkh",
    "short": "P2WPKH",
    "long": "Pay to Witness Public Key Hash"
  },
  "p2wsh": {
    "id": "p2wsh",
    "short": "P2WSH",
    "long": "Pay to Witness Script Hash"
  },
  "p2tr": {
    "id": "p2tr",
    "short": "P2TR",
    "long": "Pay to Taproot"
  },
  "p2a": {
    "id": "p2a",
    "short": "P2A",
    "long": "Pay to Anchor"
  },
  "unknown": {
    "id": "unknown_outputs",
    "short": "Unknown",
    "long": "Unknown Output Type"
  },
  "empty": {
    "id": "empty_outputs",
    "short": "Empty",
    "long": "Empty Output"
  }
});

export const AGE_RANGE_NAMES = /** @type {const} */ ({
  "up_to_1d": {
    "id": "up_to_1d_old",
    "short": "<1d",
    "long": "Up to 1 Day Old"
  },
  "_1d_to_1w": {
    "id": "at_least_1d_up_to_1w_old",
    "short": "1d-1w",
    "long": "1 Day to 1 Week Old"
  },
  "_1w_to_1m": {
    "id": "at_least_1w_up_to_1m_old",
    "short": "1w-1m",
    "long": "1 Week to 1 Month Old"
  },
  "_1m_to_2m": {
    "id": "at_least_1m_up_to_2m_old",
    "short": "1m-2m",
    "long": "1 to 2 Months Old"
  },
  "_2m_to_3m": {
    "id": "at_least_2m_up_to_3m_old",
    "short": "2m-3m",
    "long": "2 to 3 Months Old"
  },
  "_3m_to_4m": {
    "id": "at_least_3m_up_to_4m_old",
    "short": "3m-4m",
    "long": "3 to 4 Months Old"
  },
  "_4m_to_5m": {
    "id": "at_least_4m_up_to_5m_old",
    "short": "4m-5m",
    "long": "4 to 5 Months Old"
  },
  "_5m_to_6m": {
    "id": "at_least_5m_up_to_6m_old",
    "short": "5m-6m",
    "long": "5 to 6 Months Old"
  },
  "_6m_to_1y": {
    "id": "at_least_6m_up_to_1y_old",
    "short": "6m-1y",
    "long": "6 Months to 1 Year Old"
  },
  "_1y_to_2y": {
    "id": "at_least_1y_up_to_2y_old",
    "short": "1y-2y",
    "long": "1 to 2 Years Old"
  },
  "_2y_to_3y": {
    "id": "at_least_2y_up_to_3y_old",
    "short": "2y-3y",
    "long": "2 to 3 Years Old"
  },
  "_3y_to_4y": {
    "id": "at_least_3y_up_to_4y_old",
    "short": "3y-4y",
    "long": "3 to 4 Years Old"
  },
  "_4y_to_5y": {
    "id": "at_least_4y_up_to_5y_old",
    "short": "4y-5y",
    "long": "4 to 5 Years Old"
  },
  "_5y_to_6y": {
    "id": "at_least_5y_up_to_6y_old",
    "short": "5y-6y",
    "long": "5 to 6 Years Old"
  },
  "_6y_to_7y": {
    "id": "at_least_6y_up_to_7y_old",
    "short": "6y-7y",
    "long": "6 to 7 Years Old"
  },
  "_7y_to_8y": {
    "id": "at_least_7y_up_to_8y_old",
    "short": "7y-8y",
    "long": "7 to 8 Years Old"
  },
  "_8y_to_10y": {
    "id": "at_least_8y_up_to_10y_old",
    "short": "8y-10y",
    "long": "8 to 10 Years Old"
  },
  "_10y_to_12y": {
    "id": "at_least_10y_up_to_12y_old",
    "short": "10y-12y",
    "long": "10 to 12 Years Old"
  },
  "_12y_to_15y": {
    "id": "at_least_12y_up_to_15y_old",
    "short": "12y-15y",
    "long": "12 to 15 Years Old"
  },
  "from_15y": {
    "id": "at_least_15y_old",
    "short": "15y+",
    "long": "15+ Years Old"
  }
});

export const MAX_AGE_NAMES = /** @type {const} */ ({
  "_1w": {
    "id": "up_to_1w_old",
    "short": "<1w",
    "long": "Up to 1 Week Old"
  },
  "_1m": {
    "id": "up_to_1m_old",
    "short": "<1m",
    "long": "Up to 1 Month Old"
  },
  "_2m": {
    "id": "up_to_2m_old",
    "short": "<2m",
    "long": "Up to 2 Months Old"
  },
  "_3m": {
    "id": "up_to_3m_old",
    "short": "<3m",
    "long": "Up to 3 Months Old"
  },
  "_4m": {
    "id": "up_to_4m_old",
    "short": "<4m",
    "long": "Up to 4 Months Old"
  },
  "_5m": {
    "id": "up_to_5m_old",
    "short": "<5m",
    "long": "Up to 5 Months Old"
  },
  "_6m": {
    "id": "up_to_6m_old",
    "short": "<6m",
    "long": "Up to 6 Months Old"
  },
  "_1y": {
    "id": "up_to_1y_old",
    "short": "<1y",
    "long": "Up to 1 Year Old"
  },
  "_2y": {
    "id": "up_to_2y_old",
    "short": "<2y",
    "long": "Up to 2 Years Old"
  },
  "_3y": {
    "id": "up_to_3y_old",
    "short": "<3y",
    "long": "Up to 3 Years Old"
  },
  "_4y": {
    "id": "up_to_4y_old",
    "short": "<4y",
    "long": "Up to 4 Years Old"
  },
  "_5y": {
    "id": "up_to_5y_old",
    "short": "<5y",
    "long": "Up to 5 Years Old"
  },
  "_6y": {
    "id": "up_to_6y_old",
    "short": "<6y",
    "long": "Up to 6 Years Old"
  },
  "_7y": {
    "id": "up_to_7y_old",
    "short": "<7y",
    "long": "Up to 7 Years Old"
  },
  "_8y": {
    "id": "up_to_8y_old",
    "short": "<8y",
    "long": "Up to 8 Years Old"
  },
  "_10y": {
    "id": "up_to_10y_old",
    "short": "<10y",
    "long": "Up to 10 Years Old"
  },
  "_12y": {
    "id": "up_to_12y_old",
    "short": "<12y",
    "long": "Up to 12 Years Old"
  },
  "_15y": {
    "id": "up_to_15y_old",
    "short": "<15y",
    "long": "Up to 15 Years Old"
  }
});

export const MIN_AGE_NAMES = /** @type {const} */ ({
  "_1d": {
    "id": "at_least_1d_old",
    "short": "1d+",
    "long": "At Least 1 Day Old"
  },
  "_1w": {
    "id": "at_least_1w_old",
    "short": "1w+",
    "long": "At Least 1 Week Old"
  },
  "_1m": {
    "id": "at_least_1m_old",
    "short": "1m+",
    "long": "At Least 1 Month Old"
  },
  "_2m": {
    "id": "at_least_2m_old",
    "short": "2m+",
    "long": "At Least 2 Months Old"
  },
  "_3m": {
    "id": "at_least_3m_old",
    "short": "3m+",
    "long": "At Least 3 Months Old"
  },
  "_4m": {
    "id": "at_least_4m_old",
    "short": "4m+",
    "long": "At Least 4 Months Old"
  },
  "_5m": {
    "id": "at_least_5m_old",
    "short": "5m+",
    "long": "At Least 5 Months Old"
  },
  "_6m": {
    "id": "at_least_6m_old",
    "short": "6m+",
    "long": "At Least 6 Months Old"
  },
  "_1y": {
    "id": "at_least_1y_old",
    "short": "1y+",
    "long": "At Least 1 Year Old"
  },
  "_2y": {
    "id": "at_least_2y_old",
    "short": "2y+",
    "long": "At Least 2 Years Old"
  },
  "_3y": {
    "id": "at_least_3y_old",
    "short": "3y+",
    "long": "At Least 3 Years Old"
  },
  "_4y": {
    "id": "at_least_4y_old",
    "short": "4y+",
    "long": "At Least 4 Years Old"
  },
  "_5y": {
    "id": "at_least_5y_old",
    "short": "5y+",
    "long": "At Least 5 Years Old"
  },
  "_6y": {
    "id": "at_least_6y_old",
    "short": "6y+",
    "long": "At Least 6 Years Old"
  },
  "_7y": {
    "id": "at_least_7y_old",
    "short": "7y+",
    "long": "At Least 7 Years Old"
  },
  "_8y": {
    "id": "at_least_8y_old",
    "short": "8y+",
    "long": "At Least 8 Years Old"
  },
  "_10y": {
    "id": "at_least_10y_old",
    "short": "10y+",
    "long": "At Least 10 Years Old"
  },
  "_12y": {
    "id": "at_least_12y_old",
    "short": "12y+",
    "long": "At Least 12 Years Old"
  }
});

export const AMOUNT_RANGE_NAMES = /** @type {const} */ ({
  "_0sats": {
    "id": "with_0sats",
    "short": "0 sats",
    "long": "0 Sats"
  },
  "_1sat_to_10sats": {
    "id": "above_1sat_under_10sats",
    "short": "1-10 sats",
    "long": "1 to 10 Sats"
  },
  "_10sats_to_100sats": {
    "id": "above_10sats_under_100sats",
    "short": "10-100 sats",
    "long": "10 to 100 Sats"
  },
  "_100sats_to_1k_sats": {
    "id": "above_100sats_under_1k_sats",
    "short": "100-1k sats",
    "long": "100 to 1K Sats"
  },
  "_1k_sats_to_10k_sats": {
    "id": "above_1k_sats_under_10k_sats",
    "short": "1k-10k sats",
    "long": "1K to 10K Sats"
  },
  "_10k_sats_to_100k_sats": {
    "id": "above_10k_sats_under_100k_sats",
    "short": "10k-100k sats",
    "long": "10K to 100K Sats"
  },
  "_100k_sats_to_1m_sats": {
    "id": "above_100k_sats_under_1m_sats",
    "short": "100k-1M sats",
    "long": "100K to 1M Sats"
  },
  "_1m_sats_to_10m_sats": {
    "id": "above_1m_sats_under_10m_sats",
    "short": "1M-10M sats",
    "long": "1M to 10M Sats"
  },
  "_10m_sats_to_1btc": {
    "id": "above_10m_sats_under_1btc",
    "short": "0.1-1 BTC",
    "long": "0.1 to 1 BTC"
  },
  "_1btc_to_10btc": {
    "id": "above_1btc_under_10btc",
    "short": "1-10 BTC",
    "long": "1 to 10 BTC"
  },
  "_10btc_to_100btc": {
    "id": "above_10btc_under_100btc",
    "short": "10-100 BTC",
    "long": "10 to 100 BTC"
  },
  "_100btc_to_1k_btc": {
    "id": "above_100btc_under_1k_btc",
    "short": "100-1k BTC",
    "long": "100 to 1K BTC"
  },
  "_1k_btc_to_10k_btc": {
    "id": "above_1k_btc_under_10k_btc",
    "short": "1k-10k BTC",
    "long": "1K to 10K BTC"
  },
  "_10k_btc_to_100k_btc": {
    "id": "above_10k_btc_under_100k_btc",
    "short": "10k-100k BTC",
    "long": "10K to 100K BTC"
  },
  "_100k_btc_or_more": {
    "id": "above_100k_btc",
    "short": "100k+ BTC",
    "long": "100K+ BTC"
  }
});

export const GE_AMOUNT_NAMES = /** @type {const} */ ({
  "_1sat": {
    "id": "above_1sat",
    "short": "1+ sats",
    "long": "Above 1 Sat"
  },
  "_10sats": {
    "id": "above_10sats",
    "short": "10+ sats",
    "long": "Above 10 Sats"
  },
  "_100sats": {
    "id": "above_100sats",
    "short": "100+ sats",
    "long": "Above 100 Sats"
  },
  "_1k_sats": {
    "id": "above_1k_sats",
    "short": "1k+ sats",
    "long": "Above 1K Sats"
  },
  "_10k_sats": {
    "id": "above_10k_sats",
    "short": "10k+ sats",
    "long": "Above 10K Sats"
  },
  "_100k_sats": {
    "id": "above_100k_sats",
    "short": "100k+ sats",
    "long": "Above 100K Sats"
  },
  "_1m_sats": {
    "id": "above_1m_sats",
    "short": "1M+ sats",
    "long": "Above 1M Sats"
  },
  "_10m_sats": {
    "id": "above_10m_sats",
    "short": "0.1+ BTC",
    "long": "Above 0.1 BTC"
  },
  "_1btc": {
    "id": "above_1btc",
    "short": "1+ BTC",
    "long": "Above 1 BTC"
  },
  "_10btc": {
    "id": "above_10btc",
    "short": "10+ BTC",
    "long": "Above 10 BTC"
  },
  "_100btc": {
    "id": "above_100btc",
    "short": "100+ BTC",
    "long": "Above 100 BTC"
  },
  "_1k_btc": {
    "id": "above_1k_btc",
    "short": "1k+ BTC",
    "long": "Above 1K BTC"
  },
  "_10k_btc": {
    "id": "above_10k_btc",
    "short": "10k+ BTC",
    "long": "Above 10K BTC"
  }
});

export const LT_AMOUNT_NAMES = /** @type {const} */ ({
  "_10sats": {
    "id": "under_10sats",
    "short": "<10 sats",
    "long": "Under 10 Sats"
  },
  "_100sats": {
    "id": "under_100sats",
    "short": "<100 sats",
    "long": "Under 100 Sats"
  },
  "_1k_sats": {
    "id": "under_1k_sats",
    "short": "<1k sats",
    "long": "Under 1K Sats"
  },
  "_10k_sats": {
    "id": "under_10k_sats",
    "short": "<10k sats",
    "long": "Under 10K Sats"
  },
  "_100k_sats": {
    "id": "under_100k_sats",
    "short": "<100k sats",
    "long": "Under 100K Sats"
  },
  "_1m_sats": {
    "id": "under_1m_sats",
    "short": "<1M sats",
    "long": "Under 1M Sats"
  },
  "_10m_sats": {
    "id": "under_10m_sats",
    "short": "<0.1 BTC",
    "long": "Under 0.1 BTC"
  },
  "_1btc": {
    "id": "under_1btc",
    "short": "<1 BTC",
    "long": "Under 1 BTC"
  },
  "_10btc": {
    "id": "under_10btc",
    "short": "<10 BTC",
    "long": "Under 10 BTC"
  },
  "_100btc": {
    "id": "under_100btc",
    "short": "<100 BTC",
    "long": "Under 100 BTC"
  },
  "_1k_btc": {
    "id": "under_1k_btc",
    "short": "<1k BTC",
    "long": "Under 1K BTC"
  },
  "_10k_btc": {
    "id": "under_10k_btc",
    "short": "<10k BTC",
    "long": "Under 10K BTC"
  },
  "_100k_btc": {
    "id": "under_100k_btc",
    "short": "<100k BTC",
    "long": "Under 100K BTC"
  }
});

// Type definitions

/** @typedef {string} Address */
/**
 * @typedef {Object} AddressChainStats
 * @property {number} fundedTxoCount
 * @property {Sats} fundedTxoSum
 * @property {number} spentTxoCount
 * @property {Sats} spentTxoSum
 * @property {number} txCount
 * @property {TypeIndex} typeIndex
 */
/**
 * @typedef {Object} AddressMempoolStats
 * @property {number} fundedTxoCount
 * @property {Sats} fundedTxoSum
 * @property {number} spentTxoCount
 * @property {Sats} spentTxoSum
 * @property {number} txCount
 */
/**
 * @typedef {Object} AddressParam
 * @property {Address} address
 */
/**
 * @typedef {Object} AddressStats
 * @property {Address} address
 * @property {AddressChainStats} chainStats
 * @property {(AddressMempoolStats|null)=} mempoolStats
 */
/**
 * @typedef {Object} AddressTxidsParam
 * @property {(Txid|null)=} afterTxid
 * @property {number=} limit
 */
/**
 * @typedef {Object} AddressValidation
 * @property {boolean} isvalid
 * @property {?string=} address
 * @property {?string=} scriptPubKey
 * @property {?boolean=} isscript
 * @property {?boolean=} iswitness
 * @property {?number=} witnessVersion
 * @property {?string=} witnessProgram
 */
/** @typedef {TypeIndex} AnyAddressIndex */
/** @typedef {number} Bitcoin */
/** @typedef {number} BlkPosition */
/**
 * @typedef {Object} BlockCountParam
 * @property {number} blockCount
 */
/**
 * @typedef {Object} BlockFeesEntry
 * @property {Height} avgHeight
 * @property {Timestamp} timestamp
 * @property {Sats} avgFees
 */
/** @typedef {string} BlockHash */
/**
 * @typedef {Object} BlockHashParam
 * @property {BlockHash} hash
 */
/**
 * @typedef {Object} BlockHashStartIndex
 * @property {BlockHash} hash
 * @property {TxIndex} startIndex
 */
/**
 * @typedef {Object} BlockHashTxIndex
 * @property {BlockHash} hash
 * @property {TxIndex} index
 */
/**
 * @typedef {Object} BlockInfo
 * @property {BlockHash} id
 * @property {Height} height
 * @property {number} txCount
 * @property {number} size
 * @property {Weight} weight
 * @property {Timestamp} timestamp
 * @property {number} difficulty
 */
/**
 * @typedef {Object} BlockRewardsEntry
 * @property {number} avgHeight
 * @property {number} timestamp
 * @property {number} avgRewards
 */
/**
 * @typedef {Object} BlockSizeEntry
 * @property {number} avgHeight
 * @property {number} timestamp
 * @property {number} avgSize
 */
/**
 * @typedef {Object} BlockSizesWeights
 * @property {BlockSizeEntry[]} sizes
 * @property {BlockWeightEntry[]} weights
 */
/**
 * @typedef {Object} BlockStatus
 * @property {boolean} inBestChain
 * @property {(Height|null)=} height
 * @property {(BlockHash|null)=} nextBest
 */
/**
 * @typedef {Object} BlockTimestamp
 * @property {Height} height
 * @property {BlockHash} hash
 * @property {string} timestamp
 */
/**
 * @typedef {Object} BlockWeightEntry
 * @property {number} avgHeight
 * @property {number} timestamp
 * @property {number} avgWeight
 */
/** @typedef {number} Cents */
/** @typedef {Cents} Close */
/**
 * @typedef {Object} DataRangeFormat
 * @property {?number=} from
 * @property {?number=} to
 * @property {?number=} count
 * @property {Format=} format
 */
/** @typedef {number} Date */
/** @typedef {number} DateIndex */
/** @typedef {number} DecadeIndex */
/**
 * @typedef {Object} DifficultyAdjustment
 * @property {number} progressPercent
 * @property {number} difficultyChange
 * @property {number} estimatedRetargetDate
 * @property {number} remainingBlocks
 * @property {number} remainingTime
 * @property {number} previousRetarget
 * @property {Height} nextRetargetHeight
 * @property {number} timeAvg
 * @property {number} adjustedTimeAvg
 * @property {number} timeOffset
 */
/**
 * @typedef {Object} DifficultyAdjustmentEntry
 * @property {Timestamp} timestamp
 * @property {Height} height
 * @property {number} difficulty
 * @property {number} changePercent
 */
/**
 * @typedef {Object} DifficultyEntry
 * @property {Timestamp} timestamp
 * @property {number} difficulty
 * @property {Height} height
 */
/** @typedef {number} DifficultyEpoch */
/** @typedef {number} Dollars */
/**
 * @typedef {Object} EmptyAddressData
 * @property {number} txCount
 * @property {number} fundedTxoCount
 * @property {Sats} transfered
 */
/** @typedef {TypeIndex} EmptyAddressIndex */
/** @typedef {TypeIndex} EmptyOutputIndex */
/** @typedef {number} FeeRate */
/** @typedef {("json"|"csv")} Format */
/** @typedef {number} HalvingEpoch */
/**
 * @typedef {Object} HashrateEntry
 * @property {Timestamp} timestamp
 * @property {number} avgHashrate
 */
/**
 * @typedef {Object} HashrateSummary
 * @property {HashrateEntry[]} hashrates
 * @property {DifficultyEntry[]} difficulty
 * @property {number} currentHashrate
 * @property {number} currentDifficulty
 */
/**
 * @typedef {Object} Health
 * @property {string} status
 * @property {string} service
 * @property {string} timestamp
 */
/** @typedef {number} Height */
/**
 * @typedef {Object} HeightParam
 * @property {Height} height
 */
/** @typedef {string} Hex */
/** @typedef {Cents} High */
/** @typedef {("dateindex"|"decadeindex"|"difficultyepoch"|"emptyoutputindex"|"halvingepoch"|"height"|"txinindex"|"monthindex"|"opreturnindex"|"txoutindex"|"p2aaddressindex"|"p2msoutputindex"|"p2pk33addressindex"|"p2pk65addressindex"|"p2pkhaddressindex"|"p2shaddressindex"|"p2traddressindex"|"p2wpkhaddressindex"|"p2wshaddressindex"|"quarterindex"|"semesterindex"|"txindex"|"unknownoutputindex"|"weekindex"|"yearindex"|"loadedaddressindex"|"emptyaddressindex")} Index */
/**
 * @typedef {Object} IndexInfo
 * @property {Index} index
 * @property {string[]} aliases
 */
/** @typedef {number} Limit */
/**
 * @typedef {Object} LimitParam
 * @property {Limit=} limit
 */
/**
 * @typedef {Object} LoadedAddressData
 * @property {number} txCount
 * @property {number} fundedTxoCount
 * @property {number} spentTxoCount
 * @property {Sats} received
 * @property {Sats} sent
 * @property {Dollars} realizedCap
 */
/** @typedef {TypeIndex} LoadedAddressIndex */
/** @typedef {Cents} Low */
/**
 * @typedef {Object} MempoolBlock
 * @property {number} blockSize
 * @property {number} blockVSize
 * @property {number} nTx
 * @property {Sats} totalFees
 * @property {FeeRate} medianFee
 * @property {FeeRate[]} feeRange
 */
/**
 * @typedef {Object} MempoolInfo
 * @property {number} count
 * @property {VSize} vsize
 * @property {Sats} totalFee
 */
/** @typedef {string} Metric */
/**
 * @typedef {Object} MetricCount
 * @property {number} distinctMetrics
 * @property {number} totalEndpoints
 * @property {number} lazyEndpoints
 * @property {number} storedEndpoints
 */
/**
 * @typedef {Object} MetricData
 * @property {number} total
 * @property {number} from
 * @property {number} to
 * @property {*[]} data
 */
/**
 * @typedef {Object} MetricLeafWithSchema
 * @property {string} name
 * @property {string} valueType
 * @property {Index[]} indexes
 */
/**
 * @typedef {Object} MetricParam
 * @property {Metric} metric
 */
/**
 * @typedef {Object} MetricSelection
 * @property {Metrics} metrics
 * @property {Index} index
 * @property {?number=} from
 * @property {?number=} to
 * @property {?number=} count
 * @property {Format=} format
 */
/**
 * @typedef {Object} MetricSelectionLegacy
 * @property {Index} index
 * @property {Metrics} ids
 * @property {?number=} from
 * @property {?number=} to
 * @property {?number=} count
 * @property {Format=} format
 */
/**
 * @typedef {Object} MetricWithIndex
 * @property {Metric} metric
 * @property {Index} index
 */
/** @typedef {string} Metrics */
/** @typedef {number} MonthIndex */
/**
 * @typedef {Object} OHLCCents
 * @property {Open} open
 * @property {High} high
 * @property {Low} low
 * @property {Close} close
 */
/**
 * @typedef {Object} OHLCDollars
 * @property {Open} open
 * @property {High} high
 * @property {Low} low
 * @property {Close} close
 */
/**
 * @typedef {Object} OHLCSats
 * @property {Open} open
 * @property {High} high
 * @property {Low} low
 * @property {Close} close
 */
/** @typedef {TypeIndex} OpReturnIndex */
/** @typedef {Cents} Open */
/** @typedef {number} OutPoint */
/** @typedef {("p2pk65"|"p2pk33"|"p2pkh"|"p2ms"|"p2sh"|"opreturn"|"p2wpkh"|"p2wsh"|"p2tr"|"p2a"|"empty"|"unknown")} OutputType */
/** @typedef {TypeIndex} P2AAddressIndex */
/** @typedef {U8x2} P2ABytes */
/** @typedef {TypeIndex} P2MSOutputIndex */
/** @typedef {TypeIndex} P2PK33AddressIndex */
/** @typedef {U8x33} P2PK33Bytes */
/** @typedef {TypeIndex} P2PK65AddressIndex */
/** @typedef {U8x65} P2PK65Bytes */
/** @typedef {TypeIndex} P2PKHAddressIndex */
/** @typedef {U8x20} P2PKHBytes */
/** @typedef {TypeIndex} P2SHAddressIndex */
/** @typedef {U8x20} P2SHBytes */
/** @typedef {TypeIndex} P2TRAddressIndex */
/** @typedef {U8x32} P2TRBytes */
/** @typedef {TypeIndex} P2WPKHAddressIndex */
/** @typedef {U8x20} P2WPKHBytes */
/** @typedef {TypeIndex} P2WSHAddressIndex */
/** @typedef {U8x32} P2WSHBytes */
/**
 * @typedef {Object} PaginatedMetrics
 * @property {number} currentPage
 * @property {number} maxPage
 * @property {string[]} metrics
 */
/**
 * @typedef {Object} Pagination
 * @property {?number=} page
 */
/**
 * @typedef {Object} PoolBlockCounts
 * @property {number} all
 * @property {number} _24h
 * @property {number} _1w
 */
/**
 * @typedef {Object} PoolBlockShares
 * @property {number} all
 * @property {number} _24h
 * @property {number} _1w
 */
/**
 * @typedef {Object} PoolDetail
 * @property {PoolDetailInfo} pool
 * @property {PoolBlockCounts} blockCount
 * @property {PoolBlockShares} blockShare
 * @property {number} estimatedHashrate
 * @property {?number=} reportedHashrate
 */
/**
 * @typedef {Object} PoolDetailInfo
 * @property {number} id
 * @property {string} name
 * @property {string} link
 * @property {string[]} addresses
 * @property {string[]} regexes
 * @property {PoolSlug} slug
 */
/**
 * @typedef {Object} PoolInfo
 * @property {string} name
 * @property {PoolSlug} slug
 * @property {number} uniqueId
 */
/** @typedef {("unknown"|"blockfills"|"ultimuspool"|"terrapool"|"luxor"|"onethash"|"btccom"|"bitfarms"|"huobipool"|"wayicn"|"canoepool"|"btctop"|"bitcoincom"|"pool175btc"|"gbminers"|"axbt"|"asicminer"|"bitminter"|"bitcoinrussia"|"btcserv"|"simplecoinus"|"btcguild"|"eligius"|"ozcoin"|"eclipsemc"|"maxbtc"|"triplemining"|"coinlab"|"pool50btc"|"ghashio"|"stminingcorp"|"bitparking"|"mmpool"|"polmine"|"kncminer"|"bitalo"|"f2pool"|"hhtt"|"megabigpower"|"mtred"|"nmcbit"|"yourbtcnet"|"givemecoins"|"braiinspool"|"antpool"|"multicoinco"|"bcpoolio"|"cointerra"|"kanopool"|"solock"|"ckpool"|"nicehash"|"bitclub"|"bitcoinaffiliatenetwork"|"btcc"|"bwpool"|"exxbw"|"bitsolo"|"bitfury"|"twentyoneinc"|"digitalbtc"|"eightbaochi"|"mybtccoinpool"|"tbdice"|"hashpool"|"nexious"|"bravomining"|"hotpool"|"okexpool"|"bcmonster"|"onehash"|"bixin"|"tatmaspool"|"viabtc"|"connectbtc"|"batpool"|"waterhole"|"dcexploration"|"dcex"|"btpool"|"fiftyeightcoin"|"bitcoinindia"|"shawnp0wers"|"phashio"|"rigpool"|"haozhuzhu"|"sevenpool"|"miningkings"|"hashbx"|"dpool"|"rawpool"|"haominer"|"helix"|"bitcoinukraine"|"poolin"|"secretsuperstar"|"tigerpoolnet"|"sigmapoolcom"|"okpooltop"|"hummerpool"|"tangpool"|"bytepool"|"spiderpool"|"novablock"|"miningcity"|"binancepool"|"minerium"|"lubiancom"|"okkong"|"aaopool"|"emcdpool"|"foundryusa"|"sbicrypto"|"arkpool"|"purebtccom"|"marapool"|"kucoinpool"|"entrustcharitypool"|"okminer"|"titan"|"pegapool"|"btcnuggets"|"cloudhashing"|"digitalxmintsy"|"telco214"|"btcpoolparty"|"multipool"|"transactioncoinmining"|"btcdig"|"trickysbtcpool"|"btcmp"|"eobot"|"unomp"|"patels"|"gogreenlight"|"ekanembtc"|"canoe"|"tiger"|"onem1x"|"zulupool"|"secpool"|"ocean"|"whitepool"|"wk057"|"futurebitapollosolo"|"carbonnegative"|"portlandhodl"|"phoenix"|"neopool"|"maxipool"|"bitfufupool"|"luckypool"|"miningdutch"|"publicpool"|"miningsquared"|"innopolistech"|"btclab"|"parasite")} PoolSlug */
/**
 * @typedef {Object} PoolSlugParam
 * @property {PoolSlug} slug
 */
/**
 * @typedef {Object} PoolStats
 * @property {number} poolId
 * @property {string} name
 * @property {string} link
 * @property {number} blockCount
 * @property {number} rank
 * @property {number} emptyBlocks
 * @property {PoolSlug} slug
 * @property {number} share
 */
/**
 * @typedef {Object} PoolsSummary
 * @property {PoolStats[]} pools
 * @property {number} blockCount
 * @property {number} lastEstimatedHashrate
 */
/** @typedef {number} QuarterIndex */
/** @typedef {number} RawLockTime */
/**
 * @typedef {Object} RecommendedFees
 * @property {FeeRate} fastestFee
 * @property {FeeRate} halfHourFee
 * @property {FeeRate} hourFee
 * @property {FeeRate} economyFee
 * @property {FeeRate} minimumFee
 */
/**
 * @typedef {Object} RewardStats
 * @property {Height} startBlock
 * @property {Height} endBlock
 * @property {Sats} totalReward
 * @property {Sats} totalFee
 * @property {number} totalTx
 */
/** @typedef {number} Sats */
/** @typedef {number} SemesterIndex */
/** @typedef {number} StoredBool */
/** @typedef {number} StoredF32 */
/** @typedef {number} StoredF64 */
/** @typedef {number} StoredI16 */
/** @typedef {number} StoredU16 */
/** @typedef {number} StoredU32 */
/** @typedef {number} StoredU64 */
/**
 * @typedef {Object} SupplyState
 * @property {number} utxoCount
 * @property {Sats} value
 */
/** @typedef {("24h"|"3d"|"1w"|"1m"|"3m"|"6m"|"1y"|"2y"|"3y")} TimePeriod */
/**
 * @typedef {Object} TimePeriodParam
 * @property {TimePeriod} timePeriod
 */
/** @typedef {number} Timestamp */
/**
 * @typedef {Object} TimestampParam
 * @property {Timestamp} timestamp
 */
/**
 * @typedef {Object} Transaction
 * @property {(TxIndex|null)=} index
 * @property {Txid} txid
 * @property {TxVersion} version
 * @property {RawLockTime} locktime
 * @property {number} size
 * @property {Weight} weight
 * @property {number} sigops
 * @property {Sats} fee
 * @property {TxIn[]} vin
 * @property {TxOut[]} vout
 * @property {TxStatus} status
 */
/** @typedef {({ [key: string]: TreeNode }|MetricLeafWithSchema)} TreeNode */
/**
 * @typedef {Object} TxIn
 * @property {Txid} txid
 * @property {Vout} vout
 * @property {(TxOut|null)=} prevout
 * @property {string} scriptsig
 * @property {string} scriptsigAsm
 * @property {boolean} isCoinbase
 * @property {number} sequence
 * @property {?string=} innerRedeemscriptAsm
 */
/** @typedef {number} TxInIndex */
/** @typedef {number} TxIndex */
/**
 * @typedef {Object} TxOut
 * @property {string} scriptpubkey
 * @property {Sats} value
 */
/** @typedef {number} TxOutIndex */
/**
 * @typedef {Object} TxOutspend
 * @property {boolean} spent
 * @property {(Txid|null)=} txid
 * @property {(Vin|null)=} vin
 * @property {(TxStatus|null)=} status
 */
/**
 * @typedef {Object} TxStatus
 * @property {boolean} confirmed
 * @property {(Height|null)=} blockHeight
 * @property {(BlockHash|null)=} blockHash
 * @property {(Timestamp|null)=} blockTime
 */
/** @typedef {number} TxVersion */
/** @typedef {string} Txid */
/**
 * @typedef {Object} TxidParam
 * @property {Txid} txid
 */
/**
 * @typedef {Object} TxidVout
 * @property {Txid} txid
 * @property {Vout} vout
 */
/** @typedef {number} TypeIndex */
/** @typedef {number[]} U8x2 */
/** @typedef {number[]} U8x20 */
/** @typedef {number[]} U8x32 */
/** @typedef {string} U8x33 */
/** @typedef {string} U8x65 */
/** @typedef {TypeIndex} UnknownOutputIndex */
/**
 * @typedef {Object} Utxo
 * @property {Txid} txid
 * @property {Vout} vout
 * @property {TxStatus} status
 * @property {Sats} value
 */
/** @typedef {number} VSize */
/**
 * @typedef {Object} ValidateAddressParam
 * @property {string} address
 */
/** @typedef {number} Vin */
/** @typedef {number} Vout */
/** @typedef {number} WeekIndex */
/** @typedef {number} Weight */
/** @typedef {number} YearIndex */

/**
 * @typedef {Object} BrkClientOptions
 * @property {string} baseUrl - Base URL for the API
 * @property {number} [timeout] - Request timeout in milliseconds
 */

const _isBrowser = typeof window !== 'undefined' && 'caches' in window;
const _runIdle = (fn) => (globalThis.requestIdleCallback ?? setTimeout)(fn);

/** @type {Promise<Cache | null>} */
const _cachePromise = _isBrowser
  ? caches.open('__BRK_CLIENT__').catch(() => null)
  : Promise.resolve(null);

/**
 * Custom error class for BRK client errors
 */
class BrkError extends Error {
  /**
   * @param {string} message
   * @param {number} [status]
   */
  constructor(message, status) {
    super(message);
    this.name = 'BrkError';
    this.status = status;
  }
}

/**
 * A metric node that can fetch data for different indexes.
 * @template T
 */
class MetricNode {
  /**
   * @param {BrkClientBase} client
   * @param {string} path
   */
  constructor(client, path) {
    this._client = client;
    this._path = path;
  }

  /**
   * Fetch all data points for this metric.
   * @param {(value: T[]) => void} [onUpdate] - Called when data is available (may be called twice: cache then fresh)
   * @returns {Promise<T[]>}
   */
  get(onUpdate) {
    return this._client.get(this._path, onUpdate);
  }

  /**
   * Fetch data points within a range.
   * @param {string | number} [from]
   * @param {string | number} [to]
   * @param {(value: T[]) => void} [onUpdate] - Called when data is available (may be called twice: cache then fresh)
   * @returns {Promise<T[]>}
   */
  getRange(from, to, onUpdate) {
    const params = new URLSearchParams();
    if (from !== undefined) params.set('from', String(from));
    if (to !== undefined) params.set('to', String(to));
    const query = params.toString();
    return this._client.get(query ? `${this._path}?${query}` : this._path, onUpdate);
  }
}

/**
 * Base HTTP client for making requests with caching support
 */
class BrkClientBase {
  /**
   * @param {BrkClientOptions|string} options
   */
  constructor(options) {
    const isString = typeof options === 'string';
    this.baseUrl = isString ? options : options.baseUrl;
    this.timeout = isString ? 5000 : (options.timeout ?? 5000);
  }

  /**
   * Make a GET request with stale-while-revalidate caching
   * @template T
   * @param {string} path
   * @param {(value: T) => void} [onUpdate] - Called when data is available
   * @returns {Promise<T>}
   */
  async get(path, onUpdate) {
    const url = `${this.baseUrl}${path}`;
    const cache = await _cachePromise;
    const cachedRes = await cache?.match(url);
    const cachedJson = cachedRes ? await cachedRes.json() : null;

    if (cachedJson) onUpdate?.(cachedJson);
    if (!globalThis.navigator?.onLine) {
      if (cachedJson) return cachedJson;
      throw new BrkError('Offline and no cached data available');
    }

    try {
      const res = await fetch(url, { signal: AbortSignal.timeout(this.timeout) });
      if (!res.ok) throw new BrkError(`HTTP ${res.status}`, res.status);
      if (cachedRes?.headers.get('ETag') === res.headers.get('ETag')) return cachedJson;

      const cloned = res.clone();
      const json = await res.json();
      onUpdate?.(json);
      if (cache) _runIdle(() => cache.put(url, cloned));
      return json;
    } catch (e) {
      if (cachedJson) return cachedJson;
      throw e;
    }
  }
}


// Index accessor factory functions

/**
 * @template T
 * @typedef {Object} Indexes3By
 * @property {MetricNode<T>} dateindex
 * @property {MetricNode<T>} decadeindex
 * @property {MetricNode<T>} difficultyepoch
 * @property {MetricNode<T>} height
 * @property {MetricNode<T>} monthindex
 * @property {MetricNode<T>} quarterindex
 * @property {MetricNode<T>} semesterindex
 * @property {MetricNode<T>} weekindex
 * @property {MetricNode<T>} yearindex
 */

/**
 * @template T
 * @typedef {Object} Indexes3
 * @property {Indexes3By<T>} by
 */

/**
 * Create a Indexes3 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes3<T>}
 */
function createIndexes3(client, basePath) {
  return {
    by: {
      dateindex: new MetricNode(client, `${basePath}/dateindex`),
      decadeindex: new MetricNode(client, `${basePath}/decadeindex`),
      difficultyepoch: new MetricNode(client, `${basePath}/difficultyepoch`),
      height: new MetricNode(client, `${basePath}/height`),
      monthindex: new MetricNode(client, `${basePath}/monthindex`),
      quarterindex: new MetricNode(client, `${basePath}/quarterindex`),
      semesterindex: new MetricNode(client, `${basePath}/semesterindex`),
      weekindex: new MetricNode(client, `${basePath}/weekindex`),
      yearindex: new MetricNode(client, `${basePath}/yearindex`)
    }
  };
}

/**
 * @template T
 * @typedef {Object} Indexes4By
 * @property {MetricNode<T>} dateindex
 * @property {MetricNode<T>} decadeindex
 * @property {MetricNode<T>} difficultyepoch
 * @property {MetricNode<T>} monthindex
 * @property {MetricNode<T>} quarterindex
 * @property {MetricNode<T>} semesterindex
 * @property {MetricNode<T>} weekindex
 * @property {MetricNode<T>} yearindex
 */

/**
 * @template T
 * @typedef {Object} Indexes4
 * @property {Indexes4By<T>} by
 */

/**
 * Create a Indexes4 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes4<T>}
 */
function createIndexes4(client, basePath) {
  return {
    by: {
      dateindex: new MetricNode(client, `${basePath}/dateindex`),
      decadeindex: new MetricNode(client, `${basePath}/decadeindex`),
      difficultyepoch: new MetricNode(client, `${basePath}/difficultyepoch`),
      monthindex: new MetricNode(client, `${basePath}/monthindex`),
      quarterindex: new MetricNode(client, `${basePath}/quarterindex`),
      semesterindex: new MetricNode(client, `${basePath}/semesterindex`),
      weekindex: new MetricNode(client, `${basePath}/weekindex`),
      yearindex: new MetricNode(client, `${basePath}/yearindex`)
    }
  };
}

/**
 * @template T
 * @typedef {Object} Indexes26By
 * @property {MetricNode<T>} dateindex
 * @property {MetricNode<T>} decadeindex
 * @property {MetricNode<T>} height
 * @property {MetricNode<T>} monthindex
 * @property {MetricNode<T>} quarterindex
 * @property {MetricNode<T>} semesterindex
 * @property {MetricNode<T>} weekindex
 * @property {MetricNode<T>} yearindex
 */

/**
 * @template T
 * @typedef {Object} Indexes26
 * @property {Indexes26By<T>} by
 */

/**
 * Create a Indexes26 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes26<T>}
 */
function createIndexes26(client, basePath) {
  return {
    by: {
      dateindex: new MetricNode(client, `${basePath}/dateindex`),
      decadeindex: new MetricNode(client, `${basePath}/decadeindex`),
      height: new MetricNode(client, `${basePath}/height`),
      monthindex: new MetricNode(client, `${basePath}/monthindex`),
      quarterindex: new MetricNode(client, `${basePath}/quarterindex`),
      semesterindex: new MetricNode(client, `${basePath}/semesterindex`),
      weekindex: new MetricNode(client, `${basePath}/weekindex`),
      yearindex: new MetricNode(client, `${basePath}/yearindex`)
    }
  };
}

/**
 * @template T
 * @typedef {Object} IndexesBy
 * @property {MetricNode<T>} dateindex
 * @property {MetricNode<T>} decadeindex
 * @property {MetricNode<T>} monthindex
 * @property {MetricNode<T>} quarterindex
 * @property {MetricNode<T>} semesterindex
 * @property {MetricNode<T>} weekindex
 * @property {MetricNode<T>} yearindex
 */

/**
 * @template T
 * @typedef {Object} Indexes
 * @property {IndexesBy<T>} by
 */

/**
 * Create a Indexes accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes<T>}
 */
function createIndexes(client, basePath) {
  return {
    by: {
      dateindex: new MetricNode(client, `${basePath}/dateindex`),
      decadeindex: new MetricNode(client, `${basePath}/decadeindex`),
      monthindex: new MetricNode(client, `${basePath}/monthindex`),
      quarterindex: new MetricNode(client, `${basePath}/quarterindex`),
      semesterindex: new MetricNode(client, `${basePath}/semesterindex`),
      weekindex: new MetricNode(client, `${basePath}/weekindex`),
      yearindex: new MetricNode(client, `${basePath}/yearindex`)
    }
  };
}

/**
 * @template T
 * @typedef {Object} Indexes27By
 * @property {MetricNode<T>} decadeindex
 * @property {MetricNode<T>} height
 * @property {MetricNode<T>} monthindex
 * @property {MetricNode<T>} quarterindex
 * @property {MetricNode<T>} semesterindex
 * @property {MetricNode<T>} weekindex
 * @property {MetricNode<T>} yearindex
 */

/**
 * @template T
 * @typedef {Object} Indexes27
 * @property {Indexes27By<T>} by
 */

/**
 * Create a Indexes27 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes27<T>}
 */
function createIndexes27(client, basePath) {
  return {
    by: {
      decadeindex: new MetricNode(client, `${basePath}/decadeindex`),
      height: new MetricNode(client, `${basePath}/height`),
      monthindex: new MetricNode(client, `${basePath}/monthindex`),
      quarterindex: new MetricNode(client, `${basePath}/quarterindex`),
      semesterindex: new MetricNode(client, `${basePath}/semesterindex`),
      weekindex: new MetricNode(client, `${basePath}/weekindex`),
      yearindex: new MetricNode(client, `${basePath}/yearindex`)
    }
  };
}

/**
 * @template T
 * @typedef {Object} Indexes28By
 * @property {MetricNode<T>} decadeindex
 * @property {MetricNode<T>} monthindex
 * @property {MetricNode<T>} quarterindex
 * @property {MetricNode<T>} semesterindex
 * @property {MetricNode<T>} weekindex
 * @property {MetricNode<T>} yearindex
 */

/**
 * @template T
 * @typedef {Object} Indexes28
 * @property {Indexes28By<T>} by
 */

/**
 * Create a Indexes28 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes28<T>}
 */
function createIndexes28(client, basePath) {
  return {
    by: {
      decadeindex: new MetricNode(client, `${basePath}/decadeindex`),
      monthindex: new MetricNode(client, `${basePath}/monthindex`),
      quarterindex: new MetricNode(client, `${basePath}/quarterindex`),
      semesterindex: new MetricNode(client, `${basePath}/semesterindex`),
      weekindex: new MetricNode(client, `${basePath}/weekindex`),
      yearindex: new MetricNode(client, `${basePath}/yearindex`)
    }
  };
}

/**
 * @template T
 * @typedef {Object} Indexes15By
 * @property {MetricNode<T>} quarterindex
 * @property {MetricNode<T>} semesterindex
 * @property {MetricNode<T>} yearindex
 */

/**
 * @template T
 * @typedef {Object} Indexes15
 * @property {Indexes15By<T>} by
 */

/**
 * Create a Indexes15 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes15<T>}
 */
function createIndexes15(client, basePath) {
  return {
    by: {
      quarterindex: new MetricNode(client, `${basePath}/quarterindex`),
      semesterindex: new MetricNode(client, `${basePath}/semesterindex`),
      yearindex: new MetricNode(client, `${basePath}/yearindex`)
    }
  };
}

/**
 * @template T
 * @typedef {Object} Indexes13By
 * @property {MetricNode<T>} dateindex
 * @property {MetricNode<T>} height
 */

/**
 * @template T
 * @typedef {Object} Indexes13
 * @property {Indexes13By<T>} by
 */

/**
 * Create a Indexes13 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes13<T>}
 */
function createIndexes13(client, basePath) {
  return {
    by: {
      dateindex: new MetricNode(client, `${basePath}/dateindex`),
      height: new MetricNode(client, `${basePath}/height`)
    }
  };
}

/**
 * @template T
 * @typedef {Object} Indexes14By
 * @property {MetricNode<T>} monthindex
 * @property {MetricNode<T>} weekindex
 */

/**
 * @template T
 * @typedef {Object} Indexes14
 * @property {Indexes14By<T>} by
 */

/**
 * Create a Indexes14 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes14<T>}
 */
function createIndexes14(client, basePath) {
  return {
    by: {
      monthindex: new MetricNode(client, `${basePath}/monthindex`),
      weekindex: new MetricNode(client, `${basePath}/weekindex`)
    }
  };
}

/**
 * @template T
 * @typedef {Object} Indexes2By
 * @property {MetricNode<T>} height
 */

/**
 * @template T
 * @typedef {Object} Indexes2
 * @property {Indexes2By<T>} by
 */

/**
 * Create a Indexes2 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes2<T>}
 */
function createIndexes2(client, basePath) {
  return {
    by: {
      height: new MetricNode(client, `${basePath}/height`)
    }
  };
}

/**
 * @template T
 * @typedef {Object} Indexes5By
 * @property {MetricNode<T>} dateindex
 */

/**
 * @template T
 * @typedef {Object} Indexes5
 * @property {Indexes5By<T>} by
 */

/**
 * Create a Indexes5 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes5<T>}
 */
function createIndexes5(client, basePath) {
  return {
    by: {
      dateindex: new MetricNode(client, `${basePath}/dateindex`)
    }
  };
}

/**
 * @template T
 * @typedef {Object} Indexes6By
 * @property {MetricNode<T>} txindex
 */

/**
 * @template T
 * @typedef {Object} Indexes6
 * @property {Indexes6By<T>} by
 */

/**
 * Create a Indexes6 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes6<T>}
 */
function createIndexes6(client, basePath) {
  return {
    by: {
      txindex: new MetricNode(client, `${basePath}/txindex`)
    }
  };
}

/**
 * @template T
 * @typedef {Object} Indexes7By
 * @property {MetricNode<T>} decadeindex
 */

/**
 * @template T
 * @typedef {Object} Indexes7
 * @property {Indexes7By<T>} by
 */

/**
 * Create a Indexes7 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes7<T>}
 */
function createIndexes7(client, basePath) {
  return {
    by: {
      decadeindex: new MetricNode(client, `${basePath}/decadeindex`)
    }
  };
}

/**
 * @template T
 * @typedef {Object} Indexes8By
 * @property {MetricNode<T>} monthindex
 */

/**
 * @template T
 * @typedef {Object} Indexes8
 * @property {Indexes8By<T>} by
 */

/**
 * Create a Indexes8 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes8<T>}
 */
function createIndexes8(client, basePath) {
  return {
    by: {
      monthindex: new MetricNode(client, `${basePath}/monthindex`)
    }
  };
}

/**
 * @template T
 * @typedef {Object} Indexes9By
 * @property {MetricNode<T>} quarterindex
 */

/**
 * @template T
 * @typedef {Object} Indexes9
 * @property {Indexes9By<T>} by
 */

/**
 * Create a Indexes9 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes9<T>}
 */
function createIndexes9(client, basePath) {
  return {
    by: {
      quarterindex: new MetricNode(client, `${basePath}/quarterindex`)
    }
  };
}

/**
 * @template T
 * @typedef {Object} Indexes10By
 * @property {MetricNode<T>} semesterindex
 */

/**
 * @template T
 * @typedef {Object} Indexes10
 * @property {Indexes10By<T>} by
 */

/**
 * Create a Indexes10 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes10<T>}
 */
function createIndexes10(client, basePath) {
  return {
    by: {
      semesterindex: new MetricNode(client, `${basePath}/semesterindex`)
    }
  };
}

/**
 * @template T
 * @typedef {Object} Indexes11By
 * @property {MetricNode<T>} weekindex
 */

/**
 * @template T
 * @typedef {Object} Indexes11
 * @property {Indexes11By<T>} by
 */

/**
 * Create a Indexes11 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes11<T>}
 */
function createIndexes11(client, basePath) {
  return {
    by: {
      weekindex: new MetricNode(client, `${basePath}/weekindex`)
    }
  };
}

/**
 * @template T
 * @typedef {Object} Indexes12By
 * @property {MetricNode<T>} yearindex
 */

/**
 * @template T
 * @typedef {Object} Indexes12
 * @property {Indexes12By<T>} by
 */

/**
 * Create a Indexes12 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes12<T>}
 */
function createIndexes12(client, basePath) {
  return {
    by: {
      yearindex: new MetricNode(client, `${basePath}/yearindex`)
    }
  };
}

/**
 * @template T
 * @typedef {Object} Indexes16By
 * @property {MetricNode<T>} p2aaddressindex
 */

/**
 * @template T
 * @typedef {Object} Indexes16
 * @property {Indexes16By<T>} by
 */

/**
 * Create a Indexes16 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes16<T>}
 */
function createIndexes16(client, basePath) {
  return {
    by: {
      p2aaddressindex: new MetricNode(client, `${basePath}/p2aaddressindex`)
    }
  };
}

/**
 * @template T
 * @typedef {Object} Indexes17By
 * @property {MetricNode<T>} p2pk33addressindex
 */

/**
 * @template T
 * @typedef {Object} Indexes17
 * @property {Indexes17By<T>} by
 */

/**
 * Create a Indexes17 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes17<T>}
 */
function createIndexes17(client, basePath) {
  return {
    by: {
      p2pk33addressindex: new MetricNode(client, `${basePath}/p2pk33addressindex`)
    }
  };
}

/**
 * @template T
 * @typedef {Object} Indexes18By
 * @property {MetricNode<T>} p2pk65addressindex
 */

/**
 * @template T
 * @typedef {Object} Indexes18
 * @property {Indexes18By<T>} by
 */

/**
 * Create a Indexes18 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes18<T>}
 */
function createIndexes18(client, basePath) {
  return {
    by: {
      p2pk65addressindex: new MetricNode(client, `${basePath}/p2pk65addressindex`)
    }
  };
}

/**
 * @template T
 * @typedef {Object} Indexes19By
 * @property {MetricNode<T>} p2pkhaddressindex
 */

/**
 * @template T
 * @typedef {Object} Indexes19
 * @property {Indexes19By<T>} by
 */

/**
 * Create a Indexes19 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes19<T>}
 */
function createIndexes19(client, basePath) {
  return {
    by: {
      p2pkhaddressindex: new MetricNode(client, `${basePath}/p2pkhaddressindex`)
    }
  };
}

/**
 * @template T
 * @typedef {Object} Indexes20By
 * @property {MetricNode<T>} p2shaddressindex
 */

/**
 * @template T
 * @typedef {Object} Indexes20
 * @property {Indexes20By<T>} by
 */

/**
 * Create a Indexes20 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes20<T>}
 */
function createIndexes20(client, basePath) {
  return {
    by: {
      p2shaddressindex: new MetricNode(client, `${basePath}/p2shaddressindex`)
    }
  };
}

/**
 * @template T
 * @typedef {Object} Indexes21By
 * @property {MetricNode<T>} p2traddressindex
 */

/**
 * @template T
 * @typedef {Object} Indexes21
 * @property {Indexes21By<T>} by
 */

/**
 * Create a Indexes21 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes21<T>}
 */
function createIndexes21(client, basePath) {
  return {
    by: {
      p2traddressindex: new MetricNode(client, `${basePath}/p2traddressindex`)
    }
  };
}

/**
 * @template T
 * @typedef {Object} Indexes22By
 * @property {MetricNode<T>} p2wpkhaddressindex
 */

/**
 * @template T
 * @typedef {Object} Indexes22
 * @property {Indexes22By<T>} by
 */

/**
 * Create a Indexes22 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes22<T>}
 */
function createIndexes22(client, basePath) {
  return {
    by: {
      p2wpkhaddressindex: new MetricNode(client, `${basePath}/p2wpkhaddressindex`)
    }
  };
}

/**
 * @template T
 * @typedef {Object} Indexes23By
 * @property {MetricNode<T>} p2wshaddressindex
 */

/**
 * @template T
 * @typedef {Object} Indexes23
 * @property {Indexes23By<T>} by
 */

/**
 * Create a Indexes23 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes23<T>}
 */
function createIndexes23(client, basePath) {
  return {
    by: {
      p2wshaddressindex: new MetricNode(client, `${basePath}/p2wshaddressindex`)
    }
  };
}

/**
 * @template T
 * @typedef {Object} Indexes24By
 * @property {MetricNode<T>} txinindex
 */

/**
 * @template T
 * @typedef {Object} Indexes24
 * @property {Indexes24By<T>} by
 */

/**
 * Create a Indexes24 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes24<T>}
 */
function createIndexes24(client, basePath) {
  return {
    by: {
      txinindex: new MetricNode(client, `${basePath}/txinindex`)
    }
  };
}

/**
 * @template T
 * @typedef {Object} Indexes25By
 * @property {MetricNode<T>} txoutindex
 */

/**
 * @template T
 * @typedef {Object} Indexes25
 * @property {Indexes25By<T>} by
 */

/**
 * Create a Indexes25 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes25<T>}
 */
function createIndexes25(client, basePath) {
  return {
    by: {
      txoutindex: new MetricNode(client, `${basePath}/txoutindex`)
    }
  };
}

/**
 * @template T
 * @typedef {Object} Indexes29By
 * @property {MetricNode<T>} emptyaddressindex
 */

/**
 * @template T
 * @typedef {Object} Indexes29
 * @property {Indexes29By<T>} by
 */

/**
 * Create a Indexes29 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes29<T>}
 */
function createIndexes29(client, basePath) {
  return {
    by: {
      emptyaddressindex: new MetricNode(client, `${basePath}/emptyaddressindex`)
    }
  };
}

/**
 * @template T
 * @typedef {Object} Indexes30By
 * @property {MetricNode<T>} loadedaddressindex
 */

/**
 * @template T
 * @typedef {Object} Indexes30
 * @property {Indexes30By<T>} by
 */

/**
 * Create a Indexes30 accessor
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Indexes30<T>}
 */
function createIndexes30(client, basePath) {
  return {
    by: {
      loadedaddressindex: new MetricNode(client, `${basePath}/loadedaddressindex`)
    }
  };
}

// Reusable structural pattern factories

/**
 * @typedef {Object} RealizedPattern3
 * @property {Indexes5<StoredF64>} adjustedSopr
 * @property {Indexes5<StoredF64>} adjustedSopr30dEma
 * @property {Indexes5<StoredF64>} adjustedSopr7dEma
 * @property {Indexes3<Dollars>} adjustedValueCreated
 * @property {Indexes3<Dollars>} adjustedValueDestroyed
 * @property {BlockCountPattern<Dollars>} negRealizedLoss
 * @property {BlockCountPattern<Dollars>} netRealizedPnl
 * @property {Indexes<Dollars>} netRealizedPnlCumulative30dDelta
 * @property {Indexes<StoredF32>} netRealizedPnlCumulative30dDeltaRelToMarketCap
 * @property {Indexes<StoredF32>} netRealizedPnlCumulative30dDeltaRelToRealizedCap
 * @property {Indexes2<StoredF32>} netRealizedPnlRelToRealizedCap
 * @property {Indexes3<Dollars>} realizedCap
 * @property {Indexes<Dollars>} realizedCap30dDelta
 * @property {Indexes3<StoredF32>} realizedCapRelToOwnMarketCap
 * @property {BlockCountPattern<Dollars>} realizedLoss
 * @property {Indexes2<StoredF32>} realizedLossRelToRealizedCap
 * @property {Indexes3<Dollars>} realizedPrice
 * @property {ActivePriceRatioPattern} realizedPriceExtra
 * @property {BlockCountPattern<Dollars>} realizedProfit
 * @property {Indexes2<StoredF32>} realizedProfitRelToRealizedCap
 * @property {Indexes5<StoredF64>} realizedProfitToLossRatio
 * @property {Indexes3<Dollars>} realizedValue
 * @property {Indexes5<StoredF32>} sellSideRiskRatio
 * @property {Indexes5<StoredF32>} sellSideRiskRatio30dEma
 * @property {Indexes5<StoredF32>} sellSideRiskRatio7dEma
 * @property {Indexes5<StoredF64>} sopr
 * @property {Indexes5<StoredF64>} sopr30dEma
 * @property {Indexes5<StoredF64>} sopr7dEma
 * @property {BitcoinPattern2<Dollars>} totalRealizedPnl
 * @property {Indexes3<Dollars>} valueCreated
 * @property {Indexes3<Dollars>} valueDestroyed
 */

/**
 * Create a RealizedPattern3 pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {RealizedPattern3}
 */
function createRealizedPattern3(client, basePath) {
  return {
    adjustedSopr: createIndexes5(client, `${basePath}/adjusted_sopr`),
    adjustedSopr30dEma: createIndexes5(client, `${basePath}/adjusted_sopr_30d_ema`),
    adjustedSopr7dEma: createIndexes5(client, `${basePath}/adjusted_sopr_7d_ema`),
    adjustedValueCreated: createIndexes3(client, `${basePath}/adjusted_value_created`),
    adjustedValueDestroyed: createIndexes3(client, `${basePath}/adjusted_value_destroyed`),
    negRealizedLoss: createBlockCountPattern(client, `${basePath}/neg_realized_loss`),
    netRealizedPnl: createBlockCountPattern(client, `${basePath}/net_realized_pnl`),
    netRealizedPnlCumulative30dDelta: createIndexes(client, `${basePath}/net_realized_pnl_cumulative_30d_delta`),
    netRealizedPnlCumulative30dDeltaRelToMarketCap: createIndexes(client, `${basePath}/net_realized_pnl_cumulative_30d_delta_rel_to_market_cap`),
    netRealizedPnlCumulative30dDeltaRelToRealizedCap: createIndexes(client, `${basePath}/net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap`),
    netRealizedPnlRelToRealizedCap: createIndexes2(client, `${basePath}/net_realized_pnl_rel_to_realized_cap`),
    realizedCap: createIndexes3(client, `${basePath}/realized_cap`),
    realizedCap30dDelta: createIndexes(client, `${basePath}/realized_cap_30d_delta`),
    realizedCapRelToOwnMarketCap: createIndexes3(client, `${basePath}/realized_cap_rel_to_own_market_cap`),
    realizedLoss: createBlockCountPattern(client, `${basePath}/realized_loss`),
    realizedLossRelToRealizedCap: createIndexes2(client, `${basePath}/realized_loss_rel_to_realized_cap`),
    realizedPrice: createIndexes3(client, `${basePath}/realized_price`),
    realizedPriceExtra: createActivePriceRatioPattern(client, `${basePath}/realized_price_extra`),
    realizedProfit: createBlockCountPattern(client, `${basePath}/realized_profit`),
    realizedProfitRelToRealizedCap: createIndexes2(client, `${basePath}/realized_profit_rel_to_realized_cap`),
    realizedProfitToLossRatio: createIndexes5(client, `${basePath}/realized_profit_to_loss_ratio`),
    realizedValue: createIndexes3(client, `${basePath}/realized_value`),
    sellSideRiskRatio: createIndexes5(client, `${basePath}/sell_side_risk_ratio`),
    sellSideRiskRatio30dEma: createIndexes5(client, `${basePath}/sell_side_risk_ratio_30d_ema`),
    sellSideRiskRatio7dEma: createIndexes5(client, `${basePath}/sell_side_risk_ratio_7d_ema`),
    sopr: createIndexes5(client, `${basePath}/sopr`),
    sopr30dEma: createIndexes5(client, `${basePath}/sopr_30d_ema`),
    sopr7dEma: createIndexes5(client, `${basePath}/sopr_7d_ema`),
    totalRealizedPnl: createBitcoinPattern2(client, `${basePath}/total_realized_pnl`),
    valueCreated: createIndexes3(client, `${basePath}/value_created`),
    valueDestroyed: createIndexes3(client, `${basePath}/value_destroyed`)
  };
}

/**
 * @typedef {Object} Ratio1ySdPattern2
 * @property {Indexes<Dollars>} _0sdUsd
 * @property {Indexes<StoredF32>} m05sd
 * @property {Indexes<Dollars>} m05sdUsd
 * @property {Indexes<StoredF32>} m15sd
 * @property {Indexes<Dollars>} m15sdUsd
 * @property {Indexes<StoredF32>} m1sd
 * @property {Indexes<Dollars>} m1sdUsd
 * @property {Indexes<StoredF32>} m25sd
 * @property {Indexes<Dollars>} m25sdUsd
 * @property {Indexes<StoredF32>} m2sd
 * @property {Indexes<Dollars>} m2sdUsd
 * @property {Indexes<StoredF32>} m3sd
 * @property {Indexes<Dollars>} m3sdUsd
 * @property {Indexes<StoredF32>} p05sd
 * @property {Indexes<Dollars>} p05sdUsd
 * @property {Indexes<StoredF32>} p15sd
 * @property {Indexes<Dollars>} p15sdUsd
 * @property {Indexes<StoredF32>} p1sd
 * @property {Indexes<Dollars>} p1sdUsd
 * @property {Indexes<StoredF32>} p25sd
 * @property {Indexes<Dollars>} p25sdUsd
 * @property {Indexes<StoredF32>} p2sd
 * @property {Indexes<Dollars>} p2sdUsd
 * @property {Indexes<StoredF32>} p3sd
 * @property {Indexes<Dollars>} p3sdUsd
 * @property {Indexes<StoredF32>} sd
 * @property {Indexes<StoredF32>} sma
 * @property {Indexes<StoredF32>} zscore
 */

/**
 * Create a Ratio1ySdPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Ratio1ySdPattern2}
 */
function createRatio1ySdPattern2(client, basePath) {
  return {
    _0sdUsd: createIndexes(client, `${basePath}/_0sd_usd`),
    m05sd: createIndexes(client, `${basePath}/m0_5sd`),
    m05sdUsd: createIndexes(client, `${basePath}/m0_5sd_usd`),
    m15sd: createIndexes(client, `${basePath}/m1_5sd`),
    m15sdUsd: createIndexes(client, `${basePath}/m1_5sd_usd`),
    m1sd: createIndexes(client, `${basePath}/m1sd`),
    m1sdUsd: createIndexes(client, `${basePath}/m1sd_usd`),
    m25sd: createIndexes(client, `${basePath}/m2_5sd`),
    m25sdUsd: createIndexes(client, `${basePath}/m2_5sd_usd`),
    m2sd: createIndexes(client, `${basePath}/m2sd`),
    m2sdUsd: createIndexes(client, `${basePath}/m2sd_usd`),
    m3sd: createIndexes(client, `${basePath}/m3sd`),
    m3sdUsd: createIndexes(client, `${basePath}/m3sd_usd`),
    p05sd: createIndexes(client, `${basePath}/p0_5sd`),
    p05sdUsd: createIndexes(client, `${basePath}/p0_5sd_usd`),
    p15sd: createIndexes(client, `${basePath}/p1_5sd`),
    p15sdUsd: createIndexes(client, `${basePath}/p1_5sd_usd`),
    p1sd: createIndexes(client, `${basePath}/p1sd`),
    p1sdUsd: createIndexes(client, `${basePath}/p1sd_usd`),
    p25sd: createIndexes(client, `${basePath}/p2_5sd`),
    p25sdUsd: createIndexes(client, `${basePath}/p2_5sd_usd`),
    p2sd: createIndexes(client, `${basePath}/p2sd`),
    p2sdUsd: createIndexes(client, `${basePath}/p2sd_usd`),
    p3sd: createIndexes(client, `${basePath}/p3sd`),
    p3sdUsd: createIndexes(client, `${basePath}/p3sd_usd`),
    sd: createIndexes(client, `${basePath}/sd`),
    sma: createIndexes(client, `${basePath}/sma`),
    zscore: createIndexes(client, `${basePath}/zscore`)
  };
}

/**
 * @typedef {Object} RealizedPattern2
 * @property {BlockCountPattern<Dollars>} negRealizedLoss
 * @property {BlockCountPattern<Dollars>} netRealizedPnl
 * @property {Indexes<Dollars>} netRealizedPnlCumulative30dDelta
 * @property {Indexes<StoredF32>} netRealizedPnlCumulative30dDeltaRelToMarketCap
 * @property {Indexes<StoredF32>} netRealizedPnlCumulative30dDeltaRelToRealizedCap
 * @property {Indexes2<StoredF32>} netRealizedPnlRelToRealizedCap
 * @property {Indexes3<Dollars>} realizedCap
 * @property {Indexes<Dollars>} realizedCap30dDelta
 * @property {Indexes3<StoredF32>} realizedCapRelToOwnMarketCap
 * @property {BlockCountPattern<Dollars>} realizedLoss
 * @property {Indexes2<StoredF32>} realizedLossRelToRealizedCap
 * @property {Indexes3<Dollars>} realizedPrice
 * @property {ActivePriceRatioPattern} realizedPriceExtra
 * @property {BlockCountPattern<Dollars>} realizedProfit
 * @property {Indexes2<StoredF32>} realizedProfitRelToRealizedCap
 * @property {Indexes5<StoredF64>} realizedProfitToLossRatio
 * @property {Indexes3<Dollars>} realizedValue
 * @property {Indexes5<StoredF32>} sellSideRiskRatio
 * @property {Indexes5<StoredF32>} sellSideRiskRatio30dEma
 * @property {Indexes5<StoredF32>} sellSideRiskRatio7dEma
 * @property {Indexes5<StoredF64>} sopr
 * @property {Indexes5<StoredF64>} sopr30dEma
 * @property {Indexes5<StoredF64>} sopr7dEma
 * @property {BitcoinPattern2<Dollars>} totalRealizedPnl
 * @property {Indexes3<Dollars>} valueCreated
 * @property {Indexes3<Dollars>} valueDestroyed
 */

/**
 * Create a RealizedPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {RealizedPattern2}
 */
function createRealizedPattern2(client, basePath) {
  return {
    negRealizedLoss: createBlockCountPattern(client, `${basePath}/neg_realized_loss`),
    netRealizedPnl: createBlockCountPattern(client, `${basePath}/net_realized_pnl`),
    netRealizedPnlCumulative30dDelta: createIndexes(client, `${basePath}/net_realized_pnl_cumulative_30d_delta`),
    netRealizedPnlCumulative30dDeltaRelToMarketCap: createIndexes(client, `${basePath}/net_realized_pnl_cumulative_30d_delta_rel_to_market_cap`),
    netRealizedPnlCumulative30dDeltaRelToRealizedCap: createIndexes(client, `${basePath}/net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap`),
    netRealizedPnlRelToRealizedCap: createIndexes2(client, `${basePath}/net_realized_pnl_rel_to_realized_cap`),
    realizedCap: createIndexes3(client, `${basePath}/realized_cap`),
    realizedCap30dDelta: createIndexes(client, `${basePath}/realized_cap_30d_delta`),
    realizedCapRelToOwnMarketCap: createIndexes3(client, `${basePath}/realized_cap_rel_to_own_market_cap`),
    realizedLoss: createBlockCountPattern(client, `${basePath}/realized_loss`),
    realizedLossRelToRealizedCap: createIndexes2(client, `${basePath}/realized_loss_rel_to_realized_cap`),
    realizedPrice: createIndexes3(client, `${basePath}/realized_price`),
    realizedPriceExtra: createActivePriceRatioPattern(client, `${basePath}/realized_price_extra`),
    realizedProfit: createBlockCountPattern(client, `${basePath}/realized_profit`),
    realizedProfitRelToRealizedCap: createIndexes2(client, `${basePath}/realized_profit_rel_to_realized_cap`),
    realizedProfitToLossRatio: createIndexes5(client, `${basePath}/realized_profit_to_loss_ratio`),
    realizedValue: createIndexes3(client, `${basePath}/realized_value`),
    sellSideRiskRatio: createIndexes5(client, `${basePath}/sell_side_risk_ratio`),
    sellSideRiskRatio30dEma: createIndexes5(client, `${basePath}/sell_side_risk_ratio_30d_ema`),
    sellSideRiskRatio7dEma: createIndexes5(client, `${basePath}/sell_side_risk_ratio_7d_ema`),
    sopr: createIndexes5(client, `${basePath}/sopr`),
    sopr30dEma: createIndexes5(client, `${basePath}/sopr_30d_ema`),
    sopr7dEma: createIndexes5(client, `${basePath}/sopr_7d_ema`),
    totalRealizedPnl: createBitcoinPattern2(client, `${basePath}/total_realized_pnl`),
    valueCreated: createIndexes3(client, `${basePath}/value_created`),
    valueDestroyed: createIndexes3(client, `${basePath}/value_destroyed`)
  };
}

/**
 * @typedef {Object} RealizedPattern
 * @property {BlockCountPattern<Dollars>} negRealizedLoss
 * @property {BlockCountPattern<Dollars>} netRealizedPnl
 * @property {Indexes<Dollars>} netRealizedPnlCumulative30dDelta
 * @property {Indexes<StoredF32>} netRealizedPnlCumulative30dDeltaRelToMarketCap
 * @property {Indexes<StoredF32>} netRealizedPnlCumulative30dDeltaRelToRealizedCap
 * @property {Indexes2<StoredF32>} netRealizedPnlRelToRealizedCap
 * @property {Indexes3<Dollars>} realizedCap
 * @property {Indexes<Dollars>} realizedCap30dDelta
 * @property {BlockCountPattern<Dollars>} realizedLoss
 * @property {Indexes2<StoredF32>} realizedLossRelToRealizedCap
 * @property {Indexes3<Dollars>} realizedPrice
 * @property {RealizedPriceExtraPattern} realizedPriceExtra
 * @property {BlockCountPattern<Dollars>} realizedProfit
 * @property {Indexes2<StoredF32>} realizedProfitRelToRealizedCap
 * @property {Indexes3<Dollars>} realizedValue
 * @property {Indexes5<StoredF32>} sellSideRiskRatio
 * @property {Indexes5<StoredF32>} sellSideRiskRatio30dEma
 * @property {Indexes5<StoredF32>} sellSideRiskRatio7dEma
 * @property {Indexes5<StoredF64>} sopr
 * @property {Indexes5<StoredF64>} sopr30dEma
 * @property {Indexes5<StoredF64>} sopr7dEma
 * @property {BitcoinPattern2<Dollars>} totalRealizedPnl
 * @property {Indexes3<Dollars>} valueCreated
 * @property {Indexes3<Dollars>} valueDestroyed
 */

/**
 * Create a RealizedPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {RealizedPattern}
 */
function createRealizedPattern(client, basePath) {
  return {
    negRealizedLoss: createBlockCountPattern(client, `${basePath}/neg_realized_loss`),
    netRealizedPnl: createBlockCountPattern(client, `${basePath}/net_realized_pnl`),
    netRealizedPnlCumulative30dDelta: createIndexes(client, `${basePath}/net_realized_pnl_cumulative_30d_delta`),
    netRealizedPnlCumulative30dDeltaRelToMarketCap: createIndexes(client, `${basePath}/net_realized_pnl_cumulative_30d_delta_rel_to_market_cap`),
    netRealizedPnlCumulative30dDeltaRelToRealizedCap: createIndexes(client, `${basePath}/net_realized_pnl_cumulative_30d_delta_rel_to_realized_cap`),
    netRealizedPnlRelToRealizedCap: createIndexes2(client, `${basePath}/net_realized_pnl_rel_to_realized_cap`),
    realizedCap: createIndexes3(client, `${basePath}/realized_cap`),
    realizedCap30dDelta: createIndexes(client, `${basePath}/realized_cap_30d_delta`),
    realizedLoss: createBlockCountPattern(client, `${basePath}/realized_loss`),
    realizedLossRelToRealizedCap: createIndexes2(client, `${basePath}/realized_loss_rel_to_realized_cap`),
    realizedPrice: createIndexes3(client, `${basePath}/realized_price`),
    realizedPriceExtra: createRealizedPriceExtraPattern(client, `${basePath}/realized_price_extra`),
    realizedProfit: createBlockCountPattern(client, `${basePath}/realized_profit`),
    realizedProfitRelToRealizedCap: createIndexes2(client, `${basePath}/realized_profit_rel_to_realized_cap`),
    realizedValue: createIndexes3(client, `${basePath}/realized_value`),
    sellSideRiskRatio: createIndexes5(client, `${basePath}/sell_side_risk_ratio`),
    sellSideRiskRatio30dEma: createIndexes5(client, `${basePath}/sell_side_risk_ratio_30d_ema`),
    sellSideRiskRatio7dEma: createIndexes5(client, `${basePath}/sell_side_risk_ratio_7d_ema`),
    sopr: createIndexes5(client, `${basePath}/sopr`),
    sopr30dEma: createIndexes5(client, `${basePath}/sopr_30d_ema`),
    sopr7dEma: createIndexes5(client, `${basePath}/sopr_7d_ema`),
    totalRealizedPnl: createBitcoinPattern2(client, `${basePath}/total_realized_pnl`),
    valueCreated: createIndexes3(client, `${basePath}/value_created`),
    valueDestroyed: createIndexes3(client, `${basePath}/value_destroyed`)
  };
}

/**
 * @typedef {Object} Price13dEmaPattern
 * @property {Indexes<Dollars>} price
 * @property {Indexes<StoredF32>} ratio
 * @property {Indexes<StoredF32>} ratio1mSma
 * @property {Indexes<StoredF32>} ratio1wSma
 * @property {Ratio1ySdPattern2} ratio1ySd
 * @property {Ratio1ySdPattern2} ratio2ySd
 * @property {Ratio1ySdPattern2} ratio4ySd
 * @property {Indexes<StoredF32>} ratioPct1
 * @property {Indexes<Dollars>} ratioPct1Usd
 * @property {Indexes<StoredF32>} ratioPct2
 * @property {Indexes<Dollars>} ratioPct2Usd
 * @property {Indexes<StoredF32>} ratioPct5
 * @property {Indexes<Dollars>} ratioPct5Usd
 * @property {Indexes<StoredF32>} ratioPct95
 * @property {Indexes<Dollars>} ratioPct95Usd
 * @property {Indexes<StoredF32>} ratioPct98
 * @property {Indexes<Dollars>} ratioPct98Usd
 * @property {Indexes<StoredF32>} ratioPct99
 * @property {Indexes<Dollars>} ratioPct99Usd
 * @property {Ratio1ySdPattern2} ratioSd
 */

/**
 * Create a Price13dEmaPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {Price13dEmaPattern}
 */
function createPrice13dEmaPattern(client, acc) {
  return {
    price: createIndexes(client, `/${acc}`),
    ratio: createIndexes(client, `/${acc}_ratio`),
    ratio1mSma: createIndexes(client, `/${acc}_ratio_1m_sma`),
    ratio1wSma: createIndexes(client, `/${acc}_ratio_1w_sma`),
    ratio1ySd: createRatio1ySdPattern2(client, `${acc}_ratio_1y_sd`),
    ratio2ySd: createRatio1ySdPattern2(client, `${acc}_ratio_2y_sd`),
    ratio4ySd: createRatio1ySdPattern2(client, `${acc}_ratio_4y_sd`),
    ratioPct1: createIndexes(client, `/${acc}_ratio_pct1`),
    ratioPct1Usd: createIndexes(client, `/${acc}_ratio_pct1_usd`),
    ratioPct2: createIndexes(client, `/${acc}_ratio_pct2`),
    ratioPct2Usd: createIndexes(client, `/${acc}_ratio_pct2_usd`),
    ratioPct5: createIndexes(client, `/${acc}_ratio_pct5`),
    ratioPct5Usd: createIndexes(client, `/${acc}_ratio_pct5_usd`),
    ratioPct95: createIndexes(client, `/${acc}_ratio_pct95`),
    ratioPct95Usd: createIndexes(client, `/${acc}_ratio_pct95_usd`),
    ratioPct98: createIndexes(client, `/${acc}_ratio_pct98`),
    ratioPct98Usd: createIndexes(client, `/${acc}_ratio_pct98_usd`),
    ratioPct99: createIndexes(client, `/${acc}_ratio_pct99`),
    ratioPct99Usd: createIndexes(client, `/${acc}_ratio_pct99_usd`),
    ratioSd: createRatio1ySdPattern2(client, `${acc}_ratio_sd`)
  };
}

/**
 * @typedef {Object} PricePercentilesPattern
 * @property {Indexes<Dollars>} pct05
 * @property {Indexes<Dollars>} pct10
 * @property {Indexes<Dollars>} pct15
 * @property {Indexes<Dollars>} pct20
 * @property {Indexes<Dollars>} pct25
 * @property {Indexes<Dollars>} pct30
 * @property {Indexes<Dollars>} pct35
 * @property {Indexes<Dollars>} pct40
 * @property {Indexes<Dollars>} pct45
 * @property {Indexes<Dollars>} pct50
 * @property {Indexes<Dollars>} pct55
 * @property {Indexes<Dollars>} pct60
 * @property {Indexes<Dollars>} pct65
 * @property {Indexes<Dollars>} pct70
 * @property {Indexes<Dollars>} pct75
 * @property {Indexes<Dollars>} pct80
 * @property {Indexes<Dollars>} pct85
 * @property {Indexes<Dollars>} pct90
 * @property {Indexes<Dollars>} pct95
 */

/**
 * Create a PricePercentilesPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {PricePercentilesPattern}
 */
function createPricePercentilesPattern(client, basePath) {
  return {
    pct05: createIndexes(client, `${basePath}/pct05`),
    pct10: createIndexes(client, `${basePath}/pct10`),
    pct15: createIndexes(client, `${basePath}/pct15`),
    pct20: createIndexes(client, `${basePath}/pct20`),
    pct25: createIndexes(client, `${basePath}/pct25`),
    pct30: createIndexes(client, `${basePath}/pct30`),
    pct35: createIndexes(client, `${basePath}/pct35`),
    pct40: createIndexes(client, `${basePath}/pct40`),
    pct45: createIndexes(client, `${basePath}/pct45`),
    pct50: createIndexes(client, `${basePath}/pct50`),
    pct55: createIndexes(client, `${basePath}/pct55`),
    pct60: createIndexes(client, `${basePath}/pct60`),
    pct65: createIndexes(client, `${basePath}/pct65`),
    pct70: createIndexes(client, `${basePath}/pct70`),
    pct75: createIndexes(client, `${basePath}/pct75`),
    pct80: createIndexes(client, `${basePath}/pct80`),
    pct85: createIndexes(client, `${basePath}/pct85`),
    pct90: createIndexes(client, `${basePath}/pct90`),
    pct95: createIndexes(client, `${basePath}/pct95`)
  };
}

/**
 * @typedef {Object} RelativePattern2
 * @property {Indexes27<StoredF32>} negUnrealizedLossRelToMarketCap
 * @property {Indexes27<StoredF32>} negUnrealizedLossRelToOwnMarketCap
 * @property {Indexes27<StoredF32>} negUnrealizedLossRelToOwnTotalUnrealizedPnl
 * @property {Indexes26<StoredF32>} netUnrealizedPnlRelToMarketCap
 * @property {Indexes26<StoredF32>} netUnrealizedPnlRelToOwnMarketCap
 * @property {Indexes26<StoredF32>} netUnrealizedPnlRelToOwnTotalUnrealizedPnl
 * @property {Indexes27<StoredF64>} supplyInLossRelToCirculatingSupply
 * @property {Indexes27<StoredF64>} supplyInLossRelToOwnSupply
 * @property {Indexes27<StoredF64>} supplyInProfitRelToCirculatingSupply
 * @property {Indexes27<StoredF64>} supplyInProfitRelToOwnSupply
 * @property {Indexes<StoredF64>} supplyRelToCirculatingSupply
 * @property {Indexes27<StoredF32>} unrealizedLossRelToMarketCap
 * @property {Indexes27<StoredF32>} unrealizedLossRelToOwnMarketCap
 * @property {Indexes27<StoredF32>} unrealizedLossRelToOwnTotalUnrealizedPnl
 * @property {Indexes27<StoredF32>} unrealizedProfitRelToMarketCap
 * @property {Indexes27<StoredF32>} unrealizedProfitRelToOwnMarketCap
 * @property {Indexes27<StoredF32>} unrealizedProfitRelToOwnTotalUnrealizedPnl
 */

/**
 * Create a RelativePattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {RelativePattern2}
 */
function createRelativePattern2(client, basePath) {
  return {
    negUnrealizedLossRelToMarketCap: createIndexes27(client, `${basePath}/neg_unrealized_loss_rel_to_market_cap`),
    negUnrealizedLossRelToOwnMarketCap: createIndexes27(client, `${basePath}/neg_unrealized_loss_rel_to_own_market_cap`),
    negUnrealizedLossRelToOwnTotalUnrealizedPnl: createIndexes27(client, `${basePath}/neg_unrealized_loss_rel_to_own_total_unrealized_pnl`),
    netUnrealizedPnlRelToMarketCap: createIndexes26(client, `${basePath}/net_unrealized_pnl_rel_to_market_cap`),
    netUnrealizedPnlRelToOwnMarketCap: createIndexes26(client, `${basePath}/net_unrealized_pnl_rel_to_own_market_cap`),
    netUnrealizedPnlRelToOwnTotalUnrealizedPnl: createIndexes26(client, `${basePath}/net_unrealized_pnl_rel_to_own_total_unrealized_pnl`),
    supplyInLossRelToCirculatingSupply: createIndexes27(client, `${basePath}/supply_in_loss_rel_to_circulating_supply`),
    supplyInLossRelToOwnSupply: createIndexes27(client, `${basePath}/supply_in_loss_rel_to_own_supply`),
    supplyInProfitRelToCirculatingSupply: createIndexes27(client, `${basePath}/supply_in_profit_rel_to_circulating_supply`),
    supplyInProfitRelToOwnSupply: createIndexes27(client, `${basePath}/supply_in_profit_rel_to_own_supply`),
    supplyRelToCirculatingSupply: createIndexes(client, `${basePath}/supply_rel_to_circulating_supply`),
    unrealizedLossRelToMarketCap: createIndexes27(client, `${basePath}/unrealized_loss_rel_to_market_cap`),
    unrealizedLossRelToOwnMarketCap: createIndexes27(client, `${basePath}/unrealized_loss_rel_to_own_market_cap`),
    unrealizedLossRelToOwnTotalUnrealizedPnl: createIndexes27(client, `${basePath}/unrealized_loss_rel_to_own_total_unrealized_pnl`),
    unrealizedProfitRelToMarketCap: createIndexes27(client, `${basePath}/unrealized_profit_rel_to_market_cap`),
    unrealizedProfitRelToOwnMarketCap: createIndexes27(client, `${basePath}/unrealized_profit_rel_to_own_market_cap`),
    unrealizedProfitRelToOwnTotalUnrealizedPnl: createIndexes27(client, `${basePath}/unrealized_profit_rel_to_own_total_unrealized_pnl`)
  };
}

/**
 * @typedef {Object} Ratio1ySdPattern
 * @property {Indexes<StoredF32>} m05sd
 * @property {Indexes<StoredF32>} m15sd
 * @property {Indexes<StoredF32>} m1sd
 * @property {Indexes<StoredF32>} m25sd
 * @property {Indexes<StoredF32>} m2sd
 * @property {Indexes<StoredF32>} m3sd
 * @property {Indexes<StoredF32>} p05sd
 * @property {Indexes<StoredF32>} p15sd
 * @property {Indexes<StoredF32>} p1sd
 * @property {Indexes<StoredF32>} p25sd
 * @property {Indexes<StoredF32>} p2sd
 * @property {Indexes<StoredF32>} p3sd
 * @property {Indexes<StoredF32>} sd
 * @property {Indexes<StoredF32>} sma
 * @property {Indexes<StoredF32>} zscore
 */

/**
 * Create a Ratio1ySdPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {Ratio1ySdPattern}
 */
function createRatio1ySdPattern(client, basePath) {
  return {
    m05sd: createIndexes(client, `${basePath}/m0_5sd`),
    m15sd: createIndexes(client, `${basePath}/m1_5sd`),
    m1sd: createIndexes(client, `${basePath}/m1sd`),
    m25sd: createIndexes(client, `${basePath}/m2_5sd`),
    m2sd: createIndexes(client, `${basePath}/m2sd`),
    m3sd: createIndexes(client, `${basePath}/m3sd`),
    p05sd: createIndexes(client, `${basePath}/p0_5sd`),
    p15sd: createIndexes(client, `${basePath}/p1_5sd`),
    p1sd: createIndexes(client, `${basePath}/p1sd`),
    p25sd: createIndexes(client, `${basePath}/p2_5sd`),
    p2sd: createIndexes(client, `${basePath}/p2sd`),
    p3sd: createIndexes(client, `${basePath}/p3sd`),
    sd: createIndexes(client, `${basePath}/sd`),
    sma: createIndexes(client, `${basePath}/sma`),
    zscore: createIndexes(client, `${basePath}/zscore`)
  };
}

/**
 * @typedef {Object} AXbtPattern
 * @property {BlockCountPattern<StoredF32>} _1dDominance
 * @property {Indexes<StoredU32>} _1mBlocksMined
 * @property {Indexes<StoredF32>} _1mDominance
 * @property {Indexes<StoredU32>} _1wBlocksMined
 * @property {Indexes<StoredF32>} _1wDominance
 * @property {Indexes<StoredU32>} _1yBlocksMined
 * @property {Indexes<StoredF32>} _1yDominance
 * @property {BlockCountPattern<StoredU32>} blocksMined
 * @property {UnclaimedRewardsPattern} coinbase
 * @property {Indexes<StoredU16>} daysSinceBlock
 * @property {BlockCountPattern<StoredF32>} dominance
 * @property {FeePattern2} fee
 * @property {FeePattern2} subsidy
 */

/**
 * Create a AXbtPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {AXbtPattern}
 */
function createAXbtPattern(client, basePath) {
  return {
    _1dDominance: createBlockCountPattern(client, `${basePath}/1d_dominance`),
    _1mBlocksMined: createIndexes(client, `${basePath}/1m_blocks_mined`),
    _1mDominance: createIndexes(client, `${basePath}/1m_dominance`),
    _1wBlocksMined: createIndexes(client, `${basePath}/1w_blocks_mined`),
    _1wDominance: createIndexes(client, `${basePath}/1w_dominance`),
    _1yBlocksMined: createIndexes(client, `${basePath}/1y_blocks_mined`),
    _1yDominance: createIndexes(client, `${basePath}/1y_dominance`),
    blocksMined: createBlockCountPattern(client, `${basePath}/blocks_mined`),
    coinbase: createUnclaimedRewardsPattern(client, `${basePath}/coinbase`),
    daysSinceBlock: createIndexes(client, `${basePath}/days_since_block`),
    dominance: createBlockCountPattern(client, `${basePath}/dominance`),
    fee: createFeePattern2(client, `${basePath}/fee`),
    subsidy: createFeePattern2(client, `${basePath}/subsidy`)
  };
}

/**
 * @typedef {Object} ActivePriceRatioPattern
 * @property {Indexes<StoredF32>} ratio
 * @property {Indexes<StoredF32>} ratio1mSma
 * @property {Indexes<StoredF32>} ratio1wSma
 * @property {Ratio1ySdPattern} ratio1ySd
 * @property {Ratio1ySdPattern} ratio2ySd
 * @property {Ratio1ySdPattern} ratio4ySd
 * @property {Indexes<StoredF32>} ratioPct1
 * @property {Indexes<StoredF32>} ratioPct2
 * @property {Indexes<StoredF32>} ratioPct5
 * @property {Indexes<StoredF32>} ratioPct95
 * @property {Indexes<StoredF32>} ratioPct98
 * @property {Indexes<StoredF32>} ratioPct99
 * @property {Ratio1ySdPattern} ratioSd
 */

/**
 * Create a ActivePriceRatioPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {ActivePriceRatioPattern}
 */
function createActivePriceRatioPattern(client, basePath) {
  return {
    ratio: createIndexes(client, `${basePath}/ratio`),
    ratio1mSma: createIndexes(client, `${basePath}/ratio_1m_sma`),
    ratio1wSma: createIndexes(client, `${basePath}/ratio_1w_sma`),
    ratio1ySd: createRatio1ySdPattern(client, `${basePath}/ratio_1y_sd`),
    ratio2ySd: createRatio1ySdPattern(client, `${basePath}/ratio_2y_sd`),
    ratio4ySd: createRatio1ySdPattern(client, `${basePath}/ratio_4y_sd`),
    ratioPct1: createIndexes(client, `${basePath}/ratio_pct1`),
    ratioPct2: createIndexes(client, `${basePath}/ratio_pct2`),
    ratioPct5: createIndexes(client, `${basePath}/ratio_pct5`),
    ratioPct95: createIndexes(client, `${basePath}/ratio_pct95`),
    ratioPct98: createIndexes(client, `${basePath}/ratio_pct98`),
    ratioPct99: createIndexes(client, `${basePath}/ratio_pct99`),
    ratioSd: createRatio1ySdPattern(client, `${basePath}/ratio_sd`)
  };
}

/**
 * @template T
 * @typedef {Object} BitcoinPattern
 * @property {Indexes4<T>} average
 * @property {Indexes2<T>} base
 * @property {Indexes3<T>} cumulative
 * @property {Indexes4<T>} max
 * @property {Indexes5<T>} median
 * @property {Indexes4<T>} min
 * @property {Indexes5<T>} pct10
 * @property {Indexes5<T>} pct25
 * @property {Indexes5<T>} pct75
 * @property {Indexes5<T>} pct90
 * @property {Indexes4<T>} sum
 */

/**
 * Create a BitcoinPattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {BitcoinPattern<T>}
 */
function createBitcoinPattern(client, basePath) {
  return {
    average: createIndexes4(client, `${basePath}/average`),
    base: createIndexes2(client, `${basePath}/base`),
    cumulative: createIndexes3(client, `${basePath}/cumulative`),
    max: createIndexes4(client, `${basePath}/max`),
    median: createIndexes5(client, `${basePath}/median`),
    min: createIndexes4(client, `${basePath}/min`),
    pct10: createIndexes5(client, `${basePath}/pct10`),
    pct25: createIndexes5(client, `${basePath}/pct25`),
    pct75: createIndexes5(client, `${basePath}/pct75`),
    pct90: createIndexes5(client, `${basePath}/pct90`),
    sum: createIndexes4(client, `${basePath}/sum`)
  };
}

/**
 * @template T
 * @typedef {Object} BlockSizePattern
 * @property {Indexes4<T>} average
 * @property {Indexes3<T>} cumulative
 * @property {Indexes4<T>} max
 * @property {Indexes5<T>} median
 * @property {Indexes4<T>} min
 * @property {Indexes5<T>} pct10
 * @property {Indexes5<T>} pct25
 * @property {Indexes5<T>} pct75
 * @property {Indexes5<T>} pct90
 * @property {Indexes4<T>} sum
 */

/**
 * Create a BlockSizePattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {BlockSizePattern<T>}
 */
function createBlockSizePattern(client, basePath) {
  return {
    average: createIndexes4(client, `${basePath}/average`),
    cumulative: createIndexes3(client, `${basePath}/cumulative`),
    max: createIndexes4(client, `${basePath}/max`),
    median: createIndexes5(client, `${basePath}/median`),
    min: createIndexes4(client, `${basePath}/min`),
    pct10: createIndexes5(client, `${basePath}/pct10`),
    pct25: createIndexes5(client, `${basePath}/pct25`),
    pct75: createIndexes5(client, `${basePath}/pct75`),
    pct90: createIndexes5(client, `${basePath}/pct90`),
    sum: createIndexes4(client, `${basePath}/sum`)
  };
}

/**
 * @typedef {Object} RelativePattern
 * @property {Indexes27<StoredF32>} negUnrealizedLossRelToMarketCap
 * @property {Indexes26<StoredF32>} netUnrealizedPnlRelToMarketCap
 * @property {Indexes27<StoredF64>} supplyInLossRelToCirculatingSupply
 * @property {Indexes27<StoredF64>} supplyInLossRelToOwnSupply
 * @property {Indexes27<StoredF64>} supplyInProfitRelToCirculatingSupply
 * @property {Indexes27<StoredF64>} supplyInProfitRelToOwnSupply
 * @property {Indexes<StoredF64>} supplyRelToCirculatingSupply
 * @property {Indexes27<StoredF32>} unrealizedLossRelToMarketCap
 * @property {Indexes27<StoredF32>} unrealizedProfitRelToMarketCap
 */

/**
 * Create a RelativePattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {RelativePattern}
 */
function createRelativePattern(client, basePath) {
  return {
    negUnrealizedLossRelToMarketCap: createIndexes27(client, `${basePath}/neg_unrealized_loss_rel_to_market_cap`),
    netUnrealizedPnlRelToMarketCap: createIndexes26(client, `${basePath}/net_unrealized_pnl_rel_to_market_cap`),
    supplyInLossRelToCirculatingSupply: createIndexes27(client, `${basePath}/supply_in_loss_rel_to_circulating_supply`),
    supplyInLossRelToOwnSupply: createIndexes27(client, `${basePath}/supply_in_loss_rel_to_own_supply`),
    supplyInProfitRelToCirculatingSupply: createIndexes27(client, `${basePath}/supply_in_profit_rel_to_circulating_supply`),
    supplyInProfitRelToOwnSupply: createIndexes27(client, `${basePath}/supply_in_profit_rel_to_own_supply`),
    supplyRelToCirculatingSupply: createIndexes(client, `${basePath}/supply_rel_to_circulating_supply`),
    unrealizedLossRelToMarketCap: createIndexes27(client, `${basePath}/unrealized_loss_rel_to_market_cap`),
    unrealizedProfitRelToMarketCap: createIndexes27(client, `${basePath}/unrealized_profit_rel_to_market_cap`)
  };
}

/**
 * @typedef {Object} UnrealizedPattern
 * @property {Indexes26<Dollars>} negUnrealizedLoss
 * @property {Indexes26<Dollars>} netUnrealizedPnl
 * @property {SupplyPattern} supplyInLoss
 * @property {SupplyValuePattern} supplyInLossValue
 * @property {SupplyPattern} supplyInProfit
 * @property {SupplyValuePattern} supplyInProfitValue
 * @property {Indexes26<Dollars>} totalUnrealizedPnl
 * @property {Indexes26<Dollars>} unrealizedLoss
 * @property {Indexes26<Dollars>} unrealizedProfit
 */

/**
 * Create a UnrealizedPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {UnrealizedPattern}
 */
function createUnrealizedPattern(client, basePath) {
  return {
    negUnrealizedLoss: createIndexes26(client, `${basePath}/neg_unrealized_loss`),
    netUnrealizedPnl: createIndexes26(client, `${basePath}/net_unrealized_pnl`),
    supplyInLoss: createSupplyPattern(client, `${basePath}/supply_in_loss`),
    supplyInLossValue: createSupplyValuePattern(client, `${basePath}/supply_in_loss_value`),
    supplyInProfit: createSupplyPattern(client, `${basePath}/supply_in_profit`),
    supplyInProfitValue: createSupplyValuePattern(client, `${basePath}/supply_in_profit_value`),
    totalUnrealizedPnl: createIndexes26(client, `${basePath}/total_unrealized_pnl`),
    unrealizedLoss: createIndexes26(client, `${basePath}/unrealized_loss`),
    unrealizedProfit: createIndexes26(client, `${basePath}/unrealized_profit`)
  };
}

/**
 * @template T
 * @typedef {Object} Constant0Pattern
 * @property {Indexes5<T>} dateindex
 * @property {Indexes7<T>} decadeindex
 * @property {Indexes2<T>} height
 * @property {Indexes8<T>} monthindex
 * @property {Indexes9<T>} quarterindex
 * @property {Indexes10<T>} semesterindex
 * @property {Indexes11<T>} weekindex
 * @property {Indexes12<T>} yearindex
 */

/**
 * Create a Constant0Pattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {Constant0Pattern<T>}
 */
function createConstant0Pattern(client, acc) {
  return {
    dateindex: createIndexes5(client, `/${acc}`),
    decadeindex: createIndexes7(client, `/${acc}`),
    height: createIndexes2(client, `/${acc}`),
    monthindex: createIndexes8(client, `/${acc}`),
    quarterindex: createIndexes9(client, `/${acc}`),
    semesterindex: createIndexes10(client, `/${acc}`),
    weekindex: createIndexes11(client, `/${acc}`),
    yearindex: createIndexes12(client, `/${acc}`)
  };
}

/**
 * @template T
 * @typedef {Object} AddresstypeToHeightToAddrCountPattern
 * @property {Indexes16<T>} p2a
 * @property {Indexes17<T>} p2pk33
 * @property {Indexes18<T>} p2pk65
 * @property {Indexes19<T>} p2pkh
 * @property {Indexes20<T>} p2sh
 * @property {Indexes21<T>} p2tr
 * @property {Indexes22<T>} p2wpkh
 * @property {Indexes23<T>} p2wsh
 */

/**
 * Create a AddresstypeToHeightToAddrCountPattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {AddresstypeToHeightToAddrCountPattern<T>}
 */
function createAddresstypeToHeightToAddrCountPattern(client, basePath) {
  return {
    p2a: createIndexes16(client, `${basePath}/p2a`),
    p2pk33: createIndexes17(client, `${basePath}/p2pk33`),
    p2pk65: createIndexes18(client, `${basePath}/p2pk65`),
    p2pkh: createIndexes19(client, `${basePath}/p2pkh`),
    p2sh: createIndexes20(client, `${basePath}/p2sh`),
    p2tr: createIndexes21(client, `${basePath}/p2tr`),
    p2wpkh: createIndexes22(client, `${basePath}/p2wpkh`),
    p2wsh: createIndexes23(client, `${basePath}/p2wsh`)
  };
}

/**
 * @template T
 * @typedef {Object} BlockIntervalPattern
 * @property {Indexes3<T>} average
 * @property {Indexes3<T>} max
 * @property {Indexes2<T>} median
 * @property {Indexes3<T>} min
 * @property {Indexes2<T>} pct10
 * @property {Indexes2<T>} pct25
 * @property {Indexes2<T>} pct75
 * @property {Indexes2<T>} pct90
 */

/**
 * Create a BlockIntervalPattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {BlockIntervalPattern<T>}
 */
function createBlockIntervalPattern(client, acc) {
  return {
    average: createIndexes3(client, `/${acc}_avg`),
    max: createIndexes3(client, `/${acc}_max`),
    median: createIndexes2(client, `/${acc}_median`),
    min: createIndexes3(client, `/${acc}_min`),
    pct10: createIndexes2(client, `/${acc}_pct10`),
    pct25: createIndexes2(client, `/${acc}_pct25`),
    pct75: createIndexes2(client, `/${acc}_pct75`),
    pct90: createIndexes2(client, `/${acc}_pct90`)
  };
}

/**
 * @typedef {Object} _0satsPattern
 * @property {ActivityPattern} activity
 * @property {Indexes3<StoredU64>} addrCount
 * @property {PricePaidPattern} pricePaid
 * @property {RealizedPattern} realized
 * @property {RelativePattern} relative
 * @property {SupplyPattern2} supply
 * @property {UnrealizedPattern} unrealized
 */

/**
 * Create a _0satsPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {_0satsPattern}
 */
function create_0satsPattern(client, basePath) {
  return {
    activity: createActivityPattern(client, `${basePath}/activity`),
    addrCount: createIndexes3(client, `${basePath}/addr_count`),
    pricePaid: createPricePaidPattern(client, `${basePath}/price_paid`),
    realized: createRealizedPattern(client, `${basePath}/realized`),
    relative: createRelativePattern(client, `${basePath}/relative`),
    supply: createSupplyPattern2(client, `${basePath}/supply`),
    unrealized: createUnrealizedPattern(client, `${basePath}/unrealized`)
  };
}

/**
 * @typedef {Object} UpTo1dPattern
 * @property {ActivityPattern} activity
 * @property {PricePaidPattern2} pricePaid
 * @property {RealizedPattern3} realized
 * @property {RelativePattern2} relative
 * @property {SupplyPattern2} supply
 * @property {UnrealizedPattern} unrealized
 */

/**
 * Create a UpTo1dPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {UpTo1dPattern}
 */
function createUpTo1dPattern(client, basePath) {
  return {
    activity: createActivityPattern(client, `${basePath}/activity`),
    pricePaid: createPricePaidPattern2(client, `${basePath}/price_paid`),
    realized: createRealizedPattern3(client, `${basePath}/realized`),
    relative: createRelativePattern2(client, `${basePath}/relative`),
    supply: createSupplyPattern2(client, `${basePath}/supply`),
    unrealized: createUnrealizedPattern(client, `${basePath}/unrealized`)
  };
}

/**
 * @typedef {Object} _0satsPattern2
 * @property {ActivityPattern} activity
 * @property {PricePaidPattern} pricePaid
 * @property {RealizedPattern} realized
 * @property {RelativePattern} relative
 * @property {SupplyPattern2} supply
 * @property {UnrealizedPattern} unrealized
 */

/**
 * Create a _0satsPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {_0satsPattern2}
 */
function create_0satsPattern2(client, basePath) {
  return {
    activity: createActivityPattern(client, `${basePath}/activity`),
    pricePaid: createPricePaidPattern(client, `${basePath}/price_paid`),
    realized: createRealizedPattern(client, `${basePath}/realized`),
    relative: createRelativePattern(client, `${basePath}/relative`),
    supply: createSupplyPattern2(client, `${basePath}/supply`),
    unrealized: createUnrealizedPattern(client, `${basePath}/unrealized`)
  };
}

/**
 * @typedef {Object} _10yTo12yPattern
 * @property {ActivityPattern} activity
 * @property {PricePaidPattern2} pricePaid
 * @property {RealizedPattern2} realized
 * @property {RelativePattern2} relative
 * @property {SupplyPattern2} supply
 * @property {UnrealizedPattern} unrealized
 */

/**
 * Create a _10yTo12yPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {_10yTo12yPattern}
 */
function create_10yTo12yPattern(client, basePath) {
  return {
    activity: createActivityPattern(client, `${basePath}/activity`),
    pricePaid: createPricePaidPattern2(client, `${basePath}/price_paid`),
    realized: createRealizedPattern2(client, `${basePath}/realized`),
    relative: createRelativePattern2(client, `${basePath}/relative`),
    supply: createSupplyPattern2(client, `${basePath}/supply`),
    unrealized: createUnrealizedPattern(client, `${basePath}/unrealized`)
  };
}

/**
 * @typedef {Object} ActivityPattern
 * @property {BlockCountPattern<StoredF64>} coinblocksDestroyed
 * @property {BlockCountPattern<StoredF64>} coindaysDestroyed
 * @property {Indexes2<Sats>} satblocksDestroyed
 * @property {Indexes2<Sats>} satdaysDestroyed
 * @property {FeePattern2} sent
 */

/**
 * Create a ActivityPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {ActivityPattern}
 */
function createActivityPattern(client, basePath) {
  return {
    coinblocksDestroyed: createBlockCountPattern(client, `${basePath}/coinblocks_destroyed`),
    coindaysDestroyed: createBlockCountPattern(client, `${basePath}/coindays_destroyed`),
    satblocksDestroyed: createIndexes2(client, `${basePath}/satblocks_destroyed`),
    satdaysDestroyed: createIndexes2(client, `${basePath}/satdays_destroyed`),
    sent: createFeePattern2(client, `${basePath}/sent`)
  };
}

/**
 * @typedef {Object} SupplyPattern2
 * @property {SupplyPattern} supply
 * @property {ActiveSupplyPattern} supplyHalf
 * @property {ActiveSupplyPattern} supplyHalfValue
 * @property {SupplyValuePattern} supplyValue
 * @property {Indexes3<StoredU64>} utxoCount
 */

/**
 * Create a SupplyPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {SupplyPattern2}
 */
function createSupplyPattern2(client, basePath) {
  return {
    supply: createSupplyPattern(client, `${basePath}/supply`),
    supplyHalf: createActiveSupplyPattern(client, `${basePath}/supply_half`),
    supplyHalfValue: createActiveSupplyPattern(client, `${basePath}/supply_half_value`),
    supplyValue: createSupplyValuePattern(client, `${basePath}/supply_value`),
    utxoCount: createIndexes3(client, `${basePath}/utxo_count`)
  };
}

/**
 * @typedef {Object} FeePattern2
 * @property {Indexes2<Sats>} base
 * @property {BlockCountPattern<Bitcoin>} bitcoin
 * @property {BlockCountPattern<Dollars>} dollars
 * @property {SatsPattern} sats
 */

/**
 * Create a FeePattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {FeePattern2}
 */
function createFeePattern2(client, basePath) {
  return {
    base: createIndexes2(client, `${basePath}/base`),
    bitcoin: createBlockCountPattern(client, `${basePath}/bitcoin`),
    dollars: createBlockCountPattern(client, `${basePath}/dollars`),
    sats: createSatsPattern(client, `${basePath}/sats`)
  };
}

/**
 * @typedef {Object} SupplyPattern
 * @property {Indexes2<Sats>} base
 * @property {Indexes<Bitcoin>} bitcoin
 * @property {Indexes<Dollars>} dollars
 * @property {Indexes<Sats>} sats
 */

/**
 * Create a SupplyPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {SupplyPattern}
 */
function createSupplyPattern(client, basePath) {
  return {
    base: createIndexes2(client, `${basePath}/base`),
    bitcoin: createIndexes(client, `${basePath}/bitcoin`),
    dollars: createIndexes(client, `${basePath}/dollars`),
    sats: createIndexes(client, `${basePath}/sats`)
  };
}

/**
 * @typedef {Object} UnclaimedRewardsPattern
 * @property {BlockCountPattern<Bitcoin>} bitcoin
 * @property {BlockCountPattern<Dollars>} dollars
 * @property {BlockCountPattern<Sats>} sats
 */

/**
 * Create a UnclaimedRewardsPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {UnclaimedRewardsPattern}
 */
function createUnclaimedRewardsPattern(client, basePath) {
  return {
    bitcoin: createBlockCountPattern(client, `${basePath}/bitcoin`),
    dollars: createBlockCountPattern(client, `${basePath}/dollars`),
    sats: createBlockCountPattern(client, `${basePath}/sats`)
  };
}

/**
 * @typedef {Object} PricePaidPattern2
 * @property {Indexes3<Dollars>} maxPricePaid
 * @property {Indexes3<Dollars>} minPricePaid
 * @property {PricePercentilesPattern} pricePercentiles
 */

/**
 * Create a PricePaidPattern2 pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {PricePaidPattern2}
 */
function createPricePaidPattern2(client, basePath) {
  return {
    maxPricePaid: createIndexes3(client, `${basePath}/max_price_paid`),
    minPricePaid: createIndexes3(client, `${basePath}/min_price_paid`),
    pricePercentiles: createPricePercentilesPattern(client, `${basePath}/price_percentiles`)
  };
}

/**
 * @typedef {Object} CoinbasePattern
 * @property {BitcoinPattern<Bitcoin>} bitcoin
 * @property {BitcoinPattern<Dollars>} dollars
 * @property {BitcoinPattern<Sats>} sats
 */

/**
 * Create a CoinbasePattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {CoinbasePattern}
 */
function createCoinbasePattern(client, basePath) {
  return {
    bitcoin: createBitcoinPattern(client, `${basePath}/bitcoin`),
    dollars: createBitcoinPattern(client, `${basePath}/dollars`),
    sats: createBitcoinPattern(client, `${basePath}/sats`)
  };
}

/**
 * @typedef {Object} ActiveSupplyPattern
 * @property {Indexes3<Bitcoin>} bitcoin
 * @property {Indexes3<Dollars>} dollars
 * @property {Indexes3<Sats>} sats
 */

/**
 * Create a ActiveSupplyPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {ActiveSupplyPattern}
 */
function createActiveSupplyPattern(client, basePath) {
  return {
    bitcoin: createIndexes3(client, `${basePath}/bitcoin`),
    dollars: createIndexes3(client, `${basePath}/dollars`),
    sats: createIndexes3(client, `${basePath}/sats`)
  };
}

/**
 * @template T
 * @typedef {Object} BlockCountPattern
 * @property {Indexes2<T>} base
 * @property {Indexes3<T>} cumulative
 * @property {Indexes4<T>} sum
 */

/**
 * Create a BlockCountPattern pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {BlockCountPattern<T>}
 */
function createBlockCountPattern(client, basePath) {
  return {
    base: createIndexes2(client, `${basePath}/base`),
    cumulative: createIndexes3(client, `${basePath}/cumulative`),
    sum: createIndexes4(client, `${basePath}/sum`)
  };
}

/**
 * @typedef {Object} PricePaidPattern
 * @property {Indexes3<Dollars>} maxPricePaid
 * @property {Indexes3<Dollars>} minPricePaid
 */

/**
 * Create a PricePaidPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {PricePaidPattern}
 */
function createPricePaidPattern(client, basePath) {
  return {
    maxPricePaid: createIndexes3(client, `${basePath}/max_price_paid`),
    minPricePaid: createIndexes3(client, `${basePath}/min_price_paid`)
  };
}

/**
 * @typedef {Object} _1dReturns1mSdPattern
 * @property {Indexes<StoredF32>} sd
 * @property {Indexes<StoredF32>} sma
 */

/**
 * Create a _1dReturns1mSdPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} acc - Accumulated metric name
 * @returns {_1dReturns1mSdPattern}
 */
function create_1dReturns1mSdPattern(client, acc) {
  return {
    sd: createIndexes(client, `/${acc}_sd`),
    sma: createIndexes(client, `/${acc}_sma`)
  };
}

/**
 * @typedef {Object} SupplyValuePattern
 * @property {Indexes2<Bitcoin>} bitcoin
 * @property {Indexes2<Dollars>} dollars
 */

/**
 * Create a SupplyValuePattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {SupplyValuePattern}
 */
function createSupplyValuePattern(client, basePath) {
  return {
    bitcoin: createIndexes2(client, `${basePath}/bitcoin`),
    dollars: createIndexes2(client, `${basePath}/dollars`)
  };
}

/**
 * @typedef {Object} SatsPattern
 * @property {Indexes3<Sats>} cumulative
 * @property {Indexes4<Sats>} sum
 */

/**
 * Create a SatsPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {SatsPattern}
 */
function createSatsPattern(client, basePath) {
  return {
    cumulative: createIndexes3(client, `${basePath}/cumulative`),
    sum: createIndexes4(client, `${basePath}/sum`)
  };
}

/**
 * @template T
 * @typedef {Object} BitcoinPattern2
 * @property {Indexes2<T>} base
 * @property {Indexes4<T>} sum
 */

/**
 * Create a BitcoinPattern2 pattern node
 * @template T
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {BitcoinPattern2<T>}
 */
function createBitcoinPattern2(client, basePath) {
  return {
    base: createIndexes2(client, `${basePath}/base`),
    sum: createIndexes4(client, `${basePath}/sum`)
  };
}

/**
 * @typedef {Object} RealizedPriceExtraPattern
 * @property {Indexes<StoredF32>} ratio
 */

/**
 * Create a RealizedPriceExtraPattern pattern node
 * @param {BrkClientBase} client
 * @param {string} basePath
 * @returns {RealizedPriceExtraPattern}
 */
function createRealizedPriceExtraPattern(client, basePath) {
  return {
    ratio: createIndexes(client, `${basePath}/ratio`)
  };
}

// Catalog tree typedefs

/**
 * @typedef {Object} CatalogTree
 * @property {CatalogTree_Computed} computed
 * @property {CatalogTree_Indexed} indexed
 */

/**
 * @typedef {Object} CatalogTree_Computed
 * @property {CatalogTree_Computed_Blks} blks
 * @property {CatalogTree_Computed_Chain} chain
 * @property {CatalogTree_Computed_Cointime} cointime
 * @property {CatalogTree_Computed_Constants} constants
 * @property {CatalogTree_Computed_Fetched} fetched
 * @property {CatalogTree_Computed_Indexes} indexes
 * @property {CatalogTree_Computed_Market} market
 * @property {CatalogTree_Computed_Pools} pools
 * @property {CatalogTree_Computed_Price} price
 * @property {CatalogTree_Computed_Stateful} stateful
 * @property {CatalogTree_Computed_Txins} txins
 * @property {CatalogTree_Computed_Txouts} txouts
 */

/**
 * @typedef {Object} CatalogTree_Computed_Blks
 * @property {MetricNode<BlkPosition>} position
 */

/**
 * @typedef {Object} CatalogTree_Computed_Chain
 * @property {Indexes<StoredU32>} _1mBlockCount
 * @property {Indexes<StoredU32>} _1wBlockCount
 * @property {Indexes<StoredU32>} _1yBlockCount
 * @property {Indexes2<StoredU32>} _24hBlockCount
 * @property {Indexes2<Sats>} _24hCoinbaseSum
 * @property {Indexes2<Dollars>} _24hCoinbaseUsdSum
 * @property {Indexes<Sats>} annualizedVolume
 * @property {Indexes<Bitcoin>} annualizedVolumeBtc
 * @property {Indexes<Dollars>} annualizedVolumeUsd
 * @property {BlockCountPattern<StoredU32>} blockCount
 * @property {Indexes<StoredU64>} blockCountTarget
 * @property {BlockIntervalPattern<Timestamp>} blockInterval
 * @property {BlockSizePattern<StoredU64>} blockSize
 * @property {BlockSizePattern<StoredU64>} blockVbytes
 * @property {BlockSizePattern<Weight>} blockWeight
 * @property {Indexes3<StoredU32>} blocksBeforeNextDifficultyAdjustment
 * @property {Indexes3<StoredU32>} blocksBeforeNextHalving
 * @property {CoinbasePattern} coinbase
 * @property {Indexes3<StoredF32>} daysBeforeNextDifficultyAdjustment
 * @property {Indexes3<StoredF32>} daysBeforeNextHalving
 * @property {Indexes4<StoredF64>} difficulty
 * @property {Indexes3<StoredF32>} difficultyAdjustment
 * @property {Indexes3<StoredF32>} difficultyAsHash
 * @property {Indexes<DifficultyEpoch>} difficultyepoch
 * @property {BitcoinPattern<StoredU64>} emptyoutputCount
 * @property {Indexes3<StoredU64>} exactUtxoCount
 * @property {CatalogTree_Computed_Chain_Fee} fee
 * @property {Indexes5<StoredF32>} feeDominance
 * @property {CatalogTree_Computed_Chain_FeeRate} feeRate
 * @property {Indexes<HalvingEpoch>} halvingepoch
 * @property {Indexes3<StoredF32>} hashPricePhs
 * @property {Indexes3<StoredF32>} hashPricePhsMin
 * @property {Indexes3<StoredF32>} hashPriceRebound
 * @property {Indexes3<StoredF32>} hashPriceThs
 * @property {Indexes3<StoredF32>} hashPriceThsMin
 * @property {Indexes3<StoredF64>} hashRate
 * @property {Indexes<StoredF32>} hashRate1mSma
 * @property {Indexes<StoredF64>} hashRate1wSma
 * @property {Indexes<StoredF32>} hashRate1ySma
 * @property {Indexes<StoredF32>} hashRate2mSma
 * @property {Indexes3<StoredF32>} hashValuePhs
 * @property {Indexes3<StoredF32>} hashValuePhsMin
 * @property {Indexes3<StoredF32>} hashValueRebound
 * @property {Indexes3<StoredF32>} hashValueThs
 * @property {Indexes3<StoredF32>} hashValueThsMin
 * @property {Indexes<StoredF32>} inflationRate
 * @property {BlockSizePattern<StoredU64>} inputCount
 * @property {Indexes6<Sats>} inputValue
 * @property {Indexes<StoredF32>} inputsPerSec
 * @property {Indexes2<Timestamp>} interval
 * @property {Indexes6<StoredBool>} isCoinbase
 * @property {BitcoinPattern<StoredU64>} opreturnCount
 * @property {BlockSizePattern<StoredU64>} outputCount
 * @property {Indexes6<Sats>} outputValue
 * @property {Indexes<StoredF32>} outputsPerSec
 * @property {BitcoinPattern<StoredU64>} p2aCount
 * @property {BitcoinPattern<StoredU64>} p2msCount
 * @property {BitcoinPattern<StoredU64>} p2pk33Count
 * @property {BitcoinPattern<StoredU64>} p2pk65Count
 * @property {BitcoinPattern<StoredU64>} p2pkhCount
 * @property {BitcoinPattern<StoredU64>} p2shCount
 * @property {BitcoinPattern<StoredU64>} p2trCount
 * @property {BitcoinPattern<StoredU64>} p2wpkhCount
 * @property {BitcoinPattern<StoredU64>} p2wshCount
 * @property {Indexes<StoredF32>} puellMultiple
 * @property {CatalogTree_Computed_Chain_SentSum} sentSum
 * @property {CoinbasePattern} subsidy
 * @property {Indexes5<StoredF32>} subsidyDominance
 * @property {Indexes<Dollars>} subsidyUsd1ySma
 * @property {MetricNode<Timestamp>} timestamp
 * @property {Indexes<StoredF64>} txBtcVelocity
 * @property {BitcoinPattern<StoredU64>} txCount
 * @property {Indexes<StoredF32>} txPerSec
 * @property {Indexes<StoredF64>} txUsdVelocity
 * @property {BlockCountPattern<StoredU64>} txV1
 * @property {BlockCountPattern<StoredU64>} txV2
 * @property {BlockCountPattern<StoredU64>} txV3
 * @property {BlockIntervalPattern<VSize>} txVsize
 * @property {BlockIntervalPattern<Weight>} txWeight
 * @property {UnclaimedRewardsPattern} unclaimedRewards
 * @property {BitcoinPattern<StoredU64>} unknownoutputCount
 * @property {Indexes2<StoredU64>} vbytes
 * @property {Indexes6<VSize>} vsize
 * @property {Indexes6<Weight>} weight
 */

/**
 * @typedef {Object} CatalogTree_Computed_Chain_Fee
 * @property {Indexes6<Sats>} base
 * @property {BlockSizePattern<Bitcoin>} bitcoin
 * @property {Indexes6<Bitcoin>} bitcoinTxindex
 * @property {BlockSizePattern<Dollars>} dollars
 * @property {Indexes6<Dollars>} dollarsTxindex
 * @property {BlockSizePattern<Sats>} sats
 */

/**
 * @typedef {Object} CatalogTree_Computed_Chain_FeeRate
 * @property {Indexes3<FeeRate>} average
 * @property {Indexes6<FeeRate>} base
 * @property {Indexes3<FeeRate>} max
 * @property {Indexes2<FeeRate>} median
 * @property {Indexes3<FeeRate>} min
 * @property {Indexes2<FeeRate>} pct10
 * @property {Indexes2<FeeRate>} pct25
 * @property {Indexes2<FeeRate>} pct75
 * @property {Indexes2<FeeRate>} pct90
 */

/**
 * @typedef {Object} CatalogTree_Computed_Chain_SentSum
 * @property {BitcoinPattern2<Bitcoin>} bitcoin
 * @property {Indexes3<Dollars>} dollars
 * @property {Indexes3<Sats>} sats
 */

/**
 * @typedef {Object} CatalogTree_Computed_Cointime
 * @property {Indexes3<Dollars>} activeCap
 * @property {Indexes3<Dollars>} activePrice
 * @property {ActivePriceRatioPattern} activePriceRatio
 * @property {ActiveSupplyPattern} activeSupply
 * @property {Indexes3<StoredF64>} activityToVaultednessRatio
 * @property {BlockCountPattern<StoredF64>} coinblocksCreated
 * @property {BlockCountPattern<StoredF64>} coinblocksStored
 * @property {Indexes<StoredF32>} cointimeAdjInflationRate
 * @property {Indexes<StoredF64>} cointimeAdjTxBtcVelocity
 * @property {Indexes<StoredF64>} cointimeAdjTxUsdVelocity
 * @property {Indexes3<Dollars>} cointimeCap
 * @property {Indexes3<Dollars>} cointimePrice
 * @property {ActivePriceRatioPattern} cointimePriceRatio
 * @property {BlockCountPattern<StoredF64>} cointimeValueCreated
 * @property {BlockCountPattern<StoredF64>} cointimeValueDestroyed
 * @property {BlockCountPattern<StoredF64>} cointimeValueStored
 * @property {Indexes3<Dollars>} investorCap
 * @property {Indexes3<StoredF64>} liveliness
 * @property {Indexes3<Dollars>} thermoCap
 * @property {Indexes3<Dollars>} trueMarketMean
 * @property {ActivePriceRatioPattern} trueMarketMeanRatio
 * @property {Indexes3<Dollars>} vaultedCap
 * @property {Indexes3<Dollars>} vaultedPrice
 * @property {ActivePriceRatioPattern} vaultedPriceRatio
 * @property {ActiveSupplyPattern} vaultedSupply
 * @property {Indexes3<StoredF64>} vaultedness
 */

/**
 * @typedef {Object} CatalogTree_Computed_Constants
 * @property {Constant0Pattern<StoredU16>} constant0
 * @property {Constant0Pattern<StoredU16>} constant1
 * @property {Constant0Pattern<StoredU16>} constant100
 * @property {Constant0Pattern<StoredU16>} constant2
 * @property {Constant0Pattern<StoredU16>} constant3
 * @property {Constant0Pattern<StoredF32>} constant382
 * @property {Constant0Pattern<StoredU16>} constant4
 * @property {Constant0Pattern<StoredU16>} constant50
 * @property {Constant0Pattern<StoredU16>} constant600
 * @property {Constant0Pattern<StoredF32>} constant618
 * @property {Constant0Pattern<StoredI16>} constantMinus1
 * @property {Constant0Pattern<StoredI16>} constantMinus2
 * @property {Constant0Pattern<StoredI16>} constantMinus3
 * @property {Constant0Pattern<StoredI16>} constantMinus4
 */

/**
 * @typedef {Object} CatalogTree_Computed_Fetched
 * @property {Indexes13<OHLCCents>} priceOhlcInCents
 */

/**
 * @typedef {Object} CatalogTree_Computed_Indexes
 * @property {Indexes13<Date>} date
 * @property {Indexes2<Date>} dateFixed
 * @property {Indexes13<DateIndex>} dateindex
 * @property {Indexes14<StoredU64>} dateindexCount
 * @property {MetricNode<DecadeIndex>} decadeindex
 * @property {MetricNode<DifficultyEpoch>} difficultyepoch
 * @property {MetricNode<EmptyOutputIndex>} emptyoutputindex
 * @property {Indexes14<DateIndex>} firstDateindex
 * @property {MetricNode<Height>} firstHeight
 * @property {Indexes15<MonthIndex>} firstMonthindex
 * @property {Indexes7<YearIndex>} firstYearindex
 * @property {MetricNode<HalvingEpoch>} halvingepoch
 * @property {Indexes2<Height>} height
 * @property {MetricNode<StoredU64>} heightCount
 * @property {Indexes6<StoredU64>} inputCount
 * @property {MetricNode<MonthIndex>} monthindex
 * @property {Indexes15<StoredU64>} monthindexCount
 * @property {MetricNode<OpReturnIndex>} opreturnindex
 * @property {Indexes6<StoredU64>} outputCount
 * @property {Indexes16<P2AAddressIndex>} p2aaddressindex
 * @property {MetricNode<P2MSOutputIndex>} p2msoutputindex
 * @property {Indexes17<P2PK33AddressIndex>} p2pk33addressindex
 * @property {Indexes18<P2PK65AddressIndex>} p2pk65addressindex
 * @property {Indexes19<P2PKHAddressIndex>} p2pkhaddressindex
 * @property {Indexes20<P2SHAddressIndex>} p2shaddressindex
 * @property {Indexes21<P2TRAddressIndex>} p2traddressindex
 * @property {Indexes22<P2WPKHAddressIndex>} p2wpkhaddressindex
 * @property {Indexes23<P2WSHAddressIndex>} p2wshaddressindex
 * @property {MetricNode<QuarterIndex>} quarterindex
 * @property {MetricNode<SemesterIndex>} semesterindex
 * @property {Indexes2<Timestamp>} timestampFixed
 * @property {Indexes6<TxIndex>} txindex
 * @property {Indexes2<StoredU64>} txindexCount
 * @property {Indexes24<TxInIndex>} txinindex
 * @property {Indexes25<TxOutIndex>} txoutindex
 * @property {MetricNode<UnknownOutputIndex>} unknownoutputindex
 * @property {MetricNode<WeekIndex>} weekindex
 * @property {MetricNode<YearIndex>} yearindex
 * @property {Indexes7<StoredU64>} yearindexCount
 */

/**
 * @typedef {Object} CatalogTree_Computed_Market
 * @property {_1dReturns1mSdPattern} _1dReturns1mSd
 * @property {_1dReturns1mSdPattern} _1dReturns1wSd
 * @property {_1dReturns1mSdPattern} _1dReturns1ySd
 * @property {Indexes<StoredF32>} _10yCagr
 * @property {Indexes<Dollars>} _10yDcaAvgPrice
 * @property {Indexes<StoredF32>} _10yDcaCagr
 * @property {Indexes<StoredF32>} _10yDcaReturns
 * @property {Indexes<Sats>} _10yDcaStack
 * @property {Indexes<StoredF32>} _10yPriceReturns
 * @property {Indexes<StoredF32>} _1dPriceReturns
 * @property {Indexes<Dollars>} _1mDcaAvgPrice
 * @property {Indexes<StoredF32>} _1mDcaReturns
 * @property {Indexes<Sats>} _1mDcaStack
 * @property {Indexes<StoredF32>} _1mPriceReturns
 * @property {Indexes<Dollars>} _1wDcaAvgPrice
 * @property {Indexes<StoredF32>} _1wDcaReturns
 * @property {Indexes<Sats>} _1wDcaStack
 * @property {Indexes<StoredF32>} _1wPriceReturns
 * @property {Indexes<Dollars>} _1yDcaAvgPrice
 * @property {Indexes<StoredF32>} _1yDcaReturns
 * @property {Indexes<Sats>} _1yDcaStack
 * @property {Indexes<StoredF32>} _1yPriceReturns
 * @property {Indexes<StoredF32>} _2yCagr
 * @property {Indexes<Dollars>} _2yDcaAvgPrice
 * @property {Indexes<StoredF32>} _2yDcaCagr
 * @property {Indexes<StoredF32>} _2yDcaReturns
 * @property {Indexes<Sats>} _2yDcaStack
 * @property {Indexes<StoredF32>} _2yPriceReturns
 * @property {Indexes<Dollars>} _3mDcaAvgPrice
 * @property {Indexes<StoredF32>} _3mDcaReturns
 * @property {Indexes<Sats>} _3mDcaStack
 * @property {Indexes<StoredF32>} _3mPriceReturns
 * @property {Indexes<StoredF32>} _3yCagr
 * @property {Indexes<Dollars>} _3yDcaAvgPrice
 * @property {Indexes<StoredF32>} _3yDcaCagr
 * @property {Indexes<StoredF32>} _3yDcaReturns
 * @property {Indexes<Sats>} _3yDcaStack
 * @property {Indexes<StoredF32>} _3yPriceReturns
 * @property {Indexes<StoredF32>} _4yCagr
 * @property {Indexes<Dollars>} _4yDcaAvgPrice
 * @property {Indexes<StoredF32>} _4yDcaCagr
 * @property {Indexes<StoredF32>} _4yDcaReturns
 * @property {Indexes<Sats>} _4yDcaStack
 * @property {Indexes<StoredF32>} _4yPriceReturns
 * @property {Indexes<StoredF32>} _5yCagr
 * @property {Indexes<Dollars>} _5yDcaAvgPrice
 * @property {Indexes<StoredF32>} _5yDcaCagr
 * @property {Indexes<StoredF32>} _5yDcaReturns
 * @property {Indexes<Sats>} _5yDcaStack
 * @property {Indexes<StoredF32>} _5yPriceReturns
 * @property {Indexes<Dollars>} _6mDcaAvgPrice
 * @property {Indexes<StoredF32>} _6mDcaReturns
 * @property {Indexes<Sats>} _6mDcaStack
 * @property {Indexes<StoredF32>} _6mPriceReturns
 * @property {Indexes<StoredF32>} _6yCagr
 * @property {Indexes<Dollars>} _6yDcaAvgPrice
 * @property {Indexes<StoredF32>} _6yDcaCagr
 * @property {Indexes<StoredF32>} _6yDcaReturns
 * @property {Indexes<Sats>} _6yDcaStack
 * @property {Indexes<StoredF32>} _6yPriceReturns
 * @property {Indexes<StoredF32>} _8yCagr
 * @property {Indexes<Dollars>} _8yDcaAvgPrice
 * @property {Indexes<StoredF32>} _8yDcaCagr
 * @property {Indexes<StoredF32>} _8yDcaReturns
 * @property {Indexes<Sats>} _8yDcaStack
 * @property {Indexes<StoredF32>} _8yPriceReturns
 * @property {Indexes<StoredU16>} daysSincePriceAth
 * @property {Indexes<Dollars>} dcaClass2015AvgPrice
 * @property {Indexes<StoredF32>} dcaClass2015Returns
 * @property {Indexes<Sats>} dcaClass2015Stack
 * @property {Indexes<Dollars>} dcaClass2016AvgPrice
 * @property {Indexes<StoredF32>} dcaClass2016Returns
 * @property {Indexes<Sats>} dcaClass2016Stack
 * @property {Indexes<Dollars>} dcaClass2017AvgPrice
 * @property {Indexes<StoredF32>} dcaClass2017Returns
 * @property {Indexes<Sats>} dcaClass2017Stack
 * @property {Indexes<Dollars>} dcaClass2018AvgPrice
 * @property {Indexes<StoredF32>} dcaClass2018Returns
 * @property {Indexes<Sats>} dcaClass2018Stack
 * @property {Indexes<Dollars>} dcaClass2019AvgPrice
 * @property {Indexes<StoredF32>} dcaClass2019Returns
 * @property {Indexes<Sats>} dcaClass2019Stack
 * @property {Indexes<Dollars>} dcaClass2020AvgPrice
 * @property {Indexes<StoredF32>} dcaClass2020Returns
 * @property {Indexes<Sats>} dcaClass2020Stack
 * @property {Indexes<Dollars>} dcaClass2021AvgPrice
 * @property {Indexes<StoredF32>} dcaClass2021Returns
 * @property {Indexes<Sats>} dcaClass2021Stack
 * @property {Indexes<Dollars>} dcaClass2022AvgPrice
 * @property {Indexes<StoredF32>} dcaClass2022Returns
 * @property {Indexes<Sats>} dcaClass2022Stack
 * @property {Indexes<Dollars>} dcaClass2023AvgPrice
 * @property {Indexes<StoredF32>} dcaClass2023Returns
 * @property {Indexes<Sats>} dcaClass2023Stack
 * @property {Indexes<Dollars>} dcaClass2024AvgPrice
 * @property {Indexes<StoredF32>} dcaClass2024Returns
 * @property {Indexes<Sats>} dcaClass2024Stack
 * @property {Indexes<Dollars>} dcaClass2025AvgPrice
 * @property {Indexes<StoredF32>} dcaClass2025Returns
 * @property {Indexes<Sats>} dcaClass2025Stack
 * @property {Indexes<StoredU16>} maxDaysBetweenPriceAths
 * @property {Indexes<StoredF32>} maxYearsBetweenPriceAths
 * @property {Indexes<Dollars>} price10yAgo
 * @property {Price13dEmaPattern} price13dEma
 * @property {Price13dEmaPattern} price13dSma
 * @property {Price13dEmaPattern} price144dEma
 * @property {Price13dEmaPattern} price144dSma
 * @property {Indexes<Dollars>} price1dAgo
 * @property {Indexes<Dollars>} price1mAgo
 * @property {Price13dEmaPattern} price1mEma
 * @property {Indexes<Dollars>} price1mMax
 * @property {Indexes<Dollars>} price1mMin
 * @property {Price13dEmaPattern} price1mSma
 * @property {Indexes<StoredF32>} price1mVolatility
 * @property {Indexes<Dollars>} price1wAgo
 * @property {Price13dEmaPattern} price1wEma
 * @property {Indexes<Dollars>} price1wMax
 * @property {Indexes<Dollars>} price1wMin
 * @property {Price13dEmaPattern} price1wSma
 * @property {Indexes<StoredF32>} price1wVolatility
 * @property {Indexes<Dollars>} price1yAgo
 * @property {Price13dEmaPattern} price1yEma
 * @property {Indexes<Dollars>} price1yMax
 * @property {Indexes<Dollars>} price1yMin
 * @property {Price13dEmaPattern} price1ySma
 * @property {Indexes<StoredF32>} price1yVolatility
 * @property {Price13dEmaPattern} price200dEma
 * @property {Price13dEmaPattern} price200dSma
 * @property {Indexes<Dollars>} price200dSmaX08
 * @property {Indexes<Dollars>} price200dSmaX24
 * @property {Price13dEmaPattern} price200wEma
 * @property {Price13dEmaPattern} price200wSma
 * @property {Price13dEmaPattern} price21dEma
 * @property {Price13dEmaPattern} price21dSma
 * @property {Indexes<StoredF32>} price2wChoppinessIndex
 * @property {Indexes<Dollars>} price2wMax
 * @property {Indexes<Dollars>} price2wMin
 * @property {Indexes<Dollars>} price2yAgo
 * @property {Price13dEmaPattern} price2yEma
 * @property {Price13dEmaPattern} price2ySma
 * @property {Price13dEmaPattern} price34dEma
 * @property {Price13dEmaPattern} price34dSma
 * @property {Indexes<Dollars>} price3mAgo
 * @property {Indexes<Dollars>} price3yAgo
 * @property {Indexes<Dollars>} price4yAgo
 * @property {Price13dEmaPattern} price4yEma
 * @property {Price13dEmaPattern} price4ySma
 * @property {Price13dEmaPattern} price55dEma
 * @property {Price13dEmaPattern} price55dSma
 * @property {Indexes<Dollars>} price5yAgo
 * @property {Indexes<Dollars>} price6mAgo
 * @property {Indexes<Dollars>} price6yAgo
 * @property {Price13dEmaPattern} price89dEma
 * @property {Price13dEmaPattern} price89dSma
 * @property {Price13dEmaPattern} price8dEma
 * @property {Price13dEmaPattern} price8dSma
 * @property {Indexes<Dollars>} price8yAgo
 * @property {Indexes26<Dollars>} priceAth
 * @property {Indexes26<StoredF32>} priceDrawdown
 * @property {Indexes5<StoredF32>} priceTrueRange
 * @property {Indexes5<StoredF32>} priceTrueRange2wSum
 */

/**
 * @typedef {Object} CatalogTree_Computed_Pools
 * @property {Indexes2<PoolSlug>} pool
 * @property {CatalogTree_Computed_Pools_Vecs} vecs
 */

/**
 * @typedef {Object} CatalogTree_Computed_Pools_Vecs
 * @property {AXbtPattern} aXbt
 * @property {AXbtPattern} aaoPool
 * @property {AXbtPattern} antPool
 * @property {AXbtPattern} arkPool
 * @property {AXbtPattern} asicMiner
 * @property {AXbtPattern} batPool
 * @property {AXbtPattern} bcMonster
 * @property {AXbtPattern} bcpoolIo
 * @property {AXbtPattern} binancePool
 * @property {AXbtPattern} bitClub
 * @property {AXbtPattern} bitFuFuPool
 * @property {AXbtPattern} bitFury
 * @property {AXbtPattern} bitMinter
 * @property {AXbtPattern} bitalo
 * @property {AXbtPattern} bitcoinAffiliateNetwork
 * @property {AXbtPattern} bitcoinCom
 * @property {AXbtPattern} bitcoinIndia
 * @property {AXbtPattern} bitcoinRussia
 * @property {AXbtPattern} bitcoinUkraine
 * @property {AXbtPattern} bitfarms
 * @property {AXbtPattern} bitparking
 * @property {AXbtPattern} bitsolo
 * @property {AXbtPattern} bixin
 * @property {AXbtPattern} blockFills
 * @property {AXbtPattern} braiinsPool
 * @property {AXbtPattern} bravoMining
 * @property {AXbtPattern} btPool
 * @property {AXbtPattern} btcCom
 * @property {AXbtPattern} btcDig
 * @property {AXbtPattern} btcGuild
 * @property {AXbtPattern} btcLab
 * @property {AXbtPattern} btcMp
 * @property {AXbtPattern} btcNuggets
 * @property {AXbtPattern} btcPoolParty
 * @property {AXbtPattern} btcServ
 * @property {AXbtPattern} btcTop
 * @property {AXbtPattern} btcc
 * @property {AXbtPattern} bwPool
 * @property {AXbtPattern} bytePool
 * @property {AXbtPattern} canoe
 * @property {AXbtPattern} canoePool
 * @property {AXbtPattern} carbonNegative
 * @property {AXbtPattern} ckPool
 * @property {AXbtPattern} cloudHashing
 * @property {AXbtPattern} coinLab
 * @property {AXbtPattern} cointerra
 * @property {AXbtPattern} connectBtc
 * @property {AXbtPattern} dPool
 * @property {AXbtPattern} dcExploration
 * @property {AXbtPattern} dcex
 * @property {AXbtPattern} digitalBtc
 * @property {AXbtPattern} digitalXMintsy
 * @property {AXbtPattern} eclipseMc
 * @property {AXbtPattern} eightBaochi
 * @property {AXbtPattern} ekanemBtc
 * @property {AXbtPattern} eligius
 * @property {AXbtPattern} emcdPool
 * @property {AXbtPattern} entrustCharityPool
 * @property {AXbtPattern} eobot
 * @property {AXbtPattern} exxBw
 * @property {AXbtPattern} f2Pool
 * @property {AXbtPattern} fiftyEightCoin
 * @property {AXbtPattern} foundryUsa
 * @property {AXbtPattern} futureBitApolloSolo
 * @property {AXbtPattern} gbMiners
 * @property {AXbtPattern} ghashIo
 * @property {AXbtPattern} giveMeCoins
 * @property {AXbtPattern} goGreenLight
 * @property {AXbtPattern} haoZhuZhu
 * @property {AXbtPattern} haominer
 * @property {AXbtPattern} hashBx
 * @property {AXbtPattern} hashPool
 * @property {AXbtPattern} helix
 * @property {AXbtPattern} hhtt
 * @property {AXbtPattern} hotPool
 * @property {AXbtPattern} hummerpool
 * @property {AXbtPattern} huobiPool
 * @property {AXbtPattern} innopolisTech
 * @property {AXbtPattern} kanoPool
 * @property {AXbtPattern} kncMiner
 * @property {AXbtPattern} kuCoinPool
 * @property {AXbtPattern} lubianCom
 * @property {AXbtPattern} luckyPool
 * @property {AXbtPattern} luxor
 * @property {AXbtPattern} maraPool
 * @property {AXbtPattern} maxBtc
 * @property {AXbtPattern} maxiPool
 * @property {AXbtPattern} megaBigPower
 * @property {AXbtPattern} minerium
 * @property {AXbtPattern} miningCity
 * @property {AXbtPattern} miningDutch
 * @property {AXbtPattern} miningKings
 * @property {AXbtPattern} miningSquared
 * @property {AXbtPattern} mmpool
 * @property {AXbtPattern} mtRed
 * @property {AXbtPattern} multiCoinCo
 * @property {AXbtPattern} multipool
 * @property {AXbtPattern} myBtcCoinPool
 * @property {AXbtPattern} neopool
 * @property {AXbtPattern} nexious
 * @property {AXbtPattern} niceHash
 * @property {AXbtPattern} nmcBit
 * @property {AXbtPattern} novaBlock
 * @property {AXbtPattern} ocean
 * @property {AXbtPattern} okExPool
 * @property {AXbtPattern} okMiner
 * @property {AXbtPattern} okkong
 * @property {AXbtPattern} okpoolTop
 * @property {AXbtPattern} oneHash
 * @property {AXbtPattern} oneM1x
 * @property {AXbtPattern} oneThash
 * @property {AXbtPattern} ozCoin
 * @property {AXbtPattern} pHashIo
 * @property {AXbtPattern} parasite
 * @property {AXbtPattern} patels
 * @property {AXbtPattern} pegaPool
 * @property {AXbtPattern} phoenix
 * @property {AXbtPattern} polmine
 * @property {AXbtPattern} pool175btc
 * @property {AXbtPattern} pool50btc
 * @property {AXbtPattern} poolin
 * @property {AXbtPattern} portlandHodl
 * @property {AXbtPattern} publicPool
 * @property {AXbtPattern} pureBtcCom
 * @property {AXbtPattern} rawpool
 * @property {AXbtPattern} rigPool
 * @property {AXbtPattern} sbiCrypto
 * @property {AXbtPattern} secPool
 * @property {AXbtPattern} secretSuperstar
 * @property {AXbtPattern} sevenPool
 * @property {AXbtPattern} shawnP0wers
 * @property {AXbtPattern} sigmapoolCom
 * @property {AXbtPattern} simplecoinUs
 * @property {AXbtPattern} soloCk
 * @property {AXbtPattern} spiderPool
 * @property {AXbtPattern} stMiningCorp
 * @property {AXbtPattern} tangpool
 * @property {AXbtPattern} tatmasPool
 * @property {AXbtPattern} tbDice
 * @property {AXbtPattern} telco214
 * @property {AXbtPattern} terraPool
 * @property {AXbtPattern} tiger
 * @property {AXbtPattern} tigerpoolNet
 * @property {AXbtPattern} titan
 * @property {AXbtPattern} transactionCoinMining
 * @property {AXbtPattern} trickysBtcPool
 * @property {AXbtPattern} tripleMining
 * @property {AXbtPattern} twentyOneInc
 * @property {AXbtPattern} ultimusPool
 * @property {AXbtPattern} unknown
 * @property {AXbtPattern} unomp
 * @property {AXbtPattern} viaBtc
 * @property {AXbtPattern} waterhole
 * @property {AXbtPattern} wayiCn
 * @property {AXbtPattern} whitePool
 * @property {AXbtPattern} wk057
 * @property {AXbtPattern} yourbtcNet
 * @property {AXbtPattern} zulupool
 */

/**
 * @typedef {Object} CatalogTree_Computed_Price
 * @property {Indexes3<Dollars>} priceClose
 * @property {Indexes13<Cents>} priceCloseInCents
 * @property {Indexes3<Sats>} priceCloseInSats
 * @property {Indexes3<Dollars>} priceHigh
 * @property {Indexes13<Cents>} priceHighInCents
 * @property {Indexes3<Sats>} priceHighInSats
 * @property {Indexes3<Dollars>} priceLow
 * @property {Indexes13<Cents>} priceLowInCents
 * @property {Indexes3<Sats>} priceLowInSats
 * @property {Indexes3<OHLCDollars>} priceOhlc
 * @property {Indexes3<OHLCSats>} priceOhlcInSats
 * @property {Indexes3<Dollars>} priceOpen
 * @property {Indexes13<Cents>} priceOpenInCents
 * @property {Indexes3<Sats>} priceOpenInSats
 */

/**
 * @typedef {Object} CatalogTree_Computed_Stateful
 * @property {Indexes3<StoredU64>} addrCount
 * @property {CatalogTree_Computed_Stateful_AddressCohorts} addressCohorts
 * @property {CatalogTree_Computed_Stateful_AddressesData} addressesData
 * @property {AddresstypeToHeightToAddrCountPattern<StoredU64>} addresstypeToHeightToAddrCount
 * @property {AddresstypeToHeightToAddrCountPattern<StoredU64>} addresstypeToHeightToEmptyAddrCount
 * @property {AddresstypeToHeightToAddrCountPattern<StoredU64>} addresstypeToIndexesToAddrCount
 * @property {AddresstypeToHeightToAddrCountPattern<StoredU64>} addresstypeToIndexesToEmptyAddrCount
 * @property {AddresstypeToHeightToAddrCountPattern<AnyAddressIndex>} anyAddressIndexes
 * @property {Indexes2<SupplyState>} chainState
 * @property {Indexes3<StoredU64>} emptyAddrCount
 * @property {Indexes29<EmptyAddressIndex>} emptyaddressindex
 * @property {Indexes30<LoadedAddressIndex>} loadedaddressindex
 * @property {Indexes26<Dollars>} marketCap
 * @property {SupplyPattern} opreturnSupply
 * @property {SupplyPattern} unspendableSupply
 * @property {CatalogTree_Computed_Stateful_UtxoCohorts} utxoCohorts
 */

/**
 * @typedef {Object} CatalogTree_Computed_Stateful_AddressCohorts
 * @property {CatalogTree_Computed_Stateful_AddressCohorts_AmountRange} amountRange
 * @property {CatalogTree_Computed_Stateful_AddressCohorts_GeAmount} geAmount
 * @property {CatalogTree_Computed_Stateful_AddressCohorts_LtAmount} ltAmount
 */

/**
 * @typedef {Object} CatalogTree_Computed_Stateful_AddressCohorts_AmountRange
 * @property {_0satsPattern} _0sats
 * @property {_0satsPattern} _100btcTo1kBtc
 * @property {_0satsPattern} _100kBtcOrMore
 * @property {_0satsPattern} _100kSatsTo1mSats
 * @property {_0satsPattern} _100satsTo1kSats
 * @property {_0satsPattern} _10btcTo100btc
 * @property {_0satsPattern} _10kBtcTo100kBtc
 * @property {_0satsPattern} _10kSatsTo100kSats
 * @property {_0satsPattern} _10mSatsTo1btc
 * @property {_0satsPattern} _10satsTo100sats
 * @property {_0satsPattern} _1btcTo10btc
 * @property {_0satsPattern} _1kBtcTo10kBtc
 * @property {_0satsPattern} _1kSatsTo10kSats
 * @property {_0satsPattern} _1mSatsTo10mSats
 * @property {_0satsPattern} _1satTo10sats
 */

/**
 * @typedef {Object} CatalogTree_Computed_Stateful_AddressCohorts_GeAmount
 * @property {_0satsPattern} _100btc
 * @property {_0satsPattern} _100kSats
 * @property {_0satsPattern} _100sats
 * @property {_0satsPattern} _10btc
 * @property {_0satsPattern} _10kBtc
 * @property {_0satsPattern} _10kSats
 * @property {_0satsPattern} _10mSats
 * @property {_0satsPattern} _10sats
 * @property {_0satsPattern} _1btc
 * @property {_0satsPattern} _1kBtc
 * @property {_0satsPattern} _1kSats
 * @property {_0satsPattern} _1mSats
 * @property {_0satsPattern} _1sat
 */

/**
 * @typedef {Object} CatalogTree_Computed_Stateful_AddressCohorts_LtAmount
 * @property {_0satsPattern} _100btc
 * @property {_0satsPattern} _100kBtc
 * @property {_0satsPattern} _100kSats
 * @property {_0satsPattern} _100sats
 * @property {_0satsPattern} _10btc
 * @property {_0satsPattern} _10kBtc
 * @property {_0satsPattern} _10kSats
 * @property {_0satsPattern} _10mSats
 * @property {_0satsPattern} _10sats
 * @property {_0satsPattern} _1btc
 * @property {_0satsPattern} _1kBtc
 * @property {_0satsPattern} _1kSats
 * @property {_0satsPattern} _1mSats
 */

/**
 * @typedef {Object} CatalogTree_Computed_Stateful_AddressesData
 * @property {Indexes29<EmptyAddressData>} empty
 * @property {Indexes30<LoadedAddressData>} loaded
 */

/**
 * @typedef {Object} CatalogTree_Computed_Stateful_UtxoCohorts
 * @property {CatalogTree_Computed_Stateful_UtxoCohorts_AgeRange} ageRange
 * @property {CatalogTree_Computed_Stateful_UtxoCohorts_All} all
 * @property {CatalogTree_Computed_Stateful_UtxoCohorts_AmountRange} amountRange
 * @property {CatalogTree_Computed_Stateful_UtxoCohorts_Epoch} epoch
 * @property {CatalogTree_Computed_Stateful_UtxoCohorts_GeAmount} geAmount
 * @property {CatalogTree_Computed_Stateful_UtxoCohorts_LtAmount} ltAmount
 * @property {CatalogTree_Computed_Stateful_UtxoCohorts_MaxAge} maxAge
 * @property {CatalogTree_Computed_Stateful_UtxoCohorts_MinAge} minAge
 * @property {CatalogTree_Computed_Stateful_UtxoCohorts_Term} term
 * @property {CatalogTree_Computed_Stateful_UtxoCohorts_Type} type
 * @property {CatalogTree_Computed_Stateful_UtxoCohorts_Year} year
 */

/**
 * @typedef {Object} CatalogTree_Computed_Stateful_UtxoCohorts_AgeRange
 * @property {_10yTo12yPattern} _10yTo12y
 * @property {_10yTo12yPattern} _12yTo15y
 * @property {_10yTo12yPattern} _1dTo1w
 * @property {_10yTo12yPattern} _1mTo2m
 * @property {_10yTo12yPattern} _1wTo1m
 * @property {_10yTo12yPattern} _1yTo2y
 * @property {_10yTo12yPattern} _2mTo3m
 * @property {_10yTo12yPattern} _2yTo3y
 * @property {_10yTo12yPattern} _3mTo4m
 * @property {_10yTo12yPattern} _3yTo4y
 * @property {_10yTo12yPattern} _4mTo5m
 * @property {_10yTo12yPattern} _4yTo5y
 * @property {_10yTo12yPattern} _5mTo6m
 * @property {_10yTo12yPattern} _5yTo6y
 * @property {_10yTo12yPattern} _6mTo1y
 * @property {_10yTo12yPattern} _6yTo7y
 * @property {_10yTo12yPattern} _7yTo8y
 * @property {_10yTo12yPattern} _8yTo10y
 * @property {_10yTo12yPattern} from15y
 * @property {UpTo1dPattern} upTo1d
 */

/**
 * @typedef {Object} CatalogTree_Computed_Stateful_UtxoCohorts_All
 * @property {ActivityPattern} activity
 * @property {PricePaidPattern2} pricePaid
 * @property {RealizedPattern3} realized
 * @property {CatalogTree_Computed_Stateful_UtxoCohorts_All_Relative} relative
 * @property {SupplyPattern2} supply
 * @property {UnrealizedPattern} unrealized
 */

/**
 * @typedef {Object} CatalogTree_Computed_Stateful_UtxoCohorts_All_Relative
 * @property {Indexes27<StoredF32>} negUnrealizedLossRelToOwnTotalUnrealizedPnl
 * @property {Indexes26<StoredF32>} netUnrealizedPnlRelToOwnTotalUnrealizedPnl
 * @property {Indexes27<StoredF64>} supplyInLossRelToOwnSupply
 * @property {Indexes27<StoredF64>} supplyInProfitRelToOwnSupply
 * @property {Indexes27<StoredF32>} unrealizedLossRelToOwnTotalUnrealizedPnl
 * @property {Indexes27<StoredF32>} unrealizedProfitRelToOwnTotalUnrealizedPnl
 */

/**
 * @typedef {Object} CatalogTree_Computed_Stateful_UtxoCohorts_AmountRange
 * @property {_0satsPattern2} _0sats
 * @property {_0satsPattern2} _100btcTo1kBtc
 * @property {_0satsPattern2} _100kBtcOrMore
 * @property {_0satsPattern2} _100kSatsTo1mSats
 * @property {_0satsPattern2} _100satsTo1kSats
 * @property {_0satsPattern2} _10btcTo100btc
 * @property {_0satsPattern2} _10kBtcTo100kBtc
 * @property {_0satsPattern2} _10kSatsTo100kSats
 * @property {_0satsPattern2} _10mSatsTo1btc
 * @property {_0satsPattern2} _10satsTo100sats
 * @property {_0satsPattern2} _1btcTo10btc
 * @property {_0satsPattern2} _1kBtcTo10kBtc
 * @property {_0satsPattern2} _1kSatsTo10kSats
 * @property {_0satsPattern2} _1mSatsTo10mSats
 * @property {_0satsPattern2} _1satTo10sats
 */

/**
 * @typedef {Object} CatalogTree_Computed_Stateful_UtxoCohorts_Epoch
 * @property {_10yTo12yPattern} _0
 * @property {_10yTo12yPattern} _1
 * @property {_10yTo12yPattern} _2
 * @property {_10yTo12yPattern} _3
 * @property {_10yTo12yPattern} _4
 */

/**
 * @typedef {Object} CatalogTree_Computed_Stateful_UtxoCohorts_GeAmount
 * @property {_0satsPattern2} _100btc
 * @property {_0satsPattern2} _100kSats
 * @property {_0satsPattern2} _100sats
 * @property {_0satsPattern2} _10btc
 * @property {_0satsPattern2} _10kBtc
 * @property {_0satsPattern2} _10kSats
 * @property {_0satsPattern2} _10mSats
 * @property {_0satsPattern2} _10sats
 * @property {_0satsPattern2} _1btc
 * @property {_0satsPattern2} _1kBtc
 * @property {_0satsPattern2} _1kSats
 * @property {_0satsPattern2} _1mSats
 * @property {_0satsPattern2} _1sat
 */

/**
 * @typedef {Object} CatalogTree_Computed_Stateful_UtxoCohorts_LtAmount
 * @property {_0satsPattern2} _100btc
 * @property {_0satsPattern2} _100kBtc
 * @property {_0satsPattern2} _100kSats
 * @property {_0satsPattern2} _100sats
 * @property {_0satsPattern2} _10btc
 * @property {_0satsPattern2} _10kBtc
 * @property {_0satsPattern2} _10kSats
 * @property {_0satsPattern2} _10mSats
 * @property {_0satsPattern2} _10sats
 * @property {_0satsPattern2} _1btc
 * @property {_0satsPattern2} _1kBtc
 * @property {_0satsPattern2} _1kSats
 * @property {_0satsPattern2} _1mSats
 */

/**
 * @typedef {Object} CatalogTree_Computed_Stateful_UtxoCohorts_MaxAge
 * @property {UpTo1dPattern} _10y
 * @property {UpTo1dPattern} _12y
 * @property {UpTo1dPattern} _15y
 * @property {UpTo1dPattern} _1m
 * @property {UpTo1dPattern} _1w
 * @property {UpTo1dPattern} _1y
 * @property {UpTo1dPattern} _2m
 * @property {UpTo1dPattern} _2y
 * @property {UpTo1dPattern} _3m
 * @property {UpTo1dPattern} _3y
 * @property {UpTo1dPattern} _4m
 * @property {UpTo1dPattern} _4y
 * @property {UpTo1dPattern} _5m
 * @property {UpTo1dPattern} _5y
 * @property {UpTo1dPattern} _6m
 * @property {UpTo1dPattern} _6y
 * @property {UpTo1dPattern} _7y
 * @property {UpTo1dPattern} _8y
 */

/**
 * @typedef {Object} CatalogTree_Computed_Stateful_UtxoCohorts_MinAge
 * @property {_10yTo12yPattern} _10y
 * @property {_10yTo12yPattern} _12y
 * @property {_10yTo12yPattern} _1d
 * @property {_10yTo12yPattern} _1m
 * @property {_10yTo12yPattern} _1w
 * @property {_10yTo12yPattern} _1y
 * @property {_10yTo12yPattern} _2m
 * @property {_10yTo12yPattern} _2y
 * @property {_10yTo12yPattern} _3m
 * @property {_10yTo12yPattern} _3y
 * @property {_10yTo12yPattern} _4m
 * @property {_10yTo12yPattern} _4y
 * @property {_10yTo12yPattern} _5m
 * @property {_10yTo12yPattern} _5y
 * @property {_10yTo12yPattern} _6m
 * @property {_10yTo12yPattern} _6y
 * @property {_10yTo12yPattern} _7y
 * @property {_10yTo12yPattern} _8y
 */

/**
 * @typedef {Object} CatalogTree_Computed_Stateful_UtxoCohorts_Term
 * @property {UpTo1dPattern} long
 * @property {UpTo1dPattern} short
 */

/**
 * @typedef {Object} CatalogTree_Computed_Stateful_UtxoCohorts_Type
 * @property {_0satsPattern2} empty
 * @property {_0satsPattern2} p2a
 * @property {_0satsPattern2} p2ms
 * @property {_0satsPattern2} p2pk33
 * @property {_0satsPattern2} p2pk65
 * @property {_0satsPattern2} p2pkh
 * @property {_0satsPattern2} p2sh
 * @property {_0satsPattern2} p2tr
 * @property {_0satsPattern2} p2wpkh
 * @property {_0satsPattern2} p2wsh
 * @property {_0satsPattern2} unknown
 */

/**
 * @typedef {Object} CatalogTree_Computed_Stateful_UtxoCohorts_Year
 * @property {_10yTo12yPattern} _2009
 * @property {_10yTo12yPattern} _2010
 * @property {_10yTo12yPattern} _2011
 * @property {_10yTo12yPattern} _2012
 * @property {_10yTo12yPattern} _2013
 * @property {_10yTo12yPattern} _2014
 * @property {_10yTo12yPattern} _2015
 * @property {_10yTo12yPattern} _2016
 * @property {_10yTo12yPattern} _2017
 * @property {_10yTo12yPattern} _2018
 * @property {_10yTo12yPattern} _2019
 * @property {_10yTo12yPattern} _2020
 * @property {_10yTo12yPattern} _2021
 * @property {_10yTo12yPattern} _2022
 * @property {_10yTo12yPattern} _2023
 * @property {_10yTo12yPattern} _2024
 * @property {_10yTo12yPattern} _2025
 * @property {_10yTo12yPattern} _2026
 */

/**
 * @typedef {Object} CatalogTree_Computed_Txins
 * @property {Indexes24<TxOutIndex>} txoutindex
 * @property {Indexes24<Sats>} value
 */

/**
 * @typedef {Object} CatalogTree_Computed_Txouts
 * @property {Indexes25<TxInIndex>} txinindex
 */

/**
 * @typedef {Object} CatalogTree_Indexed
 * @property {CatalogTree_Indexed_Address} address
 * @property {CatalogTree_Indexed_Block} block
 * @property {CatalogTree_Indexed_Output} output
 * @property {CatalogTree_Indexed_Tx} tx
 * @property {CatalogTree_Indexed_Txin} txin
 * @property {CatalogTree_Indexed_Txout} txout
 */

/**
 * @typedef {Object} CatalogTree_Indexed_Address
 * @property {Indexes2<P2AAddressIndex>} firstP2aaddressindex
 * @property {Indexes2<P2PK33AddressIndex>} firstP2pk33addressindex
 * @property {Indexes2<P2PK65AddressIndex>} firstP2pk65addressindex
 * @property {Indexes2<P2PKHAddressIndex>} firstP2pkhaddressindex
 * @property {Indexes2<P2SHAddressIndex>} firstP2shaddressindex
 * @property {Indexes2<P2TRAddressIndex>} firstP2traddressindex
 * @property {Indexes2<P2WPKHAddressIndex>} firstP2wpkhaddressindex
 * @property {Indexes2<P2WSHAddressIndex>} firstP2wshaddressindex
 * @property {Indexes16<P2ABytes>} p2abytes
 * @property {Indexes17<P2PK33Bytes>} p2pk33bytes
 * @property {Indexes18<P2PK65Bytes>} p2pk65bytes
 * @property {Indexes19<P2PKHBytes>} p2pkhbytes
 * @property {Indexes20<P2SHBytes>} p2shbytes
 * @property {Indexes21<P2TRBytes>} p2trbytes
 * @property {Indexes22<P2WPKHBytes>} p2wpkhbytes
 * @property {Indexes23<P2WSHBytes>} p2wshbytes
 */

/**
 * @typedef {Object} CatalogTree_Indexed_Block
 * @property {Indexes2<BlockHash>} blockhash
 * @property {Indexes2<StoredF64>} difficulty
 * @property {Indexes2<Timestamp>} timestamp
 * @property {Indexes2<StoredU64>} totalSize
 * @property {Indexes2<Weight>} weight
 */

/**
 * @typedef {Object} CatalogTree_Indexed_Output
 * @property {Indexes2<EmptyOutputIndex>} firstEmptyoutputindex
 * @property {Indexes2<OpReturnIndex>} firstOpreturnindex
 * @property {Indexes2<P2MSOutputIndex>} firstP2msoutputindex
 * @property {Indexes2<UnknownOutputIndex>} firstUnknownoutputindex
 * @property {MetricNode<TxIndex>} txindex
 */

/**
 * @typedef {Object} CatalogTree_Indexed_Tx
 * @property {Indexes6<StoredU32>} baseSize
 * @property {Indexes2<TxIndex>} firstTxindex
 * @property {Indexes6<TxInIndex>} firstTxinindex
 * @property {Indexes6<TxOutIndex>} firstTxoutindex
 * @property {Indexes6<Height>} height
 * @property {Indexes6<StoredBool>} isExplicitlyRbf
 * @property {Indexes6<RawLockTime>} rawlocktime
 * @property {Indexes6<StoredU32>} totalSize
 * @property {Indexes6<Txid>} txid
 * @property {Indexes6<TxVersion>} txversion
 */

/**
 * @typedef {Object} CatalogTree_Indexed_Txin
 * @property {Indexes2<TxInIndex>} firstTxinindex
 * @property {Indexes24<OutPoint>} outpoint
 * @property {Indexes24<OutputType>} outputtype
 * @property {Indexes24<TxIndex>} txindex
 * @property {Indexes24<TypeIndex>} typeindex
 */

/**
 * @typedef {Object} CatalogTree_Indexed_Txout
 * @property {Indexes2<TxOutIndex>} firstTxoutindex
 * @property {Indexes25<OutputType>} outputtype
 * @property {Indexes25<TxIndex>} txindex
 * @property {Indexes25<TypeIndex>} typeindex
 * @property {Indexes25<Sats>} value
 */

/**
 * Main BRK client with catalog tree and API methods
 * @extends BrkClientBase
 */
class BrkClient extends BrkClientBase {
  /**
   * @param {BrkClientOptions|string} options
   */
  constructor(options) {
    super(options);
    /** @type {CatalogTree} */
    this.tree = this._buildTree('');
  }

  /**
   * @private
   * @param {string} basePath
   * @returns {CatalogTree}
   */
  _buildTree(basePath) {
    return {
      computed: {
        blks: {
          position: new MetricNode(this, '/position')
        },
        chain: {
          _1mBlockCount: createIndexes(this, '/1m_block_count'),
          _1wBlockCount: createIndexes(this, '/1w_block_count'),
          _1yBlockCount: createIndexes(this, '/1y_block_count'),
          _24hBlockCount: createIndexes2(this, '/24h_block_count'),
          _24hCoinbaseSum: createIndexes2(this, '/24h_coinbase_sum'),
          _24hCoinbaseUsdSum: createIndexes2(this, '/24h_coinbase_usd_sum'),
          annualizedVolume: createIndexes(this, '/annualized_volume'),
          annualizedVolumeBtc: createIndexes(this, '/annualized_volume_btc'),
          annualizedVolumeUsd: createIndexes(this, '/annualized_volume_usd'),
          blockCount: createBlockCountPattern(this, 'computed_chain/block_count'),
          blockCountTarget: createIndexes(this, '/block_count_target'),
          blockInterval: createBlockIntervalPattern(this, 'block_interval'),
          blockSize: createBlockSizePattern(this, 'computed_chain/block_size'),
          blockVbytes: createBlockSizePattern(this, 'computed_chain/block_vbytes'),
          blockWeight: createBlockSizePattern(this, 'computed_chain/block_weight'),
          blocksBeforeNextDifficultyAdjustment: createIndexes3(this, '/blocks_before_next_difficulty_adjustment'),
          blocksBeforeNextHalving: createIndexes3(this, '/blocks_before_next_halving'),
          coinbase: createCoinbasePattern(this, 'computed_chain/coinbase'),
          daysBeforeNextDifficultyAdjustment: createIndexes3(this, '/days_before_next_difficulty_adjustment'),
          daysBeforeNextHalving: createIndexes3(this, '/days_before_next_halving'),
          difficulty: createIndexes4(this, '/difficulty'),
          difficultyAdjustment: createIndexes3(this, '/difficulty_adjustment'),
          difficultyAsHash: createIndexes3(this, '/difficulty_as_hash'),
          difficultyepoch: createIndexes(this, '/difficultyepoch'),
          emptyoutputCount: createBitcoinPattern(this, 'computed_chain/emptyoutput_count'),
          exactUtxoCount: createIndexes3(this, '/exact_utxo_count'),
          fee: {
            base: createIndexes6(this, '/fee'),
            bitcoin: createBlockSizePattern(this, 'fee/bitcoin'),
            bitcoinTxindex: createIndexes6(this, '/fee_btc'),
            dollars: createBlockSizePattern(this, 'fee/dollars'),
            dollarsTxindex: createIndexes6(this, '/fee_usd'),
            sats: createBlockSizePattern(this, 'fee/sats')
          },
          feeDominance: createIndexes5(this, '/fee_dominance'),
          feeRate: {
            average: createIndexes3(this, '/fee_rate_avg'),
            base: createIndexes6(this, '/fee_rate'),
            max: createIndexes3(this, '/fee_rate_max'),
            median: createIndexes2(this, '/fee_rate_median'),
            min: createIndexes3(this, '/fee_rate_min'),
            pct10: createIndexes2(this, '/fee_rate_pct10'),
            pct25: createIndexes2(this, '/fee_rate_pct25'),
            pct75: createIndexes2(this, '/fee_rate_pct75'),
            pct90: createIndexes2(this, '/fee_rate_pct90')
          },
          halvingepoch: createIndexes(this, '/halvingepoch'),
          hashPricePhs: createIndexes3(this, '/hash_price_phs'),
          hashPricePhsMin: createIndexes3(this, '/hash_price_phs_min'),
          hashPriceRebound: createIndexes3(this, '/hash_price_rebound'),
          hashPriceThs: createIndexes3(this, '/hash_price_ths'),
          hashPriceThsMin: createIndexes3(this, '/hash_price_ths_min'),
          hashRate: createIndexes3(this, '/hash_rate'),
          hashRate1mSma: createIndexes(this, '/hash_rate_1m_sma'),
          hashRate1wSma: createIndexes(this, '/hash_rate_1w_sma'),
          hashRate1ySma: createIndexes(this, '/hash_rate_1y_sma'),
          hashRate2mSma: createIndexes(this, '/hash_rate_2m_sma'),
          hashValuePhs: createIndexes3(this, '/hash_value_phs'),
          hashValuePhsMin: createIndexes3(this, '/hash_value_phs_min'),
          hashValueRebound: createIndexes3(this, '/hash_value_rebound'),
          hashValueThs: createIndexes3(this, '/hash_value_ths'),
          hashValueThsMin: createIndexes3(this, '/hash_value_ths_min'),
          inflationRate: createIndexes(this, '/inflation_rate'),
          inputCount: createBlockSizePattern(this, 'computed_chain/input_count'),
          inputValue: createIndexes6(this, '/input_value'),
          inputsPerSec: createIndexes(this, '/inputs_per_sec'),
          interval: createIndexes2(this, '/interval'),
          isCoinbase: createIndexes6(this, '/is_coinbase'),
          opreturnCount: createBitcoinPattern(this, 'computed_chain/opreturn_count'),
          outputCount: createBlockSizePattern(this, 'computed_chain/output_count'),
          outputValue: createIndexes6(this, '/output_value'),
          outputsPerSec: createIndexes(this, '/outputs_per_sec'),
          p2aCount: createBitcoinPattern(this, 'computed_chain/p2a_count'),
          p2msCount: createBitcoinPattern(this, 'computed_chain/p2ms_count'),
          p2pk33Count: createBitcoinPattern(this, 'computed_chain/p2pk33_count'),
          p2pk65Count: createBitcoinPattern(this, 'computed_chain/p2pk65_count'),
          p2pkhCount: createBitcoinPattern(this, 'computed_chain/p2pkh_count'),
          p2shCount: createBitcoinPattern(this, 'computed_chain/p2sh_count'),
          p2trCount: createBitcoinPattern(this, 'computed_chain/p2tr_count'),
          p2wpkhCount: createBitcoinPattern(this, 'computed_chain/p2wpkh_count'),
          p2wshCount: createBitcoinPattern(this, 'computed_chain/p2wsh_count'),
          puellMultiple: createIndexes(this, '/puell_multiple'),
          sentSum: {
            bitcoin: createBitcoinPattern2(this, 'sent_sum/bitcoin'),
            dollars: createIndexes3(this, '/sent_sum_usd'),
            sats: createIndexes3(this, '/sent_sum')
          },
          subsidy: createCoinbasePattern(this, 'computed_chain/subsidy'),
          subsidyDominance: createIndexes5(this, '/subsidy_dominance'),
          subsidyUsd1ySma: createIndexes(this, '/subsidy_usd_1y_sma'),
          timestamp: new MetricNode(this, '/timestamp'),
          txBtcVelocity: createIndexes(this, '/tx_btc_velocity'),
          txCount: createBitcoinPattern(this, 'computed_chain/tx_count'),
          txPerSec: createIndexes(this, '/tx_per_sec'),
          txUsdVelocity: createIndexes(this, '/tx_usd_velocity'),
          txV1: createBlockCountPattern(this, 'computed_chain/tx_v1'),
          txV2: createBlockCountPattern(this, 'computed_chain/tx_v2'),
          txV3: createBlockCountPattern(this, 'computed_chain/tx_v3'),
          txVsize: createBlockIntervalPattern(this, 'tx_vsize'),
          txWeight: createBlockIntervalPattern(this, 'tx_weight'),
          unclaimedRewards: createUnclaimedRewardsPattern(this, 'computed_chain/unclaimed_rewards'),
          unknownoutputCount: createBitcoinPattern(this, 'computed_chain/unknownoutput_count'),
          vbytes: createIndexes2(this, '/vbytes'),
          vsize: createIndexes6(this, '/vsize'),
          weight: createIndexes6(this, '/weight')
        },
        cointime: {
          activeCap: createIndexes3(this, '/active_cap'),
          activePrice: createIndexes3(this, '/active_price'),
          activePriceRatio: createActivePriceRatioPattern(this, 'computed_cointime/active_price_ratio'),
          activeSupply: createActiveSupplyPattern(this, 'computed_cointime/active_supply'),
          activityToVaultednessRatio: createIndexes3(this, '/activity_to_vaultedness_ratio'),
          coinblocksCreated: createBlockCountPattern(this, 'computed_cointime/coinblocks_created'),
          coinblocksStored: createBlockCountPattern(this, 'computed_cointime/coinblocks_stored'),
          cointimeAdjInflationRate: createIndexes(this, '/cointime_adj_inflation_rate'),
          cointimeAdjTxBtcVelocity: createIndexes(this, '/cointime_adj_tx_btc_velocity'),
          cointimeAdjTxUsdVelocity: createIndexes(this, '/cointime_adj_tx_usd_velocity'),
          cointimeCap: createIndexes3(this, '/cointime_cap'),
          cointimePrice: createIndexes3(this, '/cointime_price'),
          cointimePriceRatio: createActivePriceRatioPattern(this, 'computed_cointime/cointime_price_ratio'),
          cointimeValueCreated: createBlockCountPattern(this, 'computed_cointime/cointime_value_created'),
          cointimeValueDestroyed: createBlockCountPattern(this, 'computed_cointime/cointime_value_destroyed'),
          cointimeValueStored: createBlockCountPattern(this, 'computed_cointime/cointime_value_stored'),
          investorCap: createIndexes3(this, '/investor_cap'),
          liveliness: createIndexes3(this, '/liveliness'),
          thermoCap: createIndexes3(this, '/thermo_cap'),
          trueMarketMean: createIndexes3(this, '/true_market_mean'),
          trueMarketMeanRatio: createActivePriceRatioPattern(this, 'computed_cointime/true_market_mean_ratio'),
          vaultedCap: createIndexes3(this, '/vaulted_cap'),
          vaultedPrice: createIndexes3(this, '/vaulted_price'),
          vaultedPriceRatio: createActivePriceRatioPattern(this, 'computed_cointime/vaulted_price_ratio'),
          vaultedSupply: createActiveSupplyPattern(this, 'computed_cointime/vaulted_supply'),
          vaultedness: createIndexes3(this, '/vaultedness')
        },
        constants: {
          constant0: createConstant0Pattern(this, 'constant_0'),
          constant1: createConstant0Pattern(this, 'constant_1'),
          constant100: createConstant0Pattern(this, 'constant_100'),
          constant2: createConstant0Pattern(this, 'constant_2'),
          constant3: createConstant0Pattern(this, 'constant_3'),
          constant382: createConstant0Pattern(this, 'constant_38_2'),
          constant4: createConstant0Pattern(this, 'constant_4'),
          constant50: createConstant0Pattern(this, 'constant_50'),
          constant600: createConstant0Pattern(this, 'constant_600'),
          constant618: createConstant0Pattern(this, 'constant_61_8'),
          constantMinus1: createConstant0Pattern(this, 'constant_minus_1'),
          constantMinus2: createConstant0Pattern(this, 'constant_minus_2'),
          constantMinus3: createConstant0Pattern(this, 'constant_minus_3'),
          constantMinus4: createConstant0Pattern(this, 'constant_minus_4')
        },
        fetched: {
          priceOhlcInCents: createIndexes13(this, '/price_ohlc_in_cents')
        },
        indexes: {
          date: createIndexes13(this, '/date'),
          dateFixed: createIndexes2(this, '/date_fixed'),
          dateindex: createIndexes13(this, '/dateindex'),
          dateindexCount: createIndexes14(this, '/dateindex_count'),
          decadeindex: new MetricNode(this, '/decadeindex'),
          difficultyepoch: new MetricNode(this, '/difficultyepoch'),
          emptyoutputindex: new MetricNode(this, '/emptyoutputindex'),
          firstDateindex: createIndexes14(this, '/first_dateindex'),
          firstHeight: new MetricNode(this, '/first_height'),
          firstMonthindex: createIndexes15(this, '/first_monthindex'),
          firstYearindex: createIndexes7(this, '/first_yearindex'),
          halvingepoch: new MetricNode(this, '/halvingepoch'),
          height: createIndexes2(this, '/height'),
          heightCount: new MetricNode(this, '/height_count'),
          inputCount: createIndexes6(this, '/input_count'),
          monthindex: new MetricNode(this, '/monthindex'),
          monthindexCount: createIndexes15(this, '/monthindex_count'),
          opreturnindex: new MetricNode(this, '/opreturnindex'),
          outputCount: createIndexes6(this, '/output_count'),
          p2aaddressindex: createIndexes16(this, '/p2aaddressindex'),
          p2msoutputindex: new MetricNode(this, '/p2msoutputindex'),
          p2pk33addressindex: createIndexes17(this, '/p2pk33addressindex'),
          p2pk65addressindex: createIndexes18(this, '/p2pk65addressindex'),
          p2pkhaddressindex: createIndexes19(this, '/p2pkhaddressindex'),
          p2shaddressindex: createIndexes20(this, '/p2shaddressindex'),
          p2traddressindex: createIndexes21(this, '/p2traddressindex'),
          p2wpkhaddressindex: createIndexes22(this, '/p2wpkhaddressindex'),
          p2wshaddressindex: createIndexes23(this, '/p2wshaddressindex'),
          quarterindex: new MetricNode(this, '/quarterindex'),
          semesterindex: new MetricNode(this, '/semesterindex'),
          timestampFixed: createIndexes2(this, '/timestamp_fixed'),
          txindex: createIndexes6(this, '/txindex'),
          txindexCount: createIndexes2(this, '/txindex_count'),
          txinindex: createIndexes24(this, '/txinindex'),
          txoutindex: createIndexes25(this, '/txoutindex'),
          unknownoutputindex: new MetricNode(this, '/unknownoutputindex'),
          weekindex: new MetricNode(this, '/weekindex'),
          yearindex: new MetricNode(this, '/yearindex'),
          yearindexCount: createIndexes7(this, '/yearindex_count')
        },
        market: {
          _1dReturns1mSd: create_1dReturns1mSdPattern(this, '1d_returns_1m_sd'),
          _1dReturns1wSd: create_1dReturns1mSdPattern(this, '1d_returns_1w_sd'),
          _1dReturns1ySd: create_1dReturns1mSdPattern(this, '1d_returns_1y_sd'),
          _10yCagr: createIndexes(this, '/10y_cagr'),
          _10yDcaAvgPrice: createIndexes(this, '/10y_dca_avg_price'),
          _10yDcaCagr: createIndexes(this, '/10y_dca_cagr'),
          _10yDcaReturns: createIndexes(this, '/10y_dca_returns'),
          _10yDcaStack: createIndexes(this, '/10y_dca_stack'),
          _10yPriceReturns: createIndexes(this, '/10y_price_returns'),
          _1dPriceReturns: createIndexes(this, '/1d_price_returns'),
          _1mDcaAvgPrice: createIndexes(this, '/1m_dca_avg_price'),
          _1mDcaReturns: createIndexes(this, '/1m_dca_returns'),
          _1mDcaStack: createIndexes(this, '/1m_dca_stack'),
          _1mPriceReturns: createIndexes(this, '/1m_price_returns'),
          _1wDcaAvgPrice: createIndexes(this, '/1w_dca_avg_price'),
          _1wDcaReturns: createIndexes(this, '/1w_dca_returns'),
          _1wDcaStack: createIndexes(this, '/1w_dca_stack'),
          _1wPriceReturns: createIndexes(this, '/1w_price_returns'),
          _1yDcaAvgPrice: createIndexes(this, '/1y_dca_avg_price'),
          _1yDcaReturns: createIndexes(this, '/1y_dca_returns'),
          _1yDcaStack: createIndexes(this, '/1y_dca_stack'),
          _1yPriceReturns: createIndexes(this, '/1y_price_returns'),
          _2yCagr: createIndexes(this, '/2y_cagr'),
          _2yDcaAvgPrice: createIndexes(this, '/2y_dca_avg_price'),
          _2yDcaCagr: createIndexes(this, '/2y_dca_cagr'),
          _2yDcaReturns: createIndexes(this, '/2y_dca_returns'),
          _2yDcaStack: createIndexes(this, '/2y_dca_stack'),
          _2yPriceReturns: createIndexes(this, '/2y_price_returns'),
          _3mDcaAvgPrice: createIndexes(this, '/3m_dca_avg_price'),
          _3mDcaReturns: createIndexes(this, '/3m_dca_returns'),
          _3mDcaStack: createIndexes(this, '/3m_dca_stack'),
          _3mPriceReturns: createIndexes(this, '/3m_price_returns'),
          _3yCagr: createIndexes(this, '/3y_cagr'),
          _3yDcaAvgPrice: createIndexes(this, '/3y_dca_avg_price'),
          _3yDcaCagr: createIndexes(this, '/3y_dca_cagr'),
          _3yDcaReturns: createIndexes(this, '/3y_dca_returns'),
          _3yDcaStack: createIndexes(this, '/3y_dca_stack'),
          _3yPriceReturns: createIndexes(this, '/3y_price_returns'),
          _4yCagr: createIndexes(this, '/4y_cagr'),
          _4yDcaAvgPrice: createIndexes(this, '/4y_dca_avg_price'),
          _4yDcaCagr: createIndexes(this, '/4y_dca_cagr'),
          _4yDcaReturns: createIndexes(this, '/4y_dca_returns'),
          _4yDcaStack: createIndexes(this, '/4y_dca_stack'),
          _4yPriceReturns: createIndexes(this, '/4y_price_returns'),
          _5yCagr: createIndexes(this, '/5y_cagr'),
          _5yDcaAvgPrice: createIndexes(this, '/5y_dca_avg_price'),
          _5yDcaCagr: createIndexes(this, '/5y_dca_cagr'),
          _5yDcaReturns: createIndexes(this, '/5y_dca_returns'),
          _5yDcaStack: createIndexes(this, '/5y_dca_stack'),
          _5yPriceReturns: createIndexes(this, '/5y_price_returns'),
          _6mDcaAvgPrice: createIndexes(this, '/6m_dca_avg_price'),
          _6mDcaReturns: createIndexes(this, '/6m_dca_returns'),
          _6mDcaStack: createIndexes(this, '/6m_dca_stack'),
          _6mPriceReturns: createIndexes(this, '/6m_price_returns'),
          _6yCagr: createIndexes(this, '/6y_cagr'),
          _6yDcaAvgPrice: createIndexes(this, '/6y_dca_avg_price'),
          _6yDcaCagr: createIndexes(this, '/6y_dca_cagr'),
          _6yDcaReturns: createIndexes(this, '/6y_dca_returns'),
          _6yDcaStack: createIndexes(this, '/6y_dca_stack'),
          _6yPriceReturns: createIndexes(this, '/6y_price_returns'),
          _8yCagr: createIndexes(this, '/8y_cagr'),
          _8yDcaAvgPrice: createIndexes(this, '/8y_dca_avg_price'),
          _8yDcaCagr: createIndexes(this, '/8y_dca_cagr'),
          _8yDcaReturns: createIndexes(this, '/8y_dca_returns'),
          _8yDcaStack: createIndexes(this, '/8y_dca_stack'),
          _8yPriceReturns: createIndexes(this, '/8y_price_returns'),
          daysSincePriceAth: createIndexes(this, '/days_since_price_ath'),
          dcaClass2015AvgPrice: createIndexes(this, '/dca_class_2015_avg_price'),
          dcaClass2015Returns: createIndexes(this, '/dca_class_2015_returns'),
          dcaClass2015Stack: createIndexes(this, '/dca_class_2015_stack'),
          dcaClass2016AvgPrice: createIndexes(this, '/dca_class_2016_avg_price'),
          dcaClass2016Returns: createIndexes(this, '/dca_class_2016_returns'),
          dcaClass2016Stack: createIndexes(this, '/dca_class_2016_stack'),
          dcaClass2017AvgPrice: createIndexes(this, '/dca_class_2017_avg_price'),
          dcaClass2017Returns: createIndexes(this, '/dca_class_2017_returns'),
          dcaClass2017Stack: createIndexes(this, '/dca_class_2017_stack'),
          dcaClass2018AvgPrice: createIndexes(this, '/dca_class_2018_avg_price'),
          dcaClass2018Returns: createIndexes(this, '/dca_class_2018_returns'),
          dcaClass2018Stack: createIndexes(this, '/dca_class_2018_stack'),
          dcaClass2019AvgPrice: createIndexes(this, '/dca_class_2019_avg_price'),
          dcaClass2019Returns: createIndexes(this, '/dca_class_2019_returns'),
          dcaClass2019Stack: createIndexes(this, '/dca_class_2019_stack'),
          dcaClass2020AvgPrice: createIndexes(this, '/dca_class_2020_avg_price'),
          dcaClass2020Returns: createIndexes(this, '/dca_class_2020_returns'),
          dcaClass2020Stack: createIndexes(this, '/dca_class_2020_stack'),
          dcaClass2021AvgPrice: createIndexes(this, '/dca_class_2021_avg_price'),
          dcaClass2021Returns: createIndexes(this, '/dca_class_2021_returns'),
          dcaClass2021Stack: createIndexes(this, '/dca_class_2021_stack'),
          dcaClass2022AvgPrice: createIndexes(this, '/dca_class_2022_avg_price'),
          dcaClass2022Returns: createIndexes(this, '/dca_class_2022_returns'),
          dcaClass2022Stack: createIndexes(this, '/dca_class_2022_stack'),
          dcaClass2023AvgPrice: createIndexes(this, '/dca_class_2023_avg_price'),
          dcaClass2023Returns: createIndexes(this, '/dca_class_2023_returns'),
          dcaClass2023Stack: createIndexes(this, '/dca_class_2023_stack'),
          dcaClass2024AvgPrice: createIndexes(this, '/dca_class_2024_avg_price'),
          dcaClass2024Returns: createIndexes(this, '/dca_class_2024_returns'),
          dcaClass2024Stack: createIndexes(this, '/dca_class_2024_stack'),
          dcaClass2025AvgPrice: createIndexes(this, '/dca_class_2025_avg_price'),
          dcaClass2025Returns: createIndexes(this, '/dca_class_2025_returns'),
          dcaClass2025Stack: createIndexes(this, '/dca_class_2025_stack'),
          maxDaysBetweenPriceAths: createIndexes(this, '/max_days_between_price_aths'),
          maxYearsBetweenPriceAths: createIndexes(this, '/max_years_between_price_aths'),
          price10yAgo: createIndexes(this, '/price_10y_ago'),
          price13dEma: createPrice13dEmaPattern(this, 'price_13d_ema'),
          price13dSma: createPrice13dEmaPattern(this, 'price_13d_sma'),
          price144dEma: createPrice13dEmaPattern(this, 'price_144d_ema'),
          price144dSma: createPrice13dEmaPattern(this, 'price_144d_sma'),
          price1dAgo: createIndexes(this, '/price_1d_ago'),
          price1mAgo: createIndexes(this, '/price_1m_ago'),
          price1mEma: createPrice13dEmaPattern(this, 'price_1m_ema'),
          price1mMax: createIndexes(this, '/price_1m_max'),
          price1mMin: createIndexes(this, '/price_1m_min'),
          price1mSma: createPrice13dEmaPattern(this, 'price_1m_sma'),
          price1mVolatility: createIndexes(this, '/price_1m_volatility'),
          price1wAgo: createIndexes(this, '/price_1w_ago'),
          price1wEma: createPrice13dEmaPattern(this, 'price_1w_ema'),
          price1wMax: createIndexes(this, '/price_1w_max'),
          price1wMin: createIndexes(this, '/price_1w_min'),
          price1wSma: createPrice13dEmaPattern(this, 'price_1w_sma'),
          price1wVolatility: createIndexes(this, '/price_1w_volatility'),
          price1yAgo: createIndexes(this, '/price_1y_ago'),
          price1yEma: createPrice13dEmaPattern(this, 'price_1y_ema'),
          price1yMax: createIndexes(this, '/price_1y_max'),
          price1yMin: createIndexes(this, '/price_1y_min'),
          price1ySma: createPrice13dEmaPattern(this, 'price_1y_sma'),
          price1yVolatility: createIndexes(this, '/price_1y_volatility'),
          price200dEma: createPrice13dEmaPattern(this, 'price_200d_ema'),
          price200dSma: createPrice13dEmaPattern(this, 'price_200d_sma'),
          price200dSmaX08: createIndexes(this, '/price_200d_sma_x0_8'),
          price200dSmaX24: createIndexes(this, '/price_200d_sma_x2_4'),
          price200wEma: createPrice13dEmaPattern(this, 'price_200w_ema'),
          price200wSma: createPrice13dEmaPattern(this, 'price_200w_sma'),
          price21dEma: createPrice13dEmaPattern(this, 'price_21d_ema'),
          price21dSma: createPrice13dEmaPattern(this, 'price_21d_sma'),
          price2wChoppinessIndex: createIndexes(this, '/price_2w_choppiness_index'),
          price2wMax: createIndexes(this, '/price_2w_max'),
          price2wMin: createIndexes(this, '/price_2w_min'),
          price2yAgo: createIndexes(this, '/price_2y_ago'),
          price2yEma: createPrice13dEmaPattern(this, 'price_2y_ema'),
          price2ySma: createPrice13dEmaPattern(this, 'price_2y_sma'),
          price34dEma: createPrice13dEmaPattern(this, 'price_34d_ema'),
          price34dSma: createPrice13dEmaPattern(this, 'price_34d_sma'),
          price3mAgo: createIndexes(this, '/price_3m_ago'),
          price3yAgo: createIndexes(this, '/price_3y_ago'),
          price4yAgo: createIndexes(this, '/price_4y_ago'),
          price4yEma: createPrice13dEmaPattern(this, 'price_4y_ema'),
          price4ySma: createPrice13dEmaPattern(this, 'price_4y_sma'),
          price55dEma: createPrice13dEmaPattern(this, 'price_55d_ema'),
          price55dSma: createPrice13dEmaPattern(this, 'price_55d_sma'),
          price5yAgo: createIndexes(this, '/price_5y_ago'),
          price6mAgo: createIndexes(this, '/price_6m_ago'),
          price6yAgo: createIndexes(this, '/price_6y_ago'),
          price89dEma: createPrice13dEmaPattern(this, 'price_89d_ema'),
          price89dSma: createPrice13dEmaPattern(this, 'price_89d_sma'),
          price8dEma: createPrice13dEmaPattern(this, 'price_8d_ema'),
          price8dSma: createPrice13dEmaPattern(this, 'price_8d_sma'),
          price8yAgo: createIndexes(this, '/price_8y_ago'),
          priceAth: createIndexes26(this, '/price_ath'),
          priceDrawdown: createIndexes26(this, '/price_drawdown'),
          priceTrueRange: createIndexes5(this, '/price_true_range'),
          priceTrueRange2wSum: createIndexes5(this, '/price_true_range_2w_sum')
        },
        pools: {
          pool: createIndexes2(this, '/pool'),
          vecs: {
            aXbt: createAXbtPattern(this, 'computed_pools_vecs/AXbt'),
            aaoPool: createAXbtPattern(this, 'computed_pools_vecs/AaoPool'),
            antPool: createAXbtPattern(this, 'computed_pools_vecs/AntPool'),
            arkPool: createAXbtPattern(this, 'computed_pools_vecs/ArkPool'),
            asicMiner: createAXbtPattern(this, 'computed_pools_vecs/AsicMiner'),
            batPool: createAXbtPattern(this, 'computed_pools_vecs/BatPool'),
            bcMonster: createAXbtPattern(this, 'computed_pools_vecs/BcMonster'),
            bcpoolIo: createAXbtPattern(this, 'computed_pools_vecs/BcpoolIo'),
            binancePool: createAXbtPattern(this, 'computed_pools_vecs/BinancePool'),
            bitClub: createAXbtPattern(this, 'computed_pools_vecs/BitClub'),
            bitFuFuPool: createAXbtPattern(this, 'computed_pools_vecs/BitFuFuPool'),
            bitFury: createAXbtPattern(this, 'computed_pools_vecs/BitFury'),
            bitMinter: createAXbtPattern(this, 'computed_pools_vecs/BitMinter'),
            bitalo: createAXbtPattern(this, 'computed_pools_vecs/Bitalo'),
            bitcoinAffiliateNetwork: createAXbtPattern(this, 'computed_pools_vecs/BitcoinAffiliateNetwork'),
            bitcoinCom: createAXbtPattern(this, 'computed_pools_vecs/BitcoinCom'),
            bitcoinIndia: createAXbtPattern(this, 'computed_pools_vecs/BitcoinIndia'),
            bitcoinRussia: createAXbtPattern(this, 'computed_pools_vecs/BitcoinRussia'),
            bitcoinUkraine: createAXbtPattern(this, 'computed_pools_vecs/BitcoinUkraine'),
            bitfarms: createAXbtPattern(this, 'computed_pools_vecs/Bitfarms'),
            bitparking: createAXbtPattern(this, 'computed_pools_vecs/Bitparking'),
            bitsolo: createAXbtPattern(this, 'computed_pools_vecs/Bitsolo'),
            bixin: createAXbtPattern(this, 'computed_pools_vecs/Bixin'),
            blockFills: createAXbtPattern(this, 'computed_pools_vecs/BlockFills'),
            braiinsPool: createAXbtPattern(this, 'computed_pools_vecs/BraiinsPool'),
            bravoMining: createAXbtPattern(this, 'computed_pools_vecs/BravoMining'),
            btPool: createAXbtPattern(this, 'computed_pools_vecs/BtPool'),
            btcCom: createAXbtPattern(this, 'computed_pools_vecs/BtcCom'),
            btcDig: createAXbtPattern(this, 'computed_pools_vecs/BtcDig'),
            btcGuild: createAXbtPattern(this, 'computed_pools_vecs/BtcGuild'),
            btcLab: createAXbtPattern(this, 'computed_pools_vecs/BtcLab'),
            btcMp: createAXbtPattern(this, 'computed_pools_vecs/BtcMp'),
            btcNuggets: createAXbtPattern(this, 'computed_pools_vecs/BtcNuggets'),
            btcPoolParty: createAXbtPattern(this, 'computed_pools_vecs/BtcPoolParty'),
            btcServ: createAXbtPattern(this, 'computed_pools_vecs/BtcServ'),
            btcTop: createAXbtPattern(this, 'computed_pools_vecs/BtcTop'),
            btcc: createAXbtPattern(this, 'computed_pools_vecs/Btcc'),
            bwPool: createAXbtPattern(this, 'computed_pools_vecs/BwPool'),
            bytePool: createAXbtPattern(this, 'computed_pools_vecs/BytePool'),
            canoe: createAXbtPattern(this, 'computed_pools_vecs/Canoe'),
            canoePool: createAXbtPattern(this, 'computed_pools_vecs/CanoePool'),
            carbonNegative: createAXbtPattern(this, 'computed_pools_vecs/CarbonNegative'),
            ckPool: createAXbtPattern(this, 'computed_pools_vecs/CkPool'),
            cloudHashing: createAXbtPattern(this, 'computed_pools_vecs/CloudHashing'),
            coinLab: createAXbtPattern(this, 'computed_pools_vecs/CoinLab'),
            cointerra: createAXbtPattern(this, 'computed_pools_vecs/Cointerra'),
            connectBtc: createAXbtPattern(this, 'computed_pools_vecs/ConnectBtc'),
            dPool: createAXbtPattern(this, 'computed_pools_vecs/DPool'),
            dcExploration: createAXbtPattern(this, 'computed_pools_vecs/DcExploration'),
            dcex: createAXbtPattern(this, 'computed_pools_vecs/Dcex'),
            digitalBtc: createAXbtPattern(this, 'computed_pools_vecs/DigitalBtc'),
            digitalXMintsy: createAXbtPattern(this, 'computed_pools_vecs/DigitalXMintsy'),
            eclipseMc: createAXbtPattern(this, 'computed_pools_vecs/EclipseMc'),
            eightBaochi: createAXbtPattern(this, 'computed_pools_vecs/EightBaochi'),
            ekanemBtc: createAXbtPattern(this, 'computed_pools_vecs/EkanemBtc'),
            eligius: createAXbtPattern(this, 'computed_pools_vecs/Eligius'),
            emcdPool: createAXbtPattern(this, 'computed_pools_vecs/EmcdPool'),
            entrustCharityPool: createAXbtPattern(this, 'computed_pools_vecs/EntrustCharityPool'),
            eobot: createAXbtPattern(this, 'computed_pools_vecs/Eobot'),
            exxBw: createAXbtPattern(this, 'computed_pools_vecs/ExxBw'),
            f2Pool: createAXbtPattern(this, 'computed_pools_vecs/F2Pool'),
            fiftyEightCoin: createAXbtPattern(this, 'computed_pools_vecs/FiftyEightCoin'),
            foundryUsa: createAXbtPattern(this, 'computed_pools_vecs/FoundryUsa'),
            futureBitApolloSolo: createAXbtPattern(this, 'computed_pools_vecs/FutureBitApolloSolo'),
            gbMiners: createAXbtPattern(this, 'computed_pools_vecs/GbMiners'),
            ghashIo: createAXbtPattern(this, 'computed_pools_vecs/GhashIo'),
            giveMeCoins: createAXbtPattern(this, 'computed_pools_vecs/GiveMeCoins'),
            goGreenLight: createAXbtPattern(this, 'computed_pools_vecs/GoGreenLight'),
            haoZhuZhu: createAXbtPattern(this, 'computed_pools_vecs/HaoZhuZhu'),
            haominer: createAXbtPattern(this, 'computed_pools_vecs/Haominer'),
            hashBx: createAXbtPattern(this, 'computed_pools_vecs/HashBx'),
            hashPool: createAXbtPattern(this, 'computed_pools_vecs/HashPool'),
            helix: createAXbtPattern(this, 'computed_pools_vecs/Helix'),
            hhtt: createAXbtPattern(this, 'computed_pools_vecs/Hhtt'),
            hotPool: createAXbtPattern(this, 'computed_pools_vecs/HotPool'),
            hummerpool: createAXbtPattern(this, 'computed_pools_vecs/Hummerpool'),
            huobiPool: createAXbtPattern(this, 'computed_pools_vecs/HuobiPool'),
            innopolisTech: createAXbtPattern(this, 'computed_pools_vecs/InnopolisTech'),
            kanoPool: createAXbtPattern(this, 'computed_pools_vecs/KanoPool'),
            kncMiner: createAXbtPattern(this, 'computed_pools_vecs/KncMiner'),
            kuCoinPool: createAXbtPattern(this, 'computed_pools_vecs/KuCoinPool'),
            lubianCom: createAXbtPattern(this, 'computed_pools_vecs/LubianCom'),
            luckyPool: createAXbtPattern(this, 'computed_pools_vecs/LuckyPool'),
            luxor: createAXbtPattern(this, 'computed_pools_vecs/Luxor'),
            maraPool: createAXbtPattern(this, 'computed_pools_vecs/MaraPool'),
            maxBtc: createAXbtPattern(this, 'computed_pools_vecs/MaxBtc'),
            maxiPool: createAXbtPattern(this, 'computed_pools_vecs/MaxiPool'),
            megaBigPower: createAXbtPattern(this, 'computed_pools_vecs/MegaBigPower'),
            minerium: createAXbtPattern(this, 'computed_pools_vecs/Minerium'),
            miningCity: createAXbtPattern(this, 'computed_pools_vecs/MiningCity'),
            miningDutch: createAXbtPattern(this, 'computed_pools_vecs/MiningDutch'),
            miningKings: createAXbtPattern(this, 'computed_pools_vecs/MiningKings'),
            miningSquared: createAXbtPattern(this, 'computed_pools_vecs/MiningSquared'),
            mmpool: createAXbtPattern(this, 'computed_pools_vecs/Mmpool'),
            mtRed: createAXbtPattern(this, 'computed_pools_vecs/MtRed'),
            multiCoinCo: createAXbtPattern(this, 'computed_pools_vecs/MultiCoinCo'),
            multipool: createAXbtPattern(this, 'computed_pools_vecs/Multipool'),
            myBtcCoinPool: createAXbtPattern(this, 'computed_pools_vecs/MyBtcCoinPool'),
            neopool: createAXbtPattern(this, 'computed_pools_vecs/Neopool'),
            nexious: createAXbtPattern(this, 'computed_pools_vecs/Nexious'),
            niceHash: createAXbtPattern(this, 'computed_pools_vecs/NiceHash'),
            nmcBit: createAXbtPattern(this, 'computed_pools_vecs/NmcBit'),
            novaBlock: createAXbtPattern(this, 'computed_pools_vecs/NovaBlock'),
            ocean: createAXbtPattern(this, 'computed_pools_vecs/Ocean'),
            okExPool: createAXbtPattern(this, 'computed_pools_vecs/OkExPool'),
            okMiner: createAXbtPattern(this, 'computed_pools_vecs/OkMiner'),
            okkong: createAXbtPattern(this, 'computed_pools_vecs/Okkong'),
            okpoolTop: createAXbtPattern(this, 'computed_pools_vecs/OkpoolTop'),
            oneHash: createAXbtPattern(this, 'computed_pools_vecs/OneHash'),
            oneM1x: createAXbtPattern(this, 'computed_pools_vecs/OneM1x'),
            oneThash: createAXbtPattern(this, 'computed_pools_vecs/OneThash'),
            ozCoin: createAXbtPattern(this, 'computed_pools_vecs/OzCoin'),
            pHashIo: createAXbtPattern(this, 'computed_pools_vecs/PHashIo'),
            parasite: createAXbtPattern(this, 'computed_pools_vecs/Parasite'),
            patels: createAXbtPattern(this, 'computed_pools_vecs/Patels'),
            pegaPool: createAXbtPattern(this, 'computed_pools_vecs/PegaPool'),
            phoenix: createAXbtPattern(this, 'computed_pools_vecs/Phoenix'),
            polmine: createAXbtPattern(this, 'computed_pools_vecs/Polmine'),
            pool175btc: createAXbtPattern(this, 'computed_pools_vecs/Pool175btc'),
            pool50btc: createAXbtPattern(this, 'computed_pools_vecs/Pool50btc'),
            poolin: createAXbtPattern(this, 'computed_pools_vecs/Poolin'),
            portlandHodl: createAXbtPattern(this, 'computed_pools_vecs/PortlandHodl'),
            publicPool: createAXbtPattern(this, 'computed_pools_vecs/PublicPool'),
            pureBtcCom: createAXbtPattern(this, 'computed_pools_vecs/PureBtcCom'),
            rawpool: createAXbtPattern(this, 'computed_pools_vecs/Rawpool'),
            rigPool: createAXbtPattern(this, 'computed_pools_vecs/RigPool'),
            sbiCrypto: createAXbtPattern(this, 'computed_pools_vecs/SbiCrypto'),
            secPool: createAXbtPattern(this, 'computed_pools_vecs/SecPool'),
            secretSuperstar: createAXbtPattern(this, 'computed_pools_vecs/SecretSuperstar'),
            sevenPool: createAXbtPattern(this, 'computed_pools_vecs/SevenPool'),
            shawnP0wers: createAXbtPattern(this, 'computed_pools_vecs/ShawnP0wers'),
            sigmapoolCom: createAXbtPattern(this, 'computed_pools_vecs/SigmapoolCom'),
            simplecoinUs: createAXbtPattern(this, 'computed_pools_vecs/SimplecoinUs'),
            soloCk: createAXbtPattern(this, 'computed_pools_vecs/SoloCk'),
            spiderPool: createAXbtPattern(this, 'computed_pools_vecs/SpiderPool'),
            stMiningCorp: createAXbtPattern(this, 'computed_pools_vecs/StMiningCorp'),
            tangpool: createAXbtPattern(this, 'computed_pools_vecs/Tangpool'),
            tatmasPool: createAXbtPattern(this, 'computed_pools_vecs/TatmasPool'),
            tbDice: createAXbtPattern(this, 'computed_pools_vecs/TbDice'),
            telco214: createAXbtPattern(this, 'computed_pools_vecs/Telco214'),
            terraPool: createAXbtPattern(this, 'computed_pools_vecs/TerraPool'),
            tiger: createAXbtPattern(this, 'computed_pools_vecs/Tiger'),
            tigerpoolNet: createAXbtPattern(this, 'computed_pools_vecs/TigerpoolNet'),
            titan: createAXbtPattern(this, 'computed_pools_vecs/Titan'),
            transactionCoinMining: createAXbtPattern(this, 'computed_pools_vecs/TransactionCoinMining'),
            trickysBtcPool: createAXbtPattern(this, 'computed_pools_vecs/TrickysBtcPool'),
            tripleMining: createAXbtPattern(this, 'computed_pools_vecs/TripleMining'),
            twentyOneInc: createAXbtPattern(this, 'computed_pools_vecs/TwentyOneInc'),
            ultimusPool: createAXbtPattern(this, 'computed_pools_vecs/UltimusPool'),
            unknown: createAXbtPattern(this, 'computed_pools_vecs/Unknown'),
            unomp: createAXbtPattern(this, 'computed_pools_vecs/Unomp'),
            viaBtc: createAXbtPattern(this, 'computed_pools_vecs/ViaBtc'),
            waterhole: createAXbtPattern(this, 'computed_pools_vecs/Waterhole'),
            wayiCn: createAXbtPattern(this, 'computed_pools_vecs/WayiCn'),
            whitePool: createAXbtPattern(this, 'computed_pools_vecs/WhitePool'),
            wk057: createAXbtPattern(this, 'computed_pools_vecs/Wk057'),
            yourbtcNet: createAXbtPattern(this, 'computed_pools_vecs/YourbtcNet'),
            zulupool: createAXbtPattern(this, 'computed_pools_vecs/Zulupool')
          }
        },
        price: {
          priceClose: createIndexes3(this, '/price_close'),
          priceCloseInCents: createIndexes13(this, '/price_close_in_cents'),
          priceCloseInSats: createIndexes3(this, '/price_close_in_sats'),
          priceHigh: createIndexes3(this, '/price_high'),
          priceHighInCents: createIndexes13(this, '/price_high_in_cents'),
          priceHighInSats: createIndexes3(this, '/price_high_in_sats'),
          priceLow: createIndexes3(this, '/price_low'),
          priceLowInCents: createIndexes13(this, '/price_low_in_cents'),
          priceLowInSats: createIndexes3(this, '/price_low_in_sats'),
          priceOhlc: createIndexes3(this, '/price_ohlc'),
          priceOhlcInSats: createIndexes3(this, '/price_ohlc_in_sats'),
          priceOpen: createIndexes3(this, '/price_open'),
          priceOpenInCents: createIndexes13(this, '/price_open_in_cents'),
          priceOpenInSats: createIndexes3(this, '/price_open_in_sats')
        },
        stateful: {
          addrCount: createIndexes3(this, '/addr_count'),
          addressCohorts: {
            amountRange: {
              _0sats: create_0satsPattern(this, 'computed_stateful_address_cohorts_amount_range/_0sats'),
              _100btcTo1kBtc: create_0satsPattern(this, 'computed_stateful_address_cohorts_amount_range/_100btc_to_1k_btc'),
              _100kBtcOrMore: create_0satsPattern(this, 'computed_stateful_address_cohorts_amount_range/_100k_btc_or_more'),
              _100kSatsTo1mSats: create_0satsPattern(this, 'computed_stateful_address_cohorts_amount_range/_100k_sats_to_1m_sats'),
              _100satsTo1kSats: create_0satsPattern(this, 'computed_stateful_address_cohorts_amount_range/_100sats_to_1k_sats'),
              _10btcTo100btc: create_0satsPattern(this, 'computed_stateful_address_cohorts_amount_range/_10btc_to_100btc'),
              _10kBtcTo100kBtc: create_0satsPattern(this, 'computed_stateful_address_cohorts_amount_range/_10k_btc_to_100k_btc'),
              _10kSatsTo100kSats: create_0satsPattern(this, 'computed_stateful_address_cohorts_amount_range/_10k_sats_to_100k_sats'),
              _10mSatsTo1btc: create_0satsPattern(this, 'computed_stateful_address_cohorts_amount_range/_10m_sats_to_1btc'),
              _10satsTo100sats: create_0satsPattern(this, 'computed_stateful_address_cohorts_amount_range/_10sats_to_100sats'),
              _1btcTo10btc: create_0satsPattern(this, 'computed_stateful_address_cohorts_amount_range/_1btc_to_10btc'),
              _1kBtcTo10kBtc: create_0satsPattern(this, 'computed_stateful_address_cohorts_amount_range/_1k_btc_to_10k_btc'),
              _1kSatsTo10kSats: create_0satsPattern(this, 'computed_stateful_address_cohorts_amount_range/_1k_sats_to_10k_sats'),
              _1mSatsTo10mSats: create_0satsPattern(this, 'computed_stateful_address_cohorts_amount_range/_1m_sats_to_10m_sats'),
              _1satTo10sats: create_0satsPattern(this, 'computed_stateful_address_cohorts_amount_range/_1sat_to_10sats')
            },
            geAmount: {
              _100btc: create_0satsPattern(this, 'computed_stateful_address_cohorts_ge_amount/_100btc'),
              _100kSats: create_0satsPattern(this, 'computed_stateful_address_cohorts_ge_amount/_100k_sats'),
              _100sats: create_0satsPattern(this, 'computed_stateful_address_cohorts_ge_amount/_100sats'),
              _10btc: create_0satsPattern(this, 'computed_stateful_address_cohorts_ge_amount/_10btc'),
              _10kBtc: create_0satsPattern(this, 'computed_stateful_address_cohorts_ge_amount/_10k_btc'),
              _10kSats: create_0satsPattern(this, 'computed_stateful_address_cohorts_ge_amount/_10k_sats'),
              _10mSats: create_0satsPattern(this, 'computed_stateful_address_cohorts_ge_amount/_10m_sats'),
              _10sats: create_0satsPattern(this, 'computed_stateful_address_cohorts_ge_amount/_10sats'),
              _1btc: create_0satsPattern(this, 'computed_stateful_address_cohorts_ge_amount/_1btc'),
              _1kBtc: create_0satsPattern(this, 'computed_stateful_address_cohorts_ge_amount/_1k_btc'),
              _1kSats: create_0satsPattern(this, 'computed_stateful_address_cohorts_ge_amount/_1k_sats'),
              _1mSats: create_0satsPattern(this, 'computed_stateful_address_cohorts_ge_amount/_1m_sats'),
              _1sat: create_0satsPattern(this, 'computed_stateful_address_cohorts_ge_amount/_1sat')
            },
            ltAmount: {
              _100btc: create_0satsPattern(this, 'computed_stateful_address_cohorts_lt_amount/_100btc'),
              _100kBtc: create_0satsPattern(this, 'computed_stateful_address_cohorts_lt_amount/_100k_btc'),
              _100kSats: create_0satsPattern(this, 'computed_stateful_address_cohorts_lt_amount/_100k_sats'),
              _100sats: create_0satsPattern(this, 'computed_stateful_address_cohorts_lt_amount/_100sats'),
              _10btc: create_0satsPattern(this, 'computed_stateful_address_cohorts_lt_amount/_10btc'),
              _10kBtc: create_0satsPattern(this, 'computed_stateful_address_cohorts_lt_amount/_10k_btc'),
              _10kSats: create_0satsPattern(this, 'computed_stateful_address_cohorts_lt_amount/_10k_sats'),
              _10mSats: create_0satsPattern(this, 'computed_stateful_address_cohorts_lt_amount/_10m_sats'),
              _10sats: create_0satsPattern(this, 'computed_stateful_address_cohorts_lt_amount/_10sats'),
              _1btc: create_0satsPattern(this, 'computed_stateful_address_cohorts_lt_amount/_1btc'),
              _1kBtc: create_0satsPattern(this, 'computed_stateful_address_cohorts_lt_amount/_1k_btc'),
              _1kSats: create_0satsPattern(this, 'computed_stateful_address_cohorts_lt_amount/_1k_sats'),
              _1mSats: create_0satsPattern(this, 'computed_stateful_address_cohorts_lt_amount/_1m_sats')
            }
          },
          addressesData: {
            empty: createIndexes29(this, '/emptyaddressdata'),
            loaded: createIndexes30(this, '/loadedaddressdata')
          },
          addresstypeToHeightToAddrCount: createAddresstypeToHeightToAddrCountPattern(this, 'computed_stateful/addresstype_to_height_to_addr_count'),
          addresstypeToHeightToEmptyAddrCount: createAddresstypeToHeightToAddrCountPattern(this, 'computed_stateful/addresstype_to_height_to_empty_addr_count'),
          addresstypeToIndexesToAddrCount: createAddresstypeToHeightToAddrCountPattern(this, 'computed_stateful/addresstype_to_indexes_to_addr_count'),
          addresstypeToIndexesToEmptyAddrCount: createAddresstypeToHeightToAddrCountPattern(this, 'computed_stateful/addresstype_to_indexes_to_empty_addr_count'),
          anyAddressIndexes: createAddresstypeToHeightToAddrCountPattern(this, 'computed_stateful/any_address_indexes'),
          chainState: createIndexes2(this, '/chain'),
          emptyAddrCount: createIndexes3(this, '/empty_addr_count'),
          emptyaddressindex: createIndexes29(this, '/emptyaddressindex'),
          loadedaddressindex: createIndexes30(this, '/loadedaddressindex'),
          marketCap: createIndexes26(this, '/market_cap'),
          opreturnSupply: createSupplyPattern(this, 'computed_stateful/opreturn_supply'),
          unspendableSupply: createSupplyPattern(this, 'computed_stateful/unspendable_supply'),
          utxoCohorts: {
            ageRange: {
              _10yTo12y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_age_range/_10y_to_12y'),
              _12yTo15y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_age_range/_12y_to_15y'),
              _1dTo1w: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_age_range/_1d_to_1w'),
              _1mTo2m: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_age_range/_1m_to_2m'),
              _1wTo1m: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_age_range/_1w_to_1m'),
              _1yTo2y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_age_range/_1y_to_2y'),
              _2mTo3m: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_age_range/_2m_to_3m'),
              _2yTo3y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_age_range/_2y_to_3y'),
              _3mTo4m: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_age_range/_3m_to_4m'),
              _3yTo4y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_age_range/_3y_to_4y'),
              _4mTo5m: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_age_range/_4m_to_5m'),
              _4yTo5y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_age_range/_4y_to_5y'),
              _5mTo6m: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_age_range/_5m_to_6m'),
              _5yTo6y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_age_range/_5y_to_6y'),
              _6mTo1y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_age_range/_6m_to_1y'),
              _6yTo7y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_age_range/_6y_to_7y'),
              _7yTo8y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_age_range/_7y_to_8y'),
              _8yTo10y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_age_range/_8y_to_10y'),
              from15y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_age_range/from_15y'),
              upTo1d: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_age_range/up_to_1d')
            },
            all: {
              activity: createActivityPattern(this, 'computed_stateful_utxo_cohorts_all/activity'),
              pricePaid: createPricePaidPattern2(this, 'computed_stateful_utxo_cohorts_all/price_paid'),
              realized: createRealizedPattern3(this, 'computed_stateful_utxo_cohorts_all/realized'),
              relative: {
                negUnrealizedLossRelToOwnTotalUnrealizedPnl: createIndexes27(this, '/neg_unrealized_loss_rel_to_own_total_unrealized_pnl'),
                netUnrealizedPnlRelToOwnTotalUnrealizedPnl: createIndexes26(this, '/net_unrealized_pnl_rel_to_own_total_unrealized_pnl'),
                supplyInLossRelToOwnSupply: createIndexes27(this, '/supply_in_loss_rel_to_own_supply'),
                supplyInProfitRelToOwnSupply: createIndexes27(this, '/supply_in_profit_rel_to_own_supply'),
                unrealizedLossRelToOwnTotalUnrealizedPnl: createIndexes27(this, '/unrealized_loss_rel_to_own_total_unrealized_pnl'),
                unrealizedProfitRelToOwnTotalUnrealizedPnl: createIndexes27(this, '/unrealized_profit_rel_to_own_total_unrealized_pnl')
              },
              supply: createSupplyPattern2(this, 'computed_stateful_utxo_cohorts_all/supply'),
              unrealized: createUnrealizedPattern(this, 'computed_stateful_utxo_cohorts_all/unrealized')
            },
            amountRange: {
              _0sats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_amount_range/_0sats'),
              _100btcTo1kBtc: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_amount_range/_100btc_to_1k_btc'),
              _100kBtcOrMore: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_amount_range/_100k_btc_or_more'),
              _100kSatsTo1mSats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_amount_range/_100k_sats_to_1m_sats'),
              _100satsTo1kSats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_amount_range/_100sats_to_1k_sats'),
              _10btcTo100btc: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_amount_range/_10btc_to_100btc'),
              _10kBtcTo100kBtc: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_amount_range/_10k_btc_to_100k_btc'),
              _10kSatsTo100kSats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_amount_range/_10k_sats_to_100k_sats'),
              _10mSatsTo1btc: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_amount_range/_10m_sats_to_1btc'),
              _10satsTo100sats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_amount_range/_10sats_to_100sats'),
              _1btcTo10btc: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_amount_range/_1btc_to_10btc'),
              _1kBtcTo10kBtc: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_amount_range/_1k_btc_to_10k_btc'),
              _1kSatsTo10kSats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_amount_range/_1k_sats_to_10k_sats'),
              _1mSatsTo10mSats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_amount_range/_1m_sats_to_10m_sats'),
              _1satTo10sats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_amount_range/_1sat_to_10sats')
            },
            epoch: {
              _0: create_10yTo12yPattern(this, 'epoch/_0'),
              _1: create_10yTo12yPattern(this, 'epoch/_1'),
              _2: create_10yTo12yPattern(this, 'epoch/_2'),
              _3: create_10yTo12yPattern(this, 'epoch/_3'),
              _4: create_10yTo12yPattern(this, 'epoch/_4')
            },
            geAmount: {
              _100btc: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_ge_amount/_100btc'),
              _100kSats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_ge_amount/_100k_sats'),
              _100sats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_ge_amount/_100sats'),
              _10btc: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_ge_amount/_10btc'),
              _10kBtc: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_ge_amount/_10k_btc'),
              _10kSats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_ge_amount/_10k_sats'),
              _10mSats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_ge_amount/_10m_sats'),
              _10sats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_ge_amount/_10sats'),
              _1btc: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_ge_amount/_1btc'),
              _1kBtc: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_ge_amount/_1k_btc'),
              _1kSats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_ge_amount/_1k_sats'),
              _1mSats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_ge_amount/_1m_sats'),
              _1sat: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_ge_amount/_1sat')
            },
            ltAmount: {
              _100btc: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_lt_amount/_100btc'),
              _100kBtc: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_lt_amount/_100k_btc'),
              _100kSats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_lt_amount/_100k_sats'),
              _100sats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_lt_amount/_100sats'),
              _10btc: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_lt_amount/_10btc'),
              _10kBtc: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_lt_amount/_10k_btc'),
              _10kSats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_lt_amount/_10k_sats'),
              _10mSats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_lt_amount/_10m_sats'),
              _10sats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_lt_amount/_10sats'),
              _1btc: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_lt_amount/_1btc'),
              _1kBtc: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_lt_amount/_1k_btc'),
              _1kSats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_lt_amount/_1k_sats'),
              _1mSats: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_lt_amount/_1m_sats')
            },
            maxAge: {
              _10y: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_max_age/_10y'),
              _12y: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_max_age/_12y'),
              _15y: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_max_age/_15y'),
              _1m: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_max_age/_1m'),
              _1w: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_max_age/_1w'),
              _1y: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_max_age/_1y'),
              _2m: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_max_age/_2m'),
              _2y: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_max_age/_2y'),
              _3m: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_max_age/_3m'),
              _3y: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_max_age/_3y'),
              _4m: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_max_age/_4m'),
              _4y: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_max_age/_4y'),
              _5m: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_max_age/_5m'),
              _5y: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_max_age/_5y'),
              _6m: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_max_age/_6m'),
              _6y: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_max_age/_6y'),
              _7y: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_max_age/_7y'),
              _8y: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_max_age/_8y')
            },
            minAge: {
              _10y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_min_age/_10y'),
              _12y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_min_age/_12y'),
              _1d: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_min_age/_1d'),
              _1m: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_min_age/_1m'),
              _1w: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_min_age/_1w'),
              _1y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_min_age/_1y'),
              _2m: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_min_age/_2m'),
              _2y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_min_age/_2y'),
              _3m: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_min_age/_3m'),
              _3y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_min_age/_3y'),
              _4m: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_min_age/_4m'),
              _4y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_min_age/_4y'),
              _5m: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_min_age/_5m'),
              _5y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_min_age/_5y'),
              _6m: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_min_age/_6m'),
              _6y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_min_age/_6y'),
              _7y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_min_age/_7y'),
              _8y: create_10yTo12yPattern(this, 'computed_stateful_utxo_cohorts_min_age/_8y')
            },
            term: {
              long: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_term/long'),
              short: createUpTo1dPattern(this, 'computed_stateful_utxo_cohorts_term/short')
            },
            type: {
              empty: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_type_/empty'),
              p2a: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_type_/p2a'),
              p2ms: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_type_/p2ms'),
              p2pk33: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_type_/p2pk33'),
              p2pk65: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_type_/p2pk65'),
              p2pkh: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_type_/p2pkh'),
              p2sh: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_type_/p2sh'),
              p2tr: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_type_/p2tr'),
              p2wpkh: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_type_/p2wpkh'),
              p2wsh: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_type_/p2wsh'),
              unknown: create_0satsPattern2(this, 'computed_stateful_utxo_cohorts_type_/unknown')
            },
            year: {
              _2009: create_10yTo12yPattern(this, 'year/_2009'),
              _2010: create_10yTo12yPattern(this, 'year/_2010'),
              _2011: create_10yTo12yPattern(this, 'year/_2011'),
              _2012: create_10yTo12yPattern(this, 'year/_2012'),
              _2013: create_10yTo12yPattern(this, 'year/_2013'),
              _2014: create_10yTo12yPattern(this, 'year/_2014'),
              _2015: create_10yTo12yPattern(this, 'year/_2015'),
              _2016: create_10yTo12yPattern(this, 'year/_2016'),
              _2017: create_10yTo12yPattern(this, 'year/_2017'),
              _2018: create_10yTo12yPattern(this, 'year/_2018'),
              _2019: create_10yTo12yPattern(this, 'year/_2019'),
              _2020: create_10yTo12yPattern(this, 'year/_2020'),
              _2021: create_10yTo12yPattern(this, 'year/_2021'),
              _2022: create_10yTo12yPattern(this, 'year/_2022'),
              _2023: create_10yTo12yPattern(this, 'year/_2023'),
              _2024: create_10yTo12yPattern(this, 'year/_2024'),
              _2025: create_10yTo12yPattern(this, 'year/_2025'),
              _2026: create_10yTo12yPattern(this, 'year/_2026')
            }
          }
        },
        txins: {
          txoutindex: createIndexes24(this, '/txoutindex'),
          value: createIndexes24(this, '/value')
        },
        txouts: {
          txinindex: createIndexes25(this, '/txinindex')
        }
      },
      indexed: {
        address: {
          firstP2aaddressindex: createIndexes2(this, '/first_p2aaddressindex'),
          firstP2pk33addressindex: createIndexes2(this, '/first_p2pk33addressindex'),
          firstP2pk65addressindex: createIndexes2(this, '/first_p2pk65addressindex'),
          firstP2pkhaddressindex: createIndexes2(this, '/first_p2pkhaddressindex'),
          firstP2shaddressindex: createIndexes2(this, '/first_p2shaddressindex'),
          firstP2traddressindex: createIndexes2(this, '/first_p2traddressindex'),
          firstP2wpkhaddressindex: createIndexes2(this, '/first_p2wpkhaddressindex'),
          firstP2wshaddressindex: createIndexes2(this, '/first_p2wshaddressindex'),
          p2abytes: createIndexes16(this, '/p2abytes'),
          p2pk33bytes: createIndexes17(this, '/p2pk33bytes'),
          p2pk65bytes: createIndexes18(this, '/p2pk65bytes'),
          p2pkhbytes: createIndexes19(this, '/p2pkhbytes'),
          p2shbytes: createIndexes20(this, '/p2shbytes'),
          p2trbytes: createIndexes21(this, '/p2trbytes'),
          p2wpkhbytes: createIndexes22(this, '/p2wpkhbytes'),
          p2wshbytes: createIndexes23(this, '/p2wshbytes')
        },
        block: {
          blockhash: createIndexes2(this, '/blockhash'),
          difficulty: createIndexes2(this, '/difficulty'),
          timestamp: createIndexes2(this, '/timestamp'),
          totalSize: createIndexes2(this, '/total_size'),
          weight: createIndexes2(this, '/weight')
        },
        output: {
          firstEmptyoutputindex: createIndexes2(this, '/first_emptyoutputindex'),
          firstOpreturnindex: createIndexes2(this, '/first_opreturnindex'),
          firstP2msoutputindex: createIndexes2(this, '/first_p2msoutputindex'),
          firstUnknownoutputindex: createIndexes2(this, '/first_unknownoutputindex'),
          txindex: new MetricNode(this, '/txindex')
        },
        tx: {
          baseSize: createIndexes6(this, '/base_size'),
          firstTxindex: createIndexes2(this, '/first_txindex'),
          firstTxinindex: createIndexes6(this, '/first_txinindex'),
          firstTxoutindex: createIndexes6(this, '/first_txoutindex'),
          height: createIndexes6(this, '/height'),
          isExplicitlyRbf: createIndexes6(this, '/is_explicitly_rbf'),
          rawlocktime: createIndexes6(this, '/rawlocktime'),
          totalSize: createIndexes6(this, '/total_size'),
          txid: createIndexes6(this, '/txid'),
          txversion: createIndexes6(this, '/txversion')
        },
        txin: {
          firstTxinindex: createIndexes2(this, '/first_txinindex'),
          outpoint: createIndexes24(this, '/outpoint'),
          outputtype: createIndexes24(this, '/outputtype'),
          txindex: createIndexes24(this, '/txindex'),
          typeindex: createIndexes24(this, '/typeindex')
        },
        txout: {
          firstTxoutindex: createIndexes2(this, '/first_txoutindex'),
          outputtype: createIndexes25(this, '/outputtype'),
          txindex: createIndexes25(this, '/txindex'),
          typeindex: createIndexes25(this, '/typeindex'),
          value: createIndexes25(this, '/value')
        }
      }
    };
  }

  /**
   * Address information
   * @description Retrieve comprehensive information about a Bitcoin address including balance, transaction history, UTXOs, and estimated investment metrics. Supports all standard Bitcoin address types (P2PKH, P2SH, P2WPKH, P2WSH, P2TR, etc.).
   * @param {string} address 
   * @returns {Promise<AddressStats>}
   */
  async getApiAddressByAddress(address) {
    return this.get(`/api/address/${address}`);
  }

  /**
   * Address transaction IDs
   * @description Get transaction IDs for an address, newest first. Use after_txid for pagination.
   * @param {string} address 
   * @param {string=} [after_txid] Txid to paginate from (return transactions before this one)
   * @param {string=} [limit] Maximum number of results to return. Defaults to 25 if not specified.
   * @returns {Promise<Txid[]>}
   */
  async getApiAddressByAddressTxs(address, after_txid, limit) {
    const params = new URLSearchParams();
    if (after_txid !== undefined) params.set('after_txid', String(after_txid));
    if (limit !== undefined) params.set('limit', String(limit));
    const query = params.toString();
    return this.get(`/api/address/${address}/txs${query ? '?' + query : ''}`);
  }

  /**
   * Address confirmed transactions
   * @description Get confirmed transaction IDs for an address, 25 per page. Use ?after_txid=<txid> for pagination.
   * @param {string} address 
   * @param {string=} [after_txid] Txid to paginate from (return transactions before this one)
   * @param {string=} [limit] Maximum number of results to return. Defaults to 25 if not specified.
   * @returns {Promise<Txid[]>}
   */
  async getApiAddressByAddressTxsChain(address, after_txid, limit) {
    const params = new URLSearchParams();
    if (after_txid !== undefined) params.set('after_txid', String(after_txid));
    if (limit !== undefined) params.set('limit', String(limit));
    const query = params.toString();
    return this.get(`/api/address/${address}/txs/chain${query ? '?' + query : ''}`);
  }

  /**
   * Address mempool transactions
   * @description Get unconfirmed transaction IDs for an address from the mempool (up to 50).
   * @param {string} address 
   * @returns {Promise<Txid[]>}
   */
  async getApiAddressByAddressTxsMempool(address) {
    return this.get(`/api/address/${address}/txs/mempool`);
  }

  /**
   * Address UTXOs
   * @description Get unspent transaction outputs for an address.
   * @param {string} address 
   * @returns {Promise<Utxo[]>}
   */
  async getApiAddressByAddressUtxo(address) {
    return this.get(`/api/address/${address}/utxo`);
  }

  /**
   * Block by height
   * @description Retrieve block information by block height. Returns block metadata including hash, timestamp, difficulty, size, weight, and transaction count.
   * @param {string} height 
   * @returns {Promise<BlockInfo>}
   */
  async getApiBlockHeightByHeight(height) {
    return this.get(`/api/block-height/${height}`);
  }

  /**
   * Block information
   * @description Retrieve block information by block hash. Returns block metadata including height, timestamp, difficulty, size, weight, and transaction count.
   * @param {string} hash 
   * @returns {Promise<BlockInfo>}
   */
  async getApiBlockByHash(hash) {
    return this.get(`/api/block/${hash}`);
  }

  /**
   * Raw block
   * @description Returns the raw block data in binary format.
   * @param {string} hash 
   * @returns {Promise<number[]>}
   */
  async getApiBlockByHashRaw(hash) {
    return this.get(`/api/block/${hash}/raw`);
  }

  /**
   * Block status
   * @description Retrieve the status of a block. Returns whether the block is in the best chain and, if so, its height and the hash of the next block.
   * @param {string} hash 
   * @returns {Promise<BlockStatus>}
   */
  async getApiBlockByHashStatus(hash) {
    return this.get(`/api/block/${hash}/status`);
  }

  /**
   * Transaction ID at index
   * @description Retrieve a single transaction ID at a specific index within a block. Returns plain text txid.
   * @param {string} hash Bitcoin block hash
   * @param {string} index Transaction index within the block (0-based)
   * @returns {Promise<Txid>}
   */
  async getApiBlockByHashTxidByIndex(hash, index) {
    return this.get(`/api/block/${hash}/txid/${index}`);
  }

  /**
   * Block transaction IDs
   * @description Retrieve all transaction IDs in a block by block hash.
   * @param {string} hash 
   * @returns {Promise<Txid[]>}
   */
  async getApiBlockByHashTxids(hash) {
    return this.get(`/api/block/${hash}/txids`);
  }

  /**
   * Block transactions (paginated)
   * @description Retrieve transactions in a block by block hash, starting from the specified index. Returns up to 25 transactions at a time.
   * @param {string} hash Bitcoin block hash
   * @param {string} start_index Starting transaction index within the block (0-based)
   * @returns {Promise<Transaction[]>}
   */
  async getApiBlockByHashTxsByStartIndex(hash, start_index) {
    return this.get(`/api/block/${hash}/txs/${start_index}`);
  }

  /**
   * Recent blocks
   * @description Retrieve the last 10 blocks. Returns block metadata for each block.
   * @returns {Promise<BlockInfo[]>}
   */
  async getApiBlocks() {
    return this.get(`/api/blocks`);
  }

  /**
   * Blocks from height
   * @description Retrieve up to 10 blocks going backwards from the given height. For example, height=100 returns blocks 100, 99, 98, ..., 91. Height=0 returns only block 0.
   * @param {string} height 
   * @returns {Promise<BlockInfo[]>}
   */
  async getApiBlocksByHeight(height) {
    return this.get(`/api/blocks/${height}`);
  }

  /**
   * Mempool statistics
   * @description Get current mempool statistics including transaction count, total vsize, and total fees.
   * @returns {Promise<MempoolInfo>}
   */
  async getApiMempoolInfo() {
    return this.get(`/api/mempool/info`);
  }

  /**
   * Mempool transaction IDs
   * @description Get all transaction IDs currently in the mempool.
   * @returns {Promise<Txid[]>}
   */
  async getApiMempoolTxids() {
    return this.get(`/api/mempool/txids`);
  }

  /**
   * Get supported indexes for a metric
   * @description Returns the list of indexes are supported by the specified metric. For example, `realized_price` might be available on dateindex, weekindex, and monthindex.
   * @param {string} metric 
   * @returns {Promise<Index[]>}
   */
  async getApiMetricByMetric(metric) {
    return this.get(`/api/metric/${metric}`);
  }

  /**
   * Get metric data
   * @description Fetch data for a specific metric at the given index. Use query parameters to filter by date range and format (json/csv).
   * @param {string} metric Metric name
   * @param {string} index Aggregation index
   * @param {string=} [from] Inclusive starting index, if negative counts from end
   * @param {string=} [to] Exclusive ending index, if negative counts from end
   * @param {string=} [count] Number of values to return (ignored if `to` is set)
   * @param {string=} [format] Format of the output
   * @returns {Promise<MetricData>}
   */
  async getApiMetricByMetricByIndex(metric, index, from, to, count, format) {
    const params = new URLSearchParams();
    if (from !== undefined) params.set('from', String(from));
    if (to !== undefined) params.set('to', String(to));
    if (count !== undefined) params.set('count', String(count));
    if (format !== undefined) params.set('format', String(format));
    const query = params.toString();
    return this.get(`/api/metric/${metric}/${index}${query ? '?' + query : ''}`);
  }

  /**
   * Bulk metric data
   * @description Fetch multiple metrics in a single request. Supports filtering by index and date range. Returns an array of MetricData objects.
   * @param {string} [metrics] Requested metrics
   * @param {string} [index] Index to query
   * @param {string=} [from] Inclusive starting index, if negative counts from end
   * @param {string=} [to] Exclusive ending index, if negative counts from end
   * @param {string=} [count] Number of values to return (ignored if `to` is set)
   * @param {string=} [format] Format of the output
   * @returns {Promise<MetricData[]>}
   */
  async getApiMetricsBulk(metrics, index, from, to, count, format) {
    const params = new URLSearchParams();
    params.set('metrics', String(metrics));
    params.set('index', String(index));
    if (from !== undefined) params.set('from', String(from));
    if (to !== undefined) params.set('to', String(to));
    if (count !== undefined) params.set('count', String(count));
    if (format !== undefined) params.set('format', String(format));
    const query = params.toString();
    return this.get(`/api/metrics/bulk${query ? '?' + query : ''}`);
  }

  /**
   * Metrics catalog
   * @description Returns the complete hierarchical catalog of available metrics organized as a tree structure. Metrics are grouped by categories and subcategories. Best viewed in an interactive JSON viewer (e.g., Firefox's built-in JSON viewer) for easy navigation of the nested structure.
   * @returns {Promise<TreeNode>}
   */
  async getApiMetricsCatalog() {
    return this.get(`/api/metrics/catalog`);
  }

  /**
   * Metric count
   * @description Current metric count
   * @returns {Promise<MetricCount[]>}
   */
  async getApiMetricsCount() {
    return this.get(`/api/metrics/count`);
  }

  /**
   * List available indexes
   * @description Returns all available indexes with their accepted query aliases. Use any alias when querying metrics.
   * @returns {Promise<IndexInfo[]>}
   */
  async getApiMetricsIndexes() {
    return this.get(`/api/metrics/indexes`);
  }

  /**
   * Metrics list
   * @description Paginated list of available metrics
   * @param {string=} [page] Pagination index
   * @returns {Promise<PaginatedMetrics>}
   */
  async getApiMetricsList(page) {
    const params = new URLSearchParams();
    if (page !== undefined) params.set('page', String(page));
    const query = params.toString();
    return this.get(`/api/metrics/list${query ? '?' + query : ''}`);
  }

  /**
   * Search metrics
   * @description Fuzzy search for metrics by name. Supports partial matches and typos.
   * @param {string} metric 
   * @param {string=} [limit] 
   * @returns {Promise<Metric[]>}
   */
  async getApiMetricsSearchByMetric(metric, limit) {
    const params = new URLSearchParams();
    if (limit !== undefined) params.set('limit', String(limit));
    const query = params.toString();
    return this.get(`/api/metrics/search/${metric}${query ? '?' + query : ''}`);
  }

  /**
   * Transaction information
   * @description Retrieve complete transaction data by transaction ID (txid). Returns the full transaction details including inputs, outputs, and metadata. The transaction data is read directly from the blockchain data files.
   * @param {string} txid 
   * @returns {Promise<Transaction>}
   */
  async getApiTxByTxid(txid) {
    return this.get(`/api/tx/${txid}`);
  }

  /**
   * Transaction hex
   * @description Retrieve the raw transaction as a hex-encoded string. Returns the serialized transaction in hexadecimal format.
   * @param {string} txid 
   * @returns {Promise<Hex>}
   */
  async getApiTxByTxidHex(txid) {
    return this.get(`/api/tx/${txid}/hex`);
  }

  /**
   * Output spend status
   * @description Get the spending status of a transaction output. Returns whether the output has been spent and, if so, the spending transaction details.
   * @param {string} txid Transaction ID
   * @param {string} vout Output index
   * @returns {Promise<TxOutspend>}
   */
  async getApiTxByTxidOutspendByVout(txid, vout) {
    return this.get(`/api/tx/${txid}/outspend/${vout}`);
  }

  /**
   * All output spend statuses
   * @description Get the spending status of all outputs in a transaction. Returns an array with the spend status for each output.
   * @param {string} txid 
   * @returns {Promise<TxOutspend[]>}
   */
  async getApiTxByTxidOutspends(txid) {
    return this.get(`/api/tx/${txid}/outspends`);
  }

  /**
   * Transaction status
   * @description Retrieve the confirmation status of a transaction. Returns whether the transaction is confirmed and, if so, the block height, hash, and timestamp.
   * @param {string} txid 
   * @returns {Promise<TxStatus>}
   */
  async getApiTxByTxidStatus(txid) {
    return this.get(`/api/tx/${txid}/status`);
  }

  /**
   * Difficulty adjustment
   * @description Get current difficulty adjustment information including progress through the current epoch, estimated retarget date, and difficulty change prediction.
   * @returns {Promise<DifficultyAdjustment>}
   */
  async getApiV1DifficultyAdjustment() {
    return this.get(`/api/v1/difficulty-adjustment`);
  }

  /**
   * Projected mempool blocks
   * @description Get projected blocks from the mempool for fee estimation. Each block contains statistics about transactions that would be included if a block were mined now.
   * @returns {Promise<MempoolBlock[]>}
   */
  async getApiV1FeesMempoolBlocks() {
    return this.get(`/api/v1/fees/mempool-blocks`);
  }

  /**
   * Recommended fees
   * @description Get recommended fee rates for different confirmation targets based on current mempool state.
   * @returns {Promise<RecommendedFees>}
   */
  async getApiV1FeesRecommended() {
    return this.get(`/api/v1/fees/recommended`);
  }

  /**
   * Block fees
   * @description Get average block fees for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y
   * @param {string} time_period 
   * @returns {Promise<BlockFeesEntry[]>}
   */
  async getApiV1MiningBlocksFeesByTimePeriod(time_period) {
    return this.get(`/api/v1/mining/blocks/fees/${time_period}`);
  }

  /**
   * Block rewards
   * @description Get average block rewards (coinbase = subsidy + fees) for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y
   * @param {string} time_period 
   * @returns {Promise<BlockRewardsEntry[]>}
   */
  async getApiV1MiningBlocksRewardsByTimePeriod(time_period) {
    return this.get(`/api/v1/mining/blocks/rewards/${time_period}`);
  }

  /**
   * Block sizes and weights
   * @description Get average block sizes and weights for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y
   * @param {string} time_period 
   * @returns {Promise<BlockSizesWeights>}
   */
  async getApiV1MiningBlocksSizesWeightsByTimePeriod(time_period) {
    return this.get(`/api/v1/mining/blocks/sizes-weights/${time_period}`);
  }

  /**
   * Block by timestamp
   * @description Find the block closest to a given UNIX timestamp.
   * @param {string} timestamp 
   * @returns {Promise<BlockTimestamp>}
   */
  async getApiV1MiningBlocksTimestampByTimestamp(timestamp) {
    return this.get(`/api/v1/mining/blocks/timestamp/${timestamp}`);
  }

  /**
   * Difficulty adjustments (all time)
   * @description Get historical difficulty adjustments. Returns array of [timestamp, height, difficulty, change_percent].
   * @returns {Promise<DifficultyAdjustmentEntry[]>}
   */
  async getApiV1MiningDifficultyAdjustments() {
    return this.get(`/api/v1/mining/difficulty-adjustments`);
  }

  /**
   * Difficulty adjustments
   * @description Get historical difficulty adjustments for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y. Returns array of [timestamp, height, difficulty, change_percent].
   * @param {string} time_period 
   * @returns {Promise<DifficultyAdjustmentEntry[]>}
   */
  async getApiV1MiningDifficultyAdjustmentsByTimePeriod(time_period) {
    return this.get(`/api/v1/mining/difficulty-adjustments/${time_period}`);
  }

  /**
   * Network hashrate (all time)
   * @description Get network hashrate and difficulty data for all time.
   * @returns {Promise<HashrateSummary>}
   */
  async getApiV1MiningHashrate() {
    return this.get(`/api/v1/mining/hashrate`);
  }

  /**
   * Network hashrate
   * @description Get network hashrate and difficulty data for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y
   * @param {string} time_period 
   * @returns {Promise<HashrateSummary>}
   */
  async getApiV1MiningHashrateByTimePeriod(time_period) {
    return this.get(`/api/v1/mining/hashrate/${time_period}`);
  }

  /**
   * Mining pool details
   * @description Get detailed information about a specific mining pool including block counts and shares for different time periods.
   * @param {string} slug 
   * @returns {Promise<PoolDetail>}
   */
  async getApiV1MiningPoolBySlug(slug) {
    return this.get(`/api/v1/mining/pool/${slug}`);
  }

  /**
   * List all mining pools
   * @description Get list of all known mining pools with their identifiers.
   * @returns {Promise<PoolInfo[]>}
   */
  async getApiV1MiningPools() {
    return this.get(`/api/v1/mining/pools`);
  }

  /**
   * Mining pool statistics
   * @description Get mining pool statistics for a time period. Valid periods: 24h, 3d, 1w, 1m, 3m, 6m, 1y, 2y, 3y
   * @param {string} time_period 
   * @returns {Promise<PoolsSummary>}
   */
  async getApiV1MiningPoolsByTimePeriod(time_period) {
    return this.get(`/api/v1/mining/pools/${time_period}`);
  }

  /**
   * Mining reward statistics
   * @description Get mining reward statistics for the last N blocks including total rewards, fees, and transaction count.
   * @param {string} block_count Number of recent blocks to include
   * @returns {Promise<RewardStats>}
   */
  async getApiV1MiningRewardStatsByBlockCount(block_count) {
    return this.get(`/api/v1/mining/reward-stats/${block_count}`);
  }

  /**
   * Validate address
   * @description Validate a Bitcoin address and get information about its type and scriptPubKey.
   * @param {string} address Bitcoin address to validate (can be any string)
   * @returns {Promise<AddressValidation>}
   */
  async getApiV1ValidateAddressByAddress(address) {
    return this.get(`/api/v1/validate-address/${address}`);
  }

  /**
   * Health check
   * @description Returns the health status of the API server
   * @returns {Promise<Health>}
   */
  async getHealth() {
    return this.get(`/health`);
  }

  /**
   * API version
   * @description Returns the current version of the API server
   * @returns {Promise<string>}
   */
  async getVersion() {
    return this.get(`/version`);
  }

}

export { BrkClient, BrkClientBase, BrkError, MetricNode };
