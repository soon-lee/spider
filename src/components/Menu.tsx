import {For, JSXElement} from "solid-js";
import {styled} from "solid-styled-components";
import Balloon from "@icons/Balloon.tsx";
import CompactUp from "@icons/CompactUp.tsx";
import CompactDown from "@icons/CompactDown.tsx";

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
    width: calc(100% - 25px);
    height: calc(100% - 30px);
    list-style: none;
    padding: 0 0 0 25px;
    margin: 15px 0;
`;
const MenuItemWrapper = styled.li`
    width: 100%;
    display: inline-flex;
    flex-flow: row nowrap;
    justify-content: space-between;
`;
const SubMenuWrapper = styled.ul`
    width: 100%;
    height: 100%;
    list-style: none;
    padding: 0;
`;
const SubMenuItemWrapper = styled.li`
    width: 100%;
    display: flex;
    flex-flow: row nowrap;
`;
const SubMenu = (props: MenuItem) => {
    return (
        <SubMenuWrapper>
            <SubMenuItemWrapper>
                {props.icon ? props.icon : <Balloon color={"red"}/>}
                <span>{props.label ? props.label : ""}</span>
                {props.collapsed ? <CompactDown/> : <CompactUp/>}
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