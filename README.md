# ProcDNA

**ProcDNA** is an experimental, local-first Linux process intelligence project.

It turns low-level Linux kernel events into explainable **process passports**, **behavior chains**, **timelines**, and **evidence graphs**.

> Every Linux process has a PID. ProcDNA gives it a story.

---

## 1. Project Status

ProcDNA is currently in an **experimental MVP phase**.

The active and working prototype is located under:

```text
experiments/procdna-execve-lab/
```

The root-level structure already contains the intended future production layout:

```text
crates/
├── procdna-agent
├── procdna-brain
├── procdna-cli
└── procdna-common
```

However, the current fully working implementation, including the eBPF collector, CLI, SQLite store, reports, systemd install script, and MkDocs documentation portal, lives inside:

```text
experiments/procdna-execve-lab/
```

This is intentional.

The project is being developed in stages. The experimental lab is where the design is tested and hardened before it is moved into the cleaner production crate layout.

---

## 2. What ProcDNA Does

ProcDNA observes Linux process behavior using eBPF and builds a higher-level evidence model.

Currently it collects:

```text
execve   -> what process executed?
connect  -> where did the process connect?
openat   -> which files did the process access?
```

From these events, ProcDNA builds:

```text
Process Passport
Behavior Chain
Incident Report
Timeline
Evidence Graph
AI-assisted Explanation
```

The current CLI supports:

```bash
procdna ps
procdna explain <pid>
procdna timeline <pid>
procdna graph <pid>
procdna graph <pid> --format mermaid

procdna db stats
procdna db prune --retention-config --dry-run
procdna db prune --retention-config --yes
procdna db reset --dry-run
procdna db vacuum

procdna health
```

---

## 3. What ProcDNA Is Not Yet

ProcDNA is not yet a full EDR.

It currently does **not**:

- block processes
- kill suspicious processes
- quarantine files
- enforce firewall policy
- manage multiple hosts centrally
- provide a full dashboard
- replace a SIEM
- replace a commercial EDR

The current goal is visibility, explanation, and evidence modeling.

ProcDNA is best described as:

```text
Local Linux Process Intelligence Agent
+
Endpoint Evidence Store
+
Forensic CLI
+
AI-assisted Explanation Layer
```

---

## 4. Current Repository Structure

Current repository layout:

```text
.
├── Cargo.lock
├── Cargo.toml
├── config
│   ├── agent.yaml
│   └── brain.yaml
├── crates
│   ├── procdna-agent
│   ├── procdna-brain
│   ├── procdna-cli
│   └── procdna-common
├── deploy
│   ├── ansible
│   └── systemd
├── docs
├── examples
├── experiments
│   └── procdna-execve-lab
├── README.md
└── target
```

### Important Note

At the moment, use this path for the working prototype:

```bash
cd experiments/procdna-execve-lab
```

The root-level `crates/` structure represents the future intended architecture.

---

## 5. Current Working Prototype Structure

Inside the active lab:

```text
experiments/procdna-execve-lab/
├── config/
│   ├── procdna.yaml
│   └── procdna.systemd.yaml
├── data/
│   └── procdna.db
├── deploy/
│   ├── install.sh
│   ├── uninstall.sh
│   └── systemd/
├── docs/
├── procdna-execve-lab/
│   └── src/
├── procdna-execve-lab-common/
├── procdna-execve-lab-ebpf/
├── reports/
├── requirements-docs.txt
├── Cargo.toml
└── README.md
```

The active binaries are:

```text
procdna-agent
procdna
```

Their release paths are:

```text
experiments/procdna-execve-lab/target/release/procdna-agent
experiments/procdna-execve-lab/target/release/procdna
```

---

## 6. Current Architecture

