const fs = require("fs");
const path = require("path");
const UglifyJS = require("uglify-js");

// Directory to look for JS files
const jsDir = "assets/js";

// Read files in directory
fs.readdir(jsDir, (err, files) => {
  if (err) {
    console.error("Error reading directory:", err);
    return;
  }

  files.forEach((file) => {
    if (!file.endsWith(".js")) {
      return;
    }

    const filePath = path.join(jsDir, file);

    // Read and minify file
    const result = UglifyJS.minify(fs.readFileSync(filePath, "utf8"), {
      compress: true,
      mangle: true,
      toplevel: true,
    });

    if (result.error) {
      console.error("Error minifying", file, result.error);
      return;
    }

    // Write the minified content to a new file
    fs.writeFileSync(filePath, result.code);
    console.log(`Minified ${file} to ${filePath}`);
  });
});
