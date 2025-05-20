export function setupAuthModal() {
  const sign_in_button = document.getElementById("sign-in-button");

  const login_modal = document.getElementById("login-modal");
  const login_close_button = document.getElementById("login-close-button");
  const login_link = document.getElementById("login-link");

  const register_modal = document.getElementById("register-modal");
  const register_close_button = document.getElementById(
    "register-close-button"
  );
  const register_link = document.getElementById("register-link");

  function closeModal(element) {
    element.classList.remove("flex");
    element.classList.add("hidden");
  }

  function openModal(element) {
    element.classList.remove("hidden");
    element.classList.add("flex");
  }

  sign_in_button.addEventListener("click", () => {
    if (login_modal.classList.contains("hidden")) {
      openModal(login_modal);
    }
  });

  login_link.addEventListener("click", () => {
    closeModal(register_modal);
    openModal(login_modal);
  });

  register_link.addEventListener("click", () => {
    closeModal(login_modal);
    openModal(register_modal);
  });

  login_close_button.addEventListener("click", () => {
    if (login_modal.classList.contains("flex")) {
      login_modal.classList.remove("flex");
      login_modal.classList.add("hidden");
    }
  });

  register_close_button.addEventListener("click", () => {
    if (register_modal.classList.contains("flex")) {
      register_modal.classList.remove("flex");
      register_modal.classList.add("hidden");
    }
  });
}
