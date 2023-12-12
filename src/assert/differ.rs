pub fn diff(left: &str, right: &str) -> (bool, String) {
    let mut differs = false;
    let mut diff = String::new();
    for d in diff::lines(left, right) {
        match d {
            diff::Result::Both(l, _) => {
                diff.push_str(&format!("{}\n", l));
                differs = false;
            }
            diff::Result::Left(l) => {
                diff.push_str(&format!("- {}\n", l));
                differs = true;
            }
            diff::Result::Right(r) => {
                diff.push_str(&format!("+ {}\n", r));
                differs = true;
            }
        }
    }

    (differs, diff)
}
