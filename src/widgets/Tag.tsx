import { styled } from "solid-styled-components";
import { JSXElement } from "solid-js";

interface TagProps {
  children: JSXElement;
  color?: "success" | "warning" | "error" | "info" | "primary" | string;
  onClick?: () => any;
}

const Wrapper = styled.div<{ color?: string }>`
  display: inline-flex;
  flex-flow: row nowrap;
  gap: 5px;
  padding: 5px;
  font-size: 13px;
  border-radius: 3px;
  background-color: ${(props) => {
    if (!props.color) return `rgba(22, 119, 255, 0.3)`;
    switch (props.color) {
      case "primary":
        return `rgba(22, 119, 255, 0.3)`;
      case "success":
        return `rgba(0, 180, 42, 0.3)`;
      case "warning":
        return `rgba(255, 235, 0, 0.3)`;
      case "error":
        return `rgba(255, 0, 0, 0.3)`;
      case "info":
        return `rgba(0, 180, 255, 0.3)`;
      default:
        return `rgba("${props.color}",0.3)`;
    }
  }};
  border: solid 1px
    ${(props) => {
      if (!props.color) return `rgba(22, 119, 255, 0.7)`;
      switch (props.color) {
        case "primary":
          return `rgba(22, 119, 255, 0.7)`;
        case "success":
          return `rgba(0, 180, 42, 0.7)`;
        case "warning":
          return `rgba(255, 235, 0, 0.7)`;
        case "error":
          return `rgba(255, 0, 0, 0.7)`;
        case "info":
          return `rgba(0, 180, 255, 0.7)`;
        default:
          return `rgba("${props.color}",0.7)`;
      }
    }};
  color: ${(props) => {
    if (!props.color) return "#1677ff";
    switch (props.color) {
      case "primary":
        return "#1677ff";
      case "success":
        return "#00b42a";
      case "warning":
        return "#ffc300";
      case "error":
        return "#ff0000";
      case "info":
        return "#00b4ff";
      default:
        return props.color;
    }
  }};
`;
const Tag = (props: TagProps) => {
  return <Wrapper color={props.color}>{props.children}</Wrapper>;
};
export default Tag;
