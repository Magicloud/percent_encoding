use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

fn main() -> anyhow::Result<()> {
    let mut in_buf = String::new();

    while std::io::stdin().read_line(&mut in_buf)? != 0 {
        print!("{}", utf8_percent_encode(in_buf.trim(), NON_ALPHANUMERIC));
        in_buf.clear();
    }
    Ok(())
}