// Tasks for rewriting:
//   - Apply optimizations of computing resources: processor, memory
//   - Minimize cognitive complexity
//   - Respect SRP and SoC
//   - Improve readability (understanding), reliability
//   - Optimize for maintainability, reusability, flexibility
//   - Make code testable
//   - Implement simple unittests without frameworks
//   - Try to implement in multiple paradigms: OOP, FP, procedural, mixed

use std::fmt::Display;

pub fn print_table(data: &str) {
    let mut table = Table::from_csv(data);

    if table.rows.is_empty() {
        return;
    }

    let density_values: Vec<i32> = table
        .rows
        .iter()
        .map(|row| row.values[3].parse::<i32>().unwrap())
        .collect();

    let max_density: i32 = density_values.iter().max().unwrap().to_owned();

    for (i, row) in table.rows.iter_mut().enumerate() {
        let relative_percentage = (density_values[i] * 100) / max_density;
        row.values.push(relative_percentage.to_string());
    }

    table
        .rows
        .sort_by_key(|row| row.values[5].parse::<i32>().unwrap());
    table.rows.reverse();

    println!("{}", table);
}

#[derive(Debug, Clone)]
pub struct Row {
    values: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Table {
    rows: Vec<Row>,
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const MIN_OFFSET: usize = 2;

        if self.rows.is_empty() {
            return Ok(());
        }

        let max_column_lengths: Vec<usize> = (0..self.rows[0].values.len())
            .map(|col| {
                self.rows
                    .iter()
                    .map(|row| row.values[col].len())
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
    pub fn from_csv(data: &str) -> Table {
        if data.is_empty() {
            return Table { rows: vec![] };
        }

        let mut rows: Vec<Row> = vec![];
        let mut data_lines = data.trim().lines();

        let row_length = data_lines.next().unwrap().split(',').count();
        for line in data_lines {
            let row_values: Vec<String> = line
                .trim()
                .splitn(row_length, ',')
                .map(|s| s.to_string())
                .collect();
            rows.push(Row { values: row_values });
        }

        Table { rows }
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
