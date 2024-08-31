import { styled } from "solid-styled-components";
import TextIcon from "@icons/TextIcon.tsx";
import { JSXElement } from "solid-js";

interface InputProps {
  icon?: JSXElement;
  label?: string;
  placeholder?: string;
  action?: JSXElement;
  validator?: (value: string) => boolean;
  tooltip?: {
    success?: string;
    failure?: string;
  };
  width?: string;
}

const Wrapper = styled.div<{ width: string }>`
  display: flex;
  flex-flow: row nowrap;
  justify-content: center;
  align-items: center;

  input {
    width: ${(props) => props.width};
  }
`;
const LabelWrapper = styled.label`
  flex-grow: 1;
`;
const Input = (props: InputProps) => {
  return (
    <Wrapper width={props.width || "100px"}>
      <LabelWrapper>
        <TextIcon text={"x"} />
        <span></span>
      </LabelWrapper>
      <input />
      <TextIcon text={"x"} />
    </Wrapper>
  );
};
export default Input;
