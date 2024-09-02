import { styled } from "solid-styled-components";
import { JSXElement } from "solid-js";
import TextIcon from "@icons/TextIcon.tsx";

interface ButtonProps {
  icon?: JSXElement;
  label: string;
  onClick?: () => any;
}

const Wrapper = styled.button`
  display: inline-flex;
  flex-flow: row nowrap;
  justify-content: flex-start;
  align-items: center;
  gap: 5px;
  cursor: pointer;
`;
const Button = (props: ButtonProps) => {
  return (
    <Wrapper onClick={props.onClick}>
      {props.icon || <TextIcon text={props.label} />}
      {props.label}
    </Wrapper>
  );
};
export default Button;
