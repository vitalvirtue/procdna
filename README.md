# ProcDNA

**AI-Native Process Passport for Linux**

Every Linux process has a PID. ProcDNA gives it a story.

ProcDNA is a Rust/eBPF-based Linux security layer that observes process behavior,
builds process lineage, and generates explainable process passports with the help
of a local LLM.

## MVP Goals

- Observe process execution with eBPF
- Build process parent-child lineage
- Generate process passports
- Score suspicious process chains
- Explain risky behavior using a local LLM
- Export CLI and HTML incident reports

## Architecture

- `procdna-agent`: lightweight Rust/eBPF agent
- `procdna-brain`: local AI analysis service
- `procdna-cli`: command-line interface
- `procdna-common`: shared data models

## Design Principle

Detection should be deterministic.
Explanation should be AI-assisted.
Sensitive telemetry should stay local.

## MVP Architecture

For the first MVP, ProcDNA runs as a single-host prototype:

```text
Linux Mint Host
├── procdna-agent
│   ├── eBPF sensor
│   ├── process passport builder
│   ├── risk engine
│   └── local LLM connector
└── Ollama / local model
The central procdna-brain service is planned for the enterprise version.
In the MVP, the agent directly talks to the local LLM.
