use crate::views::head_v::render_header;

//..........................................................
//.LLL...........OOOOOO........GGGGGG.....III..NNNN....NNN..
//.LLL.........OOOOOOOOOO....GGGGGGGGGG...III..NNNN....NNN..
//.LLL........OOOOOOOOOOOO...GGGGGGGGGGG..III..NNNNN...NNN..
//.LLL........OOOO....OOOO..GGGG....GGGG..III..NNNNN...NNN..
//.LLL........OOO......OOO..GGG......GG...III..NNNNNN..NNN..
//.LLL.......LOOO......OOOOOGGG...........III..NNNNNNN.NNN..
//.LLL.......LOOO......OOOOOGGG...GGGGGG..III..NNN.NNN.NNN..
//.LLL.......LOOO......OOOOOGGG...GGGGGG..III..NNN.NNNNNNN..
//.LLL........OOO......OOO..GGG...GGGGGG..III..NNN..NNNNNN..
//.LLL........OOOO....OOOO..GGGG.....GGG..III..NNN..NNNNNN..
//.LLLLLLLLLL.OOOOOOOOOOOO...GGGGGGGGGGG..III..NNN...NNNNN..
//.LLLLLLLLLL..OOOOOOOOOO....GGGGGGGGGG...III..NNN....NNNN..
//.LLLLLLLLLL....OOOOOO........GGGGGG.....III..NNN....NNNN..
//..........................................................

pub fn render_login_page(authenticity_token: String) -> maud::Markup {
	maud::html! {
			(maud::DOCTYPE)
			(render_header())
			body hx-boost="true" {
					title {
							"Login"
					}
					div class="flex justify-center items-center px-4 sm:px-6 lg:px-8 py-12 h-screen" {
							div class="space-y-3 w-full max-w-sm" {
									div {
											h2 class="font-bold text-gray-900 text-2xl text-center leading-9 tracking-tight" {
													"Sign in to your account"
											}
									}
									form
										id="login-form"
										class="space-y-3"
										hx-post="/auth/login"
										hx-swap="none"
										hx-on-htmx-after-request="if(event.detail.successful) this.reset()"
										hx-disabled-elt="find button"
									{
											input type="hidden" name="authenticity_token" value=(authenticity_token);
											div class="relative -space-y-px shadow-xs border border-gray-300 border-solid rounded-md" {
													div {
															label class="sr-only" for="email-address" {
																	"Email address"
															}
															input id="email-address" class="block focus:z-10 relative py-1.5 border-0 rounded-t-md ring-1 ring-gray-100 focus:ring-2 focus:ring-indigo-600 ring-inset focus:ring-inset w-full text-gray-900 placeholder:text-gray-400 sm:text-sm sm:leading-6" name="email" type="email" autocomplete="email" placeholder="Email address" value="user01@gmail.com";
													}
													div {
															label class="sr-only" for="password" {
																	"Password"
															}
															input id="password" class="block focus:z-10 relative py-1.5 border-0 rounded-b-md ring-1 ring-gray-100 focus:ring-2 focus:ring-indigo-600 ring-inset focus:ring-inset w-full text-gray-900 placeholder:text-gray-400 sm:text-sm sm:leading-6" name="password" type="password" autocomplete="current-password" placeholder="Password" value="";
													}
											}
											div {
													button class="flex justify-center bg-indigo-600 hover:bg-indigo-500 px-3 py-1.5 rounded-md focus-visible:outline-2 focus-visible:outline-indigo-600 focus-visible:outline-offset-2 w-full font-semibold text-white text-sm leading-6 cursor-pointer" type="submit" {
															"Sign in"
													}
											}
											a href="/auth/google/login" hx-disable {
													button class="flex justify-center bg-indigo-600 hover:bg-indigo-500 px-3 py-1.5 rounded-md focus-visible:outline-2 focus-visible:outline-indigo-600 focus-visible:outline-offset-2 w-full font-semibold text-white text-sm leading-6 cursor-pointer" type="button" {
															"Sign in with google"
													}
											}
									}
									p class="text-gray-500 text-sm text-center leading-6" {
											"Not a member?"
											a id="register-link" class="font-semibold text-indigo-600 hover:text-indigo-500" href="/auth/register" {
													" Sign Up"
											}
									}
							}
					}
					div id="toast" {}
			}
	}
}

