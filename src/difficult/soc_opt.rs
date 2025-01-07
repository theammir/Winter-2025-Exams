// Tasks for rewriting:
//   - Apply optimizations of computing resources: processor, memory
//   - Minimize cognitive complexity
//   - Respect SRP and SoC
//   - Improve readability (understanding), reliability
//   - Optimize for maintainability, reusability, flexibility
//   - Make code testable
//   - Implement simple unittests without frameworks
//   - Try to implement in multiple paradigms: OOP, FP, procedural, mixed

fn print_table(data: &str) {
    if data.is_empty() {
        return;
    }

    let csv_lines: Vec<&str> = data.split('\n').collect();
    let mut table: Vec<Vec<String>> = Vec::new();
    let mut max_density = 0;

    for line in &csv_lines[1..] {
        let row: Vec<&str> = line.trim().split(',').collect();
        let density = row[3].trim().parse::<i32>().unwrap();
        if density > max_density {
            max_density = density;
        }
        table.push(vec![
            row[0].to_string(),
            row[1].to_string(),
            row[2].to_string(),
            row[3].to_string(),
            row[4].to_string(),
        ]);
    }

    for row in table.iter_mut() {
        let density = row[3].parse::<f64>().unwrap();
        let relative_percentage = ((density * 100.0) / (max_density as f64)).round() as i32;
        row.push(relative_percentage.to_string());
    }

    table.sort_by(|r1, r2| {
        r2[5]
            .parse::<i32>()
            .unwrap()
            .cmp(&r1[5].parse::<i32>().unwrap())
    });

    for row in table.iter() {
        let mut line = String::new();
        line.push_str(&format!("{:18}", row[0]));
        line.push_str(&format!("{:>10}", row[1]));
        line.push_str(&format!("{:>8}", row[2]));
        line.push_str(&format!("{:>8}", row[3]));
        line.push_str(&format!("{:>18}", row[4]));
        line.push_str(&format!("{:>6}", row[5]));
        println!("{}", line);
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
