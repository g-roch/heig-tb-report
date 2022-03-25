// vim: foldmethod=syntax foldlevel=20
use ::function_name::named;
use alphabet::*;
use roman;
use std::collections::BTreeMap;
use std::env;
use std::fs;

static JUDGEMENT_NAMES: [&'static str; 6] = [
    "À rejeter",
    "Insufisant",
    "Passable",
    "Correct",
    "Bien",
    "Excellent",
];
static JUDGEMENT_COLORS: [&'static str; 6] = [
    "red!100",
    "orange!100",
    "yellow!100",
    "green!25!yellow",
    "green!50!yellow",
    "green!100",
];
static JUDGEMENT_ABR: [&'static str; 6] = ["--", "In", "Pa", "Co", "Bi", "Ex"];
static JUDGEMENT_COUNT: usize = JUDGEMENT_NAMES.len();
alphabet!(ALPHABET = "ABCDEFGHIJKLMNOPQRSTUVWXYZ");

struct Votes(Vec<Vote>);

impl Votes {
    fn new(votes: Vec<Vote>) -> Votes {
        assert!(votes.len() > 0, "Minimal count of voting is 1");
        let nb_opt = votes[0].number_option();
        assert!(
            votes.iter().all(|x| x.number_option() == nb_opt),
            "some vote aren't same number of option"
        );
        Votes(votes)
    }
    fn len(&self) -> usize {
        self.0.len()
    }
    fn number_option(&self) -> usize {
        self.0[0].number_option()
    }
    fn number_judgement(&self) -> usize {
        self.0
            .iter()
            .map(|vote| vote.judgement.iter().max().unwrap_or(&0).clone() as usize)
            .max()
            .unwrap()
            + 1
    }
    fn iter(&self) -> std::slice::Iter<Vote> {
        self.0.iter()
    }
}

#[derive(Debug)]
struct Vote {
    raw: Vec<f32>,
    order: Vec<usize>,
    judgement: Vec<u8>,
}

impl Vote {
    fn new(raw: Vec<f32>) -> Vote {
        let judgement = raw.iter().map(|&note| note as u8).collect();
        let mut order_tuple = raw
            .iter()
            .enumerate()
            .filter(|&a| *a.1 > 0f32)
            .collect::<Vec<(_, _)>>();
        order_tuple.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());
        order_tuple.reverse();
        let order = order_tuple.iter().map(|a| a.0).collect();
        Vote {
            raw,
            order,
            judgement,
        }
    }
    fn clone(&self) -> Vote {
        Vote {
            raw: self.raw.clone(),
            order: self.order.clone(),
            judgement: self.judgement.clone(),
        }
    }
    fn judgement_latex_cellcolor(&self) -> Vec<String> {
        self.judgement
            .iter()
            .map(|&note| {
                format!(
                    r#"\cellcolor{{{}}}{}"#,
                    JUDGEMENT_COLORS[note as usize], JUDGEMENT_ABR[note as usize]
                )
            })
            .collect::<Vec<_>>()
    }
    fn number_option(&self) -> usize {
        self.raw.len()
    }
    fn first(&self) -> Option<usize> {
        self.order.first().map(|&a| a)
    }
    fn equal(&self, other: &Vote) -> bool {
        self.order == other.order && self.judgement == other.judgement
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2, "Missing args with votation data file");
    let parts: Vec<&str> = args[1].split(".").collect();
    assert!(
        parts.len() == 2,
        "Args need one point for separate basename and target"
    );
    let basename = parts[0];
    let filename = format!("{}.dat", basename);
    let target = parts[1];

    match target {
        "all" => (),
        "votes" => (),
        "votes-n" => (),
        "votes-c" => (),
        "apro" => (),
        "maj1" => (),
        "maj2" => (),
        "jugmaj" => (),
        "bordatot" => (),
        "bordaclasse" => (),
        "alternatif" => (),
        "coombs" => (),
        "condorcet" => (),
        any => panic!("l'extension «{}» n'est pas reconnu comme une cible", any),
    }
    let figure = target == "all";

    eprintln!("Load vote {:?}", filename);
    let votes = read_votes(&filename);

    if target == "all" || target == "votes-c" || target == "votes-n" || target == "votes" {
        latex_votes(
            &votes,
            &format!("{}:votes", basename),
            "Choix des votants",
            figure,
            false,
            target == "all" || target == "votes" || target == "votes-n",
            target == "all" || target == "votes" || target == "votes-c",
        );
    }

    if target == "all" || target == "apro" {
        eprintln!(
            " => Vote par aprobation : {:?}",
            approval_voting(&votes, &format!("{}:apro", basename), figure)
        );
    }
    if target == "all" || target == "maj1" {
        eprintln!(
            " => scrutin majoritaire uninominal à 1 tour : {:?}",
            single_member_plurality_voting(&votes, &format!("{}:maj1", basename), figure)
        );
    }
    if target == "all" || target == "maj2" {
        eprintln!(
            " => scrutin majoritaire uninominal à 2 tour : {:?}",
            single_member_plurality_voting_2round(&votes, &format!("{}:maj2", basename), figure)
        );
    }
    if target == "all" || target == "jugmaj" {
        eprintln!(
            " => Jugement majoritaire : {:?}",
            majority_judgment(&votes, &format!("{}:jugmaj", basename), figure)
        );
    }
    if target == "all" || target == "bordatot" {
        eprintln!(
            " => Borda (première = nombre_option point) : {:?}",
            borda(&votes, true, &format!("{}:bordatot", basename), figure)
        );
    }
    if target == "all" || target == "bordaclasse" {
        eprintln!(
            " => Borda (première = nombre d'option classée point) : {:?}",
            borda(&votes, false, &format!("{}:bordaclasse", basename), figure)
        );
    }
    if target == "all" || target == "alternatif" {
        eprintln!(
            " => Vote alternatif : {:?}",
            alternative_vote(&votes, &format!("{}:alternatif", basename), figure)
        );
    }
    if target == "all" || target == "coombs" {
        eprintln!(
            " => Coombs : {:?}",
            coombs(&votes, &format!("{}:coombs", basename), figure)
        );
    }
    if target == "all" || target == "condorcet" {
        eprintln!(
            " => Condorcet : {:?}",
            condorcet(&votes, &format!("{}:condorcet", basename), figure)
        );
    }
    return;

    //println!("Hello, world! {:?}", args);
    //let votes = vec![1, 5, 3, 4, 6, 2, 3, 11, 2, 3, 4];
    //let min = votes.iter().max();
    //println!("{:?}", min);
}

