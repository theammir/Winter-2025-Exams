//! A `String`-based `Table` struct.
//!
//! Some obvious improvements to existing code would be:
//!     - representing rows as generic `R: Row` objects (to avoid string parsing and using literals
//!     as column names, not sure how I would approach the latter, though.)
//!     - instead of halting `Table::sort_by_heading` if unable to parse a value, treating it as
//!     less,
//!
//! but I consider myself more or less done with this snippet.

use std::{fmt::Display, str::FromStr};

/// Pretend this is business logic
pub fn print_table() {
    let mut table = Table::from_csv(
        "city,population,area,density,country
          Shanghai,24256800,6340,3826,China
          Delhi,16787941,1484,11313,India
          Lagos,16060303,1171,13712,Nigeria
          Istanbul,14160467,5461,2593,Turkey
          Tokyo,13513734,2191,6168,Japan
          Sao Paulo,12038175,1521,7914,Brazil
          Mexico City,8874724,1486,5974,Mexico
          London,8673713,1572,5431,United Kingdom
          New York City,8537673,784,10892,United States
          Bangkok,8280925,1569,5279,Thailand",
    );

    let density_values: Vec<u32> = table
        .column_by_heading("density")
        .unwrap()
        .map(|density| density.parse::<u32>().unwrap())
        .collect();
    let max_density: u32 = density_values.iter().max().unwrap().to_owned();

    let relative_densities: Vec<String> = density_values
        .into_iter()
        .map(|density| (density * 100) / max_density)
        .map(|percentage| percentage.to_string())
        .collect();
    table.push_column("rel density".to_string(), relative_densities);

    table.sort_by_heading::<u32>("rel density", true).unwrap();

    println!("{}", table);
}

/// Represents a single table row of `String` values.
#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    values: Vec<String>,
}

