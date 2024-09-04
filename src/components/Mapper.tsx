import CollapsePane from "@containers/CollapsePane.tsx";
import Mapping from "@components/Mapping.tsx";
import { styled } from "solid-styled-components";
import Information from "@widgets/Information.tsx";
import Button from "@widgets/Button.tsx";
import IconEdit from "@icons/IconEdit.tsx";
import Input from "@widgets/Input.tsx";
import Select from "@widgets/Select.tsx";
import IconSave from "@icons/IconSave.tsx";
import { createSignal } from "solid-js";
import IconMapper from "@icons/IconMapper.tsx";
import IconPlus from "@icons/IconPlus.tsx";

interface MapperData {}

interface MapperViewProps {}

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

const MapperView = (props: MapperViewProps) => {
  return (
    <ZhuWrapper>
      <IconMapper size={16} />
      <Information label={"字段"} value={"props.data.field"} />
      <Information label={"表达式"} value={"props.data.expression"} />
      <Information label={"是否列表"} value={"props.data.selector"} />
      <Button label={"编辑"} onClick={() => {}} icon={<IconEdit size={16} />} />
    </ZhuWrapper>
  );
};

interface MapperFormProps {
  data: MapperData;
  onSave?: () => any;
}

const MapperForm = (props: MapperFormProps) => {
  return (
    <form>
      <ZhuWrapper>
        <IconMapper size={16} />
        <Input value={"props.data.field"} label={"字段"} placeholder={"字段"} />
        <Input
          value={"props.data.expression"}
          label={"表达式"}
          placeholder={"表达式"}
        />
        <Select
          value={"props.data.selector"}
          label={"是否列表"}
          options={[
            { label: "是", value: "Y" },
            { label: "否", value: "N" },
          ]}
        />
        <Button
          label={"保存"}
          onClick={props.onSave}
          icon={<IconSave size={16} />}
        />
      </ZhuWrapper>
    </form>
  );
};

interface MapperProps {}

const Wrapper = styled.div`
  display: flex;
  flex-flow: column nowrap;
  align-items: center;
  justify-content: flex-start;
  gap: 5px;
`;
const Mapper = (props: MapperProps) => {
  const [data, setData] = createSignal([1, 1, 1, 1, 1, 1]);
  const [editedIndex, setEditedIndex] = createSignal<Array<number>>([2, 3]);
  return (
    <Wrapper>
      {data().map((item, index) => (
        <CollapsePane
          title={
            editedIndex().includes(index) ? (
              <MapperView />
            ) : (
              <MapperForm data={{}} />
            )
          }
        >
          <Mapping
            data={[
              {
                field: "field1",
                selector: "css",
                expression: "html div",
                operator: "text",
                regex: "d+",
              },
              {
                field: "field2",
                selector: "css",
                expression: "html div",
                operator: "text",
                regex: "d+",
              },
              {
                field: "field3",
                selector: "css",
                expression: "html div",
                operator: "text",
                regex: "d+",
              },
              {
                field: "field4",
                selector: "css",
                expression: "html div",
                operator: "text",
                regex: "d+",
              },
              {
                field: "field5",
                selector: "css",
                expression: "html div",
                operator: "text",
                regex: "d+",
              },
            ]}
            editedIndex={[]}
          />
        </CollapsePane>
      ))}
      <button>
        <IconPlus size={16} />
        <span>添加</span>
        <IconMapper size={16} />
      </button>
    </Wrapper>
  );
};
export default Mapper;
