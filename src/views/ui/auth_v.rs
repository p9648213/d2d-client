use vy::prelude::*;

pub fn render_login_modal() -> impl IntoHtml {
    div!(
        id = "login-modal",
        class = "hidden fixed inset-0 justify-center items-center bg-slate-800/60 w-full h-full text-black",
        div!(
            class = "relative bg-white shadow p-4 rounded-md w-90",
            div!(
                role = "button",
                id = "login-close-button",
                class = "top-4 right-4 absolute cursor-pointer",
                "X"
            ),
            div!(class = "mb-4 font-bold text-lg text-center", "Login"),
            form!(
                "hx-post" = "/auth/login",
                "hx-swap" = "none",
                "hx-on-htmx-after-request" = "if(event.detail.successful) this.reset()",
                "hx-disabled-elt" = "find button",
                div!(
                    class = "flex flex-col gap-4",
                    div!(
                        class = "flex flex-col gap-2",
                        label!("Email address: "),
                        input!(
                            class = "rounded-md w-full h-9.5",
                            name = "email",
                            "type" = "email",
                            autocomplete = "email",
                            placeholder = "Email address"
                        )
                    ),
                    div!(
                        class = "flex flex-col gap-2",
                        label!("Password: "),
                        input!(
                            class = "rounded-md w-full h-9.5",
                            name = "password",
                            "type" = "password",
                            autocomplete = "current-password",
                            placeholder = "Password"
                        )
                    ),
                    button!(
                        class = "bg-neutral-100 hover:bg-neutral-300 py-1.5 border border-neutral-900 rounded-md w-full h-9.5",
                        "type" = "submit",
                        "Login"
                    ),
                    a!(
                        href = "/auth/google/login",
                        "hx-disable" = "true",
                        button!(
                            class = "flex justify-center items-center gap-2 bg-neutral-100 hover:bg-neutral-300 py-1.5 border border-neutral-900 rounded-md w-full h-9.5",
                            "type" = "button",
                            div!(
                                class = "h-6",
                                img!(
                                    class = "h-full",
                                    src = "/assets/images/google.webp",
                                    alt = "google"
                                )
                            ),
                            "Login With Google"
                        )
                    )
                ),
            ),
            button!(
                id = "register-link",
                class = "mt-2 text-sky-600",
                "Not a member? Register"
            )
        )
    )
}

pub fn render_register_modal() -> impl IntoHtml {
    div!(
        id = "register-modal",
        class = "hidden fixed inset-0 justify-center items-center bg-slate-800/60 w-full h-full text-black",
        div!(
            class = "relative bg-white shadow p-4 rounded-md w-90",
            div!(
                id = "register-close-button",
                role = "button",
                class = "top-4 right-4 absolute cursor-pointer",
                "X"
            ),
            div!(class = "mb-4 font-bold text-lg text-center", "Register"),
            form!(
                "hx-post" = "/auth/register",
                "hx-swap" = "none",
                "hx-on-htmx-after-request" = "if(event.detail.successful) this.reset()",
                "hx-disabled-elt" = "find button",
                div!(
                    class = "flex flex-col gap-4",
                    div!(
                        class = "flex flex-col gap-2",
                        label!("Username: "),
                        input!(
                            class = "rounded-md w-full h-9.5",
                            name = "username",
                            "type" = "text",
                            autocomplete = "on",
                            placeholder = "Username"
                        )
                    ),
                    div!(
                        class = "flex flex-col gap-2",
                        label!("Email address: "),
                        input!(
                            class = "rounded-md w-full h-9.5",
                            name = "email",
                            "type" = "email",
                            autocomplete = "email",
                            placeholder = "Email address"
                        )
                    ),
                    div!(
                        class = "flex flex-col gap-2",
                        label!("Password: "),
                        input!(
                            class = "rounded-md w-full h-9.5",
                            name = "password",
                            "type" = "password",
                            autocomplete = "current-password",
                            placeholder = "Password"
                        )
                    ),
                    input!(
                        class = "rounded-md w-full h-9.5",
                        name = "confirm_password",
                        "type" = "password",
                        autocomplete = "current-password",
                        placeholder = "Confirm Password"
                    ),
                    button!(
                        class = "bg-neutral-100 hover:bg-neutral-300 py-1.5 border border-neutral-900 rounded-md w-full h-9.5",
                        "type" = "submit",
                        "Register"
                    )
                )
            ),
            button!(
                id = "login-link",
                class = "mt-2 text-sky-600",
                "Have an account? Login"
            )
        )
    )
}
