import {Button, Card, Form, Input, List, Radio} from "antd";
import {styled} from "styled-components";
import {PlusOutlined} from "@ant-design/icons";
import Meta from "antd/es/card/Meta";
import Mapping from "@components/prepare/Mapping.tsx";

const ListWrapper = styled(List)`
    width: 100%;
`;
const CardWrapper = styled(Card)`
    width: 100%;
`;

const Wrapper = styled.div`
    display: inline-flex;
    flex-flow: row nowrap;
    justify-content: flex-start;
    align-items: center;
    gap: 5px;
`;
const Item = styled.div`
    display: inline-flex;
    flex-flow: row nowrap;
    justify-content: flex-start;
    align-items: center;
    gap: 5px;
`;
const MappingView = (props: { field: string, expression: string, operation: string, regexp?: string }) => {
    return (
        <Wrapper>
            <Item>
                <span style={{fontWeight: "bold"}}>{props.field}</span>
                <span>{props.operation}</span>
                <span style={{fontWeight: "bold"}}>{props.expression}</span>
            </Item>
        </Wrapper>
    );
}
const MapperForm = () => {
    return (
        <Form layout="inline">
            <Form.Item label="属性" valuePropName="field">
                <Input/>
            </Form.Item>
            <Form.Item label="表达式" valuePropName="expressio">
                <Input placeholder="提取器使用解析器对应的表达式"/>
            </Form.Item>
            <Form.Item label="是否需要迭代" valuePropName="sequential">
                <Radio.Group options={[{label: '是', value: true}, {label: '否', value: false}]} optionType="button"
                             value={false}/>
            </Form.Item>
            <Form.Item label="正则" valuePropName="regexp">
                <Input/>
            </Form.Item>
        </Form>
    );
}

const MapperItem = (props: { editing: boolean }) => {
    if (props.editing) {
        return <MapperForm/>;
    } else {
        return <MappingView field="field" expression="expression" operation="operation" regexp="regexp"/>
    }
}
const Mapper = () => {
    const data = [1, 2, 3]
    return (
        <ListWrapper
            footer={<Button><PlusOutlined/></Button>}
            bordered
            dataSource={[1, 2, 3]}
            renderItem={(item) => (
                <List.Item>
                    <CardWrapper bordered={false}>
                        <Meta
                            title={<MapperItem editing={false}/>}
                            description={<Mapping/>}/>
                    </CardWrapper>
                </List.Item>
            )}
        />
    );
}
export default Mapper;