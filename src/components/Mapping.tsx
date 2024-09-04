import { styled } from "solid-styled-components";
import { createEffect, createSignal } from "solid-js";
import Select from "@widgets/Select.tsx";
import Input from "@widgets/Input.tsx";
import Button from "@widgets/Button.tsx";
import Information from "@widgets/Information.tsx";
import IconEdit from "@icons/IconEdit.tsx";
import IconMapping from "@icons/IconMapping.tsx";
import IconSave from "@icons/IconSave.tsx";
import IconPlus from "@icons/IconPlus.tsx";

interface MappingData {
  field: string;
  selector: string;
  expression: string;
  operator: string;
  regex: string;
}

interface MappingViewProps {
  data: MappingData;
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
      <IconMapping size={16} />
      <Information label={"字段"} value={props.data.field} />
      <Information label={"表达式"} value={props.data.expression} />
      <Information label={"选择器"} value={props.data.selector} />
      <Information label={"操作符"} value={props.data.operator} />
      <Information label={"正则表达式"} value={props.data.regex} />
      <Button
        label={"编辑"}
        onClick={props.onEdit}
        icon={<IconEdit size={16} />}
      />
    </ZhuWrapper>
  );
};

interface MappingFormProps {
  data: MappingData;
  onSave?: () => any;
}

const MappingForm = (props: MappingFormProps) => {
  return (
    <form>
      <ZhuWrapper>
        <IconMapping size={16} />
        <Input value={props.data.field} label={"字段"} placeholder={"字段"} />
        <Input
          value={props.data.expression}
          label={"表达式"}
          placeholder={"表达式"}
        />
        <Select
          value={props.data.selector}
          label={"选择器"}
          options={[
            { label: "CSS", value: "css" },
            { label: "XPATH", value: "xpath" },
            {
              label: "JSON",
              value: "json",
            },
          ]}
        />
        <Input
          value={props.data.operator}
          label={"操作符"}
          placeholder={"操作符"}
        />
        <Input
          value={props.data.regex}
          label={"正则表达式"}
          placeholder={"正则表达式"}
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
const Wrapper = styled.div`
  button {
    display: inline-flex;
    flex-flow: row nowrap;
    justify-content: space-between;
    align-items: center;
  }
`;

interface MappingProps {
  data: MappingData[];
  editedIndex: Array<number>;
}

const Mapping = (props: MappingProps) => {
  const [data, setData] = createSignal(props.data);
  const [editedIndex, setEditedIndex] = createSignal<number[]>(
    props.editedIndex,
  );
  createEffect(() => console.log(editedIndex()));
  return (
    <Wrapper>
      {data().map((item, index) =>
        editedIndex().includes(index) ? (
          <MappingView
            data={item}
            onEdit={() =>
              setEditedIndex((prev) => prev.filter((_, i) => i !== index))
            }
          />
        ) : (
          <MappingForm
            data={item}
            onSave={() => setEditedIndex((prev) => [...prev, index])}
          />
        ),
      )}
      <button>
        <IconPlus size={16} />
        <span style={{ "margin-left": "5px" }}>添加</span>
        <IconMapping size={16} />
      </button>
    </Wrapper>
  );
};
export default Mapping;
