//! Code relating to the generation of the https://rustsec.org web site.
//!
use crate::prelude::*;
use std::str::FromStr;
use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

use askama::Template;
use atom_syndication::{
    CategoryBuilder, ContentBuilder, Entry, EntryBuilder, FeedBuilder, FixedDateTime, LinkBuilder,
    PersonBuilder,
};
use chrono::{Date, Duration, NaiveDate, Utc};
use comrak::{markdown_to_html, ComrakOptions};
use rust_embed::RustEmbed;
use xml::escape::escape_str_attribute;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

#[derive(Template)]
#[template(path = "advisories.html")]
struct AdvisoriesListTemplate {
    advisories_per_year: Vec<AdvisoriesPerYear>,
}

struct AdvisoriesPerYear {
    year: u32,
    /// `Vec<(advisory, rendered_title, advisory_title_type)>`
    advisories: Vec<(rustsec::Advisory, String, String)>,
}

#[derive(Template)]
#[template(path = "advisory.html")]
struct AdvisoryTemplate<'a> {
    advisory: &'a rustsec::Advisory,
    rendered_description: String,
    rendered_title: String,
}

// Used for feed and included by `AdvisoryTemplate`
#[derive(Template)]
#[template(path = "advisory-content.html")]
struct AdvisoryContentTemplate<'a> {
    advisory: &'a rustsec::Advisory,
    rendered_description: String,
    rendered_title: String,
}

/// Render all advisories using the Markdown template
pub fn render_advisories(output_folder: PathBuf) {
    let mut advisories: Vec<rustsec::Advisory> = rustsec::Database::fetch()
        .unwrap()
        .iter()
        .map(|advisory| advisory.to_owned())
        .collect();

    // Render individual advisory pages.
    let advisories_folder = output_folder.join("advisories");
    fs::create_dir_all(&advisories_folder).unwrap();

    for advisory in &advisories {
        let output_path = advisories_folder.join(advisory.id().as_str().to_owned() + ".html");

        let rendered_description =
            markdown_to_html(advisory.description(), &ComrakOptions::default());
        let rendered_title = markdown_to_html(advisory.title(), &ComrakOptions::default());

        let advisory_tmpl = AdvisoryTemplate {
            advisory,
            rendered_description,
            rendered_title,
        };
        fs::write(&output_path, advisory_tmpl.render().unwrap()).unwrap();

        status_ok!("Rendered", "{}", output_path.display());
    }

    // Copy all the static assets.
    copy_static_assets(&output_folder);

    // Render the index.html (/) page.
    let index_page = IndexTemplate.render().unwrap();
    fs::write(output_folder.join("index.html"), index_page).unwrap();

    // Render the advisories.html (/advisories) page.

    // Sort the advisories by date in descending order for the big listing.
    advisories.sort_by(|a, b| b.date().cmp(&a.date()));

    let mut advisories_per_year = Vec::<AdvisoriesPerYear>::new();
    for advisory in advisories.clone() {
        let rendered_title = markdown_to_html(advisory.title(), &ComrakOptions::default());
        let advisory_title_type = title_type(&advisory);

        // If we have an AdvisoriesPerYear for this advisory's year, just
        // push it into that list. Otherwise, set one up for the year.
        match advisories_per_year
            .iter_mut()
            .find(|per_year| per_year.year == advisory.date().year())
        {
            Some(advisories_for_year) => {
                advisories_for_year
                    .advisories
                    .push((advisory, rendered_title, advisory_title_type))
            }
            None => advisories_per_year.push(AdvisoriesPerYear {
                year: advisory.date().year(),
                advisories: vec![(advisory, rendered_title, advisory_title_type)],
            }),
        }
    }

    let advisories_page_tmpl = AdvisoriesListTemplate {
        advisories_per_year,
    };
    let advisories_page = advisories_page_tmpl.render().unwrap();
    fs::write(advisories_folder.join("index.html"), advisories_page).unwrap();

    status_ok!(
        "Completed",
        "{} advisories rendered as HTML",
        advisories.len()
    );

    // Feed
    let feed_path = output_folder.join("feed.xml");
    let min_feed_len = 10;
    let last_week_len = advisories
        .iter()
        .take_while(|a| {
            Date::from_utc(
                NaiveDate::parse_from_str(a.date().as_str(), "%Y-%m-%d").unwrap(),
                Utc,
            ) > Utc::today() - Duration::days(8)
        })
        .count();

    // include max(latest week of advisories, 10 latest advisories)
    // the goal is not to miss a vulnerability in case of burst
    // and to never have an empty feed.
    let len = if advisories.len() < min_feed_len {
        advisories.len()
    } else if last_week_len > min_feed_len {
        last_week_len
    } else {
        min_feed_len
    };
    render_feed(&feed_path, &advisories[..len]);
    status_ok!("Rendered", "{}", feed_path.display());
    status_ok!("Completed", "{} advisories rendered in atom feed", len);
}

