export function setupUserDropdown() {
  const user_dropdown = document.getElementById("user-dropdown");
  const user_dropdown_options = document.getElementById(
    "user-dropdown-options"
  );

  function closeDropDown() {
    user_dropdown_options.classList.remove("flex");
    user_dropdown_options.classList.add("hidden");
  }

  function openDropDown() {
    user_dropdown_options.classList.remove("hidden");
    user_dropdown_options.classList.add("flex");
  }

  user_dropdown.addEventListener("click", () => {
    if (user_dropdown_options.classList.contains("hidden")) {
      openDropDown();
    } else {
      closeDropDown();
    }
  });

  document.addEventListener("click", (event) => {
    if (
      !user_dropdown_options.contains(event.target) &&
      !user_dropdown.contains(event.target)
    ) {
      if (user_dropdown_options.classList.contains("flex")) {
        closeDropDown();
      }
    } else if (event.target.name == "dropdown-item") {
      closeDropDown();
    }
  });
}
