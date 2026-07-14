use clap::Parser;

mod bp;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    tag: String,
    // #[arg(short, long, default_value_t = 1)]
    // count: u8,
    #[arg(short, long, default_value_t = String::from("all"))]
    stat: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let mut stat_string = String::new();
    let mut players_stats = bp::stats::PlayersStats::default();

    match bp::stats::get_all_player_stats().await {
        Ok(body) => stat_string = body,
        Err(e) => eprintln!("Request failed: {}", e),
    }

    match bp::stats::stats_to_players(stat_string) {
        Ok(stats) => players_stats = stats,
        Err(e) => eprintln!("Conversion failed: {}", e),
    }

    println!("{}", args.tag);
    for _ in 0..args.tag.len() {
        print!("-");
    }
    println!();
    match args.stat.as_str() {
        "kd" => println!(
            "kd: {:?}",
            bp::stats::get_kd(args.tag, &players_stats).unwrap()
        ),
        "hp_kd" => println!(
            "hp kd: {:?}",
            bp::stats::get_hp_kd(args.tag, &players_stats).unwrap()
        ),
        "hp_k_10m" => println!(
            "hp k 10m: {:?}",
            bp::stats::get_hp_k_10m(args.tag, &players_stats).unwrap()
        ),
        "snd_kd" => {
            println!(
                "snd kd: {:?}",
                bp::stats::get_snd_kd(args.tag.clone(), &players_stats).unwrap()
            );
            println!(
                "snd kill: {:?}",
                bp::stats::get_snd_kills(args.tag, &players_stats).unwrap()
            );
        }
        "ovl_kd" => println!(
            "ovl kd: {:?}",
            bp::stats::get_ovl_kd(args.tag, &players_stats).unwrap()
        ),
        "ovl_k_10m" => println!(
            "ovk k 10m: {:?}",
            bp::stats::get_ovl_k_10m(args.tag, &players_stats).unwrap()
        ),
        _ => println!(
            "{:?}",
            bp::stats::print_players_stats(args.tag, &players_stats)
        ),
    }
}