/// Title with the id, the package name and the advisory type
fn title_type(advisory: &rustsec::Advisory) -> String {
    use rustsec::advisory::informational::Informational;

    let id = advisory.id().as_str();
    let package = advisory.metadata.package.as_str();

    match &advisory.metadata.informational {
        Some(Informational::Notice) => format!("{}: Security notice about {}", id, package),
        Some(Informational::Unmaintained) => format!("{}: {} is unmaintained", id, package),
        Some(Informational::Unsound) => format!("{}: Unsoundness in {}", id, package),
        Some(Informational::Other(s)) => format!("{}: {} is {}", id, package, s),
        Some(_) => format!("{}: Advisory for {}", id, package),
        // Not informational => vulnerability
        None => format!("{}: Vulnerability in {}", id, package),
    }
}

/// Renders an Atom feed of advisories
fn render_feed(output_path: &Path, advisories: &[rustsec::Advisory]) {
    let mut entries: Vec<Entry> = vec![];
    let author = PersonBuilder::default().name("RustSec").build().unwrap();

    // Used as latest update to feed
    let latest_advisory_date =
        advisories.first().unwrap().date().as_str().to_owned() + "T00:00:00+00:00";

    for advisory in advisories {
        let escaped_title_type = escape_str_attribute(&title_type(advisory)).into_owned();
        let escaped_title = escape_str_attribute(advisory.title()).into_owned();
        let date_time = advisory.date().as_str().to_owned() + "T00:00:00+00:00";
        let url = "https://rustsec.org/advisories/".to_owned() + advisory.id().as_str() + ".html";

        let link = LinkBuilder::default()
            .rel("alternate")
            .mime_type(Some("text/html".to_owned()))
            .title(escaped_title_type.clone())
            .href(url.clone())
            .build()
            .unwrap();

        let mut categories = vec![];
        for category in &advisory.metadata.categories {
            categories.push(
                CategoryBuilder::default()
                    .term(category.to_string())
                    .build()
                    .unwrap(),
            );
        }

        let rendered_description =
            markdown_to_html(advisory.description(), &ComrakOptions::default());
        let rendered_title = markdown_to_html(advisory.title(), &ComrakOptions::default());
        let advisory_tmpl = AdvisoryContentTemplate {
            advisory,
            rendered_description,
            rendered_title,
        };
        let html = escape_str_attribute(&advisory_tmpl.render().unwrap()).into_owned();
        let content = ContentBuilder::default()
            .content_type(Some("html".to_owned()))
            .value(Some(html))
            .build()
            .unwrap();

        let item = EntryBuilder::default()
            .id(url)
            .title(escaped_title_type)
            .summary(Some(escaped_title))
            .links(vec![link])
            .categories(categories)
            .published(Some(FixedDateTime::from_str(&date_time).unwrap()))
            // required but we don't have precise data here
            .updated(FixedDateTime::from_str(&date_time).unwrap())
            .content(Some(content))
            .build()
            .unwrap();
        entries.push(item);
    }

    let self_url = "https://rustsec.org/feed.xml";
    let alternate_link = LinkBuilder::default()
        .href("https://rustsec.org/")
        .rel("alternate")
        .mime_type(Some("text/html".to_owned()))
        .build()
        .unwrap();
    let self_link = LinkBuilder::default()
        .href(self_url)
        .rel("self")
        .mime_type(Some("application/atom+xml".to_owned()))
        .build()
        .unwrap();

    let feed = FeedBuilder::default()
        .id(self_url)
        .title("RustSec Advisories")
        .subtitle(Some(
            "Security advisories filed against Rust crates".to_owned(),
        ))
        .links(vec![self_link, alternate_link])
        .icon("https://rustsec.org/favicon.ico".to_owned())
        .entries(entries)
        .updated(FixedDateTime::from_str(&latest_advisory_date).unwrap())
        .authors(vec![author])
        .build()
        .unwrap();

    let file = File::create(&output_path).unwrap();
    feed.write_to(file).unwrap();
}

#[derive(RustEmbed)]
#[folder = "src/web/static/"]
struct StaticAsset;

fn copy_static_assets(output_folder: &Path) {
    for file in StaticAsset::iter() {
        let asset_path = PathBuf::from(file.as_ref());

        // If the asset is in a folder, e.g css/. Make the directory first.
        if let Some(containing_folder) = asset_path.parent() {
            fs::create_dir_all(output_folder.join(containing_folder)).unwrap();
        }

        let asset_contents = StaticAsset::get(file.as_ref()).unwrap();
        fs::write(output_folder.join(file.as_ref()), asset_contents).unwrap();
    }
}

mod filters {
    use chrono::NaiveDate;
    use std::convert::TryInto;

    pub fn friendly_date(date: &&rustsec::advisory::Date) -> ::askama::Result<String> {
        Ok(
            NaiveDate::from_ymd(date.year().try_into().unwrap(), date.month(), date.day())
                .format("%B %e, %Y")
                .to_string(),
        )
    }
}
