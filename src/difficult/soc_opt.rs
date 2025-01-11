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

pub fn print_table(data: &str) {
    let mut table = Table::from_csv(data);

    if table.rows.is_empty() {
        return;
    }

    let density_values: Vec<i32> = table
        .column_by_heading("density")
        .unwrap()
        .map(|density| density.parse::<i32>().unwrap())
        .collect();
    let max_density: i32 = density_values.iter().max().unwrap().to_owned();

    table.headings.push("rel density");
    for (i, row) in table.rows.iter_mut().enumerate() {
        let relative_percentage = (density_values[i] * 100) / max_density;
        row.values.push(relative_percentage.to_string());
    }

    table.sort_by_heading::<u32>("rel density", false);

    println!("{}", table);
}

#[derive(Debug, Clone)]
pub struct Row {
    values: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Table<'a> {
    headings: Vec<&'a str>,
    rows: Vec<Row>,
}

impl Display for Table<'_> {
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

impl Table<'_> {
    pub fn from_csv(data: &str) -> Table {
        if data.is_empty() {
            return Table::default();
        }

        let mut rows: Vec<Row> = vec![];
        let mut data_lines = data.trim().lines();

        let headings: Vec<&str> = data_lines.next().unwrap().split(',').collect();
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

    pub fn column_by_heading(&self, heading: &str) -> Option<impl Iterator<Item = &str>> {
        let column_index = self.headings.iter().position(|h| *h == heading)?;
        self.column_by_index(column_index)
    }

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
        let data = "city,population,area,density,country
          Shanghai,24256800,6340,3826,China
          Delhi,16787941,1484,11313,India
          Lagos,16060303,1171,13712,Nigeria
          Istanbul,14160467,5461,2593,Turkey
          Tokyo,13513734,2191,6168,Japan
          Sao Paulo,12038175,1521,7914,Brazil
          Mexico City,8874724,1486,5974,Mexico
          London,8673713,1572,5431,United Kingdom
          New York City,8537673,784,10892,United States
          Bangkok,8280925,1569,5279,Thailand";
        print_table(data);
    }
}
