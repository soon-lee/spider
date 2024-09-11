import { JSXElement } from "solid-js";
import { styled } from "solid-styled-components";

interface ColumnLayoutProps {
  children?: Array<JSXElement>;
  gap?: string;
}

const ColumnList = styled.ul<{ gap?: string }>`
  display: flex;
  flex-flow: column nowrap;
  align-content: flex-start;
  justify-content: center;
  gap: ${(props) => props.gap || "0"};
  list-style: none;
  padding: 0;
  margin: 0;
  width: 100%;
  height: 100%;

  > li:not(:first-of-type):before {
    content: "";
    border-top: 1px black solid;
  }
`;
const ColumnItem = styled.li`
  margin: 0;
  display: flex;
  flex-flow: column nowrap;
  align-content: flex-start;
  justify-content: center;
`;
const ColumnLayout = (props: ColumnLayoutProps) => {
  return (
    <ColumnList gap={props.gap}>
      {props.children?.map((child) => <ColumnItem>{child}</ColumnItem>)}
    </ColumnList>
  );
};
export default ColumnLayout;
