use vy::prelude::*;

use crate::{
    middlewares::auth_mw::UserInfo,
    views::ui::{
        head_v::render_head,
        nav_v::{NavBarProps, render_navbar},
    },
};

pub struct HomePageProps {
    pub user_info: Option<UserInfo>,
}

pub fn render_home_page(props: &HomePageProps) -> impl IntoHtml {
    let nav_props = NavBarProps {
        user_info: props.user_info.as_ref(),
    };

    (
        DOCTYPE,
        html!(
            render_head(),
            title!("D2D | Home"),
            render_navbar(nav_props),
            main!(
                class = "flex flex-grow justify-center items-center",
                render_home_section()
            ),
            div!(id = "toast")
        ),
    )
}

pub fn render_home_section() -> impl IntoHtml {
    div!("Home")
}
