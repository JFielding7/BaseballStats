use regex::{Regex};
use term_table::row::{Row};
use term_table::{row, Table, TableStyle};
use term_table::table_cell::{TableCell};
use crate::query::{empty, get_query_param, QueryError};

macro_rules! batting_url {
    () => {
        ("https://www.baseball-reference.com/leagues/majors/bat.shtml",
        "Batting Stat Averages Per Team Per Game",
        row!("Year", "G", "PA", "AB", "R", "H", "1B", "2B", "3B", "HR", "RBI", "SB", "CS", "BB",
        "SO", "BA", "OBP", "SLG", "OPS", "TB", "GDP", "HBP", "SH", "SF", "IBB", "BIP"))
    };
}

macro_rules! pitching_url {
    () => {
        ("https://www.baseball-reference.com/leagues/majors/pitch.shtml",
        "Pitching Stat Averages Per Team Per Game",
        row!("Year", "ERA", "G", "GF", "CG", "SHO", "tSHO", "SV", "IP", "H", "R", "ER", "HR",
        "BB", "IBB", "SO", "HBP", "BK", "WP", "BF", "WHIP", "BAbip", "H9", "HR9", "BB9", "SO9", "SO/W", "E"))
    };
}

pub(crate) fn display_league_averages(query: &Vec<String>, is_batting: bool) -> Result<(), QueryError> {
    const ALL_TIME_INDEX: usize = 2;
    const COL_OFFSET: usize = 4;

    let (url, header, stat_header) = if is_batting { batting_url!() }
    else { pitching_url!() };

    let row_count = if get_query_param!(query, ALL_TIME_INDEX, empty!()) == "a" { -1 } else { 1 };

    let stats = reqwest::blocking::get(url)?.text()?;

    let table_regex = Regex::new(r"<tbody>[\S\s]*?League Year-By-Year").unwrap();
    let row_regex = Regex::new(r"<tr >(<th.*?</th>).*?</tr>").unwrap();
    let col_regex = Regex::new(r">([\d.]*)</(td|a)>").unwrap();

    let mut table = Table::new();
    table.style = TableStyle::thin();
    table.add_row(stat_header.clone());
    let mut rows = 0;

    for row in row_regex.captures_iter(table_regex.find(stats.as_str()).unwrap().as_str()) {
        let mut cols = vec![];
        let mut i = 0;

        for col in col_regex.captures_iter(&row[0]) {
            if i == 0 || i > COL_OFFSET {
                cols.push(col[1].to_string());
            }
            i += 1;
        }

        table.add_row(Row::new(cols));
        rows += 1;
        if rows == row_count { break };
        if rows & 7 == 0 { table.add_row(stat_header.clone()) }
    }

    println!("\n{}\n{}", header, table.render()
        .replace("│ ", "│")
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
    Ok(())
}