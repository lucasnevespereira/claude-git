use std::process::Command;

pub fn call(model: &str, input: &str, prompt: &str) -> Result<String, String> {
    let output = Command::new("claude")
        .args([
            "-p",
            "--model",
            model,
            "--output-format",
            "text",
            "--no-session-persistence",
            "--effort",
            "low",
            prompt,
        ])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;
            if let Some(ref mut stdin) = child.stdin {
                stdin.write_all(input.as_bytes()).ok();
            }
            child.wait_with_output()
        })
        .map_err(|e| format!("failed to run claude CLI: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("claude CLI error: {stderr}"));
    }

    let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if text.is_empty() {
        return Err("empty response from claude CLI".to_string());
    }

    Ok(text)
}
