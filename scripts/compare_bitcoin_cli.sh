#!/usr/bin/env bash
#
# Compare brk output against bitcoin-cli to find discrepancies.
#
set -euo pipefail

BRK="http://localhost:3110"
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BOLD='\033[1m'
NC='\033[0m'

pass=0
fail=0
warn=0

# Convert BTC decimal string to sats using python Decimal (no float loss)
btc_to_sats() {
    python3 -c "from decimal import Decimal; print(int(Decimal('$1') * 100000000))"
}

# Sum an array of BTC values to sats precisely
sum_btc_values_to_sats() {
    python3 -c "
import sys, json
from decimal import Decimal
vals = json.load(sys.stdin)
print(int(sum(Decimal(str(v)) for v in vals) * 100000000))
"
}

compare() {
    local label="$1" btc_val="$2" brk_val="$3"
    if [[ "$btc_val" == "$brk_val" ]]; then
        printf "${GREEN}  ✓ %-30s${NC}  %s\n" "$label" "$btc_val"
        ((pass++)) || true
    else
        printf "${RED}  ✗ %-30s${NC}  bitcoin-cli: %-20s  brk: %-20s\n" "$label" "$btc_val" "$brk_val"
        ((fail++)) || true
    fi
}

compare_float() {
    local label="$1" btc_val="$2" brk_val="$3"
    local btc_short="${btc_val:0:15}" brk_short="${brk_val:0:15}"
    if [[ "$btc_short" == "$brk_short" ]]; then
        printf "${GREEN}  ✓ %-30s${NC}  %s\n" "$label" "$btc_val"
        ((pass++)) || true
    else
        printf "${YELLOW}  ~ %-30s${NC}  bitcoin-cli: %-20s  brk: %-20s\n" "$label" "$btc_val" "$brk_val"
        ((warn++)) || true
    fi
}

section() {
    printf "\n${BOLD}── %s ──${NC}\n" "$1"
}

# ─── Get tip from brk ───
brk_height=$(curl -sf "$BRK/api/blocks/tip/height")
brk_hash=$(curl -sf "$BRK/api/blocks/tip/hash")
btc_height=$(bitcoin-cli getblockcount)
btc_hash=$(bitcoin-cli getbestblockhash)

section "Chain tip"
compare "height" "$btc_height" "$brk_height"
compare "hash" "$btc_hash" "$brk_hash"

# If tips differ, use the lower height for comparison
if [[ "$btc_height" != "$brk_height" ]]; then
    printf "${YELLOW}  Tips differ — comparing at brk height %s${NC}\n" "$brk_height"
    btc_hash=$(bitcoin-cli getblockhash "$brk_height")
    brk_hash=$(curl -sf "$BRK/api/block-height/$brk_height")
    compare "hash at $brk_height" "$btc_hash" "$brk_hash"
fi

# ─── gettxoutsetinfo ───
section "gettxoutsetinfo"
printf "  Running gettxoutsetinfo (this can take a while)...\n"
txoutset=$(bitcoin-cli gettxoutsetinfo 2>/dev/null || echo '{}')

if [[ "$txoutset" != '{}' ]]; then
    btc_txouts=$(echo "$txoutset" | jq -r '.txouts')
    btc_total_amount=$(echo "$txoutset" | jq -r '.total_amount')
    btc_txoutset_height=$(echo "$txoutset" | jq -r '.height')

    # Get brk utxoSetSize from block extras at that height
    txoutset_hash=$(bitcoin-cli getblockhash "$btc_txoutset_height")
    brk_block=$(curl -sf "$BRK/api/v1/block/$txoutset_hash")
    brk_utxo_set_size=$(echo "$brk_block" | jq -r '.extras.utxoSetSize')

    compare "txouts (UTXO count)" "$btc_txouts" "$brk_utxo_set_size"

    if [[ "$btc_txouts" != "$brk_utxo_set_size" ]]; then
        diff=$((brk_utxo_set_size - btc_txouts))
        printf "${RED}    → delta: %d${NC}\n" "$diff"
    fi

    btc_supply_sats=$(btc_to_sats "$btc_total_amount")
    printf "  ${YELLOW}total_amount:${NC} bitcoin-cli: %s BTC (%s sats) — check brk supply series\n" "$btc_total_amount" "$btc_supply_sats"
