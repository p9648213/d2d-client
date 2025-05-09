use crate::middlewares::auth_mw::UserAuth;

pub fn render_navbar(user_auth: &UserAuth) -> maud::Markup {
    maud::html! {
        nav class="flex justify-between items-center bg-neutral-50 shadow px-10 py-2.5" {
            a href="/" hx-target="main" {
                "Logo"
            }
            div class="relative" {
                @match &user_auth.0 {
                    Some(user_info) => {
                        (maud::PreEscaped(r#"
							<script defer type="module">
								import { setupUserDropdown } from "/assets/js/home/user-dropdown.js"
								setupUserDropdown()
							</script>
	          			"#))
                        div id="user-dropdown" class="flex items-center gap-2 cursor-pointer" {
                            img class="rounded-full w-7 h-7" src=(user_info.image_url) alt="avatar";
                            span class="capitalize" {
                                (user_info.username)
                            }
                            span {
                                (maud::PreEscaped("&#11167;"))
                            }
                        }
                        div
                            id="user-dropdown-options"
                            class="hidden top-9 right-0 absolute flex-col gap-1 bg-white px-3 py-2 border border-neutral-600 rounded-md"
                        {
                            a
                                name="dropdown-item"
                                href="/profile"
                                hx-target="main"
                                class="hover:opacity-50 text-start"
                            {
                                "Profile"
                            }
                            form name="dropdown-item" hx-post="/auth/logout" hx-swap="none" class="hover:opacity-50" {
                                button type="submit" {
                                    "Logout"
                                }
                            }
                        }
                    },
                    None => {
                        (maud::PreEscaped(r#"
							<script defer type="module">
								import { setupAuthModal} from "/assets/js/home/auth-modal.js"
								setupAuthModal()
							</script>
	          			"#))
                        button
                            id="sign-in-button"
                            class="bg-white hover:bg-neutral-300 shadow px-3 py-1.5 rounded-md"
                            style="box-shadow: rgba(0, 0, 0, 0.05) 0px 0px 0px 1px, rgb(209, 213, 219) 0px 0px 0px 1px inset"
                        {
                            "Sign in"
                        }
                        div
                            id="login-modal"
                            class="hidden fixed inset-0 justify-center items-center bg-slate-200/60 w-full h-full"
                        {
                            div class="relative bg-white shadow p-4 rounded-md w-90" {
                                div role="button" id="login-close-button" class="top-4 right-4 absolute cursor-pointer" {
                                    "X"
                                }
                                div class="mb-4 font-bold text-lg text-center" {
                                    "Login"
                                }
                                form
                                    hx-post="/auth/login"
                                    hx-swap="none"
                                    hx-on-htmx-after-request="if(event.detail.successful) this.reset()"
                                    hx-disabled-elt="find button"
                                {
                                    div class="flex flex-col gap-4" {
                                        div class="flex flex-col gap-2" {
                                            label for="email-address" {
                                                "Email address: "
                                            }
                                            input class="rounded-md w-full h-9.5" name="email" type="email" autocomplete="email" placeholder="Email address";
                                        }
                                        div class="flex flex-col gap-2" {
                                            label for="password" {
                                                "Password: "
                                            }
                                            input class="rounded-md w-full h-9.5" name="password" type="password" autocomplete="current-password" placeholder="Password";
                                        }
                                        button type="submit" class="bg-neutral-100 hover:bg-neutral-300 py-1.5 border border-neutral-900 rounded-md w-full h-9.5" {
                                            "Login"
                                        }
                                        a href="/auth/google/login" hx-disable {
                                            button
                                                type="button"
                                                class="flex justify-center items-center gap-2 bg-neutral-100 hover:bg-neutral-300 py-1.5 border border-neutral-900 rounded-md w-full h-9.5"
                                            {
                                                div class="h-6" {
                                                    img class="h-full" src="/assets/images/google.webp" alt="google";
                                                }
                                                "Login With Google"
                                            }
                                        }
                                    }
                                }
                                button id="register-link" class="mt-2 text-sky-600" {
                                    "Not a member? Register"
                                }
                            }
                        }
                        div
                            id="register-modal"
                            class="hidden fixed inset-0 justify-center items-center bg-slate-200/60 w-full h-full"
                        {
                            div class="relative bg-white shadow p-4 rounded-md w-90" {
                                div role="button" id="register-close-button" class="top-4 right-4 absolute cursor-pointer" {
                                    "X"
                                }
                                div class="mb-4 font-bold text-lg text-center" {
                                    "Register"
                                }
                                form
                                    hx-post="/auth/register"
                                    hx-swap="none"
                                    hx-on-htmx-after-request="if(event.detail.successful) this.reset()"
                                    hx-disabled-elt="find button"
                                {
                                    div class="flex flex-col gap-4" {
                                        div class="flex flex-col gap-2" {
                                            label for="username" {
                                                "Username: "
                                            }
                                            input class="rounded-md w-full h-9.5" name="username" type="text" autocomplete="on" placeholder="Username";
                                        }
                                        div class="flex flex-col gap-2" {
                                            label for="email-address" {
                                                "Email address: "
                                            }
                                            input class="rounded-md w-full h-9.5" name="email" type="email" autocomplete="email" placeholder="Email address";
                                        }
                                        div class="flex flex-col gap-2" {
                                            label for="password" {
                                                "Password: "
                                            }
                                            input class="rounded-md w-full h-9.5" name="password" type="password" autocomplete="current-password" placeholder="Password";
                                        }
                                        button type="submit" class="bg-neutral-100 hover:bg-neutral-300 py-1.5 border border-neutral-900 rounded-md w-full h-9.5" {
                                            "Register"
                                        }
                                    }
                                }
                                button id="login-link" class="mt-2 text-sky-600" {
                                    "Have an account? Login"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
