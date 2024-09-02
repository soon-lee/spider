import { styled } from "solid-styled-components";
import Popover from "@widgets/Popover.tsx";
import Tag from "@widgets/Tag.tsx";

interface InformationProps {
  label: string;
  info?: string;
  value: string;
}

const Wrapper = styled.div`
  display: inline-flex;
  flex-flow: row nowrap;
  justify-content: flex-end;
  align-items: center;
  gap: 5px;
`;
const Information = (props: InformationProps) => {
  return (
    <Wrapper>
      <Popover content={props.info || props.label}>{props.label}:</Popover>
      <Tag>{props.value}</Tag>
    </Wrapper>
  );
};
export default Information;
