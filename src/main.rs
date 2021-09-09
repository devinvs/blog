use std::{fs::File, fs::read_to_string, io::{BufRead, BufReader, BufWriter, Read, Write}, path::{Path, PathBuf}, str::FromStr};
use pulldown_cmark::{Parser, Options, html};
use askama::Template;
use chrono::NaiveDate;
use actix_web::{get, web, App, HttpServer};
use actix_files::{NamedFile, Files};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};


#[derive(Template)]
#[template(path="blog.html")]
struct BlogTemplate<'a> {
    title: &'a str,
    content: &'a str
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
    date: NaiveDate
}

struct AppState {
    entries: Vec<BlogEntry>
}

// Lets create some routes!
#[get("/")]
async fn index() -> NamedFile {
    NamedFile::open("./public/html/home.html").unwrap()
}

#[get("/about")]
async fn about() -> NamedFile {
    NamedFile::open("./public/html/about.html").unwrap()
}

#[get("/latest")]
async fn latest(data: web::Data<AppState>) -> NamedFile {
    let latest = &data.entries[0].path;
    NamedFile::open(latest).unwrap()
}

#[get("/archive")]
async fn archive() -> NamedFile {
    NamedFile::open("./public/html/archive.html").unwrap()
}

#[get("/post/{slug}")]
async fn get_post(path: web::Path<String>, data: web::Data<AppState>) -> NamedFile {
    let slug = path.0;
    let path = &data.entries.iter().find(|b| b.slug == slug).unwrap().path;

    NamedFile::open(path).unwrap()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Directories to read and write from
    let md_path = Path::new("./md");
    let blog_path = Path::new("./public/html");
    let article_path = Path::new("./public/html/articles");

    // So we begin by reading every markdown file in input,
    // converting it to html and then writing it to output
    let parser_options = Options::all();
    let mut md_file_iter = md_path.read_dir().unwrap();

    let mut input_buf = String::with_capacity(1000);
    let mut output_buf = String::with_capacity(1000);

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

            // Parse the markdown
            reader.read_to_string(&mut input_buf).unwrap();
            let parser = Parser::new_ext(&input_buf, parser_options);
            html::push_html(&mut output_buf, parser);

            // Create Template
            let template = BlogTemplate {
                title: title,
                content: &output_buf
            };

            // Render the template
            path.set_extension("html");
            println!("Writing {}", path.file_name().unwrap().to_str().unwrap());

            let output_path = article_path.join(path.file_name().unwrap());
            let mut writer = BufWriter::new(File::create(&output_path).unwrap());

            writer.write_all(template.render().unwrap().as_bytes()).unwrap();

            // Add to entries list
            entries.push(BlogEntry{
                path: output_path,
                title: title.to_string(),
                slug: slug.to_string(),
                date: NaiveDate::from_str(date).unwrap()
            });

            // Clear buffers
            input_buf.clear();
            output_buf.clear();
        }
    }

    // Now render the template for the home page with the latest articles
    entries.sort_unstable_by_key(|a| a.date);
    let last_3 = entries.iter().take(3)
        .map(|e| (e.title.clone(), e.slug.clone(), e.date.format("%B%e, %Y").to_string()))
        .collect::<Vec<(String, String, String)>>();

    let home_template = HomeTemplate {
        entries: last_3
    };

    println!("Writing home.html");
    let p = blog_path.join("home.html");
    let mut writer = BufWriter::new(File::create(p).unwrap());
    writer.write_all(home_template.render().unwrap().as_bytes()).unwrap();
    writer.flush().unwrap();

    // Render the archive template
    let entry_tuples = entries.iter()
        .map(|e| (e.title.clone(), e.slug.clone(), e.date.format("%B%e, %Y").to_string()))
        .collect::<Vec<(String, String, String)>>();
    let archive_template = ArchiveTemplate {
        entries: entry_tuples
    };

    println!("Writing archive.html");
    let p = blog_path.join("archive.html");
    let mut writer = BufWriter::new(File::create(p).unwrap());
    writer.write_all(archive_template.render().unwrap().as_bytes()).unwrap();
    writer.flush().unwrap();

    let debug = std::env::var("DEBUG").is_ok();

    if debug {
        // Start building our webserver, routes and all
        println!("Starting web server");
        HttpServer::new(move || {
            App::new()
                .data(AppState {
                    entries: entries[..].to_vec()
                })
                .service(index)
                .service(about)
                .service(latest)
                .service(archive)
                .service(get_post)
                .service(Files::new("/css", "./public/css").show_files_listing())
                .service(Files::new("/fonts", "./public/fonts").show_files_listing())
                .service(Files::new("/assets", "./public/assets").show_files_listing())
            })
        .bind("0.0.0.0:443")?
        .run()
        .await
    } else {
        // Certificate crap
        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder
            .set_private_key_file("/etc/letsencrypt/live/void.vstelt.dev/privkey.pem", SslFiletype::PEM)
            .unwrap();
        builder.set_certificate_chain_file("/etc/letsencrypt/live/void.vstelt.dev/fullchain.pem").unwrap();

        // Start building our webserver, routes and all
        println!("Starting web server");
        HttpServer::new(move || {
            App::new()
                .data(AppState {
                    entries: entries[..].to_vec()
                })
                .service(index)
                .service(about)
                .service(latest)
                .service(archive)
                .service(get_post)
                .service(Files::new("/css", "./public/css").show_files_listing())
                .service(Files::new("/fonts", "./public/fonts").show_files_listing())
                .service(Files::new("/assets", "./public/assets").show_files_listing())
            })
        .bind("0.0.0.0:443")?
        .run()
        .await
    }
}
