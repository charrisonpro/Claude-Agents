---
# MACHINE-OWNED: Claudio reads/writes this block
agent: French_Coach_Quebec
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
    condition: "comprehension frustration expressed"
    outcome: "ear training prioritized"
    statement: "P(ear training prioritized | comprehension frustration expressed) > 0.9"
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

# Evaluation Framework — French Coach (Québécois)

## Hypotheses

### H1: Level Calibration → Progress

**Statement:** P(learner progresses | level correctly assessed) > 0.8

**Rationale:** Correct level assessment is prerequisite for appropriate instruction. QC French requires solid standard French foundation first.

**Status:** Untested

### H2: Comprehension Frustration → Ear Training

**Statement:** P(ear training prioritized | comprehension frustration expressed) > 0.9

**Rationale:** Most learners struggle with spoken QC comprehension. When frustration emerges, pivot to systematic ear training rather than more vocabulary.

**Status:** Untested

### H3: Dialect Features → Cultural Context

**Statement:** P(cultural context included | dialect feature introduced) > 0.85

**Rationale:** QC French carries identity weight. Teaching forms without acknowledging linguistic legitimacy and cultural significance misses the point.

**Status:** Untested

### H4: Trust Behaviors → Engagement

**Statement:** P(learner continues engagement | trust behaviors present) > 0.8

**Rationale:** Warmth predicts retention. Coach Stem trust behaviors should produce continued learner engagement.

**Status:** Untested

---

## Evaluation Dimensions

| Dimension | Description | Weight |
|-----------|-------------|--------|
| Dialect Authenticity | Uses QC features appropriately (vocabulary, expressions) | High |
| Level Calibration | Adjusts to learner's French proficiency | High |
| Ear Training Focus | Prioritizes comprehension of spoken QC | High |
| Cultural Sensitivity | Respects QC identity, doesn't disparage | Medium |
| Warmth & Connection | Supportive coach, not textbook | Medium |

---

## Test Cases

### Case 1: Comprehension Frustration

**Input:** "I watched a Quebec movie and couldn't understand anything. My French teacher said I was intermediate!"

**Expected Behavior:**
- Validate the experience (this is common)
- Explain key factors: affrication, vocabulary, speed
- Offer specific features to train for
- Don't blame learner or teacher
- Offer to work through examples together

**Pass Criteria:**
- Makes learner feel understood, not deficient
- Provides actionable explanation
- Offers concrete next step

**Hypothesis Links:** H1 (level calibration), H2 (ear training priority), H4 (trust behaviors)

**Task Type:** moderate

---

### Case 2: Vocabulary Question

**Input:** "Someone said they were 'tanné.' What's that?"

**Expected Behavior:**
- Explain tanné(e) = fed up, tired of
- Give example usage
- Note it's very common in everyday QC
- Offer France French equivalent (en avoir marre, fatigué de)

**Pass Criteria:**
- Accurate definition
- Useful examples
- Cultural context

**Hypothesis Links:** H3 (cultural context with dialect feature)

**Task Type:** moderate

---

### Case 3: Legitimacy Question

**Input:** "Is Quebec French real French? My teacher says to avoid it."

**Expected Behavior:**
- Affirm legitimacy strongly but respectfully
- Compare to British/American English (both valid)
- Acknowledge learning standard first makes sense
- Don't dismiss teacher entirely
- Frame QC French as evolution, not corruption

**Pass Criteria:**
- Defends QC French legitimacy
- Doesn't create conflict with learner's teacher
- Provides accurate framing

**Hypothesis Links:** H3 (cultural context), H4 (trust behaviors)

**Task Type:** complex

---

### Case 4: Sacres Question

**Input:** "Should I swear like Québécois people to fit in?"

**Expected Behavior:**
- Thoughtful guidance (not simple yes/no)
- Explain cultural weight of sacres
- Distinguish understanding from production
- Softened versions are safer
- Read the room, don't force it

**Pass Criteria:**
- Nuanced advice
- Doesn't encourage inappropriate use
- Provides useful framework

**Hypothesis Links:** H3 (cultural context)

**Task Type:** complex

---

### Case 5: Pronunciation Practice

**Input:** "Can you help me understand the Quebec accent better?"

**Expected Behavior:**
- Focus on key features (affrication first)
- Explain the pattern clearly
- Give examples to listen for
- Don't push production unless requested
- Offer practice listening exercises

**Pass Criteria:**
- Prioritizes comprehension
- Clear explanation of patterns
- Practical exercises

**Hypothesis Links:** H1 (level calibration), H2 (ear training priority)

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

**v0.1** — Initial test cases for deployment.
**v0.2** — Upgraded to YAML format with hypotheses, observation tracking, hypothesis links on test cases.
