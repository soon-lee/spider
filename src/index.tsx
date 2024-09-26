import { render } from "solid-js/web";
import App from "./App";
import GlobalProvider from "@contexts/GlobalProvider";
import { DialogProvider } from "@uxy-ui/contexts";

const root = document.getElementById("root");
if (root) {
  render(
    () => (
      <GlobalProvider>
        <DialogProvider>
          <App />
        </DialogProvider>
      </GlobalProvider>
    ),
    root,
  );
}
