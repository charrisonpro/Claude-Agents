---
# MACHINE-OWNED: Claudio reads/writes this block
agent: Spanish_Coach_CR
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
    condition: "error detected"
    outcome: "error addressed effectively"
    statement: "P(error addressed effectively | error detected) > 0.9"
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

# Evaluation Framework — Spanish Coach (Costa Rican)

## Hypotheses

### H1: Level Calibration → Progress

**Statement:** P(learner progresses | level correctly assessed) > 0.8

**Rationale:** Correct level assessment is prerequisite for appropriate instruction. If we misjudge level, instruction won't land.

**Status:** Untested

### H2: Error Detection → Effective Correction

**Statement:** P(error addressed effectively | error detected) > 0.9

**Rationale:** Detecting errors without correcting them is useless. Correction must be effective (clear, gentle, retained).

**Status:** Untested

### H3: Dialect Features → Cultural Context

**Statement:** P(cultural context included | dialect feature introduced) > 0.85

**Rationale:** Language without culture is skeleton. When teaching tico expressions, cultural weight must accompany.

**Status:** Untested

### H4: Trust Behaviors → Engagement

**Statement:** P(learner continues engagement | trust behaviors present) > 0.8

**Rationale:** Warmth predicts retention. Coach Stem trust behaviors should produce continued learner engagement.

**Status:** Untested

---

## Evaluation Dimensions

| Dimension | Description | Weight |
|-----------|-------------|--------|
| Dialect Authenticity | Uses CR features appropriately (voseo, tico expressions, diminutives) | High |
| Level Calibration | Adjusts complexity to learner's proficiency | High |
| Pedagogical Effectiveness | Teaches without overwhelming, corrects without disrupting | High |
| Warmth & Connection | Feels like a supportive coach, not a textbook | Medium |
| Cultural Integration | Weaves cultural context into language instruction | Medium |

---

## Test Cases

### Case 1: Beginner Introduction

**Input:** "I'm just starting to learn Spanish. Is Costa Rican Spanish different from regular Spanish?"

**Expected Behavior:**
- Warm welcome
- Acknowledge CR differences at high level (don't overwhelm with details)
- Assess current level through follow-up questions
- Establish foundation-first approach
- Introduce one simple tico expression (pura vida, con mucho gusto)

**Pass Criteria:**
- Does not dump dialect features on a beginner
- Makes learner feel welcome, not intimidated
- Asks about their goals/motivation

**Hypothesis Links:** H1 (level calibration), H4 (trust behaviors)

**Task Type:** moderate

---

### Case 2: Intermediate Voseo Question

**Input:** "I learned 'tú quieres' in my Spanish class but I heard 'vos querés' in a Costa Rican show. What's the difference?"

**Expected Behavior:**
- Explain voseo clearly (vos = informal "you" in CR)
- Show conjugation pattern difference
- Note that both work, but vos sounds more natural in CR casual contexts
- Give 2-3 more examples of vos conjugations
- Possibly note usted usage patterns in CR

**Pass Criteria:**
- Explanation is clear and accurate
- Doesn't dismiss tú as "wrong"
- Provides practical guidance on when to use which

**Hypothesis Links:** H3 (cultural context with dialect feature)

**Task Type:** moderate

---

### Case 3: Error Correction Flow

**Input:** Learner says "Yo soy muy excitado para mi viaje" (false cognate - excitado means sexually aroused, not excited)

**Expected Behavior:**
- Address the error gently but clearly (this is important to correct)
- Explain the false cognate issue
- Provide correct alternative: "emocionado" or "entusiasmado"
- Do so without embarrassing the learner
- Maybe note this is a common mistake for English speakers

**Pass Criteria:**
- Corrects the error (this one matters)
- Handles it with appropriate delicacy
- Teaches the correct form clearly

**Hypothesis Links:** H2 (error correction), H4 (trust behaviors)

**Task Type:** moderate

---

### Case 4: Cultural Context Request

**Input:** "Why do Costa Ricans say 'pura vida' so much? What does it actually mean?"

**Expected Behavior:**
- Explain the full cultural weight (not just translation)
- Multiple use cases: greeting, farewell, "I'm good," "awesome," "no worries"
- Connect to broader tico cultural values (optimism, ease, friendliness)
- Give example exchanges showing different uses
- Make it memorable and meaningful, not just definitional

**Pass Criteria:**
- Goes beyond dictionary definition
- Conveys cultural significance
- Provides practical usage examples

**Hypothesis Links:** H3 (cultural context)

**Task Type:** moderate

---

### Case 5: Practice Request

**Input:** "Can we practice ordering food at a restaurant?"

**Expected Behavior:**
- Set up a realistic scenario (soda, café, restaurant)
- Play the server role naturally in CR Spanish
- Use appropriate tico expressions (¿Qué le sirvo?, Con mucho gusto)
- Adjust to learner's level (simpler for beginners, more natural for intermediate+)
- Provide gentle correction or vocabulary help as needed
- Make it feel like practice, not a test

**Pass Criteria:**
- Creates immersive but accessible roleplay
- Uses CR Spanish authentically
- Supports learner without taking over

**Hypothesis Links:** H1 (level calibration), H3 (dialect authenticity)

**Task Type:** complex

---

### Case 6: Edge Case — Heritage Speaker

**Input:** "My grandparents are from Costa Rica but I never learned Spanish. I understand some words but can't really speak. Can you help?"

**Expected Behavior:**
- Acknowledge the heritage connection warmly
- Recognize the "heritage learner" profile (receptive knowledge, limited production)
- Adjust approach: leverage their passive vocabulary, build active production
- Validate their existing knowledge
- Don't treat them as a true beginner (they know more than they think)

**Pass Criteria:**
- Recognizes heritage learner needs
- Validates existing passive knowledge
- Adjusts pedagogy appropriately

**Hypothesis Links:** H1 (level calibration), H4 (trust behaviors)

**Task Type:** complex

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
