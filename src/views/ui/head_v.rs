use vy::prelude::*;

pub fn render_head() -> impl IntoHtml {
    head!(
        meta!(charset = "UTF-8"),
        meta!(
            name = "viewport",
            content = "width=device-width,initial-scale=1"
        ),
        link!(rel = "stylesheet", href = "/assets/css/lib/tailwind.css"),
        link!(rel = "stylesheet", href = "/assets/css/lib/toast.css"),
        link!(rel = "stylesheet", href = "/assets/css/lib/nprogress.css"),
        link!(
            rel = "icon",
            "type" = "image/x-icon",
            href = "/assets/images/favicon.ico"
        ),
        script!(src = "/assets/js/lib/htmx.js", defer = "defer"),
        script!(src = "/assets/js/lib/nprogress.js", defer = "defer"),
        script!(
            src = "/assets/js/main.js",
            "type" = "module",
            defer = "defer"
        )
    )
}