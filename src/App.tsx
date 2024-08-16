import './App.css';
import { useTheme } from '@contexts/ThemeProvider';
import { useLocale } from '@contexts/LocaleProvider';
import JuLayout from '@containers/JuLayout';
import { Router, Route } from '@solidjs/router';

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
        <Router>
          <Route path="/" component={()=><div>Home</div>} />
          <Route path="/main" component={()=><div>Main</div>} />
          <Route path="/help" component={()=><div>Help</div>} />
          <Route path="*404" component={()=><div>404</div>} />
        </Router>
      } />
  );
};

export default App;
