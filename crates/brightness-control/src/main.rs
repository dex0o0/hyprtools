use std::{
    env,
    error::Error,
    process::{Command, ExitCode},
    thread,
    time::Duration,
};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let Some(input)=prompt_brightness()? else {
       return Ok(()); 
    };

    let percent = parse_brightness(&input)?;

    
    // let mut args = env::args().skip(1);
    //
    // let percent: u8 = args
    //     .next()
    //     .ok_or("Usage: brightness-control <0-100>")?
    //     .parse()?;

    if percent > 100 {
        return Err("Brightness must be between 0 and 100".into());
    }

    // if args.next().is_some() {
    //     return Err("Too many arguments".into());
    // }

    let current = get_brightness("get")?;
    let max = get_brightness("max")?;

    if max == 0 {
        return Err("Maximum brightness cannot be zero".into());
    }

    let target = (u128::from(max) * u128::from(percent) / 100) as u64;

    let diff = i128::from(target) - i128::from(current);

    // Use fewer steps for small brightness changes.
    let steps = diff.unsigned_abs().min(50);

    if steps > 0 {
        for step in 1..=steps {
            // let value = i128::from(current) + diff * i128::from(step) / i128::from(steps);
            let step = step as i128;
            let steps = steps as i128;

            let value = i128::from(current) + diff * step / steps;

            set_brightness(&value.to_string())?;

            thread::sleep(Duration::from_millis(20));
        }
    }

    // Ensure the final value matches the requested percentage.
    set_brightness(&format!("{percent}%"))?;

    Ok(())
}

fn prompt_brightness() -> Result<Option<String>, Box<dyn Error>> {
    let theme_path =
        env::var("HOME")? + "/.config/rofi/config-brightness.rasi";

    let output = Command::new("rofi")
        .args([
            "-dmenu",
            "-i",
            "-config",
            &theme_path,
            "-p",
            "Brightness",
            "-mesg",
            "Enter brightness (0-100)",
        ])
        .output()?;

    if !output.status.success() {
        return Ok(None);
    }

    let input = String::from_utf8(output.stdout)?;
    let input = input.trim();

    if input.is_empty() {
        return Ok(None);
    }

    Ok(Some(input.to_string()))
}

fn parse_brightness(input: &str) -> Result<u8, String> {
    let percent = input
        .trim()
        .parse::<u8>()
        .map_err(|_| "Enter a valid number between 0 and 100".to_string())?;

    if percent > 100 {
        return Err("Brightness must be between 0 and 100".into());
    }

    Ok(percent)
}
fn get_brightness(command: &str) -> Result<u64, Box<dyn Error>> {
    let output = Command::new("brightnessctl").arg(command).output()?;

    if !output.status.success() {
        return Err(format!(
            "brightnessctl {command} failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        )
        .into());
    }

    let value = String::from_utf8(output.stdout)?;
    Ok(value.trim().parse()?)
}

fn set_brightness(value: &str) -> Result<(), Box<dyn Error>> {
    let output = Command::new("brightnessctl")
        .args(["set", value])
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "Failed to set brightness: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        )
        .into());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parse_brightness;
    #[test]
    fn calculates_target_brightness() {
        let max = 937u128;
        let percent = 50;

        let target = (max * percent / 100) as u64;

        assert_eq!(target, 468);
    }

    #[test]
    fn calculates_increasing_brightness() {
        let current: i128 = 100;
        let target: i128 = 500;
        let steps: i128 = 50;

        let first_step = current + (target - current) / steps;

        assert_eq!(first_step, 108);
    }

    #[test]
    fn calculates_decreasing_brightness() {
        let current: i128 = 500;
        let target: i128 = 100;
        let steps: i128 = 50;

        let first_step = current + (target - current) / steps;

        assert_eq!(first_step, 492);
    }

    #[test]
    fn accepts_valid_brightness() {
        assert_eq!(parse_brightness("50").unwrap(), 50);
        assert_eq!(parse_brightness("0").unwrap(), 0);
        assert_eq!(parse_brightness("100").unwrap(), 100);
    }

    #[test]
    fn rejects_brightness_above_100() {
        assert!(parse_brightness("101").is_err());
    }

    #[test]
    fn rejects_invalid_input() {
        assert!(parse_brightness("abc").is_err());
        assert!(parse_brightness("-10").is_err());
        assert!(parse_brightness("").is_err());
    }
}

