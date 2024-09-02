import { styled } from "solid-styled-components";
import { createSignal, For } from "solid-js";
import Select from "@widgets/Select.tsx";
import Input from "@widgets/Input.tsx";
import IconSave from "@icons/IconSave.tsx";
import Button from "@widgets/Button.tsx";
import Information from "@widgets/Information.tsx";
import IconEdit from "@icons/IconEdit.tsx";
import IconPlus from "@icons/IconPlus.tsx";
import IconMapping from "@icons/IconMapping.tsx";

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
      <IconMapping size={16} />
      <Information label={"字段"} value={"test"} />
      <Information label={"表达式"} value={"test"} />
      <Information label={"选择器"} value={"test"} />
      <Information label={"操作符"} value={"test"} />
      <Information label={"正则表达式"} value={"test"} />
      <Button
        label={"编辑"}
        onClick={props.onEdit}
        icon={<IconEdit size={16} />}
      />
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
        <IconMapping size={16} />
        <Input label={"字段"} placeholder={"字段"} />
        <Input label={"表达式"} placeholder={"表达式"} />
        <Select
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
        <Input label={"操作符"} placeholder={"操作符"} />
        <Input label={"正则表达式"} placeholder={"正则表达式"} />
        <Button
          label={"保存"}
          onClick={props.onSave}
          icon={<IconSave size={16} />}
        />
      </ZhuWrapper>
    </form>
  );
};
const Wrapper = styled.div``;
const Mapping = () => {
  const [edited, setEdited] = createSignal<boolean>(false);
  const [data, setData] = createSignal<any[]>([
    { edited: false },
    { edited: true },
    { edited: false },
  ]);
  return (
    <Wrapper>
      <For each={data()}>
        {(item) =>
          item.edited ? (
            <MappingView
              field={"test"}
              expression={"html div.pb pic#person"}
              operator={"@src"}
              regex={" "}
              onEdit={() => setEdited(false)}
            />
          ) : (
            <MappingForm onSave={() => setEdited(true)} />
          )
        }
      </For>
      <Button
        icon={<IconPlus size={16} />}
        label={"添加映射"}
        onClick={() => {
          setData([...data(), { edited: false }]);
        }}
      />
    </Wrapper>
  );
};
export default Mapping;
