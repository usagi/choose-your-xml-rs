use std::env;
use std::io::BufReader;
use std::fs::File;
use std::str::{self, Utf8Error};

use quick_xml::Reader;
use quick_xml::events::{Event, BytesStart, BytesText};
use quick_xml::Error as XmlError;

type XmlReader = Reader<BufReader<File>>;

#[derive(Debug)]
enum Error {
    XmlError(XmlError),
    Utf8Error(Utf8Error),
}

macro_rules! from_error {
    ($err_type:ty, $err_name:ident) => (
        impl From<$err_type> for Error {
            fn from(value: $err_type) -> Error {
                Error::$err_name(value)
            }
        }
    )
}

from_error!(XmlError, XmlError);
from_error!(Utf8Error, Utf8Error);

type Result<T> = std::result::Result<T, Error>;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        println!("Usage:\n\tquick_xml input.xml");
        return;
    }

    if let Err(e) = parse(&args[1]) {
        println!("{:?}", e);
    }
}

fn parse(path: &str) -> Result<()> {
    let file = File::open(path).unwrap();
    let file = BufReader::new(file);
    let mut reader = Reader::from_reader(file);

    let mut buf = Vec::new();
    let mut ns_buf = Vec::new();

    let mut depth = 0;

    loop {
        match reader.read_namespaced_event(&mut buf, &mut ns_buf)? {
            (ns, Event::Start(ref e)) => {
                print_tag_name("Start", ns, e.local_name(), depth)?;
                print_attributes(&reader, &e, depth)?;
                depth += 1;
            }
            (ns, Event::Empty(ref e)) => {
                print_tag_name("Empty", ns, e.local_name(), depth)?;
                print_attributes(&reader, &e, depth)?;
            }
            (ns, Event::End(ref e)) => {
                depth -= 1;
                print_tag_name("End", ns, e.local_name(), depth)?;
            }
            (_, Event::Comment(ref e)) => {
                print_text(&reader, "Comment", e, depth)?;
            }
            (_, Event::CData(ref e)) => {
                print_text(&reader, "CDATA", e, depth)?;
            }
            (_, Event::PI(ref e)) => {
                print_text(&reader, "Processing Instruction", e, depth)?;
            }
            (_, Event::DocType(ref e)) => {
                print_text(&reader, "Document Type", e, depth)?;
            }
            (_, Event::Decl(ref e)) => {
                indent(depth);
                println!("Declaration");

                if let Ok(v) = e.version() {
                    indent(depth);
                    println!("  version=\"{}\"", str::from_utf8(v.as_ref())?);
                }

                if let Some(Ok(v)) = e.encoding() {
                    indent(depth);
                    println!("  encoding=\"{}\"", str::from_utf8(v.as_ref())?);
                }

                if let Some(Ok(v)) = e.standalone() {
                    indent(depth);
                    println!("  standalone=\"{}\"", str::from_utf8(v.as_ref())?);
                }
            }
            (_, Event::Text(ref e)) => {
                print_text(&reader, "  Text", e, depth)?;
            }
            (_, Event::Eof) => break,
        }
        buf.clear();
    }

    Ok(())
}

fn print_tag_name(title: &str, ns: Option<&[u8]>, tag_name: &[u8], depth: usize) -> Result<()> {
    indent(depth);

    match ns {
        Some(ns) => println!("{}: {} (ns: {})",
            title,
            str::from_utf8(tag_name)?,
            str::from_utf8(ns)?),
        None => println!("{}: {}",
            title,
            str::from_utf8(tag_name)?),
    }

    Ok(())
}

fn print_attributes(r: &XmlReader, e: &BytesStart, depth: usize) -> Result<()> {
    for a in e.attributes() {
        let a = a?;
        indent(depth + 1);
        println!("  Attribute: {}=\"{}\"",
            str::from_utf8(a.key)?,
            a.unescape_and_decode_value(r)?);
    }

    Ok(())
}

fn print_text(r: &XmlReader, title: &str, e: &BytesText, depth: usize) -> Result<()> {
    indent(depth);
    println!("{}: {:?}", title, e.unescape_and_decode(r)?);

    Ok(())
}

fn indent(depth: usize) {
    for _ in 0..depth {
        print!("  ");
    }
}
