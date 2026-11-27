use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None, disable_help_flag = true)]
pub struct Configuration {
	#[arg()]
	pub bib_path: String, 
	#[arg(short, long)]
	pub hg_path: Option<String>,
	#[arg(short, long)]
	pub topic: Vec<String>,
	#[arg(short, long)]
	pub relevance: Option<String>,
}