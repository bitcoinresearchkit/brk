# README Generation Prompt

Generate a professional, comprehensive README.md for each crate based SOLELY on code analysis. Use NO external documentation, commit messages, or existing READMEs.

## MANDATORY PROCESS - FOLLOW EXACTLY:
1. **IGNORE EXISTING DOCS**: Do NOT read any .md, .txt, .rst, or documentation files in the crate directory
2. **CODE-ONLY ANALYSIS**: Examine ONLY these files:
   - All .rs files in src/ directory and subdirectories
   - Cargo.toml for dependencies and metadata
   - Code structure and organization
3. **MANDATORY CODE ANALYSIS**: Before writing ANY README content, you MUST:
   - Examine all Rust files in src/ directory
   - Identify the main structs, enums, traits, and functions
   - Understand the crate's architecture and data flow
   - Determine the crate's purpose from its implementation
   - Map dependencies to understand external integrations
4. **FRESH PERSPECTIVE**: Write the README as if you're the first person to document this crate
5. Generate one complete README.md per crate
6. Focus on one crate at a time for thorough analysis

## ABSOLUTE REQUIREMENTS:
- **SOURCE OF TRUTH**: Use ONLY the actual Rust code - no external docs, comments may provide hints but focus on implementation
- **CRITICAL**: DO NOT read any existing README.md, CHANGELOG.md, or documentation files
- **IGNORE ALL TEXT FILES**: .md, .txt, .rst files are FORBIDDEN sources - treat them as if they don't exist
- **CODE ONLY**: Focus exclusively on .rs files, Cargo.toml, and code structure
- **PROFESSIONAL GRADE**: Write as if this will be published on crates.io for other developers
- **PROGRAMMER FOCUSED**: Assume audience knows Rust and relevant domain concepts
- **IMPLEMENTATION-BASED**: Describe what the code actually does, not what comments claim it should do
- **If you cannot determine functionality from code alone, state this explicitly**

## README STRUCTURE (MANDATORY):

### 1. CRATE HEADER
```markdown
# Crate Name

Brief one-line description of what this crate does (max 80 chars).

[![Crates.io](https://img.shields.io/crates/v/CRATE_NAME.svg)](https://crates.io/crates/CRATE_NAME)
[![Documentation](https://docs.rs/CRATE_NAME/badge.svg)](https://docs.rs/CRATE_NAME)
```

### 2. OVERVIEW SECTION
- **Purpose**: What problem does this crate solve?
- **Key Features**: 3-5 bullet points of main capabilities (derived from code analysis)
- **Target Use Cases**: Who would use this and for what?

### 3. INSTALLATION
```toml
[dependencies]
crate_name = "X.Y.Z"
```

### 4. QUICK START / USAGE
- **Minimal working example** showing the primary API
- **Common patterns** observed in the code
- **Key structs/traits** that users will interact with

### 5. API OVERVIEW
- **Core Types**: Main structs, enums, traits with brief descriptions
- **Key Methods**: Most important public functions
- **Module Structure**: Brief overview of how code is organized

### 6. FEATURES (if applicable)
- Cargo features and what they enable
- Optional dependencies and their purpose

### 7. EXAMPLES
- 2-3 practical code examples showing different use cases
- Based on public API analysis, not existing examples

## WRITING REQUIREMENTS:

### TONE AND STYLE:
- **Concise but comprehensive**: Every sentence must add value
- **Technical precision**: Use exact terminology, avoid marketing speak
- **Active voice**: "Provides X" not "X is provided"
- **Present tense**: "The crate handles..." not "The crate will handle..."

### FORBIDDEN PATTERNS:
- **NEVER** use vague terms: "powerful", "flexible", "robust", "comprehensive", "advanced"
- **NEVER** write marketing copy: "cutting-edge", "state-of-the-art", "enterprise-grade"
- **NEVER** make claims you can't verify from code: "blazingly fast", "memory efficient"
- **NEVER** copy-paste from existing documentation or comments
- **NEVER** read or reference existing README.md files - pretend they don't exist
- **NEVER** use phrases like "as mentioned in the documentation" or "according to the docs"
- **NEVER** let existing documentation influence your analysis or writing

