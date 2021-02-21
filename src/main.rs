use argh::FromArgs;
use std::io::BufRead;

#[derive(FromArgs)]
/// Convert to and from hex strings.
struct Args {
    /// convert the string to a hex representation
    #[argh(switch, short = 'x')]
    to_hex: bool,

    /// convert the string from a hex presentation
    #[argh(switch, short = 'd')]
    from_hex: bool,
    
    /// convert the string from a hex presentation and scale to between 0 and 1 (divide by 255)
    #[argh(switch, short = 's')]
    scale: bool,

    /// read input from file (FFFFFF format (no leading #) separated by newlines)
    #[argh(option, short = 'f')]
    file: Option<String>,

    /// the input to convert (note we implictly convert it to an integer, or fail)
    #[argh(positional)]
    input: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args: Args = argh::from_env();

    match (&args.to_hex, &args.from_hex, &args.input) {
        (true, false, Some(input)) => {
            println!("{}", hex::encode(input));
        },
        (false, true, Some(input)) => {
            println!("{:?}", hex::decode(input));
        },
        (true, false, None) => {
            if let Some(file) = args.file {
                let lines = {
                    let file = std::fs::File::open(file)?;
                    std::io::BufReader::new(file).lines()
                };
                for input in lines {
                    println!("{}", hex::encode(input?.trim()));
                }
            } else {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                println!("{}", hex::encode(input.trim()));
            }
        },
        (false, true, None) => {
            if let Some(file) = args.file {
                let lines = {
                    let file = std::fs::File::open(file)?;
                    std::io::BufReader::new(file).lines()
                };
                for input in lines {
                    if args.scale {
                        println!("{:?}", hex::decode(input?.trim())?.iter().map(|n| *n as f64 / 255.0).map(|x| (x * 10000.0).round() / 10000.0).collect::<Vec<_>>());
                    } else {
                        println!("{:?}", hex::decode(input?.trim())?);
                    }
                }
            } else {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                if args.scale {
                    println!("{:?}", hex::decode(input.trim())?.iter().map(|n| *n as f64 / 255.0).map(|x| (x * 10000.0).round() / 10000.0).collect::<Vec<_>>());
                } else {
                    println!("{:?}", hex::decode(input.trim())?);
                }
            }

            
            
        },
        (_, _, _) => {
            anyhow::bail!("to hex or from hex, pick one please")
        },
    }
    Ok(())
}