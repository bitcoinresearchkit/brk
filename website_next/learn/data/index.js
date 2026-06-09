import { addressCountSection } from "./sections/address-count.js";
import { capitalizationSection } from "./sections/capitalization.js";
import { introductionSection } from "./sections/introduction.js";
import { miningPoolsSection } from "./sections/mining-pools.js";
import { supplySection } from "./sections/supply.js";
import { utxoSetSection } from "./sections/utxo-set.js";

export const sections = [
  introductionSection,
  supplySection,
  utxoSetSection,
  addressCountSection,
  miningPoolsSection,
  capitalizationSection,
];
