use std::io::{self, Write};
use tracing::{error, info};

fn display(template: &str, params: Vec<&str>, display: impl Fn(&str) -> ()) {
    let split: Vec<&str> = template.split("{}").collect();

    if split.len() == 1 {
        display(template);
        return;
    }

    if split.len() - 1 != params.len() {
        panic!("The number of parameters should match the number of placeholders in the template");
    }

    let output = params
        .into_iter()
        .map(|p| p.to_string())
        .fold(template.to_string(), |tpl, p| {
            tpl.as_str().replacen("{}", p.as_str(), 1).to_string()
        });

    display(output.as_str());
}

pub fn print(template: &str, params: Vec<&str>, not_ending_with_linefeed: Option<bool>) {
    display(template, params, |template: &str| {
        if std::env::var("CLI").is_err() {
            info!("{}", template);
        } else {
            match not_ending_with_linefeed {
                Some(_) => {
                    print!("{}", template);
                    io::stdout().flush().unwrap();
                }
                None => println!("{}", template),
            }
        }
    });
}

pub fn eprint(template: &str, params: Vec<&str>, not_ending_with_linefeed: Option<bool>) {
    display(template, params, |template: &str| {
        if std::env::var("CLI").is_err() {
            error!("{}", template);
        } else {
            match not_ending_with_linefeed {
                Some(_) => {
                    eprint!("{}", template);
                    io::stdout().flush().unwrap();
                }
                None => eprintln!("{}", template),
            }
        }
    })
}
