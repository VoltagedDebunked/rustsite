use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub fn generate_site(source_dir: &str, destination_dir: &str, template_file: &str) -> io::Result<()> {
    let template = fs::read_to_string(template_file)?;

    for entry in fs::read_dir(source_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |e| e == "md") {
            let content = fs::read_to_string(&path)?;
            let (metadata, body) = parse_front_matter(&content);
            let html_content = markdown_to_html(&body);
            let html = render_template(&template, &metadata, &html_content);

            let relative_path = match path.strip_prefix(source_dir) {
                Ok(relative) => relative.to_str().unwrap_or("unknown"),
                Err(_) => {
                    eprintln!("Failed to strip prefix from path: {:?}", path);
                    continue;
                }
            };

            let destination_path = Path::new(destination_dir)
                .join(relative_path)
                .with_extension("html");

            if let Some(parent) = destination_path.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut file = fs::File::create(destination_path)?;
            file.write_all(html.as_bytes())?;
        }
    }

    Ok(())
}

fn parse_front_matter(content: &str) -> (HashMap<String, String>, String) {
    let mut metadata = HashMap::new();
    let mut body = String::new();
    let mut in_front_matter = false;

    for line in content.lines() {
        if line.trim() == "---" {
            in_front_matter = !in_front_matter;
            continue;
        }

        if in_front_matter {
            if let Some((key, value)) = line.split_once(':') {
                metadata.insert(key.trim().to_string(), value.trim().to_string());
            }
        } else {
            body.push_str(line);
            body.push('\n');
        }
    }

    (metadata, body)
}

fn markdown_to_html(markdown: &str) -> String {
    let mut html = String::new();

    for line in markdown.lines() {
        if line.starts_with("# ") {
            html.push_str(&format!("<h1>{}</h1>\n", &line[2..]));
        } else if line.starts_with("## ") {
            html.push_str(&format!("<h2>{}</h2>\n", &line[3..]));
        } else if line.starts_with("### ") {
            html.push_str(&format!("<h3>{}</h3>\n", &line[4..]));
        } else if line.starts_with("* ") {
            html.push_str(&format!("<li>{}</li>\n", &line[2..]));
        } else if line.trim().is_empty() {
            html.push_str("<br/>\n");
        } else {
            html.push_str(&format!("<p>{}</p>\n", line));
        }
    }

    html
}

fn render_template(template: &str, metadata: &HashMap<String, String>, content: &str) -> String {
    let mut html = template.to_string();

    for (key, value) in metadata {
        html = html.replace(&format!("{{{{ {} }}}}", key), value);
    }

    html = html.replace("{{ content }}", content);

    html
}