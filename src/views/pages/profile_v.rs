use crate::{
    middlewares::auth_mw::UserAuth,
    views::ui::{head_v::render_head, nav_v::render_navbar},
};

pub struct ProfilePageProps {
    pub user_auth: UserAuth,
}

pub fn render_profile_page(props: &ProfilePageProps) -> maud::Markup {
    maud::html! {
        (maud::DOCTYPE)
        (render_head())
        body class="flex flex-col bg-neutral-200 text-sm" hx-boost="true" {
            title {
                "D2D | Profile"
            }
            (render_navbar(&props.user_auth))
            main class="flex flex-grow justify-center items-center"  {
                (render_profile_section())
            }
            div id="toast" {}
        }
    }
}

pub fn render_profile_section() -> maud::Markup {
    maud::html! {
        div class="flex gap-5 w-full max-w-[60%]" {
            div class="flex-1 bg-white p-4 border border-neutral-600 rounded-md" {
                ul class="flex flex-col gap-1" {
                    li class="bg-neutral-300 p-2 rounded-md" { "Details" }
                    li class="hover:bg-neutral-200 p-2 rounded-md cursor-pointer" { "Transaction History" }
                    li class="hover:bg-neutral-200 p-2 rounded-md cursor-pointer" { "Betting History" }
                }
            }
            div class="flex-4 bg-white p-4 border border-neutral-600 rounded-md" {
                "User Info"
            }
         }
    }
}
