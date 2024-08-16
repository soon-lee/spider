import './App.css';
import { useTheme } from '@contexts/ThemeProvider';
import { useLocale } from '@contexts/LocaleProvider';
import JuLayout from '@containers/JuLayout';

const App = () => {
  const { getTranslation, setLocale, locale } = useLocale();
  const { setTheme, theme } = useTheme();

  return (
    <JuLayout
      header={
        <div class="content">
          <button onClick={() => setLocale(locale() === "zh-CN" ? "en-US" : "zh-CN")}>{locale() === "zh-CN" ? "en-US" : "zh-CN"}</button>
          <button onClick={() => setTheme(theme() === "light" ? "dark" : "light")}>{theme() === "light" ? "dark" : "light"}</button>
        </div>
      }
      main={
        <div><h1>Rsbuild with Solid</h1>
          <p>Start building amazing things with Rsbuild.</p>
          <span>{getTranslation("greeting")}</span></div>
      } />
  );
};

export default App;
