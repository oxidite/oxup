use std::fs::{create_dir, metadata, write, remove_file, copy};

pub fn setup_m() {
    if !metadata(format!("{}/.oxido", std::env::var("HOME").unwrap())).is_ok() {
        create_dir(format!("{}/.oxido", std::env::var("HOME").unwrap())).unwrap();
        create_dir(format!("{}/.oxido/bin", std::env::var("HOME").unwrap())).unwrap();
    }

    write(
        format!("{}/.oxido/env", std::env::var("HOME").unwrap()),
        "
#!/bin/sh
# oxup shell setup
# affix colons on either side of $PATH to simplify matching
case \":${PATH}:\" in
    *:\"$HOME/.oxido/bin\":*)
        ;;
    *)
        # Prepending path in case a system-installed oxido needs to be overridden
        export PATH=\"$HOME/.oxido/bin:$PATH\"
        ;;
esac
",
    )
    .unwrap();

    copy(
        "oxup",
        format!("{}/.oxido/bin/oxup", std::env::var("HOME").unwrap()),
    )
    .unwrap();
    if metadata("./oxup").is_ok() {
        remove_file("oxup").unwrap();
    }
    if metadata("./oxup-darwin.zip").is_ok() {
        remove_file("oxup-darwin.zip").unwrap();
    }

    println!(
        "created {}",
        format!("{}/.oxido", std::env::var("HOME").unwrap())
    )
}
