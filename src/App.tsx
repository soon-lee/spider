import './App.css';
import {useTheme} from '@contexts/ThemeProvider';
import {useLocale} from '@contexts/LocaleProvider';
import JuLayout from '@containers/JuLayout';
import {Route, Router} from '@solidjs/router';
import Menu from "@components/Menu.tsx";

const App = () => {
    const {getTranslation, setLocale, locale} = useLocale();
    const {setTheme, theme} = useTheme();

    return (
        <JuLayout
            header={
                <div class="content">
                    <button
                        onClick={() => setLocale(locale() === "zh-CN" ? "en-US" : "zh-CN")}>{locale() === "zh-CN" ? "en-US" : "zh-CN"}</button>
                    <button
                        onClick={() => setTheme(theme() === "light" ? "dark" : "light")}>{theme() === "light" ? "dark" : "light"}</button>
                </div>
            }
            aside={
                <Menu items={[{
                    name: "prepare",
                    label: "爬虫准备",
                    collapsed: true,
                    children: [{name: "website", label: "网站"}, {name: "meta", label: "结构"}]
                }, {
                    name: "construction",
                    label: "爬虫构建",
                    collapsed: true,
                    children: [{name: "extractor", label: "提取器"}, {
                        name: "extractor",
                        label: "提取器"
                    }, {name: "relation", label: "关系处理"}]
                }, {
                    name: "爬虫构建",
                    label: "爬虫构建",
                    collapsed: true,
                    children: [{name: "extractor", label: "提取器"}, {
                        name: "extractor",
                        label: "提取器"
                    }, {name: "relation", label: "关系处理"}]
                }]}/>
            }
            main={
                <Router>
                    <Route path="/" component={() => <div>Home</div>}/>
                    <Route path="/main" component={() => <div>Main</div>}/>
                    <Route path="/help" component={() => <div>Help</div>}/>
                    <Route path="*404" component={() => <div>404</div>}/>
                </Router>
            }/>
    );
};

export default App;
