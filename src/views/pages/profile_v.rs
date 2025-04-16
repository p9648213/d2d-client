use crate::{
    middlewares::auth_mw::UserAuth,
    views::ui::{head_v::render_head, nav_v::render_navbar},
};

pub struct ProfilePageProps {
    pub authenticity_token: String,
    pub user_auth: UserAuth,
}

pub fn render_profile_page(props: &ProfilePageProps) -> maud::Markup {
    maud::html! {
        (maud::DOCTYPE)
        (render_head())
        body class="flex flex-col bg-neutral-200" hx-boost="true" {
            title {
                "D2D | Profile"
            }
            (render_navbar(&props.authenticity_token, &props.user_auth))
            main class="flex flex-grow justify-center items-center"  {
                (render_profile_section())
            }
            div id="toast" {}
        }
    }
}

pub fn render_profile_section() -> maud::Markup {
    maud::html! {
        div class="flex gap-5 p-6 bg-white rounded-md border border-neutral-600 w-full max-w-[70%]" {
            div class="flex-1 p-4 border border-neutral-600 rounded-md" {
                "Profile"
            }
            div class="flex-3 p-4 border border-neutral-600 rounded-md" {
                "User Info"
            }
         }
    }
}
