# aibranch

An AI-powered Git branch name generator that follows the [Conventional Branch](https://github.com/conventional-branch/conventional-branch) naming convention. This tool analyzes your Git diff and automatically generates descriptive, standardized branch names.

## Overview

**aibranch** consists of two components:

1. **Cloudflare Worker** (TypeScript/Bun) - An AI workflow service powered by VoltAgent and OpenAI
2. **CLI Tool** (Rust) - A command-line interface that interacts with your Git repository

The CLI tool reads your current Git changes, sends them to the AI service, and suggests a conventional branch name based on your modifications.

## Features

- 🤖 **AI-Powered**: Uses OpenAI's GPT models to analyze code changes
- 📏 **Conventional Branch Standard**: Follows industry-standard branch naming conventions
- ⚡ **Fast & Lightweight**: Rust CLI with minimal dependencies
- 🌐 **Serverless**: Runs on Cloudflare Workers for scalability
- 🔒 **Secure**: Uses rustls-tls for secure communications

## Getting Started

```bash
curl https://i.jpillora.com/sakkeam/aibranch@latest! | bash
```

## Example

```
(main) $ git switch -c work
(work) $ git commit

(work) $ aibranch
Generating the new branch name...

Generated branch name: chore/add-devcontainer

Reason: Adds a new .devcontainer/devcontainer.json to set up devcontainer (VS Code extensions and pnpm onCreateCommand) — non-feature configuration change, classified as chore.

Do you want to approve this branch name? [Y/n] y

(chore/add-devcontainer) $ git push origin chore/add-devcontainer
```

## Branch Naming Convention

The tool generates branch names following these prefixes:

- `feature/` or `feat/` - New features
- `bugfix/` or `fix/` - Bug fixes
- `hotfix/` - Urgent fixes
- `release/` - Release preparation
- `chore/` - Non-code tasks (docs, dependencies, etc.)

Branch names are always lowercase with hyphens separating words (e.g., `feature/add-user-authentication`).

## Installation

### Prerequisites

- [Bun](https://bun.sh) v1.2.23 or higher
- [Rust](https://rustup.rs) (latest stable)
- [Wrangler CLI](https://developers.cloudflare.com/workers/wrangler/) for deployment
- Git

### Setup

1. **Install dependencies**:

```bash
bun install
```

2. **Build the CLI tool**:

```bash
cd cli
cargo build --release
```

The compiled binary will be available at `cli/target/release/cli`.

## Usage

### Running the Worker Locally

Start the development server:

```bash
bun run dev
```

The worker will be available at `http://0.0.0.0:8787`.

### Using the CLI

1. Make some changes in your Git repository
2. Commit or stage your changes
3. Run the CLI:

```bash
./cli/target/release/cli
```

4. The tool will:
   - Analyze your Git diff (comparing `main..HEAD`)
   - Generate a suggested branch name
   - Provide a reason for the suggestion
   - Prompt you to approve and apply the branch name

### Environment Variables

Configure the CLI to use a custom worker endpoint:

```bash
export AIBRANCH_HOST="https://your-custom-domain.workers.dev"
```

Default: `https://aibranch-worker.sakkeam.workers.dev`

## Deployment

Deploy the worker to Cloudflare:

```bash
bun run deploy
```

Make sure to configure your environment variables in Wrangler:

- `OPENAI_API_KEY` - Your OpenAI API key (required)
- `VOLTAGENT_PUBLIC_KEY` - VoltAgent public key (optional)
- `VOLTAGENT_SECRET_KEY` - VoltAgent secret key (optional)

## Development

### Project Structure

```
.
├── src/
│   ├── index.ts                          # Main worker entry point
│   └── workflows/
│       └── branch-name-generator.ts      # AI workflow logic
├── cli/
│   └── src/
│       └── main.rs                       # CLI implementation
├── package.json                          # Bun/Node dependencies
├── wrangler.jsonc                        # Cloudflare Worker config
└── tsconfig.json                         # TypeScript config
```

### Tech Stack

**Worker:**

- VoltAgent - AI agent framework
- OpenAI SDK - GPT model integration
- Hono - Lightweight web framework
- Cloudflare Workers - Serverless platform

**CLI:**

- Clap - Command-line argument parsing
- Reqwest - HTTP client with rustls-tls
- Tokio - Async runtime
- Serde - JSON serialization

### Available Scripts

```bash
bun run start      # Run TypeScript entry point locally
bun run dev        # Start Wrangler dev server
bun run build      # Build the worker
bun run deploy     # Deploy to Cloudflare
```

## How It Works

1. The CLI runs `git diff main..HEAD` to get your changes
2. The diff is sent to the Cloudflare Worker's `/workflows/branch-name-generator/execute` endpoint
3. VoltAgent processes the request through the AI workflow
4. OpenAI's GPT model analyzes the diff and generates a branch name following Conventional Branch conventions
5. The result (branch name + reasoning) is returned to the CLI
6. The CLI prompts for approval and can automatically rename your branch

## License

See [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

---

Built with ❤️ using [Bun](https://bun.sh), [Rust](https://rust-lang.org), [VoltAgent](https://voltagent.ai), and [Cloudflare Workers](https://workers.cloudflare.com)
