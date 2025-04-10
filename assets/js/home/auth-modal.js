export function setupAuthModal() {
  const sign_in_button = document.getElementById("sign-in-button");
  const login_modal = document.getElementById("login-modal");
  const login_close_button = document.getElementById("login-close-button");

  sign_in_button.addEventListener("click", () => {
    if (login_modal.classList.contains("hidden")) {
      login_modal.classList.remove("hidden");
      login_modal.classList.add("flex");
    }
  });

  login_close_button.addEventListener("click", () => {
    if (login_modal.classList.contains("flex")) {
      login_modal.classList.remove("flex");
      login_modal.classList.add("hidden");
    }
  });
}
