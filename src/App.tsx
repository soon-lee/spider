import "./App.css";
import { useTheme } from "@contexts/ThemeProvider";
import { useLocale } from "@contexts/LocaleProvider";
import JuLayout from "@containers/JuLayout";
import { Route, Router } from "@solidjs/router";
import Menu from "@components/Menu.tsx";
import { createEffect, createSignal } from "solid-js";
import Popover from "@widgets/Popover.tsx";
import Tag from "@widgets/Tag.tsx";
import TextIcon from "@icons/TextIcon.tsx";
import Mapping from "@components/Mapping.tsx";
import MappingPane from "@components/MappingPane.tsx";
import MapperPane from "@components/MapperPane.tsx";
import { IconText } from "@uxy-ui/icons";
import { useDialog } from "@uxy-ui/contexts";

const App = () => {
  const { getTranslation, setLocale, locale } = useLocale();
  const { setTheme, theme } = useTheme();
  const [asideWidth, setAsideWidth] = createSignal<string>("200px");

  const { setShow, setContent } = useDialog();

  createEffect(() => {
    setShow(true);
    setContent("test");
  });

  return (
    <JuLayout
      asideWidth={asideWidth()}
      header={
        <div class="content">
          <button
            onClick={() => setLocale(locale() === "zh-CN" ? "en-US" : "zh-CN")}
          >
            {locale() === "zh-CN" ? "en-US" : "zh-CN"}
          </button>
          <button
            onClick={() => setTheme(theme() === "light" ? "dark" : "light")}
          >
            {theme() === "light" ? "dark" : "light"}
          </button>
          <button
            onClick={() => {
              setShow(true);
              setContent("test");
            }}
          >
            show dialog
          </button>
        </div>
      }
      aside={
        <Menu
          onCompact={(compacted) =>
            compacted ? setAsideWidth("64") : setAsideWidth("200px")
          }
          title={<Popover content={"可折叠的菜单"}>caidan项</Popover>}
          groups={[
            {
              name: "prepare",
              label: "爬虫准备",
              // collapsed: false,
              // compacted: false,
              items: [
                { name: "website", label: "网站" },
                { name: "meta", label: "结构" },
              ],
            },
            {
              name: "construction",
              label: "爬虫构建",
              // collapsed: false,
              // compacted: false,
              items: [
                { name: "extractor", label: "提取器" },
                {
                  name: "extractor",
                  label: "提取器",
                },
                { name: "relation", label: "关系处理" },
              ],
            },
            {
              name: "爬虫构建",
              label: "爬虫构建",
              // collapsed: false,
              // compacted: false,
              items: [
                { name: "extractor", label: "提取器" },
                {
                  name: "extractor",
                  label: "提取器",
                },
                { name: "relation", label: "关系处理" },
              ],
            },
          ]}
        />
      }
      main={
        <Router>
          <Route
            path="/"
            component={() => (
              <div
                style={{
                  display: "flex",
                  "flex-flow": "column nowrap",
                  gap: "10px",
                }}
              >
                <Popover content={"这是top上面"} position={"top-left"}>
                  <span style={{ width: "100%", "background-color": "red" }}>
                    top
                  </span>
                </Popover>
                <Popover content={"这是bottom下面"} position={"bottom"}>
                  <span style={{ width: "100%", "background-color": "red" }}>
                    bottom
                  </span>
                </Popover>
                <Popover content={"这是left左边"} position={"left"}>
                  <span style={{ width: "100%", "background-color": "red" }}>
                    left
                  </span>
                </Popover>
                <Popover content={"这是right右边"} position={"right"}>
                  <span style={{ width: "100%", "background-color": "red" }}>
                    right
                  </span>
                </Popover>
                <Tag>hdfihsdi</Tag>
                <Tag>
                  <TextIcon text={"hdfihsdi"} width={16} height={16} />
                  hdfihsdi
                </Tag>
                <Mapping
                  data={[
                    {
                      field: "field1",
                      selector: "css",
                      expression: "html div",
                      operator: "text",
                      regex: "d+",
                    },
                  ]}
                  editedIndex={[1]}
                />
                <MappingPane
                  field={"count"}
                  expression={"html a span"}
                  operation={"count"}
                  regex={""}
                />
                <MapperPane
                  field={"site"}
                  expression={"html body div"}
                  sequential={false}
                  mappings={["index", "cover"]}
                />
                <IconText text={""} />
                <IconText text={"hello world"} />
              </div>
            )}
          />
          <Route path="/main" component={() => <div>Main</div>} />
          <Route path="/help" component={() => <div>Help</div>} />
          <Route path="*404" component={() => <div>404</div>} />
        </Router>
      }
    />
  );
};

export default App;
