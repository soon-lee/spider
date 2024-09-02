import { styled } from "solid-styled-components";
import IconClear from "@icons/IconClear.tsx";
import Popover from "@widgets/Popover.tsx";

interface InputProps {
  label: string;
  requirements?: string;
  placeholder?: string;
  regex?: string;
  onAction?: () => any;
  width?: string;
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
    text-indent: 5px;
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
    right: 5px;
    position: absolute;
  }
`;
const Input = (props: InputProps) => {
  return (
    <Wrapper width={props.width || "100px"}>
      <Popover content={props.requirements || props.label}>
        <span>{props.label}:</span>
      </Popover>
      <input
        placeholder={props.placeholder || props.label}
        pattern={props.regex || "^.*$"}
      />
      <IconClear size={16} onClick={props.onAction} />
    </Wrapper>
  );
};
export default Input;
