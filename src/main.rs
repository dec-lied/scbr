use colored::Colorize;

mod scbr;

fn main()
{
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2
    {
        if args[1].trim_end() == "help"
        {
            println!
            (
                "\n\
                command:\t\t\t\tdescription: \n\
                --------\t\t\t\t------------ \n\
                {}\t\t\t\tthis help message \n\
                {}\t\t\t\tlists available build orders(in cargo_manifest_dir/build_orders) \n\
                {}\tstarts reading build order in real time. defer waits until apostraphe is clicked, then starts reading the build order \n",
                
                "scbr help".red(),
                "scbr list".blue(),
                "scbr [build_order] (defer/await)".green(),
            );
        }
        else if args[1].trim_end() == "list"
        {
            scbr::list_build_orders();
        }
        else
        {
            match scbr::start_build_order(&args[1].trim_end().to_string())
            {
                Ok(_) => (),
                Err(err) => println!("{}", err.to_string().red())
            }
        }
    }
    else if args.len() == 3
    {
        if args[2].trim_end().to_string() == "defer" || args[2].trim_end().to_string() == "await"
        {
            use device_query::{DeviceQuery, DeviceState, Keycode};

            let device_state: DeviceState = DeviceState::new();

            loop
            {
                if device_state.get_keys().contains(&Keycode::Apostrophe)
                {
                    match scbr::start_build_order(&args[1].trim_end().to_string())
                    {
                        Ok(_) => (),
                        Err(err) => println!("{}", err.to_string().red())
                    }

                    break;
                }
            }
        }
        else
        {
            println!("invalid arguments given. type {} for more information", "scbr help".red());
        }
    }
    else
    {
        println!("invalid arguments given. type {} for more information", "scbr help".red());
    }
}
