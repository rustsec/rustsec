//! Generates a Markdown-formatted table of all platforms known to this crate

use platforms::{Platform, Tier};

/// Max width for target triples
const TARGET_TRIPLE_WIDTH: usize = 36;

/// Max width for `target_arch`
const TARGET_ARCH_WIDTH: usize = 11;

/// Max width for `target_os`
const TARGET_OS_WIDTH: usize = 10;

/// Max width for `target_env`
const TARGET_ENV_WIDTH: usize = 10;

fn main() {
    // TODO: double check the others are aligned
    assert_eq!(
        Platform::ALL
            .iter()
            .map(|p| p.target_triple.len())
            .max()
            .unwrap(),
        TARGET_TRIPLE_WIDTH
    );

    assert_eq!(
        Platform::ALL
            .iter()
            .map(|p| p.target_os.as_str().len())
            .max()
            .unwrap(),
        TARGET_OS_WIDTH
    );

    print_platforms_table();
}

/// Print the table of known platforms
fn print_platforms_table() {
    let mut current_tier: Option<Tier> = None;

    let mut platforms = Platform::ALL.to_owned();
    platforms.sort_by_key(|p| p.tier);
    for platform in &platforms {
        // Print headers if we're on a different tier from before
        if current_tier != Some(platform.tier) {
            if current_tier.is_some() {
                println!();
            }

            print_table_header(platform.tier);
            current_tier = Some(platform.tier);
        }

        print_platform_entry(platform);
    }

    // Print markdown links for lowest tier
    print_platform_links();
}

/// Print the table headers for a particular tier
fn print_table_header(tier: Tier) {
    println!("### Tier {}\n", tier.to_usize());

    println!(
        "| target triple                          | target_arch | target_os  | target_env |\n\
         |----------------------------------------|-------------|------------|------------|"
    );
}

/// Print an entry in a table
fn print_platform_entry(platform: &Platform) {
    print!(
        "| {:width$} | ",
        "[".to_owned() + platform.target_triple + "]",
        width = TARGET_TRIPLE_WIDTH + 2 // add extra width for link brackets
    );

    print!(
        "{:width$} | ",
        platform.target_arch.as_str(),
        width = TARGET_ARCH_WIDTH
    );

    print!(
        "{:width$} | ",
        platform.target_os.as_str(),
        width = TARGET_OS_WIDTH
    );

    let target_env = platform.target_env.as_str();

    println!("{:width$} |", target_env, width = TARGET_ENV_WIDTH);
}

/// Print Markdown links to specific platform struct definitions on docs.rs
fn print_platform_links() {
    println!();

    for platform in Platform::ALL {
        println!(
            "[{}]: https://docs.rs/platforms/latest/platforms/platform/constant.{}.html",
            platform.target_triple,
            platform.target_triple.to_uppercase().replace('-', "_")
        );
    }
}
