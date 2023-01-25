use clap::Parser;

#[derive(Parser)]
#[clap(version, about)]
struct Arguments {
    #[clap(short, long)]
    #[clap(short = 's')]
    component_source_dir: String,
    #[clap(short, long)]
    #[clap(short = 'o')]
    hbf_output_path: String,
    #[clap(short, long)]
    #[clap(short = 'b')]
    target_board: String,
    #[clap(short, long)]
    #[clap(short = 'f')]
    features: Vec<String>,
    #[clap(short, long)]
    #[clap(short = 'v')]
    #[clap(takes_value = false)]
    verbose: bool,
    #[clap(short, long)]
    #[clap(short = 'c')]
    #[clap(takes_value = false)]
    clean_up: bool,
}

fn process_args() -> i32 {
    // Parse args
    let args = Arguments::parse();
    // Print arguments
    println!(
        "\n--------------------\nSource dir: {}\nOutput Path: {}\nTarget Board: {}\nFeatures: {}\nVerbose: {}\nClean-up: {}\n--------------------\n",
        args.component_source_dir, args.hbf_output_path, args.target_board, args.features.join(", "), args.verbose, args.clean_up
    );
    // Launch build process
    if component_builder::build_process(
        args.component_source_dir,
        args.hbf_output_path,
        args.target_board,
        &args.features,
        args.verbose,
        args.clean_up,
    )
    .is_ok()
    {
        return 0;
    } else {
        return 1;
    }
}

fn main() {
    let res = process_args();
    std::process::exit(res);
}