### REQUIRED SPECIFICITY:
- **Data structures**: Mention specific types (HashMap, Vec, etc.)
- **Algorithms**: Reference actual implementations found in code
- **Integration points**: Specific traits implemented, dependencies used
- **Error handling**: How errors are represented and handled
- **Async/sync**: Clearly state if operations are blocking or async

### ANTI-BIAS PROTOCOL:

### BEFORE STARTING ANY ANALYSIS:
1. **Explicitly ignore**: Any README.md, CHANGELOG.md, docs/, documentation files
2. **File filtering**: Only examine .rs and Cargo.toml files
3. **Fresh eyes approach**: Analyze the code as if you've never seen this crate before
4. **Independent thinking**: Form your own understanding purely from code inspection

### IF YOU ACCIDENTALLY READ EXISTING DOCS:
- Stop immediately and restart your analysis
- Consciously disregard any information from documentation files
- Base all descriptions solely on what you observe in the code
- Ask yourself: "What would I think this code does if I had no documentation?"

### VALIDATION CHECKS:
- **Unique descriptions**: Your descriptions should differ significantly from any existing docs
- **Code-derived insights**: Every feature mentioned must be visible in the source code
- **Independent voice**: Write in your own technical style, not mimicking existing documentation
- **Fresh examples**: Create new code examples based on API analysis, not existing samples

## CODE ANALYSIS DEPTH:
**You MUST analyze and understand:**
1. **Public API surface**: All pub structs, functions, traits, modules
2. **Core abstractions**: Main data types and their relationships
3. **Error types**: Custom errors, Result patterns, panic conditions
4. **Dependencies**: How external crates are integrated
5. **Feature flags**: Conditional compilation and optional functionality
6. **Async patterns**: Use of futures, tokio, async-std, etc.
7. **Serialization**: Serde implementations, custom serialization
8. **Performance characteristics**: Algorithm complexity where obvious

### EXAMPLE STRUCTURE ANALYSIS OUTPUT:
```markdown
## Code Analysis Summary
**Main Types**: `BlockProcessor`, `Transaction`, `ValidationError`
**Core Trait**: `Validator` - implemented by `BasicValidator` and `StrictValidator`
**Async Support**: All processing methods return `impl Future`
**Error Handling**: Custom `ValidationError` enum with specific error types
**Dependencies**: Uses `tokio` for async runtime, `serde` for serialization
**Architecture**: Pipeline pattern with configurable validation stages
```

## EXAMPLES OF QUALITY:

### ❌ BAD (VAGUE):
```markdown
# My Crate
A powerful and flexible library for blockchain operations.

## Features
- Fast processing
- Easy to use
- Robust error handling
```

### ✅ GOOD (SPECIFIC):
```markdown
# brk-chain-analyzer
Bitcoin blockchain analysis tools for transaction pattern detection.

## Overview
Provides utilities for analyzing Bitcoin transaction data, detecting address clustering patterns, and computing blockchain statistics. Built around a streaming parser that processes block data without loading entire blocks into memory.

## Key Types
- `TransactionAnalyzer`: Stateful analyzer for computing fees, detecting coinbase transactions
- `ClusterDetector`: Implements common input ownership heuristics for address clustering
- `BlockStream`: Async iterator over blockchain data with configurable batch sizes
```

## FINAL REQUIREMENTS:
- **One README per crate** - don't combine multiple crates
- **Minimum 200 words** - be thorough but concise
- **Maximum 800 words** - stay focused and relevant
- **Code examples must be syntactically correct** and compilable
- **All claims must be verifiable** from the source code

**PROCESS ONE CRATE AT A TIME. ANALYZE THE CODE THOROUGHLY BEFORE WRITING.**
