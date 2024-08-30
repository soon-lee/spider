import {Col, Form, Input, List, Popover, Row, Tag} from "antd";
import {styled} from "styled-components";
import {AimOutlined, EditOutlined, PlusOutlined} from "@ant-design/icons";
import {useState} from "react";

interface MappingData {
    field: string,
    expression: string,
    operation: string,
    regexp?: string
}

const ListWrapper = styled(List)`
    width: 100%;
`;

const ListItemWrapper = styled(List.Item)`
    width: 100%;
    padding: 0;
    margin: 0;
`;
const RowWrapper = styled(Row)`
    width: 100%;
    padding: 0;
    margin: 0;
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
const MappingView = (props: {
    field: string,
    expression: string,
    operation: string,
    regexp?: string,
    onEdit?: () => any
}) => {
    return (


        <RowWrapper gutter={10}>
            {/*<ApartmentOutlined/><ApiOutlined/>*/}
            <Col span={1}><Popover content="映射"><AimOutlined/></Popover></Col>
            <Col span={3}><Popover content="属性"><Tag>{props.field}</Tag></Popover></Col>
            <Col span={11}><Popover content="表达式"><Tag>{props.expression}</Tag></Popover></Col>
            <Col span={3}><Popover content="操作"><Tag>{props.operation}</Tag></Popover></Col>
            <Col span={5}><Popover content="正则表达式"><Tag>{props.regexp || "无"}</Tag></Popover></Col>
            <Col span={1}><Popover content="编辑"><EditOutlined onClick={props.onEdit}/></Popover></Col>
        </RowWrapper>
    );
}

const MappingForm = () => {
    const [form] = Form.useForm();
    return (
        <Form layout="inline" size="small"
              form={form}
              onFinish={() => {
              }}
        >
            <Form.Item
                name="field"
                label="属性"
                rules={[{required: true, message: '请输入属性!'}]}
            >
                <Input/>
            </Form.Item>
            <Form.Item
                name="expression"
                label="表达式"
                rules={[{required: true, message: '请输入表达式!'}]}
            >
                <Input placeholder="提取器使用解析器对应的表达式"/>
            </Form.Item>
            <Form.Item
                name="operation"
                label="操作"
                rules={[{required: true, message: '请选择操作!'}]}
            >
                <Input placeholder="文本提取试用text，属性提取使用@xxx"/>
            </Form.Item>
            <Form.Item
                name="regexp"
                label="正则"
                rules={[{required: true, message: '请输入正则!'}]}
            >
                <Input/>
            </Form.Item>
        </Form>
    );
}
const MappingItem = (props: { editing: boolean }) => {
    const [editing, setEditing] = useState(props.editing);
    if (editing) {
        return <MappingForm/>
    } else {
        return <MappingView field="name" expression="name" operation="@name" onEdit={() => setEditing(true)}/>
    }
}
const Mapping = () => {

    return (
        <ListWrapper size="small"
                     footer={<PlusOutlined/>}
                     dataSource={[1, 2, 3]}
                     renderItem={(item) => (
                         <ListItemWrapper>
                             <MappingItem editing={false}/>
                         </ListItemWrapper>
                     )}
        />
    );
}
export default Mapping;