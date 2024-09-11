import { JSXElement } from "solid-js";
import { styled } from "solid-styled-components";

interface RowLayoutProps {
  children?: Array<JSXElement>;
  gap?: string;
}

const RowList = styled.ul<{ gap?: string }>`
  display: flex;
  flex-flow: row nowrap;
  justify-content: flex-start;
  align-content: center;
  gap: ${(props) => props.gap || "0"};
  list-style: none;
  padding: 0;
  margin: 0;
  width: 100%;
  height: 100%;
`;
const RowItem = styled.li`
  margin: 0;
  display: flex;
  flex-flow: row nowrap;
  justify-content: flex-start;
  align-content: center;
`;
const RowLayout = (props: RowLayoutProps) => {
  return (
    <RowList gap={props.gap}>
      {props.children?.map((child) => <RowItem>{child}</RowItem>)}
    </RowList>
  );
};
export default RowLayout;
