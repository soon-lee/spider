import ColumnLayout from "@containers/ColumnLayout.tsx";
import CollapsePane from "@containers/CollapsePane.tsx";
import RowLayout from "@containers/RowLayout.tsx";
import MappingPane from "@components/MappingPane.tsx";

interface MapperPaneProps {
  field: string;
  expression: string;
  sequential: boolean;
  mappings: Array<string>;
}

const MapperPane = (props: MapperPaneProps) => {
  return (
    <CollapsePane
      title={
        <RowLayout>
          <label>{props.field}</label>
          <label>{props.expression}</label>
        </RowLayout>
      }
    >
      <ColumnLayout>
        {props.mappings.map((mapping) => (
          <MappingPane
            field={mapping}
            expression={mapping}
            operation={mapping}
            regex={mapping}
          />
        ))}
      </ColumnLayout>
    </CollapsePane>
  );
};
export default MapperPane;
