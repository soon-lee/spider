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
  regex?: string;
}

const Wrapper = styled.div<{ width: string }>`
  display: flex;
  flex-flow: row nowrap;
  justify-content: flex-end;
  align-items: center;
  gap: 5px;

  input {
    width: ${(props) => props.width};
    height: 15px;
    padding: 5px;
    border-radius: 4px;
    font-size: 16px;
    outline: none;
    border-style: solid;
  }

  input:hover {
    border-color: #66afe9;
  }

  input:focus {
    border-color: #28a745;
  }

  input:invalid {
    border-color: #dc3545;
  }

  input:valid {
    border-color: #28a745;
  }

  input[readonly] {
    cursor: not-allowed;
  }

  input:disabled {
    cursor: not-allowed;
  }

  position: relative;

  > svg {
    position: absolute;
    right: 0;
  }
`;
const LabelWrapper = styled.label`
  flex-grow: 1;
  display: flex;
  flex-flow: row nowrap;
  justify-content: flex-end;
  align-items: center;
  gap: 5px;
`;
const Input = (props: InputProps) => {
  return (
    <Wrapper width={props.width || "100px"}>
      <LabelWrapper>
        <TextIcon text={"x"} />
        <span>{props.label}:</span>
      </LabelWrapper>
      <input pattern={props.regex || "^.*$"} />
      <TextIcon text={"x"} />
    </Wrapper>
  );
};
export default Input;
