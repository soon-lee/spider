import { createSignal, JSXElement } from "solid-js";
import { styled } from "solid-styled-components";
import IconCollapseDown from "@icons/IconCollapseDown.tsx";
import IconCollapseUp from "@icons/IconCollapseUp.tsx";

interface CollapsePaneProps {
  title?: JSXElement;
  children: JSXElement;
}

const Wrapper = styled.div`
  padding: 5px;
  width: calc(100% - 10px);
  display: flex;
  flex-flow: column nowrap;
  align-items: center;
  justify-content: flex-start;
`;
const Header = styled.div<{ collapsed: boolean }>`
  display: flex;
  flex-flow: row nowrap;
  justify-content: space-between;
  align-items: center;
  padding: 5px;
  width: calc(100% - 10px);
  cursor: pointer;
  border-bottom: ${(props) => (props.collapsed ? "none" : "1px solid #ccc")};

  form {
    flex-grow: 1;
  }
`;
const Content = styled.div`
  padding: 5px;
  width: calc(100% - 10px);
  flex-grow: 1;
`;

const CollapsePane = (props: CollapsePaneProps) => {
  const [collapsed, setCollapsed] = createSignal(false);
  return (
    <Wrapper>
      <Header collapsed={collapsed()}>
        {collapsed() ? (
          <IconCollapseDown size={16} onClick={() => setCollapsed(false)} />
        ) : (
          <IconCollapseUp size={16} onClick={() => setCollapsed(true)} />
        )}
        {props.title}
      </Header>
      {collapsed() ? <></> : <Content>{props.children}</Content>}
    </Wrapper>
  );
};
export default CollapsePane;
