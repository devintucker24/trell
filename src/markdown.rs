// Reading beliefs out of a markdown brain.
//
// The dominant shape for agent knowledge bases in the wild is a git repo of
// markdown pages: Karpathy's LLM Wiki, GBrain, Obsidian vaults. Palimpsest
// meets that shape rather than replacing it. A page keeps its prose; fenced
// `pal` blocks inside it carry the part that has to resolve deterministically.
//
// Because the page is the document a claim came from, the page is also its
// provenance. Facts written inside `notes/handbook.md` are sourced to
// `notes/handbook.md` unless they say otherwise, so `forget everything from
// notes/handbook.md` means exactly what it looks like.

/// A markdown page reduced to the Palimpsest it contains.
#[derive(Debug, Clone, PartialEq)]
pub struct Page {
    /// Provenance for every fact on the page.
    pub source: String,
    /// Authority for facts that do not name one, from `authority:` frontmatter.
    pub authority: Option<String>,
    /// Extracted code, blank-padded so line numbers match the original file.
    pub code: String,
    pub blocks: usize,
}

impl Page {
    pub fn is_empty(&self) -> bool {
        self.blocks == 0
    }
}

/// True when a fence info string opens a Palimpsest block.
fn is_pal_fence(info: &str) -> bool {
    let lang = info
        .trim()
        .split(|c: char| c.is_whitespace() || c == ',' || c == '{')
        .next()
        .unwrap_or("")
        .trim_matches('`')
        .to_ascii_lowercase();
    matches!(lang.as_str(), "pal" | "palimpsest")
}

fn fence_marker(line: &str) -> Option<(&str, usize)> {
    let trimmed = line.trim_start();
    for marker in ["```", "~~~"] {
        if let Some(rest) = trimmed.strip_prefix(marker) {
            return Some((rest, line.len() - trimmed.len()));
        }
    }
    None
}

/// Pulls `key: value` pairs out of a leading `---` frontmatter block.
fn read_frontmatter(lines: &[&str]) -> (Option<String>, Option<String>, usize) {
    if lines.first().map(|l| l.trim()) != Some("---") {
        return (None, None, 0);
    }

    let mut source = None;
    let mut authority = None;

    for (i, line) in lines.iter().enumerate().skip(1) {
        if line.trim() == "---" {
            return (source, authority, i + 1);
        }
        if let Some((key, value)) = line.split_once(':') {
            let value = value.trim().trim_matches('"').trim_matches('\'').to_string();
            if value.is_empty() {
                continue;
            }
            match key.trim().to_ascii_lowercase().as_str() {
                "source" => source = Some(value),
                "authority" => authority = Some(value),
                _ => {}
            }
        }
    }

    (source, authority, 0)
}

/// Extracts every Palimpsest block from a markdown document.
///
/// `path` is used as the default provenance, so it should be the page's
/// stable identity within the brain (a repo-relative path reads best).
pub fn extract(path: &str, content: &str) -> Page {
    let lines: Vec<&str> = content.lines().collect();
    let (fm_source, fm_authority, body_start) = read_frontmatter(&lines);

    // A blank line per skipped source line keeps parse diagnostics pointing at
    // the right line of the original markdown.
    let mut code: Vec<String> = Vec::with_capacity(lines.len());
    let mut blocks = 0;
    let mut inside = false;
    let mut indent = 0usize;

    for (i, raw) in lines.iter().enumerate() {
        if i < body_start {
            code.push(String::new());
            continue;
        }

        if let Some((info, marker_indent)) = fence_marker(raw) {
            if inside {
                inside = false;
                code.push(String::new());
                continue;
            }
            if is_pal_fence(info) {
                inside = true;
                indent = marker_indent;
                blocks += 1;
                code.push(String::new());
                continue;
            }
            // A fence for some other language: skip until it closes.
            code.push(String::new());
            continue;
        }

        if inside {
            // Strip only the fence's own indentation so blocks nested in list
            // items keep their internal structure.
            let stripped = if indent > 0 && raw.len() >= indent && raw[..indent].trim().is_empty() {
                &raw[indent..]
            } else {
                raw
            };
            code.push(stripped.to_string());
        } else {
            code.push(String::new());
        }
    }

    Page {
        source: fm_source.unwrap_or_else(|| path.to_string()),
        authority: fm_authority,
        code: code.join("\n"),
        blocks,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_fenced_blocks_and_keeps_line_numbers() {
        let md = "# Title\n\nSome prose.\n\n```pal\nalice.city is \"Berlin\"\n```\n\nmore prose\n";
        let page = extract("wiki/alice.md", md);

        assert_eq!(page.blocks, 1);
        assert_eq!(page.source, "wiki/alice.md");
        let code_lines: Vec<&str> = page.code.lines().collect();
        assert_eq!(code_lines[5], "alice.city is \"Berlin\"");
        assert!(code_lines[0].is_empty());
    }

    #[test]
    fn frontmatter_overrides_source_and_sets_authority() {
        let md = "---\nsource: hr_handbook_2026\nauthority: policy\n---\n\n```pal\npto is 20\n```\n";
        let page = extract("wiki/hr.md", md);

        assert_eq!(page.source, "hr_handbook_2026");
        assert_eq!(page.authority.as_deref(), Some("policy"));
    }

    #[test]
    fn ignores_other_languages() {
        let md = "```python\nprint('hi')\n```\n\n```palimpsest\nx is 1\n```\n";
        let page = extract("p.md", md);

        assert_eq!(page.blocks, 1);
        assert!(!page.code.contains("print"));
        assert!(page.code.contains("x is 1"));
    }
}