// Read lines from file and ignore line begin by '#', trim each line
fn read_lines(filename: &String) -> Vec<String> {
    let file = fs::read_to_string(filename).expect("Can't read the file");
    file.lines()
        .map(|line| String::from(line.trim()))
        .filter(|line| !line.starts_with('#'))
        .collect()
}

fn read_votes(filename: &String) -> Votes {
    let file = fs::read_to_string(filename).expect("Can't read the file");
    let lines = file
        .lines()
        .map(|line| String::from(line.trim()))
        .filter(|line| !line.starts_with('#') && line != "");
    let mut votes: Vec<Vote> = vec![];
    for line in lines {
        let mut list = line.split_whitespace();
        let qte = list
            .next()
            .expect("Missing data")
            .parse::<i32>()
            .expect("Fail to parse votation file");
        let vote = Vote::new(
            list.map(|note| note.parse::<f32>().expect("Fail to parse votation file"))
                .collect(),
        );
        for _ in 0..qte {
            votes.push(vote.clone())
        }
    }

    Votes::new(votes)

    //Votes::new(
    //    read_lines(filename)
    //        .iter()
    //        .map(|line| {
    //            Vote::new(
    //                line.split_whitespace()
    //                    .map(|note| note.parse::<f32>().expect("Fail to parse votation file"))
    //                    .collect(),
    //            )
    //        })
    //        .collect(),
    //)
}

