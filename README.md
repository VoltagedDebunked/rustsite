# Rustsite

`rustsite` is a customizable static site generator written in Rust. It converts Markdown files into HTML using a template engine, making it easy to generate static websites with dynamic content.

## Features

- Convert Markdown files to HTML
- Use customizable templates for HTML output
- Parse and include front matter metadata in your pages
- Simple command-line interface for generating static sites

## Installation

You can use `rustsite` directly from crates.io. To use it as a binary, install it using Cargo:
```bash
cargo install rustsite
```
## Usage

After installing `rustsite`, you can use it from the command line:

rustsite <source-dir> <destination-dir> <template-file>

- `<source-dir>`: The directory containing your Markdown files.
- `<destination-dir>`: The directory where the generated HTML files will be saved.
- `<template-file>`: The path to your HTML template file.

### Example

Assume you have the following structure:
```
project/
│
├── content/
│   ├── index.md
│   └── about.md
│
├── template.html
└── output/
```
Your `index.md` might look like this:
```
---markdown
title: "Home"
---

# Welcome to My Site

This is the home page.
```
Your `template.html` might look like this:
```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{{ title }}</title>
</head>
<body>
    {{ content }}
</body>
</html>
```
Run the following command to generate your site:

rustsite content output template.html

This will convert `index.md` and any other Markdown files in `content/` into HTML files in the `output/` directory using the `template.html` template.

## Functions

### `generate_site(source_dir: &str, destination_dir: &str, template_file: &str) -> io::Result<()>`

Generates the static site from Markdown files in `source_dir`, outputs HTML files to `destination_dir`, and uses `template_file` for the HTML template.

### `parse_front_matter(content: &str) -> (HashMap<String, String>, String)`

Parses the front matter metadata from the given Markdown content and separates it from the body.

### `markdown_to_html(markdown: &str) -> String`

Converts Markdown content to HTML.

### `render_template(template: &str, metadata: &HashMap<String, String>, content: &str) -> String`

Renders HTML content using the given template and metadata.