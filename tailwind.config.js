/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        surface: {
          0: "#110e0c",
          1: "#1a1613",
          2: "#231e1b",
          3: "#2e2824",
          4: "#3a332e",
        },
        border: {
          DEFAULT: "#3e3530",
          strong: "#524840",
        },
        primary: {
          DEFAULT: "#b5342a",
          hover: "#9c2c23",
          light: "#e06b62",
        },
        accent: {
          green: "#22c55e",
          red: "#ef4444",
          "red-dark": "#b91c1c",
          orange: "#f97316",
          yellow: "#f59e0b",
          blue: "#3b82f6",
          cyan: "#06b6d4",
        },
      },
      fontFamily: {
        sans: ["Inter", "system-ui", "sans-serif"],
        mono: ["JetBrains Mono", "Fira Code", "monospace"],
      },
    },
  },
  plugins: [],
};
