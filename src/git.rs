use std::process::Command;

pub fn has_changes() -> bool {
    let unstaged = Command::new("git")
        .args(["diff", "--quiet", "HEAD"])
        .status()
        .is_ok_and(|s| !s.success());

    let staged = Command::new("git")
        .args(["diff", "--cached", "--quiet"])
        .status()
        .is_ok_and(|s| !s.success());

    unstaged || staged
}

pub fn get_diff(max_lines: usize) -> Result<String, String> {
    // Try unstaged + staged first
    let output = Command::new("git")
        .args(["diff", "HEAD"])
        .output()
        .map_err(|e| format!("failed to run git diff: {e}"))?;

    let mut diff = String::from_utf8_lossy(&output.stdout).to_string();

    // Fall back to staged only
    if diff.trim().is_empty() {
        let output = Command::new("git")
            .args(["diff", "--cached"])
            .output()
            .map_err(|e| format!("failed to run git diff: {e}"))?;
        diff = String::from_utf8_lossy(&output.stdout).to_string();
    }

    if diff.trim().is_empty() {
        return Err("no changes found".to_string());
    }

    let lines: Vec<&str> = diff.lines().collect();
    let total = lines.len();

    if total > max_lines {
        eprintln!(
            "(diff truncated: {total} lines -> {max_lines}. adjust with: claude-git config max_lines <n>)"
        );
    }

    Ok(lines.into_iter().take(max_lines).collect::<Vec<_>>().join("\n"))
}

pub fn get_branch_diff(base: &str, max_lines: usize) -> Result<String, String> {
    let log_output = Command::new("git")
        .args(["log", &format!("{base}..HEAD"), "--oneline"])
        .output()
        .map_err(|e| format!("failed to run git log: {e}"))?;

    let commits = String::from_utf8_lossy(&log_output.stdout).to_string();
    if commits.trim().is_empty() {
        return Err(format!("no commits ahead of {base}"));
    }

    let diff_output = Command::new("git")
        .args(["diff", &format!("{base}...HEAD")])
        .output()
        .map_err(|e| format!("failed to run git diff: {e}"))?;

    let diff = String::from_utf8_lossy(&diff_output.stdout).to_string();
    let diff_truncated: String = diff.lines().take(max_lines).collect::<Vec<_>>().join("\n");

    Ok(format!("=== Commits ===\n{commits}\n=== Diff ===\n{diff_truncated}"))
}

pub fn commit(message: &str) -> Result<(), String> {
    let add = Command::new("git")
        .args(["add", "-A"])
        .status()
        .map_err(|e| format!("git add failed: {e}"))?;

    if !add.success() {
        return Err("git add failed".to_string());
    }

    let commit = Command::new("git")
        .args(["commit", "-m", message])
        .status()
        .map_err(|e| format!("git commit failed: {e}"))?;

    if !commit.success() {
        return Err("git commit failed".to_string());
    }

    Ok(())
}
