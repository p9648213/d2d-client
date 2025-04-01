use crate::views::head_v::render_header;

pub fn render_home_page(authenticity_token: String) -> maud::Markup {
    maud::html! {
        (maud::DOCTYPE)
        (render_header())
        body hx-boost="true" {
            title {
                "Home"
            }
            div {
                "Home"
            }
            form hx-post="/auth/logout" hx-swap="none" {
                button type="submit" {
                    "Logout"
                }
                input type="hidden" name="authenticity_token" value=(authenticity_token);
            }
        }
    }
}