fn option_to_letter(option: usize) -> String {
    ALPHABET.iter_words().skip(1).nth(option).unwrap()
}

fn latex_votes(
    votes: &Votes,
    basename: &str,
    title: &str,
    figure: bool,
    legend: bool,
    show_note: bool,
    show_classement: bool,
) {
    if figure {
        println!("{}", r#"\begin{table}[H]"#);
        println!(r#"\caption{{\scrutinname: {}}}"#, title);
        println!(r#"\label{{tab:scrutin:{}}}"#, basename);
        println!("{}", r#"\begin{center}"#);
    }

    print!("{}", r#"\begin{tabular}{|rrc|"#);
    if show_note {
        print!("|{}|", vec!["c"; votes.number_option()].join("|"));
    }
    if show_classement {
        print!("|l|");
    }
    println!("}}");
    println!("{}", r#"\hline"#);

    print!(r#" & ${}\times$ & "#, votes.len(),);
    if show_note {
        print!(
            r#" & {} "#,
            (0..votes.number_option())
                .map(|a| option_to_letter(a))
                .collect::<Vec<String>>()
                .join(" & "),
        );
    }
    if show_classement {
        print!(r#" & Classement"#,);
    }
    println!(r"\\");
    println!("{}", r#"\hline"#);
    println!("{}", r#"\hline"#);
    let mut count = 0;
    let mut line = 0;
    for (elector, vote) in votes.0.iter().enumerate() {
        count += 1;
        if elector + 1 < votes.0.len() && vote.equal(&votes.0[elector + 1]) {
            continue;
        }
        line += 1;
        print!(
            r#" ${} \%$ & ${}\times$&{} "#,
            100 * count / votes.len(),
            count,
            roman::to(line).unwrap().to_lowercase(),
        );
        if show_note {
            print!(r#"  & {} "#, vote.judgement_latex_cellcolor().join(" & "),);
        }
        if show_classement {
            print!(
                r#" & ${}$"#,
                vote.order
                    .iter()
                    .map(|a| option_to_letter(*a))
                    .collect::<Vec<_>>()
                    .join(r#" \succ "#),
            );
        }
        println!(r#"\\"#,);
        count = 0;
    }
    println!("{}", r#"\hline"#);
    println!("{}", r#"\end{tabular}"#);
    if legend {
        println!(r#"\\"#);
        println!(
            "{}",
            (0..JUDGEMENT_COUNT)
                .map(|a| format!(
                    r#"\colorbox{{{}}}{{{}}}"#,
                    JUDGEMENT_COLORS[a], JUDGEMENT_NAMES[a]
                ))
                .rev()
                .collect::<Vec<_>>()
                .join("$ \\succ $"),
        );
    }
    if figure {
        println!("{}", r#"\end{center}"#);
        println!("{}", r#"\end{table}"#);
    }
}

fn latex_score(
    results: &Vec<i32>,
    basename: &str,
    title: &str,
    singular: &str,
    plural: &str,
    showsum: bool,
    figure: bool,
) {
    let winner = extract_maximum_vec(&results).unwrap_or(vec![]);
    latex_score_ntour(
        &vec![results.clone()],
        &vec![winner],
        basename,
        title,
        singular,
        plural,
        showsum,
        figure,
    );
}

fn latex_score_ntour(
    results: &Vec<Vec<i32>>,
    keepeds: &Vec<Vec<usize>>,
    basename: &str,
    title: &str,
    singular: &str,
    plural: &str,
    showsum: bool,
    figure: bool,
) {
    eprintln!("{:?}", keepeds);
    assert!(
        results.len() == keepeds.len(),
        "À besoin de la listes des candidats garder après chaques tours"
    );

    if figure {
        println!("{}", r#"\begin{table}[H]"#);
        println!(r#"\caption{{\scrutinname: {}}}"#, title);
        println!(r#"\label{{tab:scrutin:{}}}"#, basename);
        println!("{}", r#"\begin{center}\adjustbox{valign=t}{"#);
    }
    for round in 0..results.len() {
        let mut sum = 0;
        let last_round = round == results.len() - 1;
        println!("{}", r#"\begin{tabular}{|c|c|}"#);
        println!("{}", r#"\hline"#);
        println!(r#"\multicolumn{{2}}{{|c|}}{{Tour {}}} \\"#, round + 1);
        println!("{}", r#"\hline"#);
        println!(r#"Opt. & Resultats \\"#);
        println!("{}", r#"\hline"#);
        println!("{}", r#"\hline"#);
        let mut results_sorted: Vec<_> = if round >= 1 {
            results[round]
                .iter()
                .enumerate()
                .filter(|(key, _)| keepeds[round - 1].contains(key))
                .collect()
        } else {
            results[round].iter().enumerate().collect()
        };
        results_sorted.sort_by_key(|(_, a)| *a);
        results_sorted.reverse();
        let color_keeped = if last_round {
            format!(r#"\cellcolor{{{}}}"#, JUDGEMENT_COLORS.last().unwrap())
        } else {
            String::new()
        };
        let color_exclued = if last_round {
            String::new()
        } else {
            format!(r#"\cellcolor{{{}}}"#, JUDGEMENT_COLORS.first().unwrap())
        };
        for (key, value) in results_sorted {
            println!(
                r#"{}{} & {} {} \\"#,
                if keepeds[round].contains(&key) {
                    &color_keeped
                } else {
                    &color_exclued
                },
                option_to_letter(key),
                value,
                if *value <= 1 { singular } else { plural }
            );
            println!("{}", r#"\hline"#);
            sum += value;
        }
        if showsum {
            println!("{}", r#"\hline"#);
            println!(
                r#"$\sum$ & {} {} \\"#,
                sum,
                if sum <= 1 { singular } else { plural }
            );
            println!("{}", r#"\hline"#);
        }

        println!("{}", r#"\end{tabular}"#);
    }
    if figure {
        println!("{}", r#"}\end{center}"#);
        println!("{}", r#"\end{table}"#);
    }
}

fn latex_result_majority_judgment(
    details: &Vec<Vec<i32>>,
    results_mention: &Vec<(u8, i32)>,
    winner: &Vec<usize>,
    required_voter: i32,
    basename: &str,
    title: &str,
    singular: &str,
    plural: &str,
    figure: bool,
) {
    assert!(details.len() >= 1);
    let mut results_mention_sorted: Vec<(_, _, _)> = results_mention
        .iter()
        .enumerate()
        .map(|(opt, (mention, voix))| (opt, *mention, *voix))
        .collect();
    results_mention_sorted.sort_by_key(|(_, mention, voix)| (*mention, *voix));
    results_mention_sorted.reverse();
    if figure {
        println!("{}", r#"\begin{table}[H]"#);
        println!(r#"\caption{{\scrutinname: {}}}"#, title);
        println!(r#"\label{{tab:scrutin:{}}}"#, basename);
        println!("{}", r#"\begin{center}"#);
    }
    println!("{}", r#"\pgfplotsset{compat=1.9}"#);
    println!("{}", r#"\adjustbox{valign=t}{\begin{tikzpicture}"#);
    println!("{}", r#"\begin{axis}["#);
    println!("{}", r#"ybar stacked,"#);
    println!("{}", r#"nodes near coords,"#);
    println!("{}", r#"enlargelimits=0.15,"#);
    println!(
        "{}",
        r#"legend style={at={(0.5,-0.20)},anchor=north,legend columns=-1},"#
    );
    println!("{}", r#"ylabel={votants},"#);
    println!(
        "{}{}{}",
        r#"symbolic x coords={"#,
        (0..details.len())
            .map(|a| option_to_letter(a))
            .collect::<Vec<String>>()
            .join(","),
        "},"
    );
    println!("{}", r#"xtick=data,"#);
    println!("{}", r#"x tick label style={anchor=north},"#);
    println!("{}", r#"]"#);
    for mention in (0..JUDGEMENT_COUNT).rev() {
        println!(
            r#"\addplot+ [fill={},draw=black!100,text=black!100] coordinates {{{}}};"#,
            JUDGEMENT_COLORS[mention],
            details
                .iter()
                .enumerate()
                .map(|(pos, val)| format!("({},{})", option_to_letter(pos), val[mention]))
                .collect::<Vec<_>>()
                .join(" "),
        );
    }
    println!(
        r#"\addplot [smooth,stack plots=false,red,line legend,sharp plot,nodes near coords={{}},
    update limits=false,shorten >=-30mm,shorten <=-30mm]
        coordinates {{({1},{0}) ({2},{0})}} node[pos=1.10,left,rotate=90]{{majorité}} node[pos=1.10,right,rotate=90]{{= {0}}}
        ;
        "#,
        required_voter,
        option_to_letter(0),
        option_to_letter(details.len() - 1),
    );
    println!(
        r#"\legend{{{}}}"#,
        (0..JUDGEMENT_COUNT)
            .rev()
            .map(|a| JUDGEMENT_ABR[a])
            .collect::<Vec<_>>()
            .join(",")
    );
    println!("{}", r#"\end{axis}"#);
    println!("{}", r#"\end{tikzpicture}}"#);
    println!("{}", r#"\adjustbox{valign=t}{\begin{tabular}{|c|c|c|}"#);
    println!("{}", r#"\hline"#);
    println!(r#"Opt. & Mention & avec \\"#);
    println!("{}", r#"\hline"#);
    println!("{}", r#"\hline"#);
    for (opt, mention, voix) in results_mention_sorted {
        println!(
            r#" {}{} & \cellcolor{{{}}}{} & {} {} \\"#,
            if winner.contains(&opt) {
                format!(r#"\cellcolor{{{}}}"#, JUDGEMENT_COLORS.last().unwrap())
            } else {
                String::new()
            },
            option_to_letter(opt),
            JUDGEMENT_COLORS[mention as usize],
            JUDGEMENT_NAMES[mention as usize],
            if mention == 0 {
                String::new()
            } else {
                format!("{}", voix)
            },
            if mention == 0 {
                ""
            } else {
                if voix <= 1 {
                    singular
                } else {
                    plural
                }
            }
        );

        println!("{}", r#"\hline"#);
    }
    println!("{}", r#"\end{tabular}}"#);
    if figure {
        println!("{}", r#"\end{center}"#);
        println!("{}", r#"\end{table}"#);
    }
}

fn latex_score_condorcet(
    duel: &Vec<Vec<i32>>,
    args_winner: &Option<Vec<usize>>,
    basename: &str,
    title: &str,
    figure: bool,
) {
    let winner_color; // = r#"\cellcolor{green!100}"#;
    let looser_color; // = "";
    let winner;
    if let Some(value) = args_winner.clone() {
        winner_color = r#"\cellcolor{green!100}"#;
        looser_color = "";
        winner = value;
    } else {
        winner_color = r#"\cellcolor{orange!100}"#;
        looser_color = r#"\cellcolor{orange!100}"#;
        winner = vec![];
    }
    if figure {
        println!(r#"\begin{{table}}[H]"#);
        println!(r#"\caption{{\scrutinname: {}}}"#, title);
        println!(r#"\label{{tab:scrutin:{}}}"#, basename);
        println!(r#"\begin{{center}}"#);
    }
    println!(
        r#"\begin{{tabular}}{{|c||{}|}}"#,
        vec!["c"; duel.len()].join("|")
    );

    println!(r#"\hline"#);
    println!(
        r#" & {} \\"#,
        (0..duel.len())
            .map(|a| option_to_letter(a))
            .collect::<Vec<_>>()
            .join(" & ")
    );
    println!(r#"\hline"#);
    println!(r#"\hline"#);
    for (i, line) in duel.iter().enumerate() {
        println!(
            r#"{}{} & {} \\"#,
            if winner.contains(&i) {
                winner_color
            } else {
                looser_color
            },
            option_to_letter(i),
            line.iter()
                .enumerate()
                .map(|(j, &value)| if j == i {
                    "--".to_string()
                } else if value > duel[j][i] {
                    format!(r#"\cellcolor{{green!100}}{}"#, value)
                } else if value == duel[j][i] {
                    format!(r#"\cellcolor{{orange!100}}{}"#, value)
                } else {
                    format!(r#"{}"#, value)
                })
                .collect::<Vec<_>>()
                .join(" & ")
        );
        println!(r#"\hline"#);
    }

    println!(r#"\end{{tabular}}"#);
    if figure {
        println!(r#"\end{{center}}"#);
        println!(r#"\end{{table}}"#);
    }
}
// scrutin_majoritaire_uninominal_1tour
fn single_member_plurality_voting(
    votes: &Votes,
    basename: &str,
    figure: bool,
) -> Option<Vec<usize>> {
    let mut results = vec![0; votes.number_option()];

    for vote in votes.iter() {
        if let Some(first) = vote.first() {
            results[first] += 1;
        }
    }

    latex_score(
        &results,
        basename,
        "Scrutin majoritaire à 1 tour",
        "vote",
        "votes",
        true,
        figure,
    );

    extract_maximum_vec(&results)
}

// scrutin_majoritaire_uninominal_2tour
#[named]
fn single_member_plurality_voting_2round(
    votes: &Votes,
    basename: &str,
    figure: bool,
) -> Option<Vec<usize>> {
    assert!(
        votes.len() > 2,
        "Le vote à 2 tours n'est possible que s'il y a 3 optiosn ou plus (+)"
    );
    let mut results_t1 = vec![0; votes.number_option()];

    for vote in votes.iter() {
        if let Some(first) = vote.first() {
            results_t1[first] += 1;
        }
    }

    let mut results_t1_sorted: Vec<_> = results_t1.iter().enumerate().collect();
    results_t1_sorted.sort_by(|a, b| a.1.cmp(b.1));
    results_t1_sorted.reverse();

    // Si le 2ème et 3ème on le même nombre de voix → on sait pas qui vas au second tour
    if results_t1_sorted.len() > 2 && results_t1_sorted[1].1 == results_t1_sorted[2].1 {
        eprintln!("{:}: Can't choose two best option", function_name!());
        return None;
    }

    let keeped = vec![results_t1_sorted[0].0, results_t1_sorted[1].0];
    eprintln!(
        "{:}: 2nd tour: reste les options {:?}",
        function_name!(),
        keeped
    );

    // SECOND TOUR
    let mut results_t2: BTreeMap<usize, _> = BTreeMap::new();
    results_t2.insert(keeped[0], 0);
    results_t2.insert(keeped[1], 0);

    for vote in votes.iter() {
        for option in &vote.order {
            if let Some(value) = results_t2.get_mut(&option) {
                *value += 1;
                break;
            }
        }
    }

    // Ne retourne None que si results_t2 est vide, ce qui est contrôler au début de la fn
    let winner = extract_maximum_map(&results_t2).unwrap();

    let mut results_t2_vec = vec![0; results_t1.len()];
    results_t2_vec[keeped[0]] = results_t2[&keeped[0]];
    results_t2_vec[keeped[1]] = results_t2[&keeped[1]];
    let results = vec![results_t1, results_t2_vec];
    let keepeds = vec![keeped, winner];
    latex_score_ntour(
        &results,
        &keepeds,
        basename,
        "Scrutin majoritaire à 2 tours",
        "vote",
        "votes",
        true,
        figure,
    );

    Some(keepeds[1].clone())
}

fn borda(
    votes: &Votes,
    first_has_nboption_point: bool,
    basename: &str,
    figure: bool,
) -> Option<Vec<usize>> {
    let mut scores: Vec<i32> = vec![0; votes.number_option()];

    for vote in votes.iter() {
        let mut point = if first_has_nboption_point {
            votes.number_option() as i32
        } else {
            vote.order.len() as i32
        };

        for &option in &vote.order {
            scores[option] += point;
            point -= 1;
        }
    }

    let title = format!(
        "Borda (max pts = nombre d'option {})",
        if first_has_nboption_point {
            "total"
        } else {
            "classé"
        }
    );
    latex_score(&scores, &basename, &title, "point", "points", false, figure);

    extract_maximum_vec(&scores)
}

fn condorcet(votes: &Votes, basename: &str, figure: bool) -> Option<Vec<usize>> {
    let mut duel = vec![vec![0; votes.number_option()]; votes.number_option()];
    for i in 0..votes.number_option() {
        for j in 0..votes.number_option() {
            if i == j {
                continue;
            }
            // combien de foix i est devant j ?
            for vote in &votes.0 {
                for &option in &vote.order {
                    if option == j {
                        // i ne bats pas j
                        break;
                    }
                    if option == i {
                        // i bats j
                        duel[i][j] += 1;
                    }
                }
            }
        }
    }

    let mut winner = None;
    for i in 0..votes.number_option() {
        let mut wins_all = true;
        for j in 0..votes.number_option() {
            if duel[i][j] < duel[j][i] {
                wins_all = false;
                break;
            }
        }
        if wins_all {
            winner = Some(vec![i]);
        }
    }

    latex_score_condorcet(&duel, &winner, basename, "Condorcet", figure);

    None
}

fn alternative_vote(votes: &Votes, basename: &str, figure: bool) -> Option<Vec<usize>> {
    alternative_vote_and_coombs(votes, false, basename, figure)
}
fn coombs(votes: &Votes, basename: &str, figure: bool) -> Option<Vec<usize>> {
    alternative_vote_and_coombs(votes, true, basename, figure)
}
#[named]
fn alternative_vote_and_coombs(
    votes: &Votes,
    coombs: bool,
    basename: &str,
    figure: bool,
) -> Option<Vec<usize>> {
    let mut all_results: Vec<Vec<i32>> = vec![];
    let mut all_keeped: Vec<Vec<usize>> = vec![];
    let mut round;

    let mut results_in_first: BTreeMap<usize, i32> = BTreeMap::new();
    let mut results_in_last: BTreeMap<usize, i32> = BTreeMap::new();
    let mut voter_count: i32;

    for i in 0..votes.number_option() {
        results_in_first.insert(i, 0);
        results_in_last.insert(i, 0);
    }

    loop {
        all_results.push(vec![0; votes.number_option()]);
        all_keeped.push(vec![]);
        round = all_results.len() - 1;

        voter_count = 0;
        for vote in votes.iter() {
            for option in &vote.order {
                if let Some(value) = results_in_first.get_mut(option) {
                    *value += 1;
                    voter_count += 1;
                    break;
                }
            }
            for option in vote.order.iter().rev() {
                if let Some(value) = results_in_last.get_mut(option) {
                    *value += 1;
                    // Pas besoin de réaugmenter le voter_count car c'est déjà
                    // fait dans la boucle précédente
                    break;
                }
            }
        }

        let best_first_score = results_in_first.iter().map(|a| a.1).max()?.clone();
        let worst_first_score = results_in_first.iter().map(|a| a.1).min()?.clone();
        let best_last_score = results_in_last.iter().map(|a| a.1).max()?.clone();

        let results_for_remove = if coombs {
            &results_in_last
        } else {
            &results_in_first
        };
        let score_for_remove = if coombs {
            best_last_score
        } else {
            worst_first_score
        };

        // Selection les options à supprimer
        let to_remove: Vec<usize> = results_for_remove
            .iter()
            .filter(|a| *a.1 == score_for_remove)
            .map(|a| a.0.clone())
            .collect();

        // Sauvegarde de l'état pour le LaTeX
        for (key, value) in &results_in_first {
            all_results[round][*key] = *value;
        }
        all_keeped[round] = results_for_remove
            .iter()
            .map(|(key, _)| *key)
            .filter(|key| !to_remove.contains(key))
            .collect();

        // S'il y a une majorité
        // (Division entière fonctionne car on fait un strictement plus grand,
        // et que dans le contage il n'y a pas de fraction de voix)
        if best_first_score > voter_count / 2 {
            all_keeped[round] = extract_maximum_map(&results_in_first).unwrap();
            break;
        }

        // S'il n'y a tout à supprimer alors tout le monde
        // est ex-equo
        if to_remove.len() == results_in_first.len() {
            break;
        }

        // Supprime les options non-retenues
        eprintln!(
            "{:}: first_score {:?}; last_score {:?}; remove {:?}",
            function_name!(),
            results_in_first,
            results_in_last,
            to_remove
        );
        for option in &to_remove {
            results_in_first.remove(option);
        }

        for (_, value) in results_in_first.iter_mut() {
            *value = 0;
        }
        results_in_last = results_in_first.clone();
    }
    latex_score_ntour(
        &all_results,
        &all_keeped,
        basename,
        if coombs { "Coombs" } else { "Vote alternatif" },
        "vote",
        "votes",
        true,
        figure,
    );
    return extract_maximum_map(&results_in_first);
}

fn approval_voting(votes: &Votes, basename: &str, figure: bool) -> Option<Vec<usize>> {
    let mut results = vec![0; votes.number_option()];

    for vote in votes.iter() {
        for &option in &vote.order {
            results[option] += 1;
        }
    }

    latex_score(
        &results,
        basename,
        "Vote par approbation",
        "approbation",
        "approbations",
        false,
        figure,
    );

    extract_maximum_vec(&results)
}

fn majority_judgment(votes: &Votes, basename: &str, figure: bool) -> Option<Vec<usize>> {
    let mut results: Vec<Vec<i32>>;
    results = vec![vec![0; votes.number_judgement()]; votes.number_option()];

    for vote in votes.iter() {
        for (option, &judgement) in vote.judgement.iter().enumerate() {
            results[option][judgement as usize] += 1;
        }
    }

    let required_voter: i32 = votes.len() as i32 / 2;
    let majority_mention: Vec<(u8, i32)> = results
        .iter()
        .map(|result| {
            let mut total = 0;
            let mut total_mention: u8 = 0;
            for (mention, count) in result.iter().enumerate().rev() {
                total += count;
                total_mention = mention as u8;
                if total > required_voter {
                    break;
                }
            }
            (total_mention, total)
        })
        .collect();

    let best_mention = majority_mention.iter().map(|a| a.0).max()?;

    // Si aucun candidat n'a au moins la pire mention → aucun n'est élu
    if best_mention == 0 {
        return None;
    }

    let majority_mention_filtered = majority_mention
        .iter()
        .enumerate()
        .filter(|a| a.1 .0 == best_mention)
        .map(|a| (a.0, a.1 .1))
        .collect::<BTreeMap<_, _>>();

    let winner = extract_maximum_map(&majority_mention_filtered).unwrap();
    latex_result_majority_judgment(
        &results,
        &majority_mention,
        &winner,
        required_voter,
        basename,
        "Jugement majoritaire",
        "vote",
        "votes",
        figure,
    );
    Some(winner)
}

fn extract_maximum_vec(scores: &Vec<i32>) -> Option<Vec<usize>> {
    let winner_score = scores.iter().max()?;
    let winner = scores
        .iter()
        .enumerate()
        .filter(|a| a.1 == winner_score)
        .map(|a| a.0)
        .collect();
    Some(winner)
}
fn extract_maximum_map(scores: &BTreeMap<usize, i32>) -> Option<Vec<usize>> {
    let winner_score = scores.iter().map(|a| a.1).max()?;
    let winner = scores
        .iter()
        .filter(|a| a.1 == winner_score)
        .map(|a| a.0.clone())
        .collect();
    Some(winner)
}
