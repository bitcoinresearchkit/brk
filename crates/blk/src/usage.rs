use owo_colors::OwoColorize;

const SEL_W: usize = 5; // longest selector token: "tip-N"
const LABEL_W: usize = 28; // longest label across OUTPUT/OPTIONS/EXAMPLES (= example cmd "blk 800000 tx.0.vout.0.value")
const FLAG_W: usize = 15; // longest flag: "--rpccookiefile"
const PH_W: usize = LABEL_W - FLAG_W - 1; // placeholder column width so flag+ph total = LABEL_W
const GAP: usize = 4;

pub fn print() {
    println!("{} - inspect a Bitcoin Core block", "blk".bold());
    println!();

    section("USAGE");
    println!(
        "    blk {} [{} ...] [OPTIONS]",
        "<selector>".bright_black(),
        "<field>".bright_black()
    );
    println!(
        "    {}",
        "no fields = full block as JSON (analog of `bitcoin-cli getblock <hash> 2`)".bright_black()
    );
    println!();

    section("SELECTOR");
    sel("<n>", "single height (e.g. 800000)");
    sel("tip", "current chain tip");
    sel("tip-N", "tip minus N");
    sel("a..b", "inclusive range, endpoints can be height/tip/tip-N");
    println!();

    section("FIELDS");
    println!(
        "    {}",
        "dotted paths drill into nested data; omit an index for arrays".bright_black()
    );
    println!();
    group("block");
    fields(&[
        "height, hash, time, version, version_hex, bits, nonce,",
        "prev, merkle, difficulty, txs, n_inputs, n_outputs,",
        "witness_txs, size, strippedsize, weight, subsidy, coinbase,",
        "header_hex, hex",
    ]);
    println!();
    group_note("tx.i", "omit i for all txs");
    fields(&[
        "txid, wtxid, version, locktime, size, base_size, vsize,",
        "weight, inputs, outputs, is_coinbase, has_witness, is_rbf,",
        "total_out, hex",
    ]);
    println!();
    group_note("tx.i.vin.j", "omit j for all inputs");
    fields(&[
        "prev_txid, prev_vout, sequence, script_sig, script_sig_asm,",
        "witness, has_witness, is_rbf, coinbase",
    ]);
    println!();
    group_note("tx.i.vout.j", "omit j for all outputs");
    fields(&["value, script_pubkey, script_pubkey_asm, type, address"]);
    println!();
    println!(
        "    {}",
        "Naked tx / tx.i / vin / vout returns the whole sub-object as JSON.".bright_black()
    );
    println!();

    section("OUTPUT");
    out("no fields", "full block JSON object, one per line (NDJSON)");
    out("1 field", "bare value, one per line");
    out("2+ fields", "compact JSON object, one per line (NDJSON)");
    out("-p, --pretty", "pretty JSON object instead");
    out(
        "-c, --compact",
        "tab-separated values, no field names (TSV)",
    );
    println!();

    section("OPTIONS");
    opt(
        "--bitcoindir",
        "<PATH>",
        "Bitcoin directory",
        Some("[OS default]"),
    );
    opt(
        "--blocksdir",
        "<PATH>",
        "Blocks directory",
        Some("[<bitcoindir>/blocks]"),
    );
    opt("--rpcconnect", "<IP>", "RPC host", Some("[localhost]"));
    opt("--rpcport", "<PORT>", "RPC port", Some("[8332]"));
    opt(
        "--rpccookiefile",
        "<PATH>",
        "RPC cookie file",
        Some("[<bitcoindir>/.cookie]"),
    );
    opt("--rpcuser", "<USERNAME>", "RPC username", None);
    opt("--rpcpassword", "<PASSWORD>", "RPC password", None);
    println!();

    section("EXAMPLES");
    ex("blk 800000", "full block as JSON");
    ex("blk 800000 hash", "bare hash");
    ex("blk 800000 height hash time", "one compact JSON line");
    ex("blk 800000 tx.0.txid", "coinbase txid");
    ex("blk 800000 tx.txid", "all txids in block (array)");
    ex("blk 800000 tx.0.vout.0.value", "coinbase output 0 sats");
    ex("blk 800000 tx.0.vout.value", "all output sats for tx 0");
    ex("blk 800000 tx.vout.value", "array of arrays (per tx)");
    ex("blk 0..2 hash tx.0.txid", "3 NDJSON lines");
    ex("blk tip tx.0", "whole coinbase tx as JSON");
}

fn section(name: &str) {
    println!("{}", format!("{name}:").bold());
}

fn group(name: &str) {
    println!("  {}", format!("{name}:").bold());
}

fn group_note(name: &str, note: &str) {
    println!(
        "  {}  {}",
        format!("{name}:").bold(),
        format!("({note})").bright_black()
    );
}

fn fields(lines: &[&str]) {
    for line in lines {
        println!("    {line}");
    }
}

fn pad(s: &str, width: usize) -> String {
    " ".repeat(width.saturating_sub(s.len()))
}

fn sel(token: &str, desc: &str) {
    println!(
        "    {}{}{}{desc}",
        token.bright_black(),
        pad(token, SEL_W),
        " ".repeat(GAP),
    );
}

fn out(label: &str, desc: &str) {
    println!(
        "    {label}{}{}{desc}",
        pad(label, LABEL_W),
        " ".repeat(GAP)
    );
}

fn opt(flag: &str, ph: &str, desc: &str, default: Option<&str>) {
    let head = format!(
        "    {flag}{} {}{}{}",
        pad(flag, FLAG_W),
        ph.bright_black(),
        pad(ph, PH_W),
        " ".repeat(GAP),
    );
    match default {
        Some(d) => println!("{head}{desc} {}", d.bright_black()),
        None => println!("{head}{desc}"),
    }
}

fn ex(cmd: &str, note: &str) {
    println!(
        "    {cmd}{}{}{}",
        pad(cmd, LABEL_W),
        " ".repeat(GAP),
        format!("# {note}").bright_black()
    );
}
