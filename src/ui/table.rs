use crate::search::SearchResult;
use colored::Colorize;
use comfy_table::*;

pub struct TableFormatter;

impl TableFormatter {
    /// Display search results in a formatted table
    pub fn display_search_results(results: &[SearchResult]) {
        if results.is_empty() {
            println!("{}", "No results found.".yellow());
            return;
        }

        let mut table = Table::new();
        table
            .set_header(vec![
                Cell::new("Source")
                    .fg(Color::Cyan)
                    .add_attribute(Attribute::Bold),
                Cell::new("Title")
                    .fg(Color::Cyan)
                    .add_attribute(Attribute::Bold),
                Cell::new("Snippet")
                    .fg(Color::Cyan)
                    .add_attribute(Attribute::Bold),
            ])
            .set_content_arrangement(ContentArrangement::Dynamic);

        for result in results {
            let source = if result.is_trusted {
                "[T] Trusted".green().to_string()
            } else {
                "[ ] Other".dimmed().to_string()
            };

            let title = if result.title.len() > 50 {
                format!("{}...", &result.title[..47])
            } else {
                result.title.clone()
            };

            let snippet = if result.snippet.len() > 80 {
                format!("{}...", &result.snippet[..77])
            } else {
                result.snippet.clone()
            };

            table.add_row(vec![
                Cell::new(source),
                Cell::new(title),
                Cell::new(snippet),
            ]);
        }

        println!("\n{}", table);
    }

    /// Display a simple key-value table
    pub fn display_config(items: &[(&str, &str)]) {
        let mut table = Table::new();
        table.set_content_arrangement(ContentArrangement::Dynamic);

        for (key, value) in items {
            table.add_row(vec![Cell::new(*key).fg(Color::Cyan), Cell::new(*value)]);
        }

        println!("\n{}", table);
    }
}
