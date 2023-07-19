import "./styles.pcss";
import "normalize.css"
import App from "./App.svelte";

const app = new App({
  target: document.getElementById("app"),
});

export default app;
document.oncontextmenu = () => false;