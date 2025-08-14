use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;

fn latex_escape(s: &str) -> String {
    s.replace("{", "\\{")
        .replace("}", "\\}")
        .replace("&", "\\&")
        .replace("%", "\\%")
        .replace("_", "\\_")
        .replace("$", "\\$")
        .replace("#", "\\#")
        .replace("^", "\\^{}")
        .replace("~", "\\~{}")
}

fn guess_bibtex_type(ris_ty: &str) -> &str {
    match ris_ty.to_uppercase().as_str() {
        "JOUR" => "article",
        "BOOK" => "book",
        "CHAP" => "incollection",
        "CONF" | "CPAPER" => "inproceedings",
        "THES" => "phdthesis",
        "RPRT" => "techreport",
        _ => "misc",
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file.ris>", args[0]);
        process::exit(1);
    }

    let filepath = &args[1];
    let ris = fs::read_to_string(filepath).unwrap_or_else(|_| {
        eprintln!("Error: Could not read file {}", filepath);
        process::exit(1);
    });

    let entries: Vec<&str> = ris.split("\nER  -").collect();

    for entry in entries {
        if entry.trim().is_empty() {
            continue;
        }

        let mut fields: HashMap<&str, Vec<String>> = HashMap::new();

        for line in entry.lines() {
            if let Some((tag, value)) = line.split_once("  - ") {
                fields
                    .entry(tag.trim())
                    .or_default()
                    .push(value.trim().to_string());
            }
        }

        let default_ty = "MISC".to_string();
        let bib_type = guess_bibtex_type(
            &fields
                .get("TY")
                .and_then(|v| v.first())
                .unwrap_or(&default_ty),
        );

        let first_author = fields
            .get("AU")
            .and_then(|v| v.first())
            .map(|name| name.split(',').next().unwrap_or("").to_string())
            .unwrap_or_else(|| "unknown".into());

        let year = fields
            .get("PY")
            .and_then(|v| v.first())
            .cloned()
            .unwrap_or_else(|| "????".into());

        let cite_key = format!("{}{}", first_author, year);

        let authors = fields
            .get("AU")
            .map(|names| names.join(" and "))
            .unwrap_or_default();

        println!(
            "@{bib_type}{{{cite_key},
                author       = {{{authors}}},
                title        = {{{title}}},
                journal      = {{{journal}}},
                booktitle    = {{{booktitle}}},
                volume       = {{{volume}}},
                number       = {{{number}}},
                pages        = {{{pages}}},
                year         = {{{year}}},
                doi          = {{{doi}}},
                url          = {{{url}}},
                issn         = {{{issn}}},
                abstract     = {{{abstract_text}}}
                }}",
            bib_type = bib_type,
            cite_key = cite_key,
            authors = latex_escape(&authors),
            title = latex_escape(&fields.get("TI").map(|v| v[0].clone()).unwrap_or_default()),
            journal = latex_escape(&fields.get("JO").map(|v| v[0].clone()).unwrap_or_default()),
            booktitle = latex_escape(&fields.get("T2").map(|v| v[0].clone()).unwrap_or_default()),
            volume = latex_escape(&fields.get("VL").map(|v| v[0].clone()).unwrap_or_default()),
            number = latex_escape(&fields.get("IS").map(|v| v[0].clone()).unwrap_or_default()),
            pages = latex_escape(&fields.get("SP").map(|v| v[0].clone()).unwrap_or_default()),
            year = year,
            doi = latex_escape(&fields.get("DO").map(|v| v[0].clone()).unwrap_or_default()),
            url = latex_escape(&fields.get("UR").map(|v| v[0].clone()).unwrap_or_default()),
            issn = latex_escape(&fields.get("SN").map(|v| v[0].clone()).unwrap_or_default()),
            abstract_text =
                latex_escape(&fields.get("AB").map(|v| v[0].clone()).unwrap_or_default()),
        );
    }
}
