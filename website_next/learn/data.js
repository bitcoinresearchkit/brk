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
  {
    title: "Ownership",
    description:
      "How coins are held across balances, entities, custody patterns, and long-term cohorts.",
    chart: "Ownership overview",
    children: [
      {
        title: "Balances",
        description:
          "Address and entity balances grouped by size, concentration, and historical change.",
        chart: "Balance cohorts",
        children: [],
      },
      {
        title: "Entities",
        description:
          "Estimated ownership clusters and how their behavior changes through market regimes.",
        chart: "Entity supply",
        children: [],
      },
      {
        title: "Custody",
        description:
          "Coins associated with exchanges, funds, miners, and other observable custody groups.",
        chart: "Custody balances",
        children: [],
      },
    ],
  },
  {
    title: "Liquidity",
    description:
      "How available supply changes as coins move between liquid, illiquid, and exchange venues.",
    chart: "Liquidity overview",
    children: [
      {
        title: "Liquid Supply",
        description:
          "Coins held by entities that tend to spend, trade, or redistribute frequently.",
        chart: "Liquid supply",
        children: [],
      },
      {
        title: "Illiquid Supply",
        description:
          "Coins held by entities with low spending history and stronger accumulation behavior.",
        chart: "Illiquid supply",
        children: [],
      },
      {
        title: "Exchange Flow",
        description:
          "Deposits, withdrawals, and balance changes across known exchange clusters.",
        chart: "Exchange netflow",
        children: [],
      },
    ],
  },
  {
    title: "Risk",
    description:
      "Stress, leverage, drawdown, and valuation conditions that shape market fragility.",
    chart: "Risk overview",
    children: [
      {
        title: "Drawdown",
        description:
          "Distance from prior highs and the depth of cycle retracements over time.",
        chart: "Drawdown",
        children: [],
      },
      {
        title: "Stress",
        description:
          "Periods where losses, volatility, and fee pressure concentrate together.",
        chart: "Network stress",
        children: [],
      },
      {
        title: "Leverage",
        description:
          "Market conditions that indicate amplified exposure and forced positioning risk.",
        chart: "Leverage proxy",
        children: [],
      },
    ],
  },
  {
    title: "Cycles",
    description:
      "How Bitcoin behaves across halvings, adoption waves, liquidity regimes, and market phases.",
    chart: "Cycle overview",
    children: [
      {
        title: "Halvings",
        description:
          "Supply issuance changes and their relationship to market and miner behavior.",
        chart: "Halving cycles",
        children: [],
      },
      {
        title: "Phases",
        description:
          "Bull, bear, recovery, and transition periods described through on-chain behavior.",
        chart: "Cycle phases",
        children: [],
      },
      {
        title: "Comparisons",
        description:
          "Cycle-to-cycle comparisons normalized by time, price, drawdown, or supply behavior.",
        chart: "Cycle comparison",
        children: [],
      },
    ],
  },
  {
    title: "Cohorts",
    description:
      "Groups of market participants organized by age, balance, cost basis, and observed behavior.",
    chart: "Cohort overview",
    children: [
      {
        title: "Short Term",
        description:
          "Recently moved coins and holders more sensitive to price, volatility, and liquidity.",
        chart: "Short-term holder supply",
        children: [],
      },
      {
        title: "Long Term",
        description:
          "Older coins and holders with stronger dormancy, conviction, or lower spend frequency.",
        chart: "Long-term holder supply",
        children: [],
      },
      {
        title: "Cost Basis",
        description:
          "Estimated acquisition prices across cohorts and how they frame profit and loss.",
        chart: "Cohort cost basis",
        children: [],
      },
    ],
  },
];
