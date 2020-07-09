use {
	markov::Chain,
	rand::{distributions::Standard, prelude::*},
	scraper::{Html, Selector},
	std::{
		borrow::Cow,
		collections::HashSet,
		fs::File,
		io::{stdin, BufRead, BufReader, BufWriter, Write},
	},
};

fn main() -> anyhow::Result<()> {
	let ikea_catalog: HashSet<String> = match download_catalog() {
		Ok(catalog) => catalog,
		Err(_) => {
			eprintln!("WARNING: Failed to refresh catalog");
			BufReader::new(File::open("./catalog.txt")?)
				.lines()
				.collect::<Result<_, _>>()?
		}
	};

	let mut chain: Chain<char> = Chain::of_order(5);
	ikea_catalog.iter().for_each(|w| {
		chain.feed(w.chars().collect::<Vec<_>>());
	});

	let mut generated = chain
		.iter()
		.filter(|g| g.len() > 3)
		.map(|g| g.into_iter().collect::<String>())
		.filter(|g| !ikea_catalog.contains(g));

	let mut not_generated = (0..).flat_map(|_| {
		let mut v = ikea_catalog.iter().map(|s| s.as_str()).collect::<Vec<_>>();
		v.shuffle(&mut rand::thread_rng());
		v
	});

	let any = (0..).map(|_| {
		let generate: bool = rand::thread_rng().sample(Standard);
		(
			generate,
			match generate {
				true => Cow::Owned(generated.next().unwrap()),
				false => Cow::Borrowed(not_generated.next().unwrap()),
			},
		)
	});

	let mut true_positives = 0;
	let mut false_positives = 0;
	let mut false_negatives = 0;
	let mut true_negatives = 0;
	let mut guess_buffer = String::new();
	for (generated, word) in any {
		let guess = loop {
			guess_buffer.clear();
			println!("Is {} an ikea item ?", word);
			stdin().read_line(&mut guess_buffer).expect("Failed to read line");
			guess_buffer.pop();
			match guess_buffer.as_str() {
				"y" => break true,
				"n" => break false,
				"" => {
					let n_wins = true_positives + true_negatives;
					let n_guesses = n_wins + false_negatives + false_positives;
					println!(
						"You got {}/{} ({}TP, {}FP, {}TN, {}FN)",
						n_wins, n_guesses, true_positives, false_positives, true_negatives, false_negatives
					);
					return Ok(());
				}
				_ => {
					println!("Invalid input");
				}
			}
		};
		match (generated, guess) {
			(true, true) => true_positives += 1,
			(true, false) => false_negatives += 1,
			(false, true) => false_positives += 1,
			(false, false) => true_negatives += 1,
		}
		println!(
			"YOU ARE {}!",
			match generated == guess {
				true => "CORRECT",
				false => "WRONG",
			}
		)
	}
	Ok(())
}

fn download_catalog() -> anyhow::Result<HashSet<String>> {
	let page: String = reqwest::blocking::get("https://lar5.com/ikea/")?.text()?;
	let doc = Html::parse_document(&page);
	let sel = Selector::parse(r#"div a[z=""]"#).unwrap();
	let ikea_catalog = doc.select(&sel).flat_map(|w| w.text());
	let mut file = BufWriter::new(File::create("catalog.txt")?);
	ikea_catalog
		.map(|w| -> anyhow::Result<_> {
			writeln!(file, "{}", w)?;
			Ok(w.to_owned())
		})
		.collect()
}
