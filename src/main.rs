use clap::Parser;

mod bp;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    tag: String,
    #[arg(short, long, default_value_t = String::from("all"))]
    stat: String,
}

fn print_header(tag: &String) {
    println!("{}", tag);
    for _ in 0..tag.len() {
        print!("-");
    }
    println!();
}

fn print_stat<T: std::fmt::Display, F: Fn(&bp::stats::Json) -> T>(
    tag: &str,
    by_tag: &std::collections::HashMap<&str, &bp::stats::Json>,
    label: &str,
    extract: F,
) {
    match bp::stats::get_stat(tag, by_tag, extract) {
        Some(val) => {
            print_header(&tag.to_string());
            println!("{}: {:.2}", label, val);
        }
        None => eprintln!("Player {} not found", tag),
    }
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

    let by_tag = bp::stats::stats_by_tag(&players_stats);
    match args.stat.as_str() {
        "kd" => print_stat(&args.tag, &by_tag, "kd", |j| j.kd),
        "hp_kd" => print_stat(&args.tag, &by_tag, "kd", |j| j.hp_kd),
        "hp_k_10m" => print_stat(&args.tag, &by_tag, "kd", |j| j.hp_k_10m),
        "snd_kd" => print_stat(&args.tag, &by_tag, "kd", |j| j.snd_kd),
        "ovl_kd" => print_stat(&args.tag, &by_tag, "kd", |j| j.ovl_kd),
        "ovl_k_10m" => print_stat(&args.tag, &by_tag, "kd", |j| j.ovl_k_10m),
        _ => match by_tag.get(args.tag.as_str()) {
            Some(player) => println!("{}", player),
            None => eprintln!("Player {} not found", args.tag),
        },
    }

    // match args.stat.as_str() {
    //     "kd" => match bp::stats::get_stat(&args.tag, &by_tag, |j| j.kd) {
    //         Some(kd) => {
    //             print_header(&args.tag);
    //             println!("kd: {:.2}", kd);
    //         }
    //         None => eprintln!("Player {} not found", args.tag),
    //     },
    //     "hp_kd" => match bp::stats::get_stat(&args.tag, &by_tag, |j| j.hp_kd) {
    //         Some(hp_kd) => {
    //             print_header(&args.tag);
    //             println!("hp kd: {:.2}", hp_kd);
    //         }
    //         None => eprintln!("Player {} not found", args.tag),
    //     },
    //     "hp_k_10m" => match bp::stats::get_stat(&args.tag, &by_tag, |j| j.hp_k_10m) {
    //         Some(hp_k_10m) => {
    //             print_header(&args.tag);
    //             println!("hp k 10m: {:.2}", hp_k_10m);
    //         }
    //         None => eprintln!("Player {} not found", args.tag),
    //     },
    //     "snd_kd" => match bp::stats::get_stat(&args.tag, &by_tag, |j| j.snd_kd) {
    //         Some(snd_kd) => {
    //             print_header(&args.tag);
    //             println!("snd kd: {:.2}", snd_kd);
    //         }
    //         None => eprintln!("Player {} not found", args.tag),
    //     },
    //     "ovl_kd" => match bp::stats::get_stat(&args.tag, &by_tag, |j| j.ovl_kd) {
    //         Some(ovl_kd) => println!("ovl kd: {:.2}", ovl_kd),
    //         None => eprintln!("Player {} not found", args.tag),
    //     },
    //     "ovl_k_10m" => match bp::stats::get_stat(&args.tag, &by_tag, |j| j.ovl_k_10m) {
    //         Some(ovl_k_10m) => {
    //             print_header(&args.tag);
    //             println!("ovl k 10m: {:.2}", ovl_k_10m);
    //         }
    //         None => eprintln!("Player {} not found", args.tag),
    //     },
    //     _ => match by_tag.get(args.tag.as_str()) {
    //         Some(player) => println!("{}", player),
    //         None => eprintln!("Player {} not found", args.tag),
    //     },
    // }
}
