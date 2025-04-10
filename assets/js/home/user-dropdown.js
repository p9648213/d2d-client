export function setupUserDropdown() {
  const user_dropdown = document.getElementById("user-dropdown");
  const user_dropdown_options = document.getElementById(
    "user-dropdown-options"
  );

  user_dropdown.addEventListener("click", () => {
    user_dropdown_options.classList.toggle("hidden");
  });
}
