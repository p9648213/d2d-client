export function scrollToTop() {
  window.scrollTo(0, 0);
}

function escapeHtmlText(value) {
  const stringValue = value.toString();
  const entityMap = {
    "&": "&amp;",
    "<": "&lt;",
    ">": "&gt;",
    '"': "&quot;",
    "'": "&#x27;",
    "/": "&#x2F;",
    "`": "&grave;",
    "=": "&#x3D;",
  };

  const regex = /[&<>"'`=/]/g;
  return stringValue.replace(regex, (match) => entityMap[match]);
}

function toast({ message = "", type = "info", duration = 3000 }) {
  const main = document.getElementById("toast");

  if (main) {
    if (main.childNodes.length > 0) {
      main.innerHTML = "";
    }

    const toast = document.createElement("div");

    const icons = {
      success: "🎉",
      info: "🔧",
      warning: "⚠️",
      error: "💥",
    };
    const icon = icons[type];
    const delay = (duration / 1000).toFixed(2);

    toast.classList.add("toast", `toast--${type}`);
    toast.style.animation = `slideInLeft ease .3s, fadeOut linear 1s ${delay}s forwards`;

    toast.innerHTML = `
      <div class="toast__icon">
          ${icon}
      </div>
      <div class="toast__body">
          <p class="toast__msg">${escapeHtmlText(message)}</p>
      </div>
    `;

    main.appendChild(toast);

    setTimeout(() => {
      main.innerHTML = "";
    }, duration + 1000);
  }
}

window.addEventListener("toastmessage", function(event) {
  if (event?.detail?.type === "success") {
    toast({
      message: event?.detail?.message,
      type: "success",
    });
  }
});

htmx.config.defaultSettleDelay = 0;

window.addEventListener("htmx:beforeRequest", function() {
  NProgress.start();

  const loginLinkEl = document.getElementById("login-link");
  const registerLinkEl = document.getElementById("register-link");

  if (loginLinkEl) {
    loginLinkEl.classList.add("disable-link");
  }

  if (registerLinkEl) {
    registerLinkEl.classList.add("disable-link");
  }
});

window.addEventListener("htmx:afterRequest", function(event) {
  const loginLinkEl = document.getElementById("login-link");
  const registerLinkEl = document.getElementById("register-link");

  if (loginLinkEl) {
    loginLinkEl.classList.remove("disable-link");
  }

  if (registerLinkEl) {
    registerLinkEl.classList.remove("disable-link");
  }

  if (event?.detail?.failed) {
    NProgress.done();

    if (event?.detail?.xhr?.responseText) {
      toast({
        message: event?.detail?.xhr?.responseText,
        type: "error",
      });
    }
  }
});

window.addEventListener("htmx:configRequest", function(event) {
  if (event.detail.verb !== "get") {
    event.detail.headers["X-Csrf-Protection"] = "1";
  }
});

window.addEventListener("htmx:afterSettle", function() {
  NProgress.done();
});

window.addEventListener("htmx:historyRestore", (_) => {
  document.body.style.visibility = "hidden";
  window.location.reload();
});
