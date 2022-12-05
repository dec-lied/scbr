const BO_PATH: &str = "C:\\Users\\Gavin\\sc2\\";

pub fn list_build_orders()
{
    use colored::Colorize;
    
    if let Ok(entries) = std::fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "\\build_orders"))
    {
        for entry in entries
        {
            if let Ok(entry) = entry
            {
                if let Ok(file_name) = entry.file_name().into_string()
                {
                    println!("{}", file_name.blue());
                }
                else
                {
                    println!("directory entry did not contain valid unicode data");
                }
            }
        }
    }
    else
    {
        println!("invalid directory.");
    }
}

pub fn read_build_order(filename: &String) -> Result<Vec<String>, std::io::Error>
{
    use std::fs::File;
    use std::io::{self, BufRead};

    let build_order_file: File = File::open(String::from(BO_PATH) + filename)?;

    let mut build_order: Vec<String> = Vec::new();
    for line in io::BufReader::new(&build_order_file).lines()
    {
        let line = line?;
        build_order.push(line);
    }

    return Ok(build_order);
}

pub fn start_build_order(filename: &String) -> Result<(), std::io::Error>
{
    use std::thread;
    use colored::Colorize;
    use std::time::{self, SystemTime};
    
    let build_order: Vec<String> = self::read_build_order(filename)?;
    let base_time: u64 = match SystemTime::now().duration_since(time::UNIX_EPOCH)
    {
        Ok(time) => time.as_secs(),
        Err(err) =>
        {
            println!("{}", err.to_string().red());
            std::process::exit(-1);
        }
    };

    let final_instruction_time: u32 =
    {
        if let Ok(final_instruction_minutes) = (&build_order[build_order.len() - 1][..=1]).parse::<u8>()
        {
            if let Ok(final_instruction_seconds) = (&build_order[build_order.len() - 1][3..=4]).parse::<u8>()
            {
                (final_instruction_minutes as u32 * 60) + final_instruction_seconds as u32
            }
            else
            {
                println!("{}", "failed to parse integer from minute section of final build order time".red());
                std::process::exit(-1);
            }
        }
        else
        {
            println!("{}", "failed to parse integer from minute section of final build order time".red());
            std::process::exit(-1);
        }
    };

    let mut current: i16 = -1;
    let mut left_bound: usize = 0;
    let mut right_bound: usize = if build_order.len() >= 11 { 11 } else { build_order.len() };

    loop
    {
        let curr_time: u64 = match SystemTime::now().duration_since(time::UNIX_EPOCH)
        {
            Ok(time) => time.as_secs(),
            Err(err) =>
            {
                println!("{}", err.to_string().red());
                std::process::exit(-1);
            }
        };

        if curr_time - final_instruction_time as u64 > base_time + 2
        {
            break;
        }

        let mut change_bounds: bool = false;

        print!("\x1B[2J\x1B[1;1H");
        for (index, instruction) in build_order[left_bound..right_bound].iter().enumerate()
        {
            // calculating time data
            let instruction_minutes: u8 = match (&instruction[..=1]).parse()
            {
                Ok(mins) => mins,
                Err(_) =>
                {
                    println!("{}", "failed to parse integer from instruction seconds".red());
                    std::process::exit(-1);
                }
            };

            let instruction_seconds: u8 = match (&instruction[3..=4]).parse()
            {
                Ok(secs) => secs,
                Err(_) =>
                {
                    println!("{}", "failed to parse integer from instruction seconds".red());
                    std::process::exit(-1);
                }
            };

            let net_seconds: u32 = (instruction_minutes as u32 * 60) + instruction_seconds as u32;

            // outputting command information to user
            if (curr_time - net_seconds as u64) > base_time - 2 && (curr_time - net_seconds as u64) < base_time + 2
            {
                println!
                (
                    "{}{}{}{}{}\t{}",
                    &instruction[0..=4].blue().on_white(),
                    " ".on_white(),
                    &instruction[6..=7].green().on_white(),
                    " ".on_white(),
                    &instruction[9..].magenta().on_white(),
                    "<--- DO NOW".green()
                );

                current = index as i16;
            }
            else if curr_time - net_seconds as u64 > base_time - 6 && (curr_time - net_seconds as u64) < base_time
            {
                println!
                (
                    "{} {} {}\t{}",
                    &instruction[0..=4].blue(),
                    &instruction[6..=7].green(),
                    &instruction[9..].magenta(),
                    "<--- UPCOMING".red()
                );
            }
            else
            {
                if index as i16 <= current
                {
                    println!
                    (
                        "{}{}{}{}{}",
                        &instruction[0..=4].blue().on_black(),
                        " ".on_black(),
                        &instruction[6..=7].green().on_black(),
                        " ".on_black(),
                        &instruction[9..].magenta().on_black()
                    );
                }
                else
                {
                    println!
                    (
                        "{} {} {}",
                        &instruction[0..=4].blue(),
                        &instruction[6..=7].green(),
                        &instruction[9..].magenta()
                    );
                }
            }
            
            if current == (((right_bound - 1 - left_bound) / 2) + left_bound) as i16 && curr_time - base_time > base_time + 2
            {
                change_bounds = true;
            }
        }

        println!("game time: {}", (curr_time - base_time as u64).to_string().magenta());

        if change_bounds
        {
            left_bound += 1;
            right_bound += 1;
        }

        thread::sleep(time::Duration::from_secs(1));
    }

    return Ok(());
}
