import { createEffect, createSignal, For, JSXElement } from "solid-js";
import { styled } from "solid-styled-components";
import TextIcon from "@icons/TextIcon.tsx";
import IconPlus from "@icons/IconPlus.tsx";
import CollapseDown from "@icons/IconCollapseDown.tsx";
import CollapseUp from "@icons/IconCollapseUp.tsx";
import IconCompactDown from "@icons/IconCompactDown.tsx";
import IconCompactUp from "@icons/IconCompactUp.tsx";
import Popover from "@widgets/Popover.tsx";

interface MenuItemProps {
  name: string;
  label: string;
  icon?: JSXElement;
  action?: JSXElement;
  compacted?: boolean;
}

interface MenuGroupProps {
  name: string;
  label: string;
  icon?: JSXElement;
  action?: JSXElement;
  compacted?: boolean;
  collapsed?: boolean;
  items: MenuItemProps[];
}

interface MenuProps {
  title?: JSXElement;
  compacted?: boolean;
  groups: MenuGroupProps[];
  onCompact?: (compacted: boolean) => any;
}

const MenuItemWrapper = styled.li`
  display: flex;
  gap: 10px;
  flex-flow: row nowrap;
  justify-content: space-between;
  align-items: center;
  width: calc(100% - 20px);
  height: calc(100% - 20px);
  padding: 5px;
  margin: 5px;

  span {
    flex-grow: 1;
  }
`;
const MenuGroupWrapper = styled.li`
  width: 100%;
  padding: 0;
  margin: 0;

  ul {
    display: flex;
    flex-flow: column nowrap;
    justify-content: flex-start;
    align-items: flex-start;
    list-style: none;
    width: 100%;
    height: 100%;
    padding: 0;
    margin: 0;

    li:nth-child(1):hover {
      svg {
        cursor: pointer;
      }
    }

    li:nth-child(n + 2):hover {
      background-color: aqua;
      border-radius: 5px;
      cursor: pointer;
    }
  }
`;
const MenuBar = styled.li`
  display: flex;
  gap: 10px;
  flex-flow: row-reverse nowrap;
  justify-content: space-between;
  align-items: center;
  width: calc(100% - 20px);
  padding: 5px;
  margin: 5px;

  svg {
    cursor: pointer;
  }
`;
const MenuWrapper = styled.ol`
  display: flex;
  flex-flow: column nowrap;
  justify-content: flex-start;
  align-items: flex-start;
  list-style: none;
  width: calc(100% - 20px);
  height: calc(100% - 20px);
  padding: 5px;
  margin: 5px;
`;
const MenuItem = (props: MenuItemProps) => {
  return (
    <MenuItemWrapper>
      {props.icon || <TextIcon text={props.name} />}
      {props.compacted ? <></> : <span>{props.label}</span>}
      {props.compacted ? <></> : props.action || <IconPlus size={16} />}
    </MenuItemWrapper>
  );
};
const MenuGroup = (props: MenuGroupProps) => {
  const [collapsed, setCollapsed] = createSignal<boolean>(
    props.collapsed || false,
  );

  createEffect(() => {
    if (props.compacted) {
      setCollapsed(false);
    }
  });

  return (
    <MenuGroupWrapper>
      <ul>
        {props.compacted ? (
          <></>
        ) : (
          <MenuItem
            {...props}
            compacted={props.compacted}
            action={
              collapsed() ? (
                <CollapseDown size={16} onClick={() => setCollapsed(false)} />
              ) : (
                <CollapseUp size={16} onClick={() => setCollapsed(true)} />
              )
            }
          />
        )}
        {collapsed() ? (
          <></>
        ) : (
          <For each={props.items}>
            {(item) => <MenuItem {...item} compacted={props.compacted} />}
          </For>
        )}
      </ul>
    </MenuGroupWrapper>
  );
};
const Menu = (props: MenuProps) => {
  const [compacted, setCompacted] = createSignal<boolean>(
    props.compacted || false,
  );
  const [collapsed, setCollapsed] = createSignal<boolean>(false);

  const handleCompact = () => {
    setCompacted((prev) => !prev);
    props.onCompact?.(compacted());
  };
  return (
    <MenuWrapper>
      <MenuBar>
        {compacted() ? (
          <Popover content={"点击展开菜单"} position="top-right">
            <IconCompactDown onClick={handleCompact} />
          </Popover>
        ) : (
          <Popover content={"点击折叠菜单"}>
            <IconCompactUp onClick={handleCompact} />
          </Popover>
        )}
        {compacted() ? <></> : props.title || ""}
      </MenuBar>
      <For each={props.groups}>
        {(group) => (
          <MenuGroup
            {...group}
            compacted={compacted()}
            collapsed={collapsed()}
          />
        )}
      </For>
    </MenuWrapper>
  );
};
export default Menu;
