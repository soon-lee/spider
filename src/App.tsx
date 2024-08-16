import './App.css';
import { useTheme } from '@contexts/ThemeProvider';
import { useLocale } from '@contexts/LocaleProvider';

const App = () => {
const {getTranslation,setLocale,locale} = useLocale();
const {setTheme,theme} = useTheme();

  return (
    <div class="content">
      <h1>Rsbuild with Solid</h1>
      <p>Start building amazing things with Rsbuild.</p>
      <span>{getTranslation("greeting")}</span>
      <button onClick={()=>setLocale(locale() === "zh-CN" ? "en-US" :"zh-CN")}>{locale() === "zh-CN" ? "en-US" :"zh-CN"}</button>
      <button onClick={()=>setTheme(theme()==="light" ? "dark":"light")}>{theme()==="light" ? "dark":"light"}</button>
    </div>
  );
};

export default App;
