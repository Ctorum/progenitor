use clap::Args;

#[derive(Args)]
pub struct Create {
    #[arg(short, long)]
    pub template: String,

    #[arg(short, long)]
    pub name: String,
}

impl Create {
    pub fn execute(&self) {
        println!("Creating project {} from template {}", self.name, self.template);
    }
}
