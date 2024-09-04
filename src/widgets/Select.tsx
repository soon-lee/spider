import { styled } from "solid-styled-components";
import { For } from "solid-js";
import Popover from "./Popover";

interface SelectOption {
  label: string;
  value: string;
}

interface SelectProps {
  label: string;
  value: string;
  requirements?: string;
  options: SelectOption[];
  onChange?: (value: string) => any;
  width?: string;
}

const Wrapper = styled.div<{ width: string }>`
  display: inline-flex;
  flex-flow: row nowrap;
  justify-content: flex-end;
  align-items: center;

  select {
    width: ${(props) => props.width};
    padding: 5px;
    color: #333;
    border: 1px solid #ccc;
    border-radius: 4px;
    background-position: right 8px center;
    background-repeat: no-repeat;
    cursor: pointer;
  }

  select:hover {
    border-color: #66afe9;
  }

  select:focus {
    border-color: #28a745;
    box-shadow: 0 0 0 0.2rem rgba(40, 167, 69, 0.25);
  }
`;
const Select = (props: SelectProps) => {
  return (
    <Wrapper width={props.width || "100px"}>
      <Popover content={props.requirements || props.label}>
        <span>{props.label}:</span>
      </Popover>
      <select
        value={props.value}
        onChange={(e) => props.onChange?.(e.target.value)}
      >
        <For each={props.options}>
          {(option) => <option value={option.value}>{option.label}</option>}
        </For>
      </select>
    </Wrapper>
  );
};
export default Select;