//.............................................................................................
//.RRRRRRRRR....EEEEEEEEEE.....GGGGGG.....III....SSSSSS....TTTTTTTTTTTEEEEEEEEEE..RRRRRRRRR....
//.RRRRRRRRRRR..EEEEEEEEEE...GGGGGGGGGG...III..SSSSSSSSS...TTTTTTTTTTTEEEEEEEEEE..RRRRRRRRRRR..
//.RRRRRRRRRRR..EEEEEEEEEE...GGGGGGGGGGG..III..SSSSSSSSSS..TTTTTTTTTTTEEEEEEEEEE..RRRRRRRRRRR..
//.RRR.....RRR..EEE.........GGGG....GGGG..III..SSS...SSSS......TTT....EEE.........RRR.....RRR..
//.RRR.....RRR..EEE.........GGG......GG...III..SSSS............TTT....EEE.........RRR.....RRR..
//.RRRRRRRRRRR..EEEEEEEEEE.EGGG...........III..SSSSSSS.........TTT....EEEEEEEEEE..RRRRRRRRRRR..
//.RRRRRRRRRR...EEEEEEEEEE.EGGG...GGGGGG..III...SSSSSSSS.......TTT....EEEEEEEEEE..RRRRRRRRRR...
//.RRRRRRRR.....EEEEEEEEEE.EGGG...GGGGGG..III.....SSSSSSS......TTT....EEEEEEEEEE..RRRRRRRR.....
//.RRR..RRRR....EEE.........GGG...GGGGGG..III.........SSSS.....TTT....EEE.........RRR..RRRR....
//.RRR...RRRR...EEE.........GGGG.....GGG..III.ISSS....SSSS.....TTT....EEE.........RRR...RRRR...
//.RRR....RRRR..EEEEEEEEEEE..GGGGGGGGGGG..III..SSSSSSSSSS......TTT....EEEEEEEEEEE.RRR....RRRR..
//.RRR....RRRR..EEEEEEEEEEE..GGGGGGGGGG...III..SSSSSSSSSS......TTT....EEEEEEEEEEE.RRR....RRRR..
//.RRR.....RRRR.EEEEEEEEEEE....GGGGGG.....III....SSSSSS........TTT....EEEEEEEEEEE.RRR.....RRR..
//.............................................................................................

pub fn render_register_page(authenticity_token: String) -> maud::Markup {
	maud::html! {
		(maud::DOCTYPE)
		(render_header())
		body hx-boost="true" {
					title {
							"Register"
					}
					div class="flex justify-center items-center px-4 sm:px-6 lg:px-8 py-12 h-screen" {
							div class="space-y-10 w-full max-w-sm" {
									div {
											h2 class="font-bold text-gray-900 text-2xl text-center leading-9 tracking-tight" {
													"Register account"
											}
									}
									form
											id="login-form"
											class="space-y-6"
											hx-post="/auth/register"
											hx-swap="none"
											hx-on-htmx-after-request="if(event.detail.successful) this.reset()"
											hx-disabled-elt="find button"
									{
											input type="hidden" name="authenticity_token" value=(authenticity_token);
											div class="relative -space-y-px shadow-xs border border-gray-300 border-solid rounded-md" {
													div {
															label class="sr-only" for="username" {
																	"Username"
															}
															input id="username" class="block focus:z-10 relative py-1.5 border-0 rounded-t-md ring-1 ring-gray-100 focus:ring-2 focus:ring-indigo-600 ring-inset focus:ring-inset w-full text-gray-900 placeholder:text-gray-400 sm:text-sm sm:leading-6" name="username" type="text" autocomplete="on" placeholder="Username";
													}
													div {
															label class="sr-only" for="email-address" {
																	"Email address"
															}
															input id="email-address" class="block focus:z-10 relative py-1.5 border-0 rounded-t-md ring-1 ring-gray-100 focus:ring-2 focus:ring-indigo-600 ring-inset focus:ring-inset w-full text-gray-900 placeholder:text-gray-400 sm:text-sm sm:leading-6" name="email" type="email" autocomplete="email" placeholder="Email address";
													}
													div {
															label class="sr-only" for="password" {
																	"Password"
															}
															input id="password" class="block focus:z-10 relative py-1.5 border-0 rounded-b-md ring-1 ring-gray-100 focus:ring-2 focus:ring-indigo-600 ring-inset focus:ring-inset w-full text-gray-900 placeholder:text-gray-400 sm:text-sm sm:leading-6" name="password" type="password" autocomplete="current-password" placeholder="Password";
													}
											}
											div {
													button class="flex justify-center bg-indigo-600 hover:bg-indigo-500 px-3 py-1.5 rounded-md focus-visible:outline-2 focus-visible:outline-indigo-600 focus-visible:outline-offset-2 w-full font-semibold text-white text-sm leading-6 cursor-pointer" type="submit" {
															"Register"
													}
											}
									}
									p class="text-gray-500 text-sm text-center leading-6" {
											"Already have an account?"
											a id="register-link" class="font-semibold text-indigo-600 hover:text-indigo-500" href="/auth/login" {
													" Login"
											}
									}
							}
					}
					div id="toast" {}
		}
	}
}
