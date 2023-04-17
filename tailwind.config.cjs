const resolve = require("path").resolve;

// Define a custom color palette for dark mode
const customColors = {
  backgroundColor: "#222222",
  foregroundColor: "#EEEEEE",
  buttonBackgroundColor: "#333333",
  buttonMouseOverBackgroundColor: "#444444",
  buttonPressedBackgroundColor: "#555555",
};

module.exports = {
  content: [
    resolve(__dirname, "index.html"),
    resolve(__dirname, "src/**/*.{vue,ts}"),
  ],
  theme: {
    colors: {
      transparent: "transparent",
      current: "currentColor",
      ...customColors,
    },
  },
};
