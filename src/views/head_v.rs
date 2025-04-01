pub fn render_header() -> maud::Markup {
  maud::html! {
    head{
      meta charset="UTF-8";
      meta name="viewport" content="width=device-width,initial-scale=1";
      link rel="stylesheet" href="/assets/css/lib/tailwind.css";
      link rel="stylesheet" href="/assets/css/lib/toast.css";
      link rel="stylesheet" href="/assets/css/lib/nprogress.css";
      script src="/assets/js/lib/htmx.js" defer="defer" {}
      script src="/assets/js/lib/nprogress.js" defer="defer" {}
      script src="/assets/js/main.js" defer="defer" type="module" {}
      link  rel="icon" type="image/x-icon" href="/assets/images/favicon.ico";
    }
  }
}
