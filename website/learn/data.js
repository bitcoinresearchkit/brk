export const sections = [
  {
    title: "Supply",
    description:
      "How bitcoin moves from issuance into long-term ownership, profit, loss, and distribution.",
    chart: "Circulating supply",
    children: [
      {
        title: "Profitability",
        description:
          "Which coins sit in profit or loss, and how that balance changes through cycles.",
        chart: "Supply in profit",
        children: [],
      },
      {
        title: "Age",
        description:
          "How long coins have remained still, from fresh movement to deep dormancy.",
        chart: "Supply by age",
        children: [],
      },
      {
        title: "Distribution",
        description:
          "How supply is spread across addresses, scripts, cohorts, and balance ranges.",
        chart: "Supply distribution",
        children: [],
      },
    ],
  },
  {
    title: "Capitalization",
    description:
      "Different ways to value the network by market price, realized cost, and accumulated flows.",
    chart: "Capitalization overview",
    children: [
      {
        title: "Market Cap",
        description:
          "The current market value of circulating bitcoin at spot price.",
        chart: "Market capitalization",
        children: [],
      },
      {
        title: "Realized Cap",
        description:
          "The aggregate value of coins priced where they last moved on-chain.",
        chart: "Realized capitalization",
        children: [],
      },
      {
        title: "Value Bands",
        description:
          "How market value compares with cost basis and historical valuation ranges.",
        chart: "Valuation bands",
        children: [],
      },
    ],
  },
  {
    title: "Activity",
    description:
      "How often the chain is used, how value moves, and how demand appears in fees and transactions.",
    chart: "Network activity",
    children: [
      {
        title: "Transactions",
        description:
          "Confirmed transaction count, throughput, and block-level settlement patterns.",
        chart: "Transaction count",
        children: [],
      },
      {
        title: "Fees",
        description:
          "The cost users pay for block space and what that reveals about demand.",
        chart: "Fee rate",
        children: [],
      },
      {
        title: "Addresses",
        description:
          "Address creation, reuse, activity, and balance changes across the network.",
        chart: "Active addresses",
        children: [],
      },
    ],
  },
  {
    title: "Mining",
    description:
      "The security budget, difficulty adjustments, pool behavior, and miner revenue.",
    chart: "Mining overview",
    children: [
      {
        title: "Hashrate",
        description:
          "Estimated computational power securing the network over time.",
        chart: "Hashrate",
        children: [],
      },
      {
        title: "Difficulty",
        description:
          "How Bitcoin adjusts mining difficulty to keep block production steady.",
        chart: "Difficulty",
        children: [],
      },
      {
        title: "Rewards",
        description:
          "Subsidy, fees, and the changing economics of block production.",
        chart: "Miner rewards",
        children: [],
      },
    ],
  },
  {
    title: "Market",
    description:
      "Price behavior, returns, volatility, and the market context around on-chain patterns.",
    chart: "Market overview",
    children: [
      {
        title: "Price",
        description:
          "Bitcoin price across time, cycles, drawdowns, and all-time highs.",
        chart: "Price",
        children: [],
      },
      {
        title: "Returns",
        description:
          "How returns vary by holding period, entry point, and cycle phase.",
        chart: "Returns",
        children: [],
      },
      {
        title: "Volatility",
        description:
          "The scale and rhythm of price movement across different windows.",
        chart: "Volatility",
        children: [],
      },
    ],
  },
];
