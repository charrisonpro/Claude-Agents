# PM Helper

Async project manager agent for the Claude PE Framework ecosystem.

## Purpose

Processes queued administrative tasks during downtime to save tokens on primary agents (Opus/Sonnet).

## Model

**Haiku** — optimized for speed over nuance, constrained tasks.

## Usage

1. Other agents queue tasks to `Claude PE Framework/Agent Files/PM_Queue.md`
2. Run PM Helper when not doing critical work
3. PM processes queue, outputs status report, marks items complete

## Task Types

- `TRACK_QUESTION` — Log open questions
- `TRACK_ACTION` — Log action items with owner
- `LOG_DECISION` — Record decisions
- `CHECK_CONVENTIONS` — Review artifacts for formatting
- `DRAFT_VERSION_ENTRY` — Write changelog entries
- `FLAG_HEAVY_OP` — Flag expensive ops with manual alternatives
- `STATUS_REQUEST` — Generate full status report

## Output

Status reports written to `Output/PM_Status_[Date].md`
