// Tasks for rewriting:
//   - Apply optimizations of computing resources: processor, memory
//   - Minimize cognitive complexity
//   - Respect SRP and SoC
//   - Improve readability (understanding), reliability
//   - Optimize for maintainability, reusability, flexibility
//   - Make code testable
//   - Implement simple unittests without frameworks
//   - Try to implement in multiple paradigms: OOP, FP, procedural, mixed

pub fn print_table(data: &str) {
    let mut table = Table::from_csv(data);

    let max_density = table.columns[3]
        .values
        .iter()
        .fold(0, |acc, i| acc.max(i.parse().unwrap()));

    let mut relative_column = Column {
        heading: "rel density".to_string(),
        values: vec![],
    };

    for density in table.columns[3].values.clone() {
        let density = density.parse::<f64>().unwrap();
        let relative_percentage = ((density * 100.0) / (max_density as f64)).round() as i32;
        relative_column.values.push(relative_percentage.to_string());
    }
    table.columns.push(relative_column);

    // TODO: Implement sorting.

    // TODO: Implement printing.
    for column in table.columns.iter() {
        println!("{:?}", column);
    }
}

#[derive(Debug, Clone)]
pub struct Column {
    heading: String,
    values: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Table {
    columns: Vec<Column>,
}

impl Table {
    pub fn from_csv(data: &str) -> Table {
        if data.is_empty() {
            return Table { columns: vec![] };
        }

        // PERF: There's probably something to be done about this.
        let mut columns: Vec<Column> = vec![];
        let cells: Vec<Vec<String>> = data
            .trim()
            .lines()
            .map(|line| line.trim().split(',').map(|s| s.to_string()))
            .map(|row| row.collect())
            .collect();

        (0..cells[0].len()).for_each(|i| {
            columns.push(Column {
                heading: cells[0][i].to_string(),
                values: cells[1..].iter().map(|row| row[i].clone()).collect(),
            });
        });

        Table { columns }
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
