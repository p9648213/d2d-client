export function scrollToTop() {
  window.scrollTo(0, 0);
}

//..............................................................
//.TTTTTTTTTTT..OOOOOOO........AAAA.......SSSSSSS...SSTTTTTTTT..
//.TTTTTTTTTTT.OOOOOOOOOO.....AAAAAA.....SSSSSSSSS..SSTTTTTTTT..
//.TTTTTTTTTTTOOOOOOOOOOOO....AAAAAA....ASSSSSSSSSS.SSTTTTTTTT..
//....TTTT...TOOOO...OOOOO....AAAAAAA...ASSS...SSSS.....TTTT....
//....TTTT...TOOO.....OOOOO..AAAAAAAA...ASSSS...........TTTT....
//....TTTT...TOOO......OOOO..AAAAAAAA....SSSSSSS........TTTT....
//....TTTT...TOOO......OOOO..AAAA.AAAA....SSSSSSSS......TTTT....
//....TTTT...TOOO......OOOO.AAAAAAAAAA......SSSSSSS.....TTTT....
//....TTTT...TOOO.....OOOOO.AAAAAAAAAAA........SSSSS....TTTT....
//....TTTT...TOOOOO..OOOOO.OAAAAAAAAAAA.ASSS...SSSSS....TTTT....
//....TTTT....OOOOOOOOOOOO.OAAA....AAAA.ASSSSSSSSSS.....TTTT....
//....TTTT.....OOOOOOOOOO..OAAA....AAAAA.SSSSSSSSSS.....TTTT....
//....TTTT......OOOOOOO...OOAAA.....AAAA..SSSSSSS.......TTTT....
//..............................................................

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

window.addEventListener("toastmessage", function (event) {
  if (event?.detail?.type === "success") {
    toast({
      message: event?.detail?.message,
      type: "success",
    });
  }
});

//....................................................
//.HHHH...HHHH..TTTTTTTTTTTMMMMM...MMMMMMMXXXX..XXXX..
//.HHHH...HHHH..TTTTTTTTTTTMMMMM...MMMMMM.XXXX..XXXX..
//.HHHH...HHHH..TTTTTTTTTTTMMMMM...MMMMMM.XXXXXXXXXX..
//.HHHH...HHHH.....TTTT...TMMMMMM.MMMMMMM..XXXXXXXX...
//.HHHH...HHHH.....TTTT...TMMMMMM.MMMMMMM...XXXXXX....
//.HHHHHHHHHHH.....TTTT...TMMMMMM.MMMMMMM...XXXXXX....
//.HHHHHHHHHHH.....TTTT...TMMMMMMMMMMMMMM...XXXXX.....
//.HHHHHHHHHHH.....TTTT...TMMMMMMMMMMMMMM...XXXXXX....
//.HHHH...HHHH.....TTTT...TMMMMMMMMMMMMMM..XXXXXXXX...
//.HHHH...HHHH.....TTTT...TMMM.MMMMM.MMMM..XXXXXXXX...
//.HHHH...HHHH.....TTTT...TMMM.MMMMM.MMMM.XXXX.XXXXX..
//.HHHH...HHHH.....TTTT...TMMM.MMMMM.MMMMMXXXX..XXXX..
//.HHHH...HHHH.....TTTT...TMMM.MMMMM.MMMMMXXX....XXX..
//....................................................
htmx.config.defaultSettleDelay = 0;

window.addEventListener("htmx:beforeRequest", function (event) {
  NProgress.start();

  if (event?.detail?.pathInfo?.requestPath?.includes("/rso/location")) {
    const locationDropdownEl = document.getElementById("location-dropdown");
    locationDropdownEl.classList.add("disable-link");
  }

  if (event?.detail?.pathInfo?.requestPath?.includes("/rso/property-types")) {
    const propertyTypeEl = document.getElementById("property-types-dropdown");
    propertyTypeEl.classList.add("disable-link");
  }

  const loginLinkEl = document.getElementById("login-link");
  const registerLinkEl = document.getElementById("register-link");

  if (loginLinkEl) {
    loginLinkEl.classList.add("disable-link");
  }

  if (registerLinkEl) {
    registerLinkEl.classList.add("disable-link");
  }
});

window.addEventListener("htmx:afterRequest", function (event) {
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

window.addEventListener("htmx:configRequest", function (event) {
  if (event.detail.verb !== "get") {
    event.detail.headers["X-Csrf-Protection"] = "1";
  }
});

window.addEventListener("htmx:afterSettle", function () {
  NProgress.done();
});

window.addEventListener("htmx:historyRestore", (_) => {
  document.body.style.visibility = "hidden";
  window.location.reload();
});
