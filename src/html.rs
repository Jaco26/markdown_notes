

const STYLES: &str = r#"
* {
    box-sizing: border-box;
}
body {
    line-height: 1.3;
    font-family: 'Lucida Sans', 'Lucida Sans Regular', 'Lucida Grande', 'Lucida Sans Unicode', Geneva, Verdana, sans-serif;
    font-size: 18px;
    font-weight: 300;
    color: #222;
    margin: 0 16px;
    display: flex;
    justify-content: center;
}
main {
    padding: 0 32px;
    width: 710px;
}
a {
    color: rgb(58, 128, 131);
    font: inherit;
}
a:visited {
    color: rgb(58, 128, 131);
}
.theme--dark a {
    color: rgb(98, 168, 171);
}
.theme--dark a:visited {
    color: rgb(98, 168, 171);
}
strong {
    font-weight: bold;
}
code {
    font-size: 14px;
    background: #f0e9ec;
    padding: 2px;
    border-radius: 4px;
    color: #004297;
}
blockquote {
    margin: 0;
    padding-left: 20px;
    color: #555;
    font-size: 16px;
    border-left: 4px solid #555;
}
.theme--dark blockquote {
    color: #ccc;
    border-color: #aaa;
}
hr {
    border-bottom: none;
    margin: 40px 0;
    border-color: #aeaeae;
}
.toolbar {
    display: flex;
    justify-content: flex-end;
    padding: 4px 0;
}
.theme--light {
    background-color: #efefef;
    color: #333333;
}
.theme--dark {
    background-color: #223;
    color: #dddddd;
}
.md h1,
.md h2,
.md h3,
.md h4,
.md h5,
.md h6,
.md p {
    margin: 20px 0;
    line-height: 1.4;
}
.md p img {
    width: 100%;
}
.md h1 {
    font-size: 28px;
}
.md h2 {
    font-size: 24px;
}
.md h3 {
    font-size: 20px;
}
.md h4{
    font-size: 16px;
}
.md hr {
    border: none;
    border-bottom: 1px solid #ccc;
}
.md a {
    text-decoration: none;
}
.md blockquote {
    color: #555;
    margin: 16px 4px;
    padding: 0 16px;
    border-left: 4px solid #ccc;
    border-radius: 2px;
}
.md code {
    background-color: #eee;
    padding: 2px;
    border-radius: 2px;
}
.md li {
    margin: 4px 0;
}
"#;

const SCRIPT: &str = r#"

const DEFAULT_THEME = 'light';

const themeSelect = document.getElementById('theme-select');

themeSelect.addEventListener('change', onChangeThemeSelect);

onPageLoad();

function onPageLoad() {

    document.querySelectorAll('a').forEach(anchorElement => {
        if (anchorElement.getAttribute('href').startsWith('#')) {
            return;
        }
        anchorElement.setAttribute('target', '_blank');
        anchorElement.setAttribute('rel', 'noopener noreferrer');
    });

    let savedTheme = getSavedTheme();
    if (!savedTheme) {
        setSavedTheme(DEFAULT_THEME);
        savedTheme = getSavedTheme();
    }
    themeSelect.value = savedTheme;
    setDocumentTheme(savedTheme);
}

function getSavedTheme() {
    return localStorage.getItem('md_notes_theme');
}

function setSavedTheme(themeName) {
    localStorage.setItem('md_notes_theme', themeName);
}

function setDocumentTheme(themeName) {
    document.body.setAttribute('class', `theme--${themeName}`);
}

function onChangeThemeSelect(event) {
    setSavedTheme(themeSelect.value);
    setDocumentTheme(themeSelect.value);
}

"#;

pub fn md_to_html(title: &str, body_content: &str) -> String {
    let mut options = comrak::ComrakOptions::default();
    options.extension.header_ids = Some("".to_string());
    format!(
r#"
<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <title>{}</title>
        <style>{}</style>
    </head>
    <body>
        <main>
            <div class="toolbar">
                <select id="theme-select">
                    <option value="light">Light</option>
                    <option value="dark">Dark</option>
                </select>
            </div>
            <div>
                {}
            </div>
        </main>
        <script>{}</script>
    </body>
</html>
"#,
        title,
        STYLES,
        comrak::markdown_to_html(
            &body_content,
            &options
        ),
        SCRIPT
    )
}