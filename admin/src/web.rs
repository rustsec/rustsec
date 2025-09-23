//! Code relating to the generation of the <https://rustsec.org> web site.

use std::{
    collections::{HashMap, HashSet},
    fs::{self, File},
    iter,
    ops::Deref,
    path::{Path, PathBuf},
    str::FromStr,
};

use crate::prelude::*;
use askama::Template;
use atom_syndication::{
    CategoryBuilder, ContentBuilder, Entry, EntryBuilder, FeedBuilder, FixedDateTime, LinkBuilder,
    PersonBuilder, Text,
};
use chrono::{Duration, NaiveDate, Utc};
use comrak::{ComrakOptions, markdown_to_html};
use rust_embed::RustEmbed;
use rustsec::advisory::Id;
use rustsec::osv::OsvAdvisory;
use rustsec::repository::git::GitModificationTimes;
use rustsec::repository::git::GitPath;
use rustsec::{Repository, advisory};
use xml::escape::escape_str_attribute;

// TODO(tarcieri): replace with `DateTime`
#[allow(deprecated)]
use chrono::Date;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

#[derive(Template)]
#[template(path = "search.html")]
struct SearchTemplate;

#[derive(Template)]
#[template(path = "static.html")]
struct StaticTemplate {
    title: String,
    content: String,
}

#[derive(Template)]
#[template(path = "advisories.html")]
struct AdvisoriesListTemplate<'a> {
    /// `Vec<(advisory data, rendered_title, advisory_title_type)>`
    advisories: Vec<(&'a AdvisoryData, String, String)>,
}

#[derive(Template)]
#[template(path = "advisories-sublist.html")]
struct AdvisoriesSubList<'a> {
    title: String,
    group_by: String,
    /// `Vec<(advisory data, rendered_title, advisory_title_type)>`
    advisories: Vec<(&'a AdvisoryData, String, String)>,
}

#[derive(Template)]
#[template(path = "advisory.html")]
struct AdvisoryTemplate<'a> {
    advisory: &'a AdvisoryData,
    rendered_description: String,
    rendered_title: String,
}

// Used for feed and included by `AdvisoryTemplate`
#[derive(Template)]
#[template(path = "advisory-content.html")]
struct AdvisoryContentTemplate<'a> {
    advisory: &'a AdvisoryData,
    rendered_description: String,
    rendered_title: String,
}

#[derive(Template)]
#[template(path = "sublist-index.html")]
struct ItemsList {
    title: String,
    /// `Vec<(name, url, option(count))>`
    items: Vec<(String, String, Option<usize>)>,
}

fn render_list_index(title: &str, mut items: Vec<(String, String, Option<usize>)>, folder: &Path) {
    items.sort_by(|a, b| a.0.to_lowercase().partial_cmp(&b.0.to_lowercase()).unwrap());
    let index_data = ItemsList {
        title: title.to_owned(),
        items,
    };
    let index_path = folder.join("index.html");
    fs::write(&index_path, index_data.render().unwrap()).unwrap();
    status_ok!("Rendered", "{}", index_path.display());
}

