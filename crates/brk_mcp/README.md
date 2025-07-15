# BRK MCP

A Model Context Protocol (MCP) which gives LLMs access to all available tools in BRK

## URLs

- https://eu1.bitcoinresearchkit.org/mcp
- https://eu2.bitcoinresearchkit.org/mcp

## Usage

To connect to the MCP use any of the previous URL, no token or auth is needed.

This implementation has only been tested with Claude and the [MCP inspector](https://modelcontextprotocol.io/docs/tools/inspector).

Please be aware that the technology is evolving very rapidly, thus having issues is probably expected. If you, you can join the discord see if there is a solution.

### Claude

#### Step 1

First we need to connect BRK to Claude. To do that we need to go to the "Connect apps" menu from the home screen of Claude desktop.

![Image of Claude Desktop home screen](https://github.com/bitcoinresearchkit/brk/blob/main/assets/claude-step1.png)

#### Step 2

Then simply go to "Add integration".

![Image of the Connect app" menu of Claude Desktop](https://github.com/bitcoinresearchkit/brk/blob/main/assets/claude-step2.png)

#### Step 3

Claude's MCP client is (for now?) session based thus using a URL pointing to a load balancer will not work.

Use one of the following URL instead:

- https://eu1.bitcoinresearchkit.org/mcp
- https://eu2.bitcoinresearchkit.org/mcp

![Image of Add Integration menu of Claude Desktop](https://github.com/bitcoinresearchkit/brk/blob/main/assets/claude-step3.png)

#### Step 4

Verify that it has access to BRK's tools.

Optionally and highly recommended, giving it unsupervised access gives a more fluid experience and prevents possible issues and errors.

![Image of edit integration meny on Claude Desktop](https://github.com/bitcoinresearchkit/brk/blob/main/assets/claude-step4.png)
