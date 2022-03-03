use std::{fs::File, io::{BufRead, BufReader, BufWriter, Read, Write}, path::{Path, PathBuf}};
use pulldown_cmark::{Parser, Options, html};
use askama::Template;
use std::error::Error;
use chrono::offset::Local;
use chrono::prelude::*;
use regex::Regex;

#[derive(Eq, PartialEq, Clone, Copy, Debug, Ord)]
struct Date {
    day: u8,
    month: u8,
    year: usize
}

impl Date {
    fn from_str(date_str: &str) -> Result<Date, Box<dyn Error>> {
        let mut parts = date_str.split('-');

        let year = parts.next().unwrap().parse::<usize>()?;
        let month = parts.next().unwrap().parse::<u8>()?;
        let day = parts.next().unwrap().trim().parse::<u8>()?;

        Ok(Date {
            day,
            month,
            year
        })
    }

    fn to_string(&self) -> String {
        let mut output = String::with_capacity(18); // Max size of date in this format

        output.push_str(
            match self.month {
                1 => "January ",
                2 => "Feburary ",
                3 => "March ",
                4 => "April ",
                5 => "May ",
                6 => "June ",
                7 => "July ",
                8 => "August ",
                9 => "September ",
                10 => "October ",
                11 => "November ",
                12 => "December ",
                _ => ""
        });

        output.push_str(self.day.to_string().as_str());
        output.push_str(", ");

        output.push_str(self.year.to_string().as_str());

        output
    }

    fn to_rfc2822(&self) -> String {
        let date = Local.ymd(
            self.year as i32,
            self.month as u32,
            self.day as u32
        ).and_hms(0, 0, 0);

        date.to_rfc2822()
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Date) -> Option<std::cmp::Ordering> {
        if self.year > other.year {
            return Some(std::cmp::Ordering::Greater);
        } else if self.year < other.year {
            return Some(std::cmp::Ordering::Less);
        }

        if self.month > other.month {
            return Some(std::cmp::Ordering::Greater);

        } else if self.month < other.month {
            return Some(std::cmp::Ordering::Less);

        }

        if self.day > other.day {
            return Some(std::cmp::Ordering::Greater);

        } else if self.day < other.day {
            return Some(std::cmp::Ordering::Less);
        }

        Some(std::cmp::Ordering::Equal)
    }
}

#[derive(Template, Clone, Debug)]
#[template(path="blog.html")]
struct BlogTemplate<'a> {
    title: &'a str,
    content: &'a str,
    latest: bool
}

#[derive(Template)]
#[template(path="home.html")]
struct HomeTemplate {
    entries: Vec<(String, String, String)>
}

#[derive(Template)]
#[template(path="archive.html")]
struct ArchiveTemplate {
    entries: Vec<(String, String, String)>
}

#[derive(Debug, Clone)]
struct BlogEntry {
    title: String,
    path: PathBuf,
    slug: String,
    date: Date,
    content: String
}

#[derive(Template)]
#[template(path="rss.xml")]
struct RSSTemplate {
    today: String,
    entries: Vec<(String, String, String)>
}

