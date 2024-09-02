import { JSXElement } from "solid-js";
import { styled } from "solid-styled-components";

interface PopoverProps {
  children: JSXElement;
  content: JSXElement;
  position?: "top" | "bottom" | "left" | "right" | "top-left" | "top-right";
}

const Wrapper = styled.div<{ position?: string }>`
  display: flex;
  flex-flow: row nowrap;
  justify-content: center;
  align-items: center;
  position: relative;
  cursor: pointer;

  &:hover {
    > div:nth-child(1) {
      display: block;
      ${(props) => {
        switch (props.position) {
          case "top": {
            return `
          top: -40px;
          left: 50%;
          transform: translateX(-50%);
        `;
          }
          case "top-left": {
            return `
          top: -40px;
          left: 75%;
          transform: translateX(-20%);
        `;
          }
          case "top-right": {
            return `
          top: -40px;
          left: 25%;
          transform: translateX(-20%);
        `;
          }
          case "bottom": {
            return `
          bottom: -40px;
          left: 50%;
          transform: translateX(-50%);
        `;
          }
          case "left": {
            return `
          top:50%;
          left: -25px;
          transform: translate(-50%,-50%)  rotate(90deg);
        `;
          }
          case "right": {
            return `
          top:50%;
          right: -25px;
          transform: translate(50%,-50%)  rotate(-90deg);
        `;
          }
        }
      }}
    }

    &:after {
      position: absolute;
      content: "";
      width: 0;
      height: 0;
      border: 10px solid;
      ${(props) => {
        switch (props.position) {
          case "top": {
            return `
          top: -10px;
          left: 50%;
          transform: translateX(-50%);
          border-color: aquamarine transparent transparent transparent;
        `;
          }
          case "top-left": {
            return `
          top: -10px;
          left: 75%;
          transform: translateX(-20%);
          border-color: aquamarine transparent transparent transparent;
        `;
          }
          case "top-right": {
            return `
          top: -10px;
          left: 25%;
          transform: translateX(-20%);
          border-color: aquamarine transparent transparent transparent;
        `;
          }
          case "bottom": {
            return `
          bottom: -10px;
          left: 50%;
          transform: translateX(-50%);
          border-color: transparent transparent aquamarine transparent;
        `;
          }
          case "left": {
            return `
          top: 50%;
          left: -10px;
          transform: translateY(-50%);
          border-color: transparent transparent transparent aquamarine ;
        `;
          }
          case "right": {
            return `
          top: 50%;
          right: -10px;
          transform: translateY( -50%);
          border-color: transparent aquamarine transparent transparent;
        `;
          }
        }
      }}
    }
  }
`;
const Popup = styled.div`
  position: absolute;
  z-index: calc(1 / 0);
  height: 20px;
  padding: 5px;
  display: none;
  white-space: nowrap;
  background-color: aquamarine;
  border-radius: 3px;
`;
const Content = styled.div`
  width: 100%;
  height: 100%;
  display: flex;
  flex-flow: row nowrap;
  justify-content: center;
  align-items: center;
`;
const Popover = (props: PopoverProps) => {
  return (
    <Wrapper position={props.position || "top"}>
      <Popup>{props.content}</Popup>
      <Content>{props.children}</Content>
    </Wrapper>
  );
};
export default Popover;
