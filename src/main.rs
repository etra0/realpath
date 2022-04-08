fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args();
    let filename = args.nth(1);

    if filename.is_none() {
        return Err("The name of the program is missing".into());
    }

    let full_path = std::fs::canonicalize(filename.unwrap());

    match full_path {
        Ok(path) => print!("{}", path.to_str().unwrap()),
        Err(e) => return Err(e.into()),
    };

    Ok(())
}
