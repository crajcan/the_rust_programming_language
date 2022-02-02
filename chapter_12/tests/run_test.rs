extern crate chapter_12;

use std::io::stdout;

use chapter_12::Config;

#[test]
fn it_prints_to_stdout() {
    let mut writer = vec![];

    let config = Config {
        query: "to".to_string(),
        filename: "poem.txt".to_string(),
        case_sensitive: true,
        output: &mut writer,
    };

    chapter_12::run(config);

    assert_eq!(
        String::from_utf8(writer).expect("Not UTF-8"),
        r#"Are you nobody? too?
How dreary to be somebody!
"#,
    );
}
