import { For, JSXElement, Show } from "solid-js";
import { styled } from "solid-styled-components";

interface SubMenuItem {
  name: string;
  label: string;
  icon?: JSXElement;
  action?: JSXElement;
}
interface MenuItem {
  name: string;
  label: string;
  icon?: JSXElement;
  collapsed: boolean;
  children?: SubMenuItem[];
}

const MenuWrapper = styled.ol`

`;
const MenuItemWrapper = styled.li`

`;
const SubMenuWrapper = styled.ul`

`;
const SubMenuItemWrapper = styled.li`

`;
const SubMenu = (props: MenuItem) => {
  return (
    <SubMenuWrapper>
      <SubMenuItemWrapper>
        {props.icon ? props.icon : <svg></svg>}
        <span>{props.label ? props.label : ""}</span>
        {props.collapsed ? <svg></svg> : <svg></svg>}
      </SubMenuItemWrapper>
      <For each={props.children}>
        {
          (item) => <SubMenuItemWrapper>
            {item.icon ? item.icon : <svg></svg>}
            <span>{item.label ? item.label : ""}</span>
            {item.action ? item.action : <svg></svg>}
          </SubMenuItemWrapper>
        }
      </For>
    </SubMenuWrapper>
  );
}
const Menu = (props: { items: MenuItem[] }) => {
  return (
    <MenuWrapper>
      <For each={props.items}>
        {
          (item) => <MenuItemWrapper>{<SubMenu {...item} />}</MenuItemWrapper>
        }
      </For>
    </MenuWrapper>
  );
}
export default Menu;