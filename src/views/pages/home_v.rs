use crate::{
	middlewares::auth_mw::UserAuth,
	views::ui::{head_v::render_head, nav_v::render_navbar},
};

pub struct HomePageProps {
	pub authenticity_token: String,
	pub user_auth: UserAuth,
}

pub fn render_home_page(props: &HomePageProps) -> maud::Markup {
	maud::html! {
		(maud::DOCTYPE)
		(render_head())
		body class="bg-neutral-200" hx-boost="true" {
			title {
				"D2D"
			}
			(render_navbar(&props.authenticity_token, &props.user_auth))
			div id="toast" {}
		}
	}
}