```text
Single Linux Host
=================

Linux Kernel
  ├── tracepoint/sys_enter_execve
  ├── tracepoint/sys_enter_connect
  └── tracepoint/sys_enter_openat
          │
          ▼
procdna-agent
  ├── loads eBPF programs
  ├── reads kernel events
  ├── builds process passports
  ├── enriches with /proc
  ├── applies deterministic risk rules
  ├── correlates behavior chains
  ├── optionally calls local Ollama
  ├── writes SQLite evidence
  └── writes HTML/JSON reports
          │
          ▼
SQLite Local Store
  ├── process_passports
  ├── network_connections
  ├── file_accesses
  ├── behavior_chains
  └── incident_reports
          │
          ▼
procdna CLI
  ├── ps
  ├── explain
  ├── timeline
  ├── graph
  ├── db
  └── health
```

---

## 7. Future Architecture

The long-term direction is:

```text
Endpoint Agents
  ├── eBPF collection
  ├── local SQLite buffer
  ├── local risk scoring
  └── secure forwarding

ProcDNA Brain
  ├── ingest API
  ├── multi-host correlation
  ├── central storage
  ├── LLM explanation service
  ├── dashboard API
  └── SIEM exporter

ProcDNA CLI
  ├── local mode
  └── central mode

ProcDNA UI
  ├── host inventory
  ├── process explorer
  ├── timeline viewer
  ├── graph viewer
  └── incident reports
```

The future root crate layout is intended to support this:

```text
crates/
├── procdna-agent   -> endpoint collector
├── procdna-cli     -> investigation CLI
├── procdna-brain   -> future central service
└── procdna-common  -> shared event/data types
```

---

## 8. Requirements

This project is designed for Linux.

Tested development environment:

```text
Linux Mint / Ubuntu-like distribution
Rust stable + nightly
Aya eBPF stack
Ollama optional
SQLite
systemd
```

Install system packages:

```bash
sudo apt update

sudo apt install -y \
  build-essential clang llvm lld pkg-config libssl-dev libelf-dev zlib1g-dev \
  linux-headers-$(uname -r) linux-tools-common linux-tools-generic bpftool \
  git curl wget make cmake jq tree ca-certificates sqlite3 python3-venv
```

Install Rust:

```bash
curl --proto '=https' -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

rustup install stable
rustup toolchain install nightly --component rust-src
rustup default stable
```

Install eBPF linker:

```bash
cargo install bpf-linker
```

Optional: Install Ollama for local AI explanations.

```bash
curl -fsSL https://ollama.com/install.sh | sh
ollama pull qwen3.5:4b
```

---

## 9. Build the Current Prototype

Go to the active experimental lab:

```bash
cd ~/Dev-Env/procdna/experiments/procdna-execve-lab
```

Format and build:

```bash
cargo fmt
cargo build --release --bin procdna-agent --bin procdna
```

Expected binaries:

```text
target/release/procdna-agent
target/release/procdna
```

---

## 10. Development Mode

Development mode uses:

```text
config/procdna.yaml
data/procdna.db
reports/
```

Start the agent manually:

```bash
sudo env RUST_LOG=info ./target/release/procdna-agent run --config config/procdna.yaml
```

In another terminal, generate some test activity:

```bash
bash -c 'echo dev-test > /tmp/procdna-dev-test.txt ; curl -4 -I https://example.com >/dev/null 2>&1 ; cat /etc/passwd >/dev/null'
```

Query local evidence:

```bash
./target/release/procdna --config config/procdna.yaml ps
./target/release/procdna --config config/procdna.yaml db stats
```

Explain a process:

```bash
./target/release/procdna --config config/procdna.yaml explain <pid>
```

Show timeline:

```bash
./target/release/procdna --config config/procdna.yaml timeline <pid>
```

Show graph:

```bash
./target/release/procdna --config config/procdna.yaml graph <pid>
./target/release/procdna --config config/procdna.yaml graph <pid> --format mermaid
```

---

## 11. Production-like systemd Install

Build first:

```bash
cd ~/Dev-Env/procdna/experiments/procdna-execve-lab

cargo fmt
cargo build --release --bin procdna-agent --bin procdna
```

Install:

```bash
sudo ./deploy/install.sh
```

