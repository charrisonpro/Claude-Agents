# Toolkit

Reference documentation for available tools. Read this file when you need tool details.

---

## File Tools

### read_file
Read a file from `Agent Files/`.

**Parameters:**
- `filename` (required): Relative path to the file

---

### write_file
Write content to `Agent Files/`. Creates or overwrites.

**Parameters:**
- `filename` (required): Relative path
- `content` (required): Content to write

---

### list_files
List available files in `Agent Files/`.

**Parameters:**
- `include_archive` (optional): Set `true` to include `Arch/` contents

---

### save_history
Save content to `History/` for session persistence.

**Parameters:**
- `filename` (required): Filename
- `content` (required): Content to save

---

### write_output
Write deliverables to `Output/`.

**Parameters:**
- `filename` (required): Filename
- `content` (required): Content to write
