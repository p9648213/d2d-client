use crate::{middlewares::auth_mw::UserAuth, views::head_v::render_header};

pub struct HomePageProps {
	pub authenticity_token: String,
	pub user_auth: UserAuth,
}

pub fn render_home_page(props: &HomePageProps) -> maud::Markup {
	maud::html! {
		(maud::DOCTYPE)
		(render_header())
		body class="bg-neutral-200" hx-boost="true" {
			title {
				"D2D"
			}
			(render_navbar(&props.authenticity_token, &props.user_auth))
			div id="toast" {}
		}
	}
}

pub fn render_navbar(authenticity_token: &str, user_auth: &UserAuth) -> maud::Markup {
	maud::html! {
		nav class="flex justify-between items-center py-4 px-10 shadow bg-neutral-50" {
			div {
				"Logo"
			}
			div class="relative" {
				@match &user_auth.0 {
					Some(user_info) => {
						(maud::PreEscaped(r#"
	          <script defer type="module">
	          import {
	            setupUserDropdown
	          } from "/assets/js/home/user-dropdown.js"
	          setupUserDropdown()
	          </script>
	          "#))
						div id="user-dropdown" class="flex gap-2 items-center cursor-pointer" {
							img class="w-7 h-7 rounded-full" src=(user_info.image_url) alt="avatar";
							span class="capitalize" {
								(user_info.username)
							}
							span {
								(maud::PreEscaped("&#11167;"))
							}
						}
						div
						id="user-dropdown-options"
						class="hidden absolute right-0 top-10 py-2 px-3 bg-white rounded-md border border-neutral-600"
						{
							form hx-post="/auth/logout" hx-swap="none" {
								button type="submit" class="cursor-pointer" {
									"Logout"
								}
								input type="hidden" name="authenticity_token" value=(authenticity_token);
							}
						}
					},
					None => {
						(maud::PreEscaped(r#"
	          <script defer type="module">
	          import {
	            setupAuthModal
	          } from "/assets/js/home/auth-modal.js"
	          setupAuthModal()
	          </script>
	          "#))
						button
						id="sign-in-button"
						class="py-1.5 px-3 bg-white rounded-md shadow cursor-pointer"
						style="box-shadow: rgba(0, 0, 0, 0.05) 0px 0px 0px 1px, rgb(209, 213, 219) 0px 0px 0px 1px inset"
						{
							"Sign in"
						}
						div
						id="login-modal"
						class="hidden fixed inset-0 justify-center items-center w-full h-full bg-slate-200/60"
						{
							div class="relative p-4 bg-white rounded-md shadow w-100" {
								button id="login-close-button" class="absolute top-4 right-4 cursor-pointer" {
									"X"
								}
								div class="mb-4 text-lg font-bold text-center" {
									"Login"
								}
								form
								hx-post="/auth/login"
								hx-swap="none"
								hx-on-htmx-after-request="if(event.detail.successful) this.reset()"
								hx-disabled-elt="find button"
								{
									input type="hidden" name="authenticity_token" value=(authenticity_token);
									div class="flex flex-col gap-4" {
										div class="flex flex-col gap-2" {
											label for="email-address" {
												"Email address: "
											}
											input class="w-full rounded-md" name="email" type="email" autocomplete="email" placeholder="Email address";
										}
										div class="flex flex-col gap-2" {
											label for="password" {
												"Password: "
											}
											input class="w-full rounded-md" name="password" type="password" autocomplete="current-password" placeholder="Password";
										}
										button type="submit" class="py-1.5 w-full rounded-md border cursor-pointer bg-neutral-100 border-neutral-900" {
											"Login"
										}
										a href="/auth/google/login" hx-disable {
											button
											type="button"
											class="flex gap-2 justify-center items-center py-1.5 w-full rounded-md border cursor-pointer bg-neutral-100 border-neutral-900"
											{
												div class="h-6" {
													img class="h-full" src="/assets/images/google.webp" alt="google";
												}
												"Login With Google"
											}
										}
									}
								}
								button id="register-link" class="mt-2 cursor-pointer text-sky-600" {
									"Not a member? Register"
								}
							}
						}
						div
						id="register-modal"
						class="hidden fixed inset-0 justify-center items-center w-full h-full bg-slate-200/60"
						{
							div class="relative p-4 bg-white rounded-md shadow w-100" {
								button id="register-close-button" class="absolute top-4 right-4 cursor-pointer" {
									"X"
								}
								div class="mb-4 text-lg font-bold text-center" {
									"Register"
								}
								form
								hx-post="/auth/register"
								hx-swap="none"
								hx-on-htmx-after-request="if(event.detail.successful) this.reset()"
								hx-disabled-elt="find button"
								{
									input type="hidden" name="authenticity_token" value=(authenticity_token);
									div class="flex flex-col gap-4" {
										div class="flex flex-col gap-2" {
											label for="username" {
												"Username: "
											}
											input class="w-full rounded-md" name="username" type="text" autocomplete="on" placeholder="Username";
										}
										div class="flex flex-col gap-2" {
											label for="email-address" {
												"Email address: "
											}
											input class="w-full rounded-md" name="email" type="email" autocomplete="email" placeholder="Email address";
										}
										div class="flex flex-col gap-2" {
											label for="password" {
												"Password: "
											}
											input class="w-full rounded-md" name="password" type="password" autocomplete="current-password" placeholder="Password";
										}
										button type="submit" class="py-1.5 w-full rounded-md border cursor-pointer bg-neutral-100 border-neutral-900" {
											"Register"
										}
									}
								}
								button id="login-link" class="mt-2 cursor-pointer text-sky-600" {
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
