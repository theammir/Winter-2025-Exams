// Tasks for rewriting:
//   - Apply optimizations of computing resources: processor, memory
//   - Minimize cognitive complexity
//   - Respect SRP and SoC
//   - Improve readability (understanding), reliability
//   - Optimize for maintainability, reusability, flexibility
//   - Make code testable
//   - Implement simple unittests without frameworks
//   - Try to implement in multiple paradigms: OOP, FP, procedural, mixed

fn main() {
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

    if data.len() > 0 {
        let mut lines = data.split('\n').collect::<Vec<&str>>();
        lines.pop();
        let mut table: Vec<Vec<String>> = Vec::new();
        let mut first = true;
        let mut max = 0;

        for line in lines {
            if first {
                first = false;
            } else {
                let cells = line.split(',').collect::<Vec<&str>>();
                let d = cells[3].trim().parse::<i32>().unwrap();
                if d > max {
                    max = d;
                }
                table.push(vec![
                    cells[0].to_string(),
                    cells[1].to_string(),
                    cells[2].to_string(),
                    cells[3].to_string(),
                    cells[4].to_string(),
                ]);
            }
        }

        for row in table.iter_mut() {
            let density = row[3].parse::<f64>().unwrap();
            let a = ((density * 100.0) / (max as f64)).round() as i32;
            row.push(a.to_string());
        }

        table.sort_by(|r1, r2| {
            r2[5]
                .parse::<i32>()
                .unwrap()
                .cmp(&r1[5].parse::<i32>().unwrap())
        });

        for row in table.iter() {
            let mut s = String::new();
            s.push_str(&format!("{:18}", row[0]));
            s.push_str(&format!("{:>10}", row[1]));
            s.push_str(&format!("{:>8}", row[2]));
            s.push_str(&format!("{:>8}", row[3]));
            s.push_str(&format!("{:>18}", row[4]));
            s.push_str(&format!("{:>6}", row[5]));
            println!("{}", s);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::main;

    #[test]
    fn run_soc_opt() {
        main();
    }
}