This installs:

```text
/usr/local/bin/procdna-agent
/usr/local/bin/procdna

/etc/procdna/procdna.yaml

/var/lib/procdna/procdna.db
/var/lib/procdna/reports

/etc/systemd/system/procdna-agent.service
```

Enable and start service:

```bash
sudo systemctl enable --now procdna-agent
```

Check status:

```bash
sudo systemctl status procdna-agent --no-pager
```

Follow logs:

```bash
sudo journalctl -u procdna-agent -f
```

Important:

```text
journalctl does not start the agent.
It only follows logs.
```

To start the agent:

```bash
sudo systemctl start procdna-agent
```

---

## 12. Production CLI Usage

Production mode uses:

```text
/etc/procdna/procdna.yaml
/var/lib/procdna/procdna.db
/var/lib/procdna/reports
```

Check health:

```bash
sudo procdna health
```

Check DB:

```bash
sudo procdna db stats
```

List process passports:

```bash
sudo procdna ps
```

Generate test activity:

```bash
bash -c 'echo prod-test > /tmp/procdna-prod-test.txt ; curl -4 -I https://example.com >/dev/null 2>&1 ; cat /etc/passwd >/dev/null'
```

Then:

```bash
sudo procdna ps
```

---

## 13. Non-root CLI Access

The systemd layout uses the `procdna` group.

To use the CLI without `sudo`:

```bash
sudo usermod -aG procdna "$USER"
newgrp procdna
```

Check:

```bash
id
```

The `procdna` group should appear.

Then:

```bash
procdna db stats
procdna ps
procdna health
```

Expected production DB path:

```text
/var/lib/procdna/procdna.db
```

If normal user shows:

```text
data/procdna.db
```

then the CLI is probably not reading the production config due to permissions or config fallback behavior.

---

## 14. Development vs Production Paths

Development:

```text
config/procdna.yaml
data/procdna.db
reports/
```

Production:

```text
/etc/procdna/procdna.yaml
/var/lib/procdna/procdna.db
/var/lib/procdna/reports
```

Development commands should explicitly pass config:

```bash
./target/release/procdna --config config/procdna.yaml ps
```

Production commands can use default config:

```bash
procdna ps
```

---

## 15. DB Maintenance

Show stats:

```bash
procdna db stats
```

Dry-run retention cleanup:

```bash
procdna db prune --retention-config --dry-run
```

Delete old telemetry:

```bash
sudo systemctl stop procdna-agent
sudo procdna db prune --retention-config --yes
sudo procdna db vacuum
sudo systemctl start procdna-agent
```

Reset all telemetry:

```bash
sudo systemctl stop procdna-agent
sudo procdna db reset --dry-run
sudo procdna db reset --yes
sudo procdna db vacuum
sudo systemctl start procdna-agent
```

SQLite note:

```text
DELETE removes rows logically.
VACUUM reclaims physical disk space.
```

---

## 16. Documentation Portal

ProcDNA also includes a MkDocs Material documentation portal.

From the active lab:

```bash
cd ~/Dev-Env/procdna/experiments/procdna-execve-lab

python3 -m venv .venv-docs
source .venv-docs/bin/activate

pip install --upgrade pip
pip install -r requirements-docs.txt

mkdocs serve
```

Open:

```text
http://127.0.0.1:8000
```

Build static docs:

```bash
mkdocs build
```

Output:

```text
site/
```

---

## 17. Key Documentation Files

```text
docs/
├── ARCHITECTURE.md
├── ROADMAP.md
├── FUTURE_HARDENING.md
├── DATA_MODEL.md
├── OPERATIONS.md
├── THREAT_MODEL.md
├── DEVELOPMENT.md
├── components/
├── runbooks/
├── design/
└── ADR/
```

Important ADRs:

```text
ADR-0001: Local-first agent architecture
ADR-0002: SQLite as endpoint-local store
ADR-0003: Agent and CLI split
ADR-0004: AI explanation boundaries
ADR-0005: systemd production layout
```