else
    printf "  ${YELLOW}gettxoutsetinfo unavailable or timed out${NC}\n"
fi

# ─── getblock comparison ───
section "getblock vs /api/v1/block (at tip: $brk_hash)"

btc_block=$(bitcoin-cli getblock "$brk_hash")
brk_v1=$(curl -sf "$BRK/api/v1/block/$brk_hash")

btc_size=$(echo "$btc_block" | jq -r '.size')
btc_weight=$(echo "$btc_block" | jq -r '.weight')
btc_nTx=$(echo "$btc_block" | jq -r '.nTx')
btc_difficulty=$(echo "$btc_block" | jq -r '.difficulty')
btc_version=$(echo "$btc_block" | jq -r '.version')
# bitcoin-cli returns bits as hex string, brk as decimal — convert
btc_bits_hex=$(echo "$btc_block" | jq -r '.bits')
btc_bits=$(printf '%d' "0x$btc_bits_hex" 2>/dev/null || echo "$btc_bits_hex")
btc_nonce=$(echo "$btc_block" | jq -r '.nonce')
btc_timestamp=$(echo "$btc_block" | jq -r '.time')
btc_mediantime=$(echo "$btc_block" | jq -r '.mediantime')
btc_merkle=$(echo "$btc_block" | jq -r '.merkleroot')
btc_prevhash=$(echo "$btc_block" | jq -r '.previousblockhash')

brk_size=$(echo "$brk_v1" | jq -r '.size')
brk_weight=$(echo "$brk_v1" | jq -r '.weight')
brk_nTx=$(echo "$brk_v1" | jq -r '.tx_count')
brk_difficulty=$(echo "$brk_v1" | jq -r '.difficulty')
brk_version=$(echo "$brk_v1" | jq -r '.version')
brk_bits=$(echo "$brk_v1" | jq -r '.bits')
brk_nonce=$(echo "$brk_v1" | jq -r '.nonce')
brk_timestamp=$(echo "$brk_v1" | jq -r '.timestamp')
brk_mediantime=$(echo "$brk_v1" | jq -r '.mediantime')
brk_merkle=$(echo "$brk_v1" | jq -r '.merkle_root')
brk_prevhash=$(echo "$brk_v1" | jq -r '.previousblockhash')

compare "size" "$btc_size" "$brk_size"
compare "weight" "$btc_weight" "$brk_weight"
compare "nTx / tx_count" "$btc_nTx" "$brk_nTx"
compare_float "difficulty" "$btc_difficulty" "$brk_difficulty"
compare "version" "$btc_version" "$brk_version"
compare "bits" "$btc_bits" "$brk_bits"
compare "nonce" "$btc_nonce" "$brk_nonce"
compare "timestamp" "$btc_timestamp" "$brk_timestamp"
compare "mediantime" "$btc_mediantime" "$brk_mediantime"
compare "merkle_root" "$btc_merkle" "$brk_merkle"
compare "previousblockhash" "$btc_prevhash" "$brk_prevhash"

# ─── Block extras sanity checks ───
section "Block extras sanity (at tip)"

btc_block_v2=$(bitcoin-cli getblock "$brk_hash" 2 2>/dev/null || echo '{}')

