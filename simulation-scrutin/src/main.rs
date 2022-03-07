// vim: foldmethod=syntax foldlevel=20
use ::function_name::named;
use std::collections::BTreeMap;
use std::env;
use std::fs;

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
    fn judgement_human(&self) -> String {
        self.judgement
            .iter()
            .map(|&note| match note {
                5 => "TB",
                4 => " B",
                3 => "AB",
                2 => " P",
                1 => " I",
                0 => "--",
                _ => "¿?",
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
    fn order_human(&self) -> String {
        self.order
            .iter()
            .map(|a| format!("{:2}", a))
            .collect::<Vec<_>>()
            .join(" > ")
    }
    fn number_option(&self) -> usize {
        self.raw.len()
    }
    fn blank_vote(&self) -> bool {
        self.order.len() == 0
    }
    fn first(&self) -> Option<usize> {
        self.order.first().map(|&a| a)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() >= 2, "Missing args with votation data file");
    eprintln!("Load vote {:?}", args[1]);
    let votes = read_votes(&args[1]);

    for (elector, vote) in votes.0.iter().enumerate() {
        eprintln!(
            "{:3}: {:} ; {:}",
            elector,
            vote.judgement_human(),
            vote.order_human(),
        );
    }

    println!(
        " => scrutin majoritaire uninominal à 1 tour : {:?}",
        single_member_plurality_voting(&votes)
    );
    println!(
        " => scrutin majoritaire uninominal à 2 tour : {:?}",
        single_member_plurality_voting_2round(&votes)
    );
    println!(
        " => Borda (première = nombre_option point) : {:?}",
        borda(&votes, true)
    );
    println!(
        " => Borda (première = nombre d'option classée point) : {:?}",
        borda(&votes, false)
    );
    println!(" => Vote alternatif : {:?}", alternative_vote(&votes));
    println!(" => Coombs : {:?}", coombs(&votes));
    println!(" => Vote par aprobation : {:?}", approval_voting(&votes));
    println!(" => Jugement majoritaire : {:?}", majority_judgment(&votes));

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
    Votes::new(
        read_lines(filename)
            .iter()
            .map(|line| {
                Vote::new(
                    line.split_whitespace()
                        .map(|note| note.parse::<f32>().expect("Fail to parse votation file"))
                        .collect(),
                )
            })
            .collect(),
    )
}

// scrutin_majoritaire_uninominal_1tour
fn single_member_plurality_voting(votes: &Votes) -> Option<Vec<usize>> {
    let mut results = vec![0; votes.number_option()];

    for vote in votes.iter() {
        if let Some(first) = vote.first() {
            results[first] += 1;
        }
    }

    extract_maximum_vec(&results)
}

// scrutin_majoritaire_uninominal_2tour
#[named]
fn single_member_plurality_voting_2round(votes: &Votes) -> Option<Vec<usize>> {
    assert!(
        votes.len() > 2,
        "Le vote à 2 tours n'est possible que s'il y a plus (+) que 2 option"
    );
    let mut results = vec![0; votes.number_option()];

    for vote in votes.iter() {
        if let Some(first) = vote.first() {
            results[first] += 1;
        }
    }

    let mut option_order: Vec<_> = results.iter().enumerate().collect();
    option_order.sort_by(|a, b| a.1.cmp(b.1));
    option_order.reverse();

    // Si le 2ème et 3ème on le même nombre de voix → on sait pas qui vas au second tour
    if option_order.len() > 2 && option_order[1].1 == option_order[2].1 {
        eprintln!("{:}: Can't choose two best option", function_name!());
        return None;
    }

    let keeped = vec![option_order[0].0, option_order[1].0];
    eprintln!(
        "{:}: 2nd tour: reste les options {:?}",
        function_name!(),
        keeped
    );

    // SECOND TOUR
    let mut results: BTreeMap<usize, _> = BTreeMap::new();
    results.insert(keeped[0], 0);
    results.insert(keeped[1], 0);

    for vote in votes.iter() {
        for option in &vote.order {
            if let Some(value) = results.get_mut(&option) {
                *value += 1;
            }
        }
    }

    extract_maximum_map(&results)
}

fn borda(votes: &Votes, first_has_nboption_point: bool) -> Option<Vec<usize>> {
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

    extract_maximum_vec(&scores)
}

//fn condorcet(votes: &Votes) -> Option<Vec<usize>> {}

fn alternative_vote(votes: &Votes) -> Option<Vec<usize>> {
    alternative_vote_and_coombs(votes, false)
}
fn coombs(votes: &Votes) -> Option<Vec<usize>> {
    alternative_vote_and_coombs(votes, true)
}
#[named]
fn alternative_vote_and_coombs(votes: &Votes, coombs: bool) -> Option<Vec<usize>> {
    let mut results_in_first: BTreeMap<usize, i32> = BTreeMap::new();
    let mut results_in_last: BTreeMap<usize, i32> = BTreeMap::new();
    let mut voter_count: i32;

    for i in 0..votes.number_option() {
        results_in_first.insert(i, 0);
        results_in_last.insert(i, 0);
    }

    loop {
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

        // S'il y a une majorité
        // (Division entière fonctionne car on fait un strictement plus grand,
        // et que dans le contage il n'y a pas de fraction de voix)
        if best_first_score > voter_count / 2 {
            return extract_maximum_map(&results_in_first);
        }

        let results_for_remove = if coombs {
            &results_in_last
        } else {
            &results_in_first
        };
        let score_for_remove = if coombs {
            best_last_score
        } else {
            best_first_score
        };

        // Selection les options à supprimer
        let to_remove: Vec<usize> = results_for_remove
            .iter()
            .filter(|a| *a.1 == score_for_remove)
            .map(|a| a.0.clone())
            .collect();

        // S'il n'y a tout à supprimer alors tout le monde
        // est ex-equo
        if to_remove.len() == results_in_first.len() {
            return extract_maximum_map(&results_in_first);
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
}

fn approval_voting(votes: &Votes) -> Option<Vec<usize>> {
    let mut results = vec![0; votes.number_option()];

    for vote in votes.iter() {
        for &option in &vote.order {
            results[option] += 1;
        }
    }

    extract_maximum_vec(&results)
}

fn majority_judgment(votes: &Votes) -> Option<Vec<usize>> {
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

    extract_maximum_map(&majority_mention_filtered)
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