/// Render all advisories using the Markdown template
pub fn render_advisories(output_folder: PathBuf) {
    // Create dest
    fs::create_dir_all(&output_folder).unwrap();

    // Get static pages from repository
    let repo = Repository::fetch_default_repo().unwrap();
    let contributing_path = repo.path().join("CONTRIBUTING.md");

    // Get publication and latest modification dates
    let mod_times = GitModificationTimes::new(&repo).unwrap();

    // Get advisories
    let db = rustsec::Database::fetch().unwrap();
    let mut advisories = db
        .into_iter()
        .map(|a| AdvisoryData::new(a, &repo, &mod_times))
        .collect::<Vec<_>>();

    // Render static pages from repository
    let contributing_md = fs::read_to_string(contributing_path).unwrap();
    let static_template = StaticTemplate {
        title: "Reporting Vulnerabilities".to_string(),
        content: markdown_to_html(&contributing_md, &ComrakOptions::default()),
    };
    let contributing_page = static_template.render().unwrap();
    fs::write(output_folder.join("contributing.html"), contributing_page).unwrap();

    // Render individual advisory pages (/advisories/${id}.html)
    let advisories_folder = output_folder.join("advisories");
    fs::create_dir_all(&advisories_folder).unwrap();

    for data in &advisories {
        let output_path = advisories_folder.join(data.id().as_str().to_owned() + ".html");

        let rendered_description = markdown_to_html(data.description(), &ComrakOptions::default());
        let rendered_title = markdown_to_html(data.title(), &ComrakOptions::default());

        let advisory_tmpl = AdvisoryTemplate {
            advisory: data,
            rendered_description,
            rendered_title,
        };
        fs::write(&output_path, advisory_tmpl.render().unwrap()).unwrap();

        status_ok!("Rendered", "{}", output_path.display());
    }

    // Copy all the static assets.
    copy_static_assets(&output_folder);

    // Render the index.html (/) page.
    let index_template = IndexTemplate;
    let index_page = index_template.render().unwrap();
    fs::write(output_folder.join("index.html"), index_page).unwrap();

    // Render the search.html page.
    let search_page = SearchTemplate.render().unwrap();
    fs::write(output_folder.join("search.html"), search_page).unwrap();

    // Render the advisories.html (/advisories) page.

    // Sort the advisories by date in descending order for the big listing.
    #[allow(clippy::unnecessary_sort_by)]
    advisories.sort_by(|a, b| b.created.cmp(&a.created));

    let mut advisories_index = vec![];
    for data in &advisories {
        let rendered_title = markdown_to_html(data.title(), &ComrakOptions::default());
        let advisory_title_type = title_type(data);
        advisories_index.push((data, rendered_title, advisory_title_type));
    }

    let advisories_page_tmpl = AdvisoriesListTemplate {
        advisories: advisories_index,
    };
    let advisories_page = advisories_page_tmpl.render().unwrap();
    fs::write(advisories_folder.join("index.html"), advisories_page).unwrap();

    status_ok!(
        "Completed",
        "{} advisories rendered as HTML",
        advisories.len()
    );

    // Render the per-package pages (/packages/${package}.html).
    let mut advisories_per_package = Vec::<AdvisoriesSubList>::new();
    let mut packages = Vec::<(String, String, Option<usize>)>::new();
    for data in &advisories {
        let rendered_title = markdown_to_html(data.title(), &ComrakOptions::default());
        let advisory_title_type = title_type(data);
        let package = data.metadata.package.to_string();

        match packages.iter_mut().find(|(n, _, _)| *n == package) {
            Some(p) => p.2 = Some(p.2.unwrap() + 1),
            None => packages.push((
                package.clone(),
                format!("/packages/{}.html", package.clone()),
                Some(1),
            )),
        }
        match advisories_per_package
            .iter_mut()
            .find(|advisories| advisories.group_by == data.metadata.package.to_string())
        {
            Some(advisories) => {
                advisories
                    .advisories
                    .push((data, rendered_title, advisory_title_type))
            }
            None => advisories_per_package.push(AdvisoriesSubList {
                title: format!("Advisories for package '{}'", data.metadata.package),
                group_by: data.metadata.package.to_string(),
                advisories: vec![(data, rendered_title, advisory_title_type)],
            }),
        }
    }
    let folder = output_folder.join("packages");
    fs::create_dir_all(&folder).unwrap();
    // index
    render_list_index("Packages", packages, folder.as_ref());
    // per package page
    for tpl in &advisories_per_package {
        // Write HTML
        let output_path = folder.join(tpl.group_by.clone() + ".html");
        fs::write(&output_path, tpl.render().unwrap()).unwrap();
        status_ok!("Rendered", "{}", output_path.display());

        // Write OSV JSON
        let osv_path = folder.join(tpl.group_by.clone() + ".json");
        let mut osv_advisories = Vec::new();
        for (data, _, _) in &tpl.advisories {
            let git_path = GitPath::new(&repo, &data.path).unwrap();
            osv_advisories.push(OsvAdvisory::from_rustsec(
                data.advisory.clone(),
                &mod_times,
                git_path,
            ));
        }
        let json = serde_json::to_string_pretty(&osv_advisories).unwrap();
        fs::write(&osv_path, json).unwrap();
        status_ok!("Rendered", "{}", osv_path.display());
    }
    status_ok!(
        "Completed",
        "{} packages rendered as HTML",
        advisories_per_package.len()
    );

    // Render the per-keyword pages (/keywords/${keyword}.html).
    let mut advisories_per_keyword = Vec::<AdvisoriesSubList>::new();
    let mut keywords = Vec::<(String, String, Option<usize>)>::new();
    for data in &advisories {
        let rendered_title = markdown_to_html(data.title(), &ComrakOptions::default());
        let advisory_title_type = title_type(data);

        // merge keywords with the same slug
        let mut slug_keywords = data
            .metadata
            .keywords
            .as_slice()
            .iter()
            .map(|k| filters::safe_keyword(k.as_str(), askama::NO_VALUES).unwrap())
            .collect::<Vec<String>>();
        slug_keywords.sort();
        slug_keywords.dedup();

        for keyword in slug_keywords {
            if !keywords.iter().any(|(n, _, _)| *n == keyword) {
                keywords.push((
                    keyword.clone(),
                    format!("/keywords/{}.html", keyword.clone()),
                    None,
                ));
            }

            match advisories_per_keyword
                .iter_mut()
                .find(|advisories| advisories.group_by == keyword.as_str())
            {
                Some(advisories) => advisories.advisories.push((
                    data,
                    rendered_title.clone(),
                    advisory_title_type.clone(),
                )),
                None => advisories_per_keyword.push(AdvisoriesSubList {
                    title: format!("Advisories with keyword '{}'", keyword.as_str()),
                    group_by: keyword.as_str().to_string(),
                    advisories: vec![(data, rendered_title.clone(), advisory_title_type.clone())],
                }),
            }
        }
    }
    let folder = output_folder.join("keywords");
    fs::create_dir_all(&folder).unwrap();
    render_list_index("Keywords", keywords, folder.as_ref());
    for tpl in &advisories_per_keyword {
        let output_path = folder.join(tpl.group_by.clone() + ".html");
        fs::write(&output_path, tpl.render().unwrap()).unwrap();
        status_ok!("Rendered", "{}", output_path.display());
    }
    status_ok!(
        "Completed",
        "{} packages rendered as HTML",
        advisories_per_keyword.len()
    );

    // Render the per-category pages (/categories/${category}.html).
    let mut advisories_per_category = Vec::<AdvisoriesSubList>::new();
    let mut categories = Vec::<(String, String, Option<usize>)>::new();
    for data in &advisories {
        let rendered_title = markdown_to_html(data.title(), &ComrakOptions::default());
        let advisory_title_type = title_type(data);

        for category in data.metadata.categories.as_slice() {
            if !categories.iter().any(|(n, _, _)| n == category.name()) {
                categories.push((
                    category.name().to_owned(),
                    format!("/categories/{}.html", category.name()),
                    None,
                ));
            }

            match advisories_per_category
                .iter_mut()
                .find(|advisories| advisories.group_by == category.name())
            {
                Some(advisories) => advisories.advisories.push((
                    data,
                    rendered_title.clone(),
                    advisory_title_type.clone(),
                )),
                None => advisories_per_category.push(AdvisoriesSubList {
                    title: format!("Advisories in category '{}'", category.name()),
                    group_by: category.name().to_string(),
                    advisories: vec![(data, rendered_title.clone(), advisory_title_type.clone())],
                }),
            }
        }
    }
    let folder = output_folder.join("categories");
    fs::create_dir_all(&folder).unwrap();

    // index
    render_list_index("Categories", categories, folder.as_ref());
    // par value page
    for tpl in &advisories_per_category {
        let output_path = folder.join(tpl.group_by.clone() + ".html");
        fs::write(&output_path, tpl.render().unwrap()).unwrap();
        status_ok!("Rendered", "{}", output_path.display());
    }
    status_ok!(
        "Completed",
        "{} packages rendered as HTML",
        advisories_per_category.len() + 1
    );

    // Index
    let index_path = output_folder.join("js").join("index.js");
    render_index(&index_path, &advisories);
    status_ok!("Rendered", "{}", index_path.display());
    status_ok!(
        "Completed",
        "{} advisories rendered in search index as JS",
        advisories.len()
    );

    // Feed
    let feed_path = output_folder.join("feed.xml");
    let min_feed_len = 10;

    // TODO(tarcieri): replace with `DateTime`
    #[allow(deprecated)]
    let last_week_len = advisories
        .iter()
        .take_while(|data| {
            Date::from_utc(
                NaiveDate::parse_from_str(data.created.as_str(), "%Y-%m-%d").unwrap(),
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
    use rustsec::advisory::Informational;

    let id = advisory.id().as_str();
    let package = advisory.metadata.package.as_str();

    match &advisory.metadata.informational {
        Some(Informational::Notice) => format!("{id}: Security notice about {package}"),
        Some(Informational::Unmaintained) => format!("{id}: {package} is unmaintained"),
        Some(Informational::Unsound) => format!("{id}: Unsoundness in {package}"),
        Some(Informational::Other(s)) => format!("{id}: {package} is {s}"),
        Some(_) => format!("{id}: Advisory for {package}"),
        // Not informational => vulnerability
        None => format!("{id}: Vulnerability in {package}"),
    }
}

/// Renders the local search index
fn render_index(output_path: &Path, advisories: &[AdvisoryData]) {
    // Map of `lowercase(ID) -> related IDs` (including self, avoid redirecting to non-existent IDs)
    let mut ids: HashMap<String, Vec<Id>> = HashMap::new();
    // List of packages
    let mut packages = HashSet::new();

    for data in advisories {
        let id = data.id().to_owned();
        for alias in data
            .metadata
            .aliases
            .iter()
            .chain(data.metadata.related.iter())
            .chain(iter::once(&id))
        {
            // allows case-insensitive access
            let alias = alias.to_string().to_lowercase();
            ids.entry(alias)
                .and_modify(|v| v.push(id.clone()))
                .or_insert_with(|| vec![id.clone()]);
        }
        packages.insert(data.metadata.package.to_string().to_lowercase());
    }
    let ids_json = serde_json::to_string(&ids).unwrap();
    let package_json = serde_json::to_string(&packages).unwrap();

    let js = format!("var ids = {ids_json}\nvar packages = {package_json}\n");
    fs::write(output_path, js).unwrap();
}

/// Renders an Atom feed of advisories
fn render_feed(output_path: &Path, advisories: &[AdvisoryData]) {
    let mut entries: Vec<Entry> = vec![];
    let author = PersonBuilder::default().name("RustSec").build();

    // Used as latest update to feed
    let latest_advisory_date =
        advisories.first().unwrap().created.as_str().to_owned() + "T12:00:00+00:00";

    for data in advisories {
        let escaped_title_type = escape_str_attribute(&title_type(data)).into_owned();
        let escaped_title = escape_str_attribute(data.title()).into_owned();
        let cdate_time = data.created.as_str().to_owned() + "T12:00:00+00:00";
        let mdate_time = data.modified.as_str().to_owned() + "T12:00:00+00:00";
        let url = "https://rustsec.org/advisories/".to_owned() + data.id().as_str() + ".html";

        let link = LinkBuilder::default()
            .rel("alternate")
            .mime_type(Some("text/html".to_owned()))
            .title(escaped_title_type.clone())
            .href(url.clone())
            .build();

        let mut categories = vec![];
        for category in &data.metadata.categories {
            categories.push(
                CategoryBuilder::default()
                    .term(category.to_string())
                    .build(),
            );
        }

        let rendered_description = markdown_to_html(data.description(), &ComrakOptions::default());
        let rendered_title = markdown_to_html(data.title(), &ComrakOptions::default());

        let advisory_tmpl = AdvisoryContentTemplate {
            advisory: data,
            rendered_description,
            rendered_title,
        };
        let html = advisory_tmpl.render().unwrap();
        let content = ContentBuilder::default()
            .content_type(Some("html".to_owned()))
            .lang("en".to_owned())
            .value(Some(html))
            .build();

        let mut summary = Text::plain(escaped_title);
        summary.lang = Some("en".to_owned());

        let item = EntryBuilder::default()
            .id(url)
            .title(escaped_title_type)
            .summary(Some(summary))
            .links(vec![link])
            .categories(categories)
            .published(Some(FixedDateTime::from_str(&cdate_time).unwrap()))
            // required but we don't have precise data here
            .updated(FixedDateTime::from_str(&mdate_time).unwrap())
            .content(Some(content))
            .build();
        entries.push(item);
    }

    let self_url = "https://rustsec.org/feed.xml";
    let alternate_link = LinkBuilder::default()
        .href("https://rustsec.org/")
        .rel("alternate")
        .mime_type(Some("text/html".to_owned()))
        .build();
    let self_link = LinkBuilder::default()
        .href(self_url)
        .rel("self")
        .mime_type(Some("application/atom+xml".to_owned()))
        .build();

    let mut subtitle = Text::plain("Security advisories filed against Rust crates".to_owned());
    subtitle.lang = Some("en".to_owned());

    let feed = FeedBuilder::default()
        .id(self_url)
        .title("RustSec Advisories")
        .subtitle(Some(subtitle))
        .links(vec![self_link, alternate_link])
        .icon("https://rustsec.org/favicon.ico".to_owned())
        .entries(entries)
        .updated(FixedDateTime::from_str(&latest_advisory_date).unwrap())
        .authors(vec![author])
        .build();

    let file = File::create(output_path).unwrap();
    feed.write_to(file).unwrap();
}

#[derive(RustEmbed)]
#[folder = "src/web/static/"]
struct StaticAsset;

fn copy_static_assets(output_folder: &Path) {
    for file in StaticAsset::iter() {
        let asset_path = PathBuf::from(file.as_ref());

        // If the asset is in a folder, e.g. css/. Make the directory first.
        if let Some(containing_folder) = asset_path.parent() {
            fs::create_dir_all(output_folder.join(containing_folder)).unwrap();
        }

        let asset = StaticAsset::get(file.as_ref()).unwrap();
        fs::write(output_folder.join(file.as_ref()), asset.data).unwrap();
    }
}

mod filters {
    use chrono::NaiveDate;
    use rustsec::advisory;
    use std::borrow::Borrow;

    pub fn friendly_date<T: Borrow<advisory::Date>>(
        date: T,
        _: &dyn askama::Values,
    ) -> ::askama::Result<String> {
        let date = date.borrow();

        // TODO(tarcieri): fix deprecation of `NaiveDate::from_ymd`
        #[allow(deprecated)]
        let date = NaiveDate::from_ymd(date.year().try_into().unwrap(), date.month(), date.day())
            .format("%B %e, %Y")
            .to_string();

        Ok(date)
    }

    pub fn safe_keyword(s: &str, _: &dyn askama::Values) -> ::askama::Result<String> {
        Ok(s.chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                    c
                } else {
                    '-'
                }
            })
            .collect())
    }
}

struct AdvisoryData {
    advisory: rustsec::Advisory,
    path: PathBuf,
    created: advisory::Date,
    modified: advisory::Date,
}

impl AdvisoryData {
    fn new(
        advisory: rustsec::Advisory,
        repo: &Repository,
        mod_times: &GitModificationTimes,
    ) -> Self {
        let path = PathBuf::from(format!(
            "{}/{}/{}.md",
            advisory.metadata.collection.unwrap(),
            advisory.metadata.package,
            advisory.id()
        ));

        let git_path = GitPath::new(repo, &path).unwrap();
        Self {
            advisory,
            created: mod_times.cdate_for_path(git_path),
            modified: mod_times.mdate_for_path(git_path),
            path,
        }
    }
}

impl Deref for AdvisoryData {
    type Target = rustsec::Advisory;

    fn deref(&self) -> &Self::Target {
        &self.advisory
    }
}
