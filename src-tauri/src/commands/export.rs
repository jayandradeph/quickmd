use crate::utils::error::AppError;
use std::fs;
use std::path::PathBuf;

/// Generate and save a self-contained HTML file from rendered markdown
#[tauri::command]
pub async fn export_html(
    path: String,
    content: String,
    title: String,
) -> Result<(), String> {
    let html = wrap_html(&content, &title);
    fs::write(&path, html).map_err(|e| AppError::ExportError(e.to_string()).to_string())?;
    Ok(())
}

/// Get a temporary directory path for export operations
#[tauri::command]
pub async fn get_temp_path() -> Result<String, String> {
    let temp_dir = std::env::temp_dir().join("quickmd");
    fs::create_dir_all(&temp_dir)
        .map_err(|e| AppError::ExportError(e.to_string()).to_string())?;
    Ok(temp_dir.to_string_lossy().to_string())
}

fn wrap_html(content: &str, title: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>{title}</title>
<style>
  body {{
    max-width: 900px;
    margin: 0 auto;
    padding: 2em;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    line-height: 1.6;
    color: #24292e;
  }}
  pre {{
    background: #f6f8fa;
    padding: 16px;
    border-radius: 6px;
    overflow-x: auto;
  }}
  code {{
    font-family: "JetBrains Mono", "Fira Code", "Consolas", monospace;
    font-size: 0.9em;
  }}
  table {{
    border-collapse: collapse;
    width: 100%;
  }}
  th, td {{
    border: 1px solid #dfe2e5;
    padding: 8px 12px;
    text-align: left;
  }}
  th {{
    background: #f6f8fa;
  }}
  blockquote {{
    border-left: 4px solid #dfe2e5;
    margin: 0;
    padding: 0 1em;
    color: #6a737d;
  }}
  img {{
    max-width: 100%;
  }}
  @media (prefers-color-scheme: dark) {{
    body {{
      background: #0d1117;
      color: #c9d1d9;
    }}
    pre, th {{
      background: #161b22;
    }}
    th, td {{
      border-color: #30363d;
    }}
    blockquote {{
      border-left-color: #30363d;
      color: #8b949e;
    }}
    a {{
      color: #58a6ff;
    }}
  }}
</style>
</head>
<body>
{content}
</body>
</html>"#,
        title = title,
        content = content
    )
}
