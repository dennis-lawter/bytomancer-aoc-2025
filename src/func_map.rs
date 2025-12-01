use colored::Colorize;

pub async fn run(func: &str, submit: bool, example: bool) {
    match func {
        "d00s1" => crate::solutions::d00s1::solve(submit, example).await,
        "d00s2" => crate::solutions::d00s2::solve(submit, example).await,
        "d01s1" => crate::solutions::d01s1::solve(submit, example).await,
        "d01s2" => crate::solutions::d01s2::solve(submit, example).await,
        // AUTOMATED EXPANSION PLACEHOLDER
        invalid => {
            println!(
                "{}\n",
                format!("Unrecognized function: {}", invalid.bold()).on_red()
            )
        }
    }
}
