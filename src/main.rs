use anyhow::Context;
use select::document::Document;
use select::predicate::Class;
use std::fs::OpenOptions;
use std::io::Write;
use std::{
    fs::File,
    io::{prelude::*, LineWriter},
};

fn main() -> anyhow::Result<()> {
    let mut page = 1;

    while page != 10 {
        let url = format!(
            "https://www.jumia.com.ng/catalog/?q=iphone&viewType=grid&page={:?}#catalog-listing",
            page
        );
        let res = reqwest::blocking::get(url).with_context(|| format!("opening url error"))?;

        let document = Document::from_read(res).context("parsing response")?;
        let writer = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open("box.txt")
            .with_context(|| format!("opening file "))?;
        
        let mut writer = LineWriter::new(writer);

        let jumia = document.find(Class("info"));
        let link = document
            .find(Class("core"))
            .next()
            .context("writing to the output file")?;

        for node in jumia {
            let name = node
                .find(Class("name"))
                .next()
                .context("writing to the output file")?;
            let price = node
                .find(Class("prc"))
                .next()
                .context("writing to the output file")?;
            

            writeln!(
                writer,
                "{:?}---{:?}---{:?}",
                name.text(),
                price.text(),
                link.attr("href")
            )
            .context("writing to the output file")?;
        }
        page = page + 1;
    }

    Ok(())
}
