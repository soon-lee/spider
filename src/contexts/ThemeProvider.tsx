import { createContext, createSignal, createEffect, useContext } from "solid-js";

export const ThemeContext = createContext<{
  setTheme: (theme: string) => void;
  theme: () => string;
}>({
  setTheme: () => {},
  theme: () => '',
});
export const ThemeProvider = (props:{children:any}) => {
  const [theme, setTheme] = createSignal(localStorage.getItem("nsp-ux-theme") || 'light');

  createEffect(() => {
    document.documentElement.setAttribute('theme', theme());
    localStorage.setItem("nsp-ux-theme", theme());
  });

  return (
    <ThemeContext.Provider value={{ setTheme, theme }}>
      {props.children}
    </ThemeContext.Provider>
  );
};
export const useTheme = () => useContext(ThemeContext);