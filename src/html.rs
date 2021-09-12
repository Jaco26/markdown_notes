
const STYLES: &str = r#"
body {
    font-family: "Avenir", sans-serif;
    font-size: 18px;
    font-weight: 300;
    background-color: #fcf5f5;
    color: #222;
    margin: 0 64px;
}
blockquote {
    color: #555;
    font-size: 16px;
}
"#;

pub fn md_to_html(title: &str, body_content: &str) -> String {
    format!(
r#"
<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="utf-8" />
        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">
        <title>{}</title>
        <style>{}</style>
    </head>
    <body>
        {}
    </body>
</html>
"#,
        title,
        STYLES,
        comrak::markdown_to_html(
            &body_content,
            &comrak::ComrakOptions::default()
        )
    )
}