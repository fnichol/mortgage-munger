use chrono::NaiveDate;
use csv::{Writer, WriterBuilder};
use std::error;
use std::io::{self, Read};
use std::result;

/// Number of columns in each pasted report
const INPUT_COLS: usize = 6;
/// Number of rows in each pasted report
const INPUT_ROWS: usize = 5;

/// Program result type alias.
type Result<T> = result::Result<T, Box<dyn error::Error>>;

/// Program state.
#[derive(Debug, PartialEq)]
enum State {
    /// No CSV title has been printed
    NoTitle,
    /// A CSV title has been printed
    TitlePrinted,
}

fn main() -> Result<()> {
    // Initialize state
    let mut state = State::NoTitle;
    // Create a CSV writer backed by a vec
    let mut writer = WriterBuilder::new().from_writer(vec![]);
    // Read in the pastes
    read_pastes(&mut writer, &mut state)?;

    // Write the final CSV to stdout
    println!("{}", String::from_utf8(writer.into_inner()?)?);

    Ok(())
}

/// Read a pasted report page on `stdin`.
fn read_pastes(writer: &mut Writer<Vec<u8>>, state: &mut State) -> Result<()> {
    loop {
        // Read `stdin` to a buffer
        let mut buf = String::new();
        io::stdin().lock().read_to_string(&mut buf)?;
        // If the buffer is empty, then we stop looping for input
        if buf.trim().is_empty() {
            break;
        }

        // Process the buffer
        process_paste(&buf, writer, state)?;
    }

    Ok(())
}

/// Process each pasted text buffer.
fn process_paste(buf: &str, writer: &mut Writer<Vec<u8>>, state: &mut State) -> Result<()> {
    // Arrange all trimmed fields on all lines to be one linear vec
    let input = buf
        .lines()
        .flat_map(|line| line.split('\t').map(str::trim))
        .collect::<Vec<_>>();

    // Prepare an output vec that's initialized with values (required by `transpose`)
    let mut output = Vec::with_capacity(input.len());
    output.extend(std::iter::repeat("").take(input.len()));

    // Transpose the "matrix", turning rows into columns
    transpose::transpose(&input, &mut output, INPUT_COLS, INPUT_ROWS);

    // Create an iterator of row fields to be re-output as lines
    let mut lines = output.chunks(INPUT_ROWS);

    // The a title line has not been written, add it to the CSV writer, and otherwise skip it
    let titles = lines.next().expect("title line should exist");
    if *state == State::NoTitle {
        writer.write_record(titles.iter().map(|title| title.replace(":", "")))?;
        *state = State::TitlePrinted;
    }

    for record in lines {
        // Create an iterator over the output fields
        let mut fields = record.iter();

        // Parse the first field as a date and write out the only sane, unambiguous date format pls
        let date = NaiveDate::parse_from_str(
            fields.next().expect("should contain a date field"),
            "%b %e, %Y",
        )?;
        writer.write_field(format!("{}", date))?;

        // Strip the dollar sign off the currency amounts and write out all remaining fields
        for field in fields {
            writer.write_field(field.replace("$", ""))?;
        }
        // Write an empty row to signal the record line is terminated
        writer.write_record(None::<&[u8]>)?;
    }

    Ok(())
}