/// Represents a table with as many items in a row as there are columns at most.
/// Each column is represented by a `String` heading and its index in the vector.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Table {
    headings: Vec<String>,
    rows: Vec<Row>,
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const MIN_OFFSET: usize = 2;

        if self.rows.is_empty() {
            return Ok(());
        }

        let max_column_lengths: Vec<usize> = (0..self.headings.len())
            .map(|column| {
                self.column_by_index(column)
                    .unwrap()
                    .map(|s| s.len())
                    .max()
                    .unwrap()
            })
            .collect();

        for row in self.rows.iter() {
            for (i, cell) in row.values.iter().enumerate() {
                let offset = max_column_lengths[i] + MIN_OFFSET;

                let formatted_cell = if i == 0 {
                    format!("{:<width$}", cell, width = offset)
                } else {
                    format!("{:>width$}", cell, width = offset)
                };

                write!(f, "{}", formatted_cell)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum SortingError {
    ColumnNotFound,
    ParsingFailed,
}

impl Table {
    /// Creates a `Table` from a valid CSV string with headings declared in the first line.
    ///
    /// Skipping values inside rows is allowed.
    pub fn from_csv(data: &str) -> Table {
        if data.is_empty() {
            return Table::default();
        }

        let mut rows: Vec<Row> = vec![];
        let mut data_lines = data.trim().lines();

        let headings: Vec<String> = data_lines
            .next()
            .unwrap()
            .trim()
            .split(',')
            .map(str::to_string)
            .collect();
        let row_length = headings.len();

        for line in data_lines {
            let row_values: Vec<String> = line
                .trim()
                .splitn(row_length, ',')
                .map(|s| s.to_string())
                .collect();
            rows.push(Row { values: row_values });
        }

        Table { headings, rows }
    }

    /// Adds a new `Row` to the bottom of the table, consuming the vector.
    ///
    /// When the amount of cells is insufficient, either fills in empty `String`s or shrinks the
    /// vector accordingly.
    /// Supplying an empty vector can be useful for visual separation of data. Note that it might
    /// not be a good idea to sort such a table.
    pub fn push_vec_row(&mut self, mut row: Vec<String>) {
        let expected_len = self.headings.len();
        if row.len() != expected_len {
            row.resize(expected_len, String::new());
        }

        self.rows.push(Row { values: row });
    }

    /// Adds a new column to the right of the table, consuming the vector.
    ///
    /// When the amount of cells is less than there are rows, fills in empty `String`s.
    /// Supplying excessive cells has no effect.
    pub fn push_column(&mut self, heading: String, mut column: Vec<String>) {
        if column.len() < self.rows.len() {
            column.resize(self.rows.len(), String::new());
        }

        self.headings.push(heading);
        for (i, row) in self.rows.iter_mut().enumerate() {
            row.values.push(column[i].clone());
        }
    }

    /// Returns an iterator over a column of cells by its index.
    ///
    /// The index is naturally `0`-based.
    ///
    /// Returns `None` if the index is out of bounds.
    pub fn column_by_index(&self, index: usize) -> Option<impl Iterator<Item = &str>> {
        if index >= self.headings.len() {
            return None;
        }

        Some(self.rows.iter().map(move |row| row.values[index].as_str()))
    }

    /// Returns an iterator over a column of cells by its heading.
    ///
    /// If `Table`'s `headers` vector contains multiple identical entries,
    /// returns the first occurence.
    ///
    /// Returns `None` if none were found.
    pub fn column_by_heading(&self, heading: &str) -> Option<impl Iterator<Item = &str>> {
        let column_index = self.headings.iter().position(|h| h == heading)?;
        self.column_by_index(column_index)
    }

    /// Sorts the table by its column that corresponds with the heading, treating the cells as
    /// values of type `T`.
    pub fn sort_by_heading<T>(&mut self, heading: &str, reverse: bool) -> Result<(), SortingError>
    where
        T: Ord + FromStr,
    {
        let column_index = self
            .headings
            .iter()
            .position(|h| h == heading)
            .ok_or(SortingError::ColumnNotFound)?;

        let mut indexed_column = Vec::with_capacity(self.rows.len());
        for (i, row) in self.rows.iter().enumerate() {
            let value = row.values[column_index]
                .parse::<T>()
                .map_err(|_| SortingError::ParsingFailed)?;
            indexed_column.push((i, value));
        }

        indexed_column.sort_by(|(_, a), (_, b)| if reverse { b.cmp(a) } else { a.cmp(b) });

        self.rows = indexed_column
            .into_iter()
            .map(|(i, _)| self.rows[i].clone())
            .collect();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_from_csv() {
        let table = Table::from_csv("a,b,");
        assert_eq!(table.headings[2], "");

        let table = Table::from_csv("");
        assert_eq!(table, Table::default());

        let table = Table::from_csv("1,2\n3,4,\n5,\n,6\n,\n");
        assert!(table.rows.iter().all(|row| row.values.len() == 2));
        assert_eq!(table.rows.len(), 4);
        assert_eq!(&table.rows[1].values, &["5", ""]);
        assert_eq!(&table.rows[2].values, &["", "6"]);
        assert_eq!(&table.rows[3].values, &["", ""]);
    }

    #[test]
    fn test_table_display() {
        let table = Table::from_csv("a,b,c\n100,200,300\n400,,500");
        assert_eq!(table.to_string(), "100    200  300\n400         500\n");

        let table = Table::default();
        assert_eq!(table.to_string(), "");
    }

    #[test]
    fn test_column_getters() {
        let table = Table::from_csv("a,b,c\n1,2,3\n4,5,6\n7,8,9");
        assert_eq!(
            table.column_by_index(0).unwrap().collect::<Vec<&str>>(),
            vec!["1", "4", "7"]
        );
        assert_eq!(
            table.column_by_heading("c").unwrap().collect::<Vec<&str>>(),
            vec!["3", "6", "9"]
        );

        assert!(table.column_by_index(3).is_none());
        assert!(table.column_by_heading("d").is_none());
    }

    #[test]
    fn test_table_mutations() {
        let mut table = Table::default();
        table.push_vec_row(vec!["foo".to_string()]);
        assert!(table.headings.is_empty());
        assert!(table.rows[0].values.is_empty());

        table.push_column("bar".to_string(), vec!["baz".to_string()]);
        table.push_column("empty".to_string(), vec![]);
        table.push_column(
            "toomuch".to_string(),
            vec!["A".to_string(), "B".to_string()],
        );

        assert_eq!(
            table.rows[0].values,
            vec!["baz".to_string(), "".to_string(), "A".to_string()]
        );
    }

    #[test]
    fn test_table_sorting() {
        let mut table = Table::from_csv("alphabet\nd\ne\na\nd\nb\ne\ne\nf");
        table.sort_by_heading::<String>("alphabet", false).unwrap();
        assert_eq!(
            table.column_by_index(0).unwrap().collect::<Vec<&str>>(),
            vec!["a", "b", "d", "d", "e", "e", "e", "f"]
        );

        let mut table = Table::from_csv("id,balance\n1,100\n2,324\n5,1234\n10,46");
        table.sort_by_heading::<u32>("balance", true).unwrap();
        assert_eq!(
            table.column_by_index(0).unwrap().collect::<Vec<&str>>(),
            vec!["5", "2", "1", "10"]
        );
    }

    #[test]
    fn run_soc_opt() {
        print_table();
    }
}
