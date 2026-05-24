import type { Config } from "tailwindcss";

const config: Config = {
  content: ["./index.html", "./src/**/*.{ts,tsx}"],
  theme: {
    extend: {
      colors: {
        ink: "#0f172a",
        mist: "#f1f5f9",
        marine: "#134e4a",
        sand: "#f8fafc"
      }
    }
  },
  plugins: []
};

export default config;
