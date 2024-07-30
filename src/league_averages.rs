use std::ptr::replace;
use regex::Regex;
use term_table::row::Row;
use term_table::{Table, TableStyle};
use crate::query::QueryError;

pub(crate) fn display_league_batting_averages() -> Result<(), QueryError> {
    let hitting_data = reqwest::blocking::get("https://www.baseball-reference.com/leagues/majors/bat.shtml")?.text()?;
    let row_regex = Regex::new(r"<tr >(<th.*?</th>).*?</tr>").unwrap();
    let col_regex = Regex::new(r">([\d.]+)<").unwrap();

    let mut matches = 0;
    for cap in row_regex.captures_iter(hitting_data.as_str()) {
        let row = &cap[0];
        let mut i = 0;

        let mut table = Table::new();
        table.style = TableStyle::thin();

        let mut cols = vec![];
        for col in col_regex.captures_iter(row) {
            cols.push(col[1].to_string());
            // println!("Res {i}: {}", &col[1]);
            i += 1;
        }
        table.add_row(Row::new(cols.clone()));
        table.add_row(Row::new(cols));
        println!("{}", table.render().replace("│ ", "│")
            .replace(" │", "│")
            .replace("─┴─", "┴")
            .replace("└─", "└")
            .replace("─┘", "┘")
            .replace("─┬─", "┬")
            .replace("┌─", "┌")
            .replace("─┐", "┐")
            .replace("┼─", "┼")
            .replace("─┼", "┼")
            .replace("├─", "├")
            .replace("─┤", "┤")
        );
        break;
    }
    println!("{matches}");
    Ok(())
}