---

## 18. Troubleshooting

### Service not found

Wrong:

```bash
systemctl status procdna
```

Correct:

```bash
systemctl status procdna-agent
```

The service name is:

```text
procdna-agent.service
```

### `journalctl -f` seems required

It is not required.

`journalctl` only follows logs.

Start the service:

```bash
sudo systemctl enable --now procdna-agent
```

### Normal user sees wrong DB

If this:

```bash
procdna db stats
```

shows:

```text
data/procdna.db
```

but this:

```bash
sudo procdna db stats
```

shows:

```text
/var/lib/procdna/procdna.db
```

then fix group access:

```bash
sudo usermod -aG procdna "$USER"
newgrp procdna
```

Check permissions:

```bash
ls -ld /etc/procdna
ls -lah /etc/procdna/procdna.yaml
ls -ld /var/lib/procdna
ls -lah /var/lib/procdna
```

### DB readonly error

Development DB may be root-owned if the agent was run with sudo.

Fix:

```bash
sudo chown -R "$USER:$USER" data
```

Production DB maintenance should usually be done with sudo while the agent is stopped:

```bash
sudo systemctl stop procdna-agent
sudo procdna db vacuum
sudo systemctl start procdna-agent
```

### Ollama unavailable

Check:

```bash
systemctl status ollama --no-pager
curl http://127.0.0.1:11434/api/tags
```

Temporarily disable AI:

```yaml
llm:
  enabled: false
```

Then:

```bash
sudo systemctl restart procdna-agent
```

---

## 19. Current Security Design Principles

ProcDNA follows these principles:

```text
AI explains evidence.
AI does not decide risk.

Risk score is deterministic.
Rules should be explainable.

Agent and CLI are separate.
Agent is privileged.
CLI should be safe by default.

SQLite is local evidence storage.
Reports are separate evidence artifacts.

Destructive DB operations require --yes.
Dry-run should be available where possible.
```

---

## 20. Future Hardening Plan

Important next technical milestones:

```text
Stage 54: Documentation portal
Stage 55: Schema migration framework
Stage 56: Process identity v2
Stage 57: Report relationship hardening
Stage 58: Reports management commands
Stage 59: Automatic retention
Stage 60: Doctor command
Stage 61: SIEM export foundation
Stage 62: Central brain prototype
```

Most important future technical change:

```text
Move from PID-based lookup to process_instance_id.
```

Target identity:

```text
process_instance_id = host_id + boot_id + pid + process_start_time
```

This is required because Linux can reuse PIDs.

---

## 21. Git Workflow

Before committing:

```bash
cargo fmt
cargo build --release --bin procdna-agent --bin procdna
```

Check status:

```bash
git status --short
```

Commit example:

```bash
git add .
git commit -m "add ProcDNA documentation portal"
```

Push:

```bash
git push origin main
```

---

## 22. Suggested `.gitignore`

Make sure local runtime/build artifacts are not pushed:

```gitignore
# ProcDNA local runtime data
data/
reports/
*.db
*.db-wal
*.db-shm

# Build outputs
target/
site/

# Local virtualenvs
.venv/
.venv-docs/

# Editor/system noise
.DS_Store
.idea/
.vscode/
```

---

## 23. License

The experimental Aya template may include Apache, MIT and GPL-related license files.

Before public release, review the final license strategy for:

```text
eBPF code
userspace Rust code
Aya template-derived files
documentation
```

---

## 24. Short Project Description

ProcDNA is a local-first Linux process intelligence prototype. It uses eBPF to observe process execution, network connections, and file access events, then transforms them into process passports, behavior chains, timelines, evidence graphs, and AI-assisted explanations.

The current implementation is experimental and lives under:

```text
experiments/procdna-execve-lab
```

The future production architecture will move this logic into the root-level crate structure:

```text
crates/procdna-agent
crates/procdna-cli
crates/procdna-brain
crates/procdna-common
```
