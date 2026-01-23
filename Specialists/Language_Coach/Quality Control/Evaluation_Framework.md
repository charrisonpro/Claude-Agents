---
# MACHINE-OWNED: Claudio reads/writes this block
agent: Japanese_Coach_Kyoto
version: v1.0
last_updated: 2025-01-19

hypotheses:
  - id: H1
    condition: "learner level correctly assessed"
    outcome: "learner progresses appropriately"
    statement: "P(learner progresses | level correctly assessed) > 0.8"
    status: untested
    observation_count: 0
    consistent_count: 0
    inconsistent_count: 0
    observations: []

  - id: H2
    condition: "dialect feature introduced"
    outcome: "recognition prioritized over production"
    statement: "P(recognition prioritized | dialect feature introduced) > 0.9"
    status: untested
    observation_count: 0
    consistent_count: 0
    inconsistent_count: 0
    observations: []

  - id: H3
    condition: "dialect feature introduced"
    outcome: "cultural context included"
    statement: "P(cultural context included | dialect feature introduced) > 0.85"
    status: untested
    observation_count: 0
    consistent_count: 0
    inconsistent_count: 0
    observations: []

  - id: H4
    condition: "trust behaviors present"
    outcome: "learner continues engagement"
    statement: "P(learner continues engagement | trust behaviors present) > 0.8"
    status: untested
    observation_count: 0
    consistent_count: 0
    inconsistent_count: 0
    observations: []

observations: []
---

# Evaluation Framework — Japanese Coach (Kyoto-ben)

## Hypotheses

### H1: Level Calibration → Progress

**Statement:** P(learner progresses | level correctly assessed) > 0.8

**Rationale:** Correct level assessment determines appropriate instruction depth. Kyoto-ben layered on insufficient standard Japanese foundation will fail.

**Status:** Untested

### H2: Dialect Introduction → Recognition Priority

**Statement:** P(recognition prioritized | dialect feature introduced) > 0.9

**Rationale:** Most learners need to understand Kyoto-ben, not speak it. Production is higher bar—don't push prematurely.

**Status:** Untested

### H3: Dialect Features → Cultural Context

**Statement:** P(cultural context included | dialect feature introduced) > 0.85

**Rationale:** Kyoto-ben carries centuries of cultural significance. Teaching forms without history/connotation misses the point.

**Status:** Untested

### H4: Trust Behaviors → Engagement

**Statement:** P(learner continues engagement | trust behaviors present) > 0.8

**Rationale:** Warmth predicts retention. Coach Stem trust behaviors should produce continued learner engagement.

**Status:** Untested

---

## Evaluation Dimensions

| Dimension | Description | Weight |
|-----------|-------------|--------|
| Dialect Authenticity | Uses Kyoto features appropriately (はる, expressions) | High |
| Level Calibration | Adjusts to learner's Japanese proficiency | High |
| Recognition vs. Production | Prioritizes recognition for most learners | High |
| Cultural Integration | Weaves historical/cultural context naturally | Medium |
| Warmth & Connection | Supportive coach, not textbook | Medium |

---

## Test Cases

### Case 1: Beginner Curiosity

**Input:** "I'm going to Kyoto next month. What should I know about the dialect?"

**Expected Behavior:**
- High-level orientation (don't overwhelm with grammar)
- Mention おおきに as useful to recognize
- Explain they'll hear はる forms
- Reassure standard Japanese works fine
- Cultural appreciation framing

**Pass Criteria:**
- Does not dump conjugation tables on beginner
- Makes Kyoto-ben feel approachable, not intimidating
- Offers practical recognition tips

**Hypothesis Links:** H1 (level calibration), H2 (recognition priority), H4 (trust behaviors)

**Task Type:** moderate

---

### Case 2: Intermediate Grammar Question

**Input:** "I heard someone say 行かはる. What's that?"

**Expected Behavior:**
- Clear explanation of はる suffix
- Show pattern: verb stem + はる
- Note it's for others' actions (respectful)
- Give 2-3 more examples
- Compare to standard keigo (softer than formal)

**Pass Criteria:**
- Explanation is accurate
- Provides usable pattern
- Doesn't overcomplicate

**Hypothesis Links:** H3 (cultural context with dialect feature)

**Task Type:** moderate

---

### Case 3: Cultural Nuance

**Input:** "Is it true Kyoto people are passive-aggressive?"

**Expected Behavior:**
- Address thoughtfully without dismissing or confirming stereotype
- Explain indirect communication patterns (real feature)
- Give linguistic examples (考えときます, tea offers)
- Note individual variation exists
- Avoid essentializing

**Pass Criteria:**
- Handles sensitive topic with nuance
- Provides useful cultural insight
- Doesn't reinforce harmful stereotypes

**Hypothesis Links:** H3 (cultural context)

**Task Type:** complex

---

### Case 4: Production Request (Advanced)

**Input:** "Can you help me practice using はる forms?"

**Expected Behavior:**
- Confirm learner's level is appropriate
- Provide structured practice
- Give scenarios for appropriate use
- Correct errors gently
- Note when はる is/isn't appropriate

**Pass Criteria:**
- Appropriate level check
- Useful practice structure
- Clear correction

**Hypothesis Links:** H1 (level calibration), H2 (recognition vs production check)

**Task Type:** complex

---

### Case 5: Dialect Confusion

**Input:** "What's the difference between Kyoto-ben and Osaka-ben?"

**Expected Behavior:**
- Acknowledge they're related but distinct
- Key differences: はる usage, vocabulary, intonation
- Note common conflation in media
- Don't disparage either variety

**Pass Criteria:**
- Accurate comparison
- Respectful of both varieties
- Useful distinctions

**Hypothesis Links:** H3 (cultural context)

**Task Type:** moderate

---

## Radiation Log

Observations that should propagate to other components:

| Date | Observation | Target | Status |
|------|-------------|--------|--------|
| | | | |

---

## Feedback Log

| Date | Observer | Observation | Severity | Status |
|------|----------|-------------|----------|--------|
| | | | | |

---

## Notes

Space for PM synthesis, patterns observed, open questions.

---

## Version History

**v0.1** — Initial test cases.
**v0.2** — Upgraded to YAML format with hypotheses, observation tracking, hypothesis links on test cases.
