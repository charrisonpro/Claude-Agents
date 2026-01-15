# Language Coach — Version History

**System:** Kyoto Dialect Language Coach  
**Target Model:** Claude Sonnet 4.5  
**Domain:** Conversational agent (educational)

---

## Version History

**v1.1** — 2026-01-13 — Consolidated Learner Profile

- Created dedicated Learner Profile section as single source of truth for mutable state
- Removed hardcoded level references ("intermediate beginner") from body text
- Extended progression path to include N2+ phase for long-term use
- Changed final instruction from "begin with introductions in Kyoto dialect" to level-appropriate start
- Opening line now references Learner Profile rather than stating level directly

**Lesson learned:** Mutable state (user level, preferences) should be consolidated in one updateable section. Other sections reference implicitly — avoids maintenance burden and inconsistency as user advances. (Flagged for Lesson_Notes.md)

---

**v1.0** — 2026-01-13 — Initial release

- Established learner profile (N5-N4 baseline, hiragana fluent, some katakana)
- Defined four learning goals (dialect fluency, kanji expansion, cultural nuance, politeness levels)
- Created two-part lesson structure to prevent response overload
- Added long-horizon difficulty progression (N5 → N4 → N3 preparation) with natural transition signals
- Added session management for save/resume capability across fresh starts
- Specified parenthetical furigana format for cross-platform compatibility
- Set encouraging, patient tone

**Source edits from review:**
- Removed meta-commentary from original prompt (broke frame)
- Added explicit difficulty calibration logic (prevents drift)
- Split lesson into two parts (pacing for Sonnet, prevents walls of text)
- Added session save/resume mechanism (enables stateless operation)

---

## Pending Considerations

- **Furigana rendering:** Currently using parenthetical format 漢字(かんじ). If migrating to environment with ruby text support, consider switching to proper `<ruby>` tags.
- **API migration:** Will need lesson state artifact or external storage injection when moving off Claude.ai projects.
- **Testing:** No failure modes observed yet. Monitor for: level progression pacing, response length on Sonnet, session summary quality.
- **Learner Profile updates:** User should update the Learner Profile section as they advance through JLPT levels.

---

## File Manifest

| File | Purpose |
|------|---------|
| Language_Coach_v1.1.md | Main prompt (operational) |
| Language_Coach_Version_History.md | This file (tracking) |
