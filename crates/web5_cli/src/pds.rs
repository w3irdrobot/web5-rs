use clap::Subcommand;
use web5::credentials::presentation_definition::PresentationDefinition;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Creates a Presentation Definition
    Create {
        /// A unique identifier that distinguishes this PD from others
        id: String,
        /// A human-friendly name for easier identification of the PD
        #[arg(long, short)]
        name: Option<String>,
        /// A description outlining why the information requested by the PD is needed
        #[arg(long, short)]
        purpose: Option<String>,
    },
}

impl Commands {
    pub async fn command(&self) {
        match self {
            Commands::Create { id, name, purpose } => {
                let pd = PresentationDefinition {
                    // Note: these clones could be removed if the method took ownership
                    id: id.clone(),
                    name: name.clone(),
                    purpose: purpose.clone(),
                    input_descriptors: Vec::default(),
                    submission_requirements: None,
                };

                println!("{}", serde_json::to_string(&pd).unwrap())
            }
        };
    }
}
