use askama::Template;

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate;

pub fn render_home_page() -> String {
    HomeTemplate.to_string()
}
