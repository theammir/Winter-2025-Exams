// Tasks for rewriting:
//   - Apply optimizations of computing resources: processor, memory
//   - Minimize cognitive complexity
//   - Respect SRP and SoC
//   - Improve readability (understanding), reliability
//   - Optimize for maintainability, reusability, flexibility
//   - Make code testable
//   - Implement simple unittests without frameworks
//   - Try to implement in multiple paradigms: OOP, FP, procedural, mixed

use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

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
        .iter()
        .map(|density| (*density * 100) / max_density)
        .map(|percentage| percentage.to_string())
        .collect();
    table.push_column("rel density".to_string(), relative_densities);

    table.sort_by_heading::<u32>("rel density", false);

    println!("{}", table);
}

/// Represents a single table row of `String` values.
#[derive(Debug, Clone)]
pub struct Row {
    values: Vec<String>,
}

/// Represents a table with as many items in a row as there are columns at most.
/// Each column is represented by a `String` heading and its index in the vector.
#[derive(Debug, Clone, Default)]
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
            .split(',')
            .map(|s| s.to_string())
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

    /// Adds a new `Row` to the bottom of the table.
    ///
    /// When the amount of cells is insufficient, either fills in empty `String`s or shrinks the
    /// vector accordingly.
    /// Supplying an empty vector can be useful for visual separation of data. Note that it might
    /// not be a good idea to sort such a table.
    pub fn push_vec_row(&mut self, row: Vec<String>) {
        let expected_len = self.headings.len();
        if row.len() < expected_len {
            let mut row = row.clone();
            row.resize(expected_len, String::new());
        }
        if row.len() > expected_len {
            let mut row = row.clone();
            row.truncate(expected_len);
        }

        self.rows.push(Row { values: row });
    }

    /// Adds a new column to the right of the table.
    ///
    /// When the amount of cells is less than there are rows, fills in empty `String`s.
    /// Supplying excessive cells has no effect.
    pub fn push_column(&mut self, heading: String, column: Vec<String>) {
        if column.len() < self.rows.len() {
            let mut column = column.clone();
            column.resize(self.headings.len(), String::new());
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

        Some(
            self.rows
                .iter()
                .map(move |row| row.values.get(index).map(|s| s.as_str()).unwrap()),
        )
    }

    /// Returns an iterator over a column of cells by its heading.
    ///
    /// If `Table`'s `headers` vector contains multiple identical entries,
    /// returns the first occurence.
    ///
    /// Returns `None` if none were found.
    pub fn column_by_heading(&self, heading: &str) -> Option<impl Iterator<Item = &str>> {
        let column_index = self.headings.iter().position(|h| *h == heading)?;
        self.column_by_index(column_index)
    }

    /// Sorts the table by its column that corresponds with the heading, treating the cells as
    /// values of type `T`.
    ///
    /// Note that the order is already reversed by default, so that numerical values go biggest to
    /// smallest. This makes less sense for regular strings. I should probably not be doing
    /// this...
    ///
    /// # Panics
    ///
    /// Parsing the values into `T` is unchecked, and as such can cause the program to panic.
    /// The caller is responsible for providing valid values (for now).
    pub fn sort_by_heading<T>(&mut self, heading: &str, reverse: bool) -> Option<()>
    where
        T: Ord + FromStr,
        T::Err: Debug,
    {
        let column_index = self.headings.iter().position(|h| *h == heading)?;
        self.rows
            .sort_by_key(|row| row.values[column_index].parse::<T>().unwrap());

        if !reverse {
            self.rows.reverse();
        }
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::print_table;

    #[test]
    fn run_soc_opt() {
        print_table();
    }
}
