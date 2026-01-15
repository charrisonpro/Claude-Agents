# Bug Report - Claude PE Framework Infrastructure Issues
*Documented: [Current Date]*
*Reported by: Prompt Engineer during Coach Personality Stem project*

## Issue 1: Directory Discovery Limitation

### Problem
The `list_agents` function fails to detect sibling directories that exist in the Claude-Agents folder.

### Specific Example
- **Expected**: Language Coaches folder at `Claude-Agents/"Language Coaches"` should be visible
- **Actual**: Only detects 2 projects (Claude PE Framework, Coach Personality Stem)
- **Location**: Parallel to Coach Personality Stem directory

### Likely Cause
`list_agents` only finds directories containing `Agent Files/Instructions.md` - may need to handle:
- Different internal structures
- Folders with quotes in names
- Non-standard agent project layouts

### Suggested Fix
Either:
1. Make `list_agents` more flexible in detection criteria
2. Add a general directory browsing function
3. Document what structures are required for detection

---

## Issue 2: GUI Response Truncation

### Problem
Agent responses are getting cut off at the bottom of the GUI interface, making it difficult to read complete responses.

### Symptoms
- User message: "your responses are getting trapped at the bottom of" [message itself cut off]
- Responses appear truncated in the interface
- User needs to request white space padding as workaround

### Current Workaround
Agent adds multiple blank lines at the end of responses to push content into visible area

### Suggested Fix
GUI should either:
1. Auto-scroll to show complete responses
2. Adjust viewport to accommodate full response length
3. Add proper padding/margin at bottom of response area
4. Ensure response container expands to fit content

### Test Case
Have agent write a response of 20+ lines and verify it's fully visible without manual whitespace padding

---

## Additional Context
Both issues discovered during Coach Personality Stem prompt development session. Issues are infrastructure-related, not prompt engineering problems.