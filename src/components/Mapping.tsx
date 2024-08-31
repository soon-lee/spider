import { styled } from "solid-styled-components";
import TextIcon from "@icons/TextIcon.tsx";
import Tag from "@widgets/Tag.tsx";
import { createSignal } from "solid-js";

interface MappingViewProps {
  field: string;
  expression: string;
  operator: string;
  regex?: string;
  onEdit?: () => any;
}

const ZhuWrapper = styled.ul`
  margin: 0;
  padding: 5px;
  width: calc(100% - 10px);
  display: flex;
  flex-flow: row nowrap;
  justify-content: flex-start;
  align-items: center;
  gap: 5px;
  list-style: none;
`;
const JieWrapper = styled.li<{ percent: number }>`
  width: calc(${(props) => props.percent + "%"} - 5px);
  display: flex;
  flex-flow: row nowrap;
  justify-content: center;
  align-items: center;
`;
const MappingView = (props: MappingViewProps) => {
  return (
    <ZhuWrapper>
      <JieWrapper percent={5}>
        <TextIcon text={"M"} />
      </JieWrapper>
      <JieWrapper percent={10}>
        <Tag>{props.field}</Tag>
      </JieWrapper>
      <JieWrapper percent={50}>
        <Tag>{props.expression}</Tag>
      </JieWrapper>
      <JieWrapper percent={10}>
        <Tag>{props.operator}</Tag>
      </JieWrapper>
      <JieWrapper percent={20}>
        <Tag>{props.regex}</Tag>
      </JieWrapper>
      <JieWrapper percent={5}>
        <TextIcon text={"E"} onClick={props.onEdit} />
      </JieWrapper>
    </ZhuWrapper>
  );
};

interface MappingFormProps {
  onSave?: () => any;
}

const MappingForm = (props: MappingFormProps) => {
  return (
    <form>
      <ZhuWrapper>
        <JieWrapper percent={10}>
          <input />
        </JieWrapper>
        <JieWrapper percent={50}>
          <input />
        </JieWrapper>
        <JieWrapper percent={10}>
          <input />
        </JieWrapper>
        <JieWrapper percent={20}>
          <input />
        </JieWrapper>
        <JieWrapper percent={5}>
          <TextIcon text={"C"} onClick={props.onSave} />
        </JieWrapper>
      </ZhuWrapper>
    </form>
  );
};
const Wrapper = styled.div``;
const Mapping = () => {
  const [edited, setEdited] = createSignal<boolean>(false);
  return (
    <Wrapper>
      {edited() ? (
        <MappingView
          field={"test"}
          expression={"html div.pb pic#person"}
          operator={"@src"}
          regex={" "}
          onEdit={() => setEdited(false)}
        />
      ) : (
        <MappingForm onSave={() => setEdited(true)} />
      )}
      <TextIcon text={"+"} />
    </Wrapper>
  );
};
export default Mapping;
