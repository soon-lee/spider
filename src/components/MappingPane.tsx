import CollapsePane from "@containers/CollapsePane.tsx";
import RowLayout from "@containers/RowLayout.tsx";
import Input from "@widgets/Input.tsx";

interface MappingPaneProps {
  field: string;
  expression: string;
  operation: string;
  regex: string;
}

const MappingPane = (props: MappingPaneProps) => {
  return (
    <CollapsePane
      title={
        <RowLayout>
          <>
            <label>属性：</label>
            <label>{props.field}</label>
          </>
          <>
            <label>表达式：</label>
            <label>{props.expression}</label>
          </>
          <>
            <label>操作：</label>
            <label>{props.operation}</label>
          </>
          <>
            <label>正则表达式：</label>
            <label>{props.regex}</label>
          </>
        </RowLayout>
      }
    >
      <form>
        <RowLayout>
          <Input label={"属性"} value={props.field} />
          <Input label={"表达式"} value={props.expression} />
          <Input label={"操作"} value={props.operation} />
          <Input label={"正则表达式"} value={props.regex} />
        </RowLayout>
      </form>
    </CollapsePane>
  );
};
export default MappingPane;
