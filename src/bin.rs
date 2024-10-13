use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// search for offsets relative to a class/struct
    Member {
        /// path to the PDB
        pdb_path: String,
        /// name of the struct/class that contains the member, can be a regex
        symbol_name: String,
        /// name of the member within the struct/class, can be a regex
        member_name: String,
    },
    /// search for offsets of each member relative to a class/struct
    Struct {
        /// path to the PDB
        pdb_path: String,
        /// name of the struct/class
        symbol_name: String,
    },
    /// search for offsets relative to the base address of a given DLL or EXE
    Data {
        /// path to the PDB
        pdb_path: String,
        /// name of the symbol that holds data e.g. a global variable
        symbol_name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Member {
            pdb_path,
            symbol_name,
            member_name,
        }) => {
            if let Ok(offsets) = offsetter::get_member_offsets(pdb_path, symbol_name, member_name) {
                if let Ok(json) = serde_json::to_string(&offsets) {
                    println!("{}", json);
                }
            }
        }
        Some(Commands::Struct {
            pdb_path,
            symbol_name,
        }) => {
            if let Ok(offsets) = offsetter::get_struct_offsets(pdb_path, symbol_name) {
                if let Ok(json) = serde_json::to_string(&offsets) {
                    println!("{}", json);
                }
            }
        }
        Some(Commands::Data {
            pdb_path,
            symbol_name,
        }) => {
            if let Ok(offsets) = offsetter::get_data_offsets(pdb_path, symbol_name) {
                if let Ok(json) = serde_json::to_string(&offsets) {
                    println!("{}", json);
                }
            }
        }
        None => println!("Unsupported!"),
    }
}
