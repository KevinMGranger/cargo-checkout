use ::structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "cargo pando", author = "")]
pub struct Opts {
    /// Check out the index of your repository.
    ///
    /// Mutually exclusive with --copy and --no-copy.
    #[structopt(short, long, conflicts_with = "copy", conflicts_with = "no_copy")]
    index: bool,

    /// Copy src and Cargo.{toml,lock} against each toolchain.
    ///
    /// Mutually exclusive with --index and --no-copy.
    #[structopt(short, long)]
    copy: bool,

    /// Don't copy any files, use the existing ones in target/pando.
    ///
    /// Mutually exclusive with --index and --copy.
    #[structopt(long)]
    no_copy: bool,

    /// Specify one or many toolchains to use. Reads from .travis.yml if unused.
    ///
    /// Mutually exclusive with --all.
    #[structopt(short, long, number_of_values = 1)]
    toolchain: Vec<String>,

    /// Use all installed toolchains except for the current default.
    ///
    /// Mutually exclusive with --toolchain.
    #[structopt(short, long, conflicts_with = "toolchain")]
    all: bool,

    #[structopt(subcommand)]
    action: ActionOpt,
}

#[derive(StructOpt, Debug)]
#[structopt(author = "")]
pub enum ActionOpt {
    #[structopt(name = "test")]
    /// Runs cargo test on each checkout, with the applicable toolchain.
    CargoTest {
        /// Install the proper toolchain if it's not already present.
        #[structopt(long)]
        install: bool,

        /// Max active tasks. Defaults to number of logical CPUs.
        #[structopt(short, long)]
        jobs: Option<usize>,

        /// Arguments passed along to cargo test.
        test_args: Vec<String>,
    },

    #[structopt(name = "build")]
    /// Runs cargo build on each checkout, with the applicable toolchain.
    CargoBuild {
        /// Install the proper toolchain if it's not already present.
        #[structopt(long)]
        install: bool,

        /// Max active tasks. Defaults to number of logical CPUs.
        #[structopt(short, long)]
        jobs: Option<usize>,

        /// Arguments passed along to cargo build.
        build_args: Vec<String>,
    },

    /// Any arbitrary cargo subcommand.
    #[structopt(name = "cargo")]
    CargoAny {
        /// Install the proper toolchain if it's not already present.
        #[structopt(long)]
        install: bool,

        /// Max active tasks. Defaults to number of logical CPUs.
        #[structopt(short, long)]
        jobs: Option<usize>,

        subcommand: String,
        /// Arguments passed along to cargo.
        args: Vec<String>,
    },

    /// Execute the given command once per checkout.
    ///
    /// The directory will be changed to the checkout dir.
    /// Any argument named ``{}`` will be replaced by the toolchain version.
    #[structopt(name = "each")]
    Each {
        /// Install the proper toolchain if it's not already present.
        #[structopt(long)]
        install: bool,

        /// Max active tasks. Defaults to number of logical CPUs.
        #[structopt(short, long)]
        jobs: Option<usize>,

        utility: String,
        args: Vec<String>,
    },

    /// Execute the given command once for _all_ checkouts.
    /// Will not switch to a specific checkout.
    ///
    /// Any argument named ``{}`` will be replaced by multiple arguments of each toolchain version.
    #[structopt(name = "all")]
    All { utility: String, args: Vec<String> },
}