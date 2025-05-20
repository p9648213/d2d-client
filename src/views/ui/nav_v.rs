use crate::{
    middlewares::auth_mw::UserInfo,
    views::ui::auth_v::{render_login_modal, render_register_modal},
};
use vy::prelude::*;

pub struct NavBarProps<'a> {
    pub user_info: Option<&'a UserInfo>,
}

pub fn render_navbar(props: NavBarProps) -> impl IntoHtml {
    nav!(
        class = "flex justify-between items-center bg-neutral-50 shadow px-10 py-2.5",
        a!(href = "/", "hx-target" = "main", "Logo"),
        div!(
            class = "relative",
            if let Some(user_info) = props.user_info {
                (
                    PreEscaped(
                        r#"
                            <script defer type="module">
                                import { setupUserDropdown } from "/assets/js/home/user-dropdown.js"
                                setupUserDropdown()
                            </script>
                        "#,
                    ),
                    div!(
                        id = "user-dropdown",
                        class = "flex items-center gap-2 cursor-pointer",
                        img!(
                            class = "rounded-full w-7 h-7",
                            src = &user_info.image_url,
                            alt = "user-image"
                        ),
                        span!(&user_info.username),
                        span!(PreEscaped("&#11167;"))
                    ),
                    div!(
                        id = "user-dropdown-options",
                        class = "hidden top-9 right-0 absolute flex-col gap-1 bg-white px-3 py-2 border border-neutral-600 rounded-md",
                        a!(
                            class = "hover:opacity-50 text-start",
                            name = "dropdown-item",
                            href = "/profile",
                            "hx-target" = "main",
                            "Profile"
                        ),
                        form!(
                            class = "hover:opacity-50",
                            name = "dropdown-item",
                            "hx-post" = "/auth/logout",
                            "hx-swap" = "none",
                            button!("type" = "submit", "Sign out")
                        )
                    ),
                )
            } else {
                (
                    PreEscaped(
                        r#"<script defer type="module">
                            import { setupAuthModal} from "/assets/js/home/auth-modal.js"
                            setupAuthModal()
                        </script>"#,
                    ),
                    button!(
                        id = "sign-in-button",
                        class = "bg-white hover:bg-neutral-300 shadow px-3 py-1.5 rounded-md",
                        style = "box-shadow: rgba(0, 0, 0, 0.05) 0px 0px 0px 1px, rgb(209, 213, 219) 0px 0px 0px 1px inset",
                        "Sign in"
                    ),
                    render_login_modal(),
                    render_register_modal(),
                )
            }
        )
    )
}