fn main() {
    // Directories to read and write from
    let md_path = Path::new("./md");
    let blog_path = Path::new("./public/html");
    let article_path = Path::new("./public/html/articles");
    let rss_path = Path::new("./public/rss");

    // So we begin by reading every markdown file in input,
    // converting it to html and then writing it to output
    let parser_options = Options::all();
    let mut md_file_iter = md_path.read_dir().unwrap();

    let mut input_buf = String::with_capacity(1000);
    let mut replace_buf = String::with_capacity(1000);
    let mut output_buf = String::with_capacity(1000);

    let inline = Regex::new(r"\$(.*)\$").unwrap();
    let single = Regex::new(r"\\\[(.*)\\\]").unwrap();

    // For each markdown file in md, convert to html file in articles using
    // the markdown parser and the template engine. Additionally, store metadata
    // Vector for the webserver
    let mut entries: Vec<BlogEntry> = Vec::new();
    while let Some(Ok(file)) = md_file_iter.next() {
        let mut path = file.path();
        if path.is_file() && path.extension().is_some() && path.extension().unwrap()=="md" {
            // We now have a proper markdown file to convert
            // Read the file to a string buffer
            let mut reader = BufReader::new(File::open(path.clone()).unwrap());

            // First line contains metadata and important information
            let mut metadata_buffer = String::with_capacity(100);
            reader.read_line(&mut metadata_buffer).unwrap();

            // Parse the metadata
            let mut data = metadata_buffer.split("|");

            let title = data.next().unwrap();
            let slug = data.next().unwrap();
            let date = data.next().unwrap();

            // Read the file
            reader.read_to_string(&mut input_buf).unwrap();
            replace_buf = input_buf.clone();

            // Replace all latex math strings with katex
            let inline_captures: Vec<_> = inline.captures_iter(&input_buf).collect();
            let single_captures: Vec<_> = single.captures_iter(&input_buf).collect();

            for cap in inline_captures {
                let math = katex::render(&cap[1]).unwrap();
                replace_buf = replace_buf.replace(&cap[0], &math);
            }

            let opts = katex::Opts::builder()
                .display_mode(true)
                .build()
                .unwrap();

            for cap in single_captures {
                let math = katex::render_with_opts(&cap[1], &opts).unwrap();
                replace_buf = replace_buf.replace(&cap[0], &math);
            }

            // parse the markdown
            let parser = Parser::new_ext(&replace_buf, parser_options);
            html::push_html(&mut output_buf, parser);

            // Add content to entry
            path.set_extension("html");

            let output_path = article_path.join(path.file_name().unwrap());

            // Add to entries list
            entries.push(BlogEntry{
                path: output_path,
                title: title.to_string(),
                slug: slug.to_string(),
                date: Date::from_str(date).unwrap(),
                content: output_buf.clone()
            });

            // Clear buffers
            input_buf.clear();
            replace_buf.clear();
            output_buf.clear();
        }
    }
    entries.sort_unstable_by_key(|a| a.date);
    entries.reverse();

    for (i, entry) in entries.iter().enumerate() {
        println!("Writing {}", entry.path.file_name().unwrap().to_str().unwrap());

        let template = BlogTemplate {
            title: entry.title.as_str(),
            content: entry.content.as_str(),
            latest: i==0
        };

        let mut writer = BufWriter::new(File::create(&entry.path).unwrap());
        writer.write_all(template.render().unwrap().as_bytes()).unwrap();
    }


    // Now render the template for the home page with the latest articles
    let last_7 = entries.iter().take(7)
        .map(|e| (e.title.clone(), e.slug.clone(), e.date.to_string()))
        .collect::<Vec<(String, String, String)>>();

    let home_template = HomeTemplate {
        entries: last_7
    };

    println!("Writing home.html");
    let p = blog_path.join("home.html");
    let mut writer = BufWriter::new(File::create(p).unwrap());
    writer.write_all(home_template.render().unwrap().as_bytes()).unwrap();
    writer.flush().unwrap();

    // Render the archive template
    let entry_tuples = entries.iter()
        .map(|e| (e.title.clone(), e.slug.clone(), e.date.to_string()))
        .collect::<Vec<(String, String, String)>>();
    let archive_template = ArchiveTemplate {
        entries: entry_tuples
    };

    println!("Writing archive.html");
    let p = blog_path.join("archive.html");
    let mut writer = BufWriter::new(File::create(p).unwrap());
    writer.write_all(archive_template.render().unwrap().as_bytes()).unwrap();
    writer.flush().unwrap();

    println!("Writing latest.html");
    let latest = &entries[0];
    let p = blog_path.join("latest.html");
    std::fs::copy(latest.path.clone(), p).unwrap();

    println!("Writing rss.xml");
    let today = Local::now();
    let rss_entries =  entries.iter().map(|e| {
        (e.title.clone(), e.slug.clone(), e.date.to_rfc2822())
    }).collect();
    let rss_template = RSSTemplate {
        today: today.to_rfc2822(),
        entries: rss_entries
    };
    let p = rss_path.join("rss.xml");
    let mut writer = BufWriter::new(File::create(p).unwrap());
    writer.write_all(rss_template.render().unwrap().as_bytes()).unwrap();
}