if [[ "$btc_block_v2" != '{}' ]]; then
    btc_total_outputs=$(echo "$btc_block_v2" | jq '[.tx[].vout | length] | add')
    btc_total_inputs=$(echo "$btc_block_v2" | jq '[.tx[1:][].vin | length] | add // 0')

    brk_total_outputs=$(echo "$brk_v1" | jq -r '.extras.totalOutputs')
    brk_total_inputs=$(echo "$brk_v1" | jq -r '.extras.totalInputs')

    compare "totalOutputs" "$btc_total_outputs" "$brk_total_outputs"
    compare "totalInputs" "$btc_total_inputs" "$brk_total_inputs"

    # totalOutputAmt excludes coinbase (matches mempool.space), so compare non-coinbase outputs only
    btc_total_output_amt=$(echo "$btc_block_v2" | jq '[.tx[1:][].vout[].value]' | sum_btc_values_to_sats)
    brk_total_output_amt=$(echo "$brk_v1" | jq -r '.extras.totalOutputAmt')
    compare "totalOutputAmt (sats)" "$btc_total_output_amt" "$brk_total_output_amt"

    if [[ "$btc_total_output_amt" != "$brk_total_output_amt" ]]; then
        delta=$((brk_total_output_amt - btc_total_output_amt))
        printf "${RED}    → delta: %d sats (%.8f BTC)${NC}\n" "$delta" "$(python3 -c "print($delta / 1e8)")"
    fi

    # Reward = subsidy + fees. bitcoin-cli coinbase output sum = reward
    btc_coinbase_value=$(echo "$btc_block_v2" | jq '[.tx[0].vout[].value]' | sum_btc_values_to_sats)
    brk_reward=$(echo "$brk_v1" | jq -r '.extras.reward')
    compare "reward (coinbase sats)" "$btc_coinbase_value" "$brk_reward"

    # Total input amount — needs verbosity 3 for prevout data
    btc_block_v3=$(bitcoin-cli getblock "$brk_hash" 3 2>/dev/null || echo '{}')
    if [[ "$btc_block_v3" != '{}' ]]; then
        btc_total_input_amt=$(echo "$btc_block_v3" | jq '[.tx[1:][].vin[].prevout.value]' | sum_btc_values_to_sats)
        brk_total_input_amt=$(echo "$brk_v1" | jq -r '.extras.totalInputAmt')
        compare "totalInputAmt (sats)" "$btc_total_input_amt" "$brk_total_input_amt"

        if [[ "$btc_total_input_amt" != "$brk_total_input_amt" ]]; then
            delta=$((brk_total_input_amt - btc_total_input_amt))
            printf "${RED}    → delta: %d sats (%.8f BTC)${NC}\n" "$delta" "$(python3 -c "print($delta / 1e8)")"
        fi

        # fees = non-coinbase inputs - non-coinbase outputs
        btc_fees=$((btc_total_input_amt - btc_total_output_amt))
        brk_fees=$(echo "$brk_v1" | jq -r '.extras.totalFees')
        compare "totalFees (sats)" "$btc_fees" "$brk_fees"
    else
        printf "  ${YELLOW}getblock verbosity 3 unavailable — skipping totalInputAmt${NC}\n"
    fi
else
    printf "  ${YELLOW}getblock verbosity 2 unavailable${NC}\n"
fi

# ─── getmempoolinfo ───
section "getmempoolinfo vs /api/mempool"

btc_mempool=$(bitcoin-cli getmempoolinfo)
brk_mempool=$(curl -sf "$BRK/api/mempool")

btc_mp_count=$(echo "$btc_mempool" | jq -r '.size')
btc_mp_vsize=$(echo "$btc_mempool" | jq -r '.bytes')
brk_mp_count=$(echo "$brk_mempool" | jq -r '.count')
brk_mp_vsize=$(echo "$brk_mempool" | jq -r '.vsize')

printf "  ${YELLOW}%-30s${NC}  bitcoin-cli: %-12s  brk: %-12s  (live, may differ)\n" "tx count" "$btc_mp_count" "$brk_mp_count"
printf "  ${YELLOW}%-30s${NC}  bitcoin-cli: %-12s  brk: %-12s  (live, may differ)\n" "vsize" "$btc_mp_vsize" "$brk_mp_vsize"

# ─── Difficulty adjustment ───
section "Difficulty adjustment"

compare_float "current difficulty" "$(echo "$btc_block" | jq -r '.difficulty')" "$(echo "$brk_v1" | jq -r '.difficulty')"

# ─── Summary ───
section "Summary"
printf "  ${GREEN}Passed: %d${NC}  ${RED}Failed: %d${NC}  ${YELLOW}Approximate: %d${NC}\n" "$pass" "$fail" "$warn"

if [[ $fail -gt 0 ]]; then
    exit 1
fi